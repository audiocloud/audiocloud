// TODO: input node

use async_trait::async_trait;

use crate::{DeviceBuffers, InputPads, Node, NodeInfo, OutputPads, PlayHead};

pub struct InputNode {}

#[async_trait]
impl Node for InputNode {
  async fn prepare_to_play(&mut self, play: PlayHead, accumulated_latency: usize) -> crate::Result {
    todo!()
  }

  async fn process(&mut self,
                   play: PlayHead,
                   _device_buffers: DeviceBuffers,
                   inputs: &mut InputPads,
                   outputs: &mut OutputPads)
                   -> crate::Result {
    todo!()
  }

  async fn stop(&mut self, play: PlayHead) -> crate::Result {
    todo!()
  }

  fn get_node_info(&self, play: PlayHead) -> NodeInfo {
    todo!()
  }
}
