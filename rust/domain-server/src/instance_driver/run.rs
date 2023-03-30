use std::time::{Duration, Instant};

use tokio::select;
use tokio::sync::mpsc;
use tokio::task::block_in_place;
use tokio::time::sleep;
use tracing::{info, trace, warn};

use api::driver::{InstanceDriverEvent, SetInstanceParameterRequest};

use super::{Driver, Result};

#[derive(Debug)]
pub enum InstanceDriverCommand {
  SetParameters(SetInstanceParameterRequest),
  Terminate,
}

pub async fn run_driver_server<Drv: Driver>(instance_id: String,
                                            config: Drv::Config,
                                            mut rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                                            tx_evt: mpsc::Sender<InstanceDriverEvent>)
                                            -> Result {
  let mut shared = block_in_place(|| Drv::create_shared())?;

  loop {
    while let Ok(InstanceDriverCommand::Terminate) = rx_cmd.try_recv() {
      return Ok(());
    }

    let mut instance = match block_in_place(|| Drv::new(&instance_id, &mut shared, config.clone())) {
      | Ok(instance) => instance,
      | Err(err) => {
        warn!(instance_id, %err, "Failed to create instance");
        sleep(Duration::from_secs(1)).await;
        continue;
      }
    };

    while instance.can_continue() {
      select! {
        Some(cmd) = rx_cmd.recv() => {
          info!(instance_id, ?cmd, "received");
          match cmd {
            | InstanceDriverCommand::SetParameters(SetInstanceParameterRequest { parameter, value, channel }) => {
              if let Err(err) = instance.set_parameter(&mut shared, &parameter, channel, value) {
                warn!(instance_id, %err, parameter, channel, value, "Failed to set parameter");
              }
            }
            | InstanceDriverCommand::Terminate => {
              return Ok(())
            }
          }
        }
        _ = sleep(Duration::from_millis(10)) => {
          let deadline = Instant::now() + Duration::from_millis(100);
          match block_in_place(|| instance.poll(&mut shared, deadline)) {
            | Err(err) => {
              warn!(instance_id, ?err, "Failed to poll instance");
            }
            | Ok(events) => {
              for event in events {
                trace!(instance_id, ?event, "generated event");
                tx_evt.send(event).await?;
              }
            }
          }
        }
      }
    }
  }
}
