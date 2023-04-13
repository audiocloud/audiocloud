use std::collections::HashMap;
use std::time::Duration;

use futures::StreamExt;
use governor::clock::QuantaClock;
use governor::middleware::NoOpMiddleware;
use governor::state::keyed::DashMapStateStore;
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::{select, spawn};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamMap;
use tracing::error;

use api::instance::driver::events::{instance_driver_events, InstanceDriverEvent};
use api::instance::driver::requests::{
  set_instance_parameters_request, SetInstanceParameter, SetInstanceParameterResponse, SetInstanceParametersRequest,
};
use api::instance::spec::InstanceSpec;
use api::instance::{InstanceConnectionState, InstancePowerState};
use api::BucketKey;

use crate::instance::driver::scripting::ScriptingEngine;
use crate::nats::{Nats, RequestStream, WatchStream};

use super::run_driver::{run_driver_server, InstanceDriverCommand};
use super::Result;

pub struct DriverService {
  nats:                   Nats,
  host:                   String,
  instances:              HashMap<String, InstanceDriver>,
  watch_instance_specs:   WatchStream<InstanceSpec>,
  watch_instance_power:   WatchStream<InstancePowerState>,
  instance_driver_events: StreamMap<String, ReceiverStream<InstanceDriverEvent>>,
  set_parameter_req:      StreamMap<String, RequestStream<Vec<SetInstanceParameter>, SetInstanceParameterResponse>>,
  scripting_engine:       ScriptingEngine,
  respawn_limiter:        RateLimiter<String, DashMapStateStore<String>, QuantaClock, NoOpMiddleware>,
}

impl DriverService {
  pub fn new(nats: Nats, scripting_engine: ScriptingEngine, host: String) -> Self {
    let watch_instance_specs = nats.instance_spec.watch_all();
    let watch_instance_power = nats.instance_power_state.watch_all();
    let instance_driver_events = StreamMap::new();
    let set_parameter_req = StreamMap::new();
    let instances = HashMap::new();
    let respawn_limiter = RateLimiter::new(Quota::per_minute(nonzero!(5u32)).allow_burst(nonzero!(10u32)),
                                           DashMapStateStore::new(),
                                           &QuantaClock::default());

    Self { nats,
           host,
           instances,
           watch_instance_specs,
           watch_instance_power,
           instance_driver_events,
           set_parameter_req,
           scripting_engine,
           respawn_limiter }
  }

  pub async fn run(mut self) -> Result {
    loop {
      select! {
        Some((instance_id, maybe_new_spec)) = self.watch_instance_specs.next() => {
          self.handle_maybe_instance_spec(instance_id, maybe_new_spec).await;
        },
        Some((instance_id, maybe_new_power)) = self.watch_instance_power.next() => {
          self.handle_maybe_instance_power(instance_id, maybe_new_power).await;
        },
        Some((instance_id, event)) = self.instance_driver_events.next(), if !self.instance_driver_events.is_empty() => {
          self.handle_instance_driver_event(instance_id, event).await;
        },
        Some((instance_id, (_, request, response))) = self.set_parameter_req.next(), if !self.set_parameter_req.is_empty() => {
          self.handle_set_parameter_request(instance_id, request, response);
        },
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
          self.respawn_instance_drivers().await;
        },
        else => break
      }
    }
    Ok(())
  }

  async fn handle_maybe_instance_power(&mut self, instance_id: String, maybe_new_power: Option<InstancePowerState>) {
    self.instances.entry(instance_id).or_default().power_state = maybe_new_power;
    self.respawn_instance_drivers().await;
  }

  async fn handle_maybe_instance_spec(&mut self, instance_id: String, maybe_new_spec: Option<InstanceSpec>) {
    let entry = self.instances.entry(instance_id).or_default();
    entry.prev_spec = entry.spec.take();
    entry.spec = maybe_new_spec;
    self.respawn_instance_drivers().await;
  }

  async fn respawn_instance_drivers(&mut self) {
    for (instance_id, driver) in self.instances.iter_mut() {
      if let Some(spec) = &driver.spec {
        let spec_changed = &driver.prev_spec != &driver.spec;
        let driver_running = driver.running
                                   .as_ref()
                                   .map(|running| !running.handle.is_finished())
                                   .unwrap_or(false);

        if !driver_running || spec_changed {
          drop(driver.running.take());

          let can_respawn = &spec.host == &self.host;

          let driver_needs_power = spec.power.as_ref().map(|power| power.driver_needs_power).unwrap_or(false);

          let can_respawn = can_respawn
                            && if driver_needs_power {
                              driver.power_state.as_ref().map(InstancePowerState::is_on).unwrap_or(false)
                            } else {
                              true
                            }
                            && can_respawn;

          let can_respawn = can_respawn && self.respawn_limiter.check_key(&instance_id).is_ok();

          if can_respawn {
            let (tx_cmd, rx_cmd) = mpsc::channel(0xff);
            let (tx_evt, rx_evt) = mpsc::channel(0xff);

            let handle = spawn(run_driver_server(instance_id.clone(),
                                                 spec.driver.clone(),
                                                 self.scripting_engine.clone(),
                                                 rx_cmd,
                                                 tx_evt));

            self.instance_driver_events.insert(instance_id.clone(), ReceiverStream::new(rx_evt));
            self.set_parameter_req.insert(instance_id.clone(),
                                          self.nats.serve_requests(set_instance_parameters_request(&instance_id)));

            driver.running = Some(RunningInstanceDriver { tx_cmd, handle });
            driver.prev_spec = driver.spec.clone();
          }
        }
      }
    }
  }

  async fn handle_instance_driver_event(&mut self, instance_id: String, event: InstanceDriverEvent) {
    match event {
      | InstanceDriverEvent::Connected { connected } => {
        self.instance_connection_changed(instance_id.clone(), connected).await;
      }
      | _ => {}
    }

    if let Err(err) = self.nats.publish_event(instance_driver_events(&instance_id), event).await {
      error!(?err, "Failed to publish driver event: {err}");
    }
  }

  async fn instance_connection_changed(&mut self, instance_id: String, connected: bool) {
    use InstanceConnectionState::*;

    let connection_state = if connected { Connected } else { Disconnected };

    let _ = self.nats
                .instance_connection_state
                .put(BucketKey::new(&instance_id), connection_state)
                .await;
  }

  fn handle_set_parameter_request(&self,
                                  instance_id: String,
                                  changes: Vec<SetInstanceParameter>,
                                  response: oneshot::Sender<SetInstanceParameterResponse>) {
    if let Some(driver) = self.instances
                              .get(&instance_id)
                              .and_then(|instance_driver| instance_driver.running.as_ref())
    {
      let _ = driver.tx_cmd
                    .try_send(InstanceDriverCommand::SetParameters(SetInstanceParametersRequest { instance_id, changes }, response));
    } else {
      let _ = response.send(SetInstanceParameterResponse::NotConnected);
    }
  }
}

#[derive(Default)]
struct InstanceDriver {
  prev_spec:   Option<InstanceSpec>,
  spec:        Option<InstanceSpec>,
  power_state: Option<InstancePowerState>,
  running:     Option<RunningInstanceDriver>,
}

struct RunningInstanceDriver {
  tx_cmd: mpsc::Sender<InstanceDriverCommand>,
  handle: JoinHandle<Result>,
}

impl Drop for RunningInstanceDriver {
  fn drop(&mut self) {
    if !self.handle.is_finished() {
      let _ = self.tx_cmd.try_send(InstanceDriverCommand::Terminate);
    }
  }
}
