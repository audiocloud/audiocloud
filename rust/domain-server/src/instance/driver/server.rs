use std::collections::HashMap;
use std::time::Duration;

use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::{select, spawn};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamMap;
use tracing::error;

use api::instance::driver::events::{instance_driver_events, InstanceDriverEvent};
use api::instance::spec::InstanceSpec;
use api::Events;

use crate::instance::driver::scripting::ScriptingEngine;
use crate::nats::{Nats, WatchStream};

use super::run_driver::{run_driver_server, InstanceDriverCommand};
use super::Result;

pub struct DriverService {
  nats:                 Nats,
  host:                 String,
  drivers:              HashMap<String, DriverServer>,
  watch_instance_specs: WatchStream<InstanceSpec>,
  driver_events:        StreamMap<String, ReceiverStream<InstanceDriverEvent>>,
  scripting_engine:     ScriptingEngine,
}

impl DriverService {
  pub fn new(nats: Nats, scripting_engine: ScriptingEngine, host: String) -> Self {
    let watch_instance_specs = nats.instance_spec.watch_all();
    let driver_events = StreamMap::new();
    let drivers = HashMap::new();

    Self { nats,
           host,
           drivers,
           watch_instance_specs,
           driver_events,
           scripting_engine }
  }

  pub async fn run(mut self) -> Result {
    loop {
      select! {
        Some((instance_id, maybe_new_spec)) = self.watch_instance_specs.next() => {
          if let Some(new_spec) = maybe_new_spec {
            if &new_spec.host == &self.host {
              self.add_driver_if_changed(instance_id, new_spec);
            } else {
              self.remove_driver(&instance_id);
            }
          } else {
            self.remove_driver(&instance_id);
          }
        },
        Some((instance_id, event)) = self.driver_events.next() => {
          self.handle_driver_event(instance_id, event).await;
        },
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
          self.redeploy_failed_drivers();
        },
        else => break
      }
    }

    Ok(())
  }

  fn add_driver_if_changed(&mut self, instance_id: String, new_spec: InstanceSpec) {
    if let Some(driver) = self.drivers.get(&instance_id) {
      if &driver.spec.driver != &new_spec.driver {
        self.remove_driver(&instance_id);
        self.add_driver(instance_id, new_spec);
      }
    } else {
      self.add_driver(instance_id, new_spec);
    }
  }

  fn add_driver(&mut self, instance_id: String, spec: InstanceSpec) {
    let (tx_cmd, rx_cmd) = mpsc::channel(0xff);
    let (tx_evt, rx_evt) = mpsc::channel(0xff);

    let handle = spawn(run_driver_server(instance_id.clone(),
                                         spec.driver.clone(),
                                         self.scripting_engine.clone(),
                                         rx_cmd,
                                         tx_evt));

    let events_subject = instance_driver_events(&instance_id);

    self.driver_events.insert(instance_id.clone(), ReceiverStream::new(rx_evt));
    self.drivers.insert(instance_id.clone(), DriverServer { spec,
                                                            tx_cmd,
                                                            handle,
                                                            events_subject });
  }

  fn redeploy_failed_drivers(&mut self) {
    let mut failed_drivers = vec![];
    self.drivers.retain(|instance_id, driver| {
                  if driver.handle.is_finished() {
                    failed_drivers.push((instance_id.clone(), driver.spec.clone()));
                    false
                  } else {
                    true
                  }
                });

    for (instance_id, spec) in failed_drivers {
      self.add_driver(instance_id, spec);
    }
  }

  async fn handle_driver_event(&mut self, instance_id: String, event: InstanceDriverEvent) {
    if let Err(err) = self.nats.publish_event(instance_driver_events(instance_id), event).await {
      error!(?err, "Failed to publish driver event: {err}");
    }
  }

  fn remove_driver(&mut self, instance_id: &str) {
    if let Some(driver) = self.drivers.remove(instance_id) {
      let _ = driver.tx_cmd.try_send(InstanceDriverCommand::Terminate);
    }
  }
}

pub struct DriverServer {
  spec:           InstanceSpec,
  tx_cmd:         mpsc::Sender<InstanceDriverCommand>,
  handle:         JoinHandle<super::Result>,
  events_subject: Events<InstanceDriverEvent>,
}