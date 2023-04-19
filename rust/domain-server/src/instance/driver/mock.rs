use futures::StreamExt;
use tracing::{debug, trace};

use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;

use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::driver::scripting::ScriptingEngine;

use super::Result;

// TODO: in the future we will use the scripting engine

pub async fn run_mock_driver(_instance_id: String,
                             rx_cmd: flume::Receiver<InstanceDriverCommand>,
                             tx_evt: flume::Sender<InstanceDriverEvent>,
                             _scripting_engine: ScriptingEngine)
                             -> Result {
  let _ = tx_evt.send_async(InstanceDriverEvent::Connected { connected: true }).await;

  while let Ok(cmd) = rx_cmd.recv_async().await {
    match cmd {
      | InstanceDriverCommand::SetParameters(params, ok) => {
        for change in params.changes {
          trace!(channel = change.channel, value = change.value, "Set");
        }

        let _ = ok.send_async(SetInstanceParameterResponse::Success).await;
      }
      | InstanceDriverCommand::Terminate => {
        debug!("Terminate");
        break;
      }
    }
  }

  Ok(())
}
