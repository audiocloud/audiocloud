use anyhow::Error;

use api::task::graph::NodeId;

use crate::player::GraphPlayer;

impl GraphPlayer {
  pub(crate) fn handle_error(&mut self, err: Error) {
    // TODO: 1 ... unsub all devices
    // TODO: 2 ... send self state to error
  }

  pub(crate) fn handle_node_error(&mut self, node_id: NodeId, err: Error) {
    // TODO: ...
  }
}
