use tokio::sync::mpsc;
use tracing::{debug, trace};

use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::SetInstanceParameterResponse;

use crate::instance::driver::run_driver::InstanceDriverCommand;
use crate::instance::driver::scripting::ScriptingEngine;

use super::Result;

// TODO: in the future we will use the scripting engine

pub async fn run_mock_driver(_instance_id: String,
                             mut rx_cmd: mpsc::Receiver<InstanceDriverCommand>,
                             tx_evt: mpsc::Sender<InstanceDriverEvent>,
                             _scripting_engine: ScriptingEngine)
                             -> Result {
  let _ = tx_evt.send(InstanceDriverEvent::Connected { connected: true }).await;

  while let Some(cmd) = rx_cmd.recv().await {
    match cmd {
      | InstanceDriverCommand::SetParameters(params, ok) => {
        for change in params.changes {
          trace!(channel = change.channel, value = change.value, "Set");
        }

        let _ = ok.send(SetInstanceParameterResponse::Success);
      }
      | InstanceDriverCommand::Terminate => {
        debug!("Terminate");
        break;
      }
    }
  }

  Ok(())
}
