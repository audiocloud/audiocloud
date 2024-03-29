use std::collections::HashMap;
use std::time::Duration;

use futures::StreamExt;
use governor::clock::QuantaClock;
use governor::middleware::NoOpMiddleware;
use governor::state::keyed::DashMapStateStore;
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use tokio::task::JoinHandle;
use tokio::{select, spawn};
use tokio_stream::StreamMap;
use tracing::{error, info, warn};

use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::{SetInstanceParameter, SetInstanceParameterResponse, SetInstanceParametersRequest};
use api::instance::spec::InstanceSpec;
use api::instance::{InstanceConnectionState, InstancePowerState};

use crate::instance::driver::flume_utils::{flume_stream, from_oneshot, FlumeStream};
use crate::instance::driver::scripting::ScriptingEngine;
use crate::nats::{RequestStream, WatchStream};
use crate::service::Service;

use super::run_driver::{run_driver_server, InstanceDriverCommand};
use super::Result;

pub struct DriverService {
  service:                Service,
  host:                   String,
  instances:              HashMap<String, InstanceDriver>,
  watch_instance_specs:   WatchStream<String, InstanceSpec>,
  watch_instance_power:   WatchStream<String, InstancePowerState>,
  instance_driver_events: StreamMap<String, FlumeStream<InstanceDriverEvent>>,
  set_parameter_req:      StreamMap<String, RequestStream<Vec<SetInstanceParameter>, SetInstanceParameterResponse>>,
  scripting_engine:       ScriptingEngine,
  respawn_limiter:        RateLimiter<String, DashMapStateStore<String>, QuantaClock, NoOpMiddleware>,
}

impl DriverService {
  pub fn new(service: Service, scripting_engine: ScriptingEngine, host: String) -> Self {
    let watch_instance_specs = service.watch_all_instance_specs();
    let watch_instance_power = service.watch_all_instance_power_states();
    let instance_driver_events = StreamMap::new();
    let set_parameter_req = StreamMap::new();
    let instances = HashMap::new();
    let respawn_limiter = RateLimiter::new(Quota::per_minute(nonzero!(5u32)).allow_burst(nonzero!(10u32)),
                                           DashMapStateStore::new(),
                                           &QuantaClock::default());

    Self { service,
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
    use tokio::time::sleep;

    loop {
      select! {
        Some((instance_id, event)) = self.instance_driver_events.next(), if !self.instance_driver_events.is_empty() => {
          self.handle_instance_driver_event(instance_id, event).await;
        },
        Some((instance_id, (_, request, response))) = self.set_parameter_req.next(), if !self.set_parameter_req.is_empty() => {
          self.handle_set_parameter_request(instance_id, request, from_oneshot(response));
        },
        Some((instance_id, maybe_new_spec)) = self.watch_instance_specs.next() => {
          self.handle_maybe_instance_spec(instance_id, maybe_new_spec).await;
        },
        Some((instance_id, maybe_new_power)) = self.watch_instance_power.next() => {
          self.handle_maybe_instance_power(instance_id, maybe_new_power).await;
        },
        _ = sleep(Duration::from_secs(1)) => {
          self.respawn_instance_drivers().await;
        },
        _ = sleep(Duration::from_secs(10)) => {
          self.update_connection_state().await;
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

  async fn update_connection_state(&mut self) {
    for (instance_id, instance) in &self.instances {
      let Some(spec) = instance.spec.as_ref() else { continue; };
      if &spec.host != &self.host {
        continue;
      }

      let is_running = instance.running
                               .as_ref()
                               .map(|running| running.handle.is_finished() && running.received_connected)
                               .unwrap_or(false);

      let nats = self.service.clone();
      let instance_id = instance_id.clone();

      spawn(async move {
        let connection_state = if is_running {
          InstanceConnectionState::Connected
        } else {
          InstanceConnectionState::Disconnected
        };

        nats.set_instance_connection_state(&instance_id, connection_state).await
      });
    }
  }

  async fn respawn_instance_drivers(&mut self) {
    for (instance_id, driver) in self.instances.iter_mut() {
      if let Some(spec) = &driver.spec {
        let spec_changed = &driver.prev_spec != &driver.spec;

        if spec_changed {
          if let Some(running) = driver.running.as_mut() {
            if running.terminate_requested > 10 {
              warn!("Instance driver {} is not terminating, killing it", instance_id);
              running.handle.abort();
            } else {
              running.terminate_requested += 1;
              let _ = running.tx_cmd.try_send(InstanceDriverCommand::Terminate);
            }
          }
        }

        let driver_running = driver.running
                                   .as_ref()
                                   .map(|running| !running.handle.is_finished())
                                   .unwrap_or(false);

        if !driver_running {
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
            let (tx_cmd, rx_cmd) = flume::bounded(0xff);
            let (tx_evt, rx_evt) = flume::bounded(0xff);

            let handle = spawn(run_driver_server(instance_id.clone(),
                                                 spec.driver.clone(),
                                                 self.scripting_engine.clone(),
                                                 rx_cmd,
                                                 tx_evt));

            let received_connected = false;
            let terminate_requested = 0;

            self.instance_driver_events.insert(instance_id.clone(), flume_stream(rx_evt));
            self.set_parameter_req.insert(instance_id.clone(),
                                          self.service.serve_set_instance_parameters_requests(&instance_id));

            driver.running = Some(RunningInstanceDriver { tx_cmd,
                                                          handle,
                                                          received_connected,
                                                          terminate_requested });
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

    if let Err(err) = self.service.publish_instance_driver_event(&instance_id, event).await {
      error!(?err, "Failed to publish driver event: {err}");
    }
  }

  async fn instance_connection_changed(&mut self, instance_id: String, connected: bool) {
    use InstanceConnectionState::*;

    let connection_state = if connected { Connected } else { Disconnected };

    info!(instance_id, connected = ?connection_state, "Instance connection state changed: {instance_id}->{connection_state}");

    if let Some(running) = self.instances.get_mut(&instance_id).and_then(|instance| instance.running.as_mut()) {
      running.received_connected = connected;
    }

    let _ = self.service.set_instance_connection_state(&instance_id, connection_state).await;
  }

  fn handle_set_parameter_request(&mut self,
                                  instance_id: String,
                                  changes: Vec<SetInstanceParameter>,
                                  response: flume::Sender<SetInstanceParameterResponse>) {
    if let Some(driver) = self.instances
                              .get_mut(&instance_id)
                              .and_then(|instance_driver| instance_driver.running.as_mut())
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
  tx_cmd:              flume::Sender<InstanceDriverCommand>,
  handle:              JoinHandle<Result>,
  received_connected:  bool,
  terminate_requested: u32,
}

impl Drop for RunningInstanceDriver {
  fn drop(&mut self) {
    if !self.handle.is_finished() {
      let _ = self.tx_cmd.try_send(InstanceDriverCommand::Terminate);
    }
  }
}
