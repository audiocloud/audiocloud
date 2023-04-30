use std::time::Instant;

use api::task::player::PlayHead;

use crate::buffer::{fill_slice, DeviceBuffers};
use crate::juce::JuceAudioReader;
use crate::{Node, NodeInfo, Result};

const BUF_SIZE: usize = 1 << 12;

struct JuceSourceReaderNode {
  buf0:   [f32; BUF_SIZE],
  buf1:   [f32; BUF_SIZE],
  buf2:   [f32; BUF_SIZE],
  buf3:   [f32; BUF_SIZE],
  buf4:   [f32; BUF_SIZE],
  buf5:   [f32; BUF_SIZE],
  buf6:   [f32; BUF_SIZE],
  buf7:   [f32; BUF_SIZE],
  info:   NodeInfo,
  reader: JuceAudioReader,
}

macro_rules! buffers {
  ($x:ident) => {
    [&mut $x.buf0[..],
     &mut $x.buf1[..],
     &mut $x.buf2[..],
     &mut $x.buf3[..],
     &mut $x.buf4[..],
     &mut $x.buf5[..],
     &mut $x.buf6[..],
     &mut $x.buf7[..]]
  };
}

impl JuceSourceReaderNode {
  pub fn new(path: &str) -> crate::Result<Self> {
    let juce_reader = JuceAudioReader::new(path)?;
    let node_info = NodeInfo { num_outputs: juce_reader.get_channel_count() as usize,
                               ..Default::default() };

    Ok(Self { buf0:   [0.0; BUF_SIZE],
              buf1:   [0.0; BUF_SIZE],
              buf2:   [0.0; BUF_SIZE],
              buf3:   [0.0; BUF_SIZE],
              buf4:   [0.0; BUF_SIZE],
              buf5:   [0.0; BUF_SIZE],
              buf6:   [0.0; BUF_SIZE],
              buf7:   [0.0; BUF_SIZE],
              info:   node_info,
              reader: juce_reader, })
  }

  pub fn set_source(&mut self, path: &str) -> Result {
    self.reader = JuceAudioReader::new(path)?;
    self.info = NodeInfo { num_outputs: self.reader.get_channel_count() as usize,
                           ..Default::default() };

    Ok(())
  }
}

impl Node for JuceSourceReaderNode {
  fn get_node_info(&self, play: PlayHead) -> NodeInfo {
    NodeInfo { num_outputs: self.reader.get_channel_count() as usize,
               ..Default::default() }
  }

  fn prepare_to_play(&mut self, play: PlayHead, _accumulated_latency: usize) -> crate::Result {
    let mut buffers = buffers!(self);

    self.reader
        .read_samples(&mut buffers[..self.info.num_outputs], play.position as i64, play.buffer_size as i32);

    Ok(())
  }

  fn process(&mut self,
             play: PlayHead,
             _device_buffers: DeviceBuffers,
             _inputs: &[&[f64]],
             outputs: &mut [&mut [f64]],
             _deadline: Instant)
             -> Result {
    let mut buffers = buffers!(self);

    let channels = self.info.num_outputs;
    let buffer_size = play.buffer_size as usize;

    self.reader
        .read_samples(&mut buffers[..channels], play.position as i64, play.buffer_size as i32);

    for (i, output) in outputs.iter_mut().enumerate() {
      fill_slice(output, buffers[i].iter().take(buffer_size).map(|f| *f as f64));
    }

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use std::time::{Duration, Instant};

  use api::task::player::PlayHead;

  use crate::buffer::DeviceBuffers;
  use crate::juce::JuceAudioReader;
  use crate::juce_source_reader_node::JuceSourceReaderNode;
  use crate::Node;

  #[test]
  fn test_io_perf() {
    const BUF_SIZE: usize = 1 << 8;
    let mut buffer0 = [0.0; BUF_SIZE];

    let src = JuceAudioReader::new("../../test-files/StarWars3.wav").expect("Failed to open file");

    let start = Instant::now();
    let length = src.get_total_length();
    let mut read = 0;
    let duration = Duration::from_secs(2);
    let channels = src.get_channel_count() as usize;
    let mut pos = 0;

    while start.elapsed() < duration {
      let mut buffers = [&mut buffer0[..]];
      src.read_samples(&mut buffers[..channels], pos, BUF_SIZE as i32);
      read += 1;
      pos += BUF_SIZE as i64;

      if pos >= length {
        pos = 0;
      }
    }

    println!("Read {} blocks in {:?}", read, start.elapsed());
    println!("Equals {0:1} MB/s",
             (read as f64 * BUF_SIZE as f64 * 2 as f64 / (1024f64 * 1024f64)) / start.elapsed().as_secs_f64());
  }

  #[test]
  fn test_node_perf() {
    let mut node = JuceSourceReaderNode::new("../../test-files/StarWars3.wav").expect("Failed to open file");
    let mut play_head = PlayHead::default();
    play_head.play_region.end = 60000;
    play_head.play_region.looping = true;

    node.prepare_to_play(play_head, 0).expect("Failed to prepare to play");
    let device_buffers = DeviceBuffers::default();

    let start = Instant::now();
    let duration = Duration::from_secs(5);

    let mut buf0 = [0.0; 1024];
    let mut buf1 = [0.0; 1024];

    while start.elapsed() < duration {
      let deadline = Instant::now() + Duration::from_secs(1);

      node.process(play_head, device_buffers, &[], &mut [&mut buf0, &mut buf1], deadline)
          .expect("Failed to process");
    }

    println!("Processed {} blocks in {:?}", play_head.play_region.end / 1024, start.elapsed());
  }
}
