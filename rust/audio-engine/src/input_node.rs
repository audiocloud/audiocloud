use async_trait::async_trait;
use lru::LruCache;

use crate::{DeviceBuffers, InputPads, Node, NodeInfo, OutputPads, PlayHead};

pub struct InputNode {
  cache:                LruCache<u64, Vec<f64>>,
  pre_cache_cell_count: usize,
}

impl InputNode {
  async fn read_cell(&mut self, position: u64) {
    if self.cache.contains(&position) {
      return;
    }
  }
}

#[async_trait]
impl Node for InputNode {
  fn get_node_info(&self, play: PlayHead) -> NodeInfo {
    todo!()
  }

  async fn prepare_to_play(&mut self, play: PlayHead, accumulated_latency: usize) -> crate::Result {
    self.cache.clear();

    let mut play = play;
    for i in 0..self.pre_cache_cell_count {
      self.read_cell(play.position).await;
      play = play.advance_position();
    }

    Ok(())
  }

  async fn process(&mut self,
                   play: PlayHead,
                   _device_buffers: DeviceBuffers,
                   _inputs: &mut InputPads,
                   outputs: &mut OutputPads)
                   -> crate::Result {
  }

  async fn stop(&mut self, play: PlayHead) -> crate::Result {
    todo!()
  }
}
