use std::time::{Duration, Instant};

use tokio::select;
use tokio::sync::mpsc;
use tokio::task::block_in_place;
use tokio::time::sleep;
use tracing::{debug, info, instrument, trace, warn};

use api::driver::{InstanceDriverEvent, SetInstanceParameterRequest};

use super::{Driver, Result};

#[derive(Debug)]
pub enum InstanceDriverCommand {
  SetParameters(SetInstanceParameterRequest),
  Terminate,
}

#[instrument(err, skip(config, rx_cmd, tx_evt))]
pub async fn run_driver_server<Drv: Driver>(instance_id: String,
                                            config: Drv::Config,
                                            mut rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                                            tx_evt: mpsc::Sender<InstanceDriverEvent>)
                                            -> Result {
  info!(?config, " -- Starting driver server -- ");

  let mut shared = block_in_place(|| Drv::create_shared())?;
  debug!("Created driver shared data");

  loop {
    while let Ok(InstanceDriverCommand::Terminate) = rx_cmd.try_recv() {
      debug!("Requested termination before driver is ready, exiting");
      return Ok(());
    }

    let mut instance = match block_in_place(|| Drv::new(&instance_id, &mut shared, config.clone())) {
      | Ok(instance) => instance,
      | Err(err) => {
        warn!(%err, "Failed to create instance");
        sleep(Duration::from_secs(120)).await;
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
                warn!(%err, parameter, channel, value, "Failed to set parameter");
              }
            }
            | InstanceDriverCommand::Terminate => {
              return Ok(())
            }
          }
        }
        _ = sleep(Duration::from_millis(20)) => {
          let deadline = Instant::now() + Duration::from_millis(20);
          match block_in_place(|| instance.poll(&mut shared, deadline)) {
            | Err(err) => {
              warn!(?err, "Failed to poll instance");
            }
            | Ok(events) => {
              for event in events {
                // trace!(?event, "generated event");
                tx_evt.send(event).await?;
              }
            }
          }
        }
      }
    }
  }
}
