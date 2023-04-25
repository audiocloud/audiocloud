use std::num::NonZeroUsize;

use anyhow::anyhow;
use lru::LruCache;

use crate::juce::JuceAudioReader;
use crate::{DeviceBuffers, InputPads, Node, NodeInfo, OutputPads, PlayHead};

use super::Result;

pub struct InputNode {
  cache:                LruCache<u64, Vec<f64>>,
  pre_cache_cell_count: usize,
  info:                 NodeInfo,
  length:               u64,
  native_sample_rate:   u32,
}

impl InputNode {
  pub fn new(path: String, native_sample_rate: u32) -> Result<Self> {
    let reader = JuceAudioReader::new(&path)?;

    if reader.get_sample_rate() as u32 != native_sample_rate {
      return Err(anyhow!("Input file sample rate ({}) does not match native sample rate ({})",
                         reader.get_sample_rate(),
                         native_sample_rate));
    }

    let length = reader.get_total_length();
    let mut info = NodeInfo::default();

    info.num_outputs = reader.get_channel_count() as usize;

    Ok(Self { cache:                { LruCache::new(NonZeroUsize::new(16).unwrap()) },
              pre_cache_cell_count: { 16 },
              info:                 { info },
              length:               { length as u64 },
              native_sample_rate:   { native_sample_rate }, })
  }

  fn read_cell(&mut self, position: u64) {
    if self.cache.contains(&position) {
      return;
    }
  }
}

impl Node for InputNode {
  fn get_node_info(&self, play: PlayHead) -> NodeInfo {
    self.info
  }

  fn prepare_to_play(&mut self, play: PlayHead, accumulated_latency: usize) -> crate::Result {
    self.cache.clear();

    let mut play = play;
    for i in 0..self.pre_cache_cell_count {
      self.read_cell(play.position);
      play = play.advance_position();
    }

    Ok(())
  }

  fn process(&mut self,
             play: PlayHead,
             _device_buffers: DeviceBuffers,
             _inputs: &mut InputPads,
             outputs: &mut OutputPads)
             -> crate::Result {
    Ok(())
  }

  fn stop(&mut self, play: PlayHead) -> crate::Result {
    todo!()
  }
}
