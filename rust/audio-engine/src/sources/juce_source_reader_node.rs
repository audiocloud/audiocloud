use std::time::Instant;

use anyhow::anyhow;
use r8brain_rs::PrecisionProfile;

use api::task::player::PlayHead;

use crate::buffer::{cast_sample_ref, fill_slice, DevicesBuffers, NodeBuffers};
use crate::events::{make_report, slice_peak_level_db};
use crate::juce::JuceAudioReader;
use crate::{Node, NodeEvent, NodeInfo, Result};

use super::reports;

const BUF_SIZE: usize = 512;
const PRELOAD_BUFFER_COUNT: usize = 32;

pub struct JuceSourceReaderNode {
  buf0:       [f32; BUF_SIZE],
  buf1:       [f32; BUF_SIZE],
  buf2:       [f32; BUF_SIZE],
  buf3:       [f32; BUF_SIZE],
  buf4:       [f32; BUF_SIZE],
  buf5:       [f32; BUF_SIZE],
  buf6:       [f32; BUF_SIZE],
  buf7:       [f32; BUF_SIZE],
  info:       NodeInfo,
  reader:     JuceAudioReader,
  resamplers: Option<Vec<r8brain_rs::ResamplerQueue>>,
  play_head:  PlayHead,
}

unsafe impl Send for JuceSourceReaderNode {}
unsafe impl Sync for JuceSourceReaderNode {}

impl JuceSourceReaderNode {
  pub fn new(path: &str, play_head: PlayHead, num_channels: usize) -> Result<Self> {
    let juce_reader = JuceAudioReader::new(path)?;
    let source_num_channels = juce_reader.get_channel_count() as usize;
    let node_info = NodeInfo { num_outputs: num_channels,
                               reports: reports::create(source_num_channels),
                               ..Default::default() };

    let r8b_resampler = Self::make_resamplers(play_head.sample_rate, source_num_channels, juce_reader.get_sample_rate() as u32);

    Ok(Self { buf0:       [0.0; BUF_SIZE],
              buf1:       [0.0; BUF_SIZE],
              buf2:       [0.0; BUF_SIZE],
              buf3:       [0.0; BUF_SIZE],
              buf4:       [0.0; BUF_SIZE],
              buf5:       [0.0; BUF_SIZE],
              buf6:       [0.0; BUF_SIZE],
              buf7:       [0.0; BUF_SIZE],
              info:       node_info,
              resamplers: r8b_resampler,
              reader:     juce_reader,
              play_head:  PlayHead::default(), })
  }

  fn make_resamplers(native_sample_rate: u32, source_num_channels: usize, source_rate: u32) -> Option<Vec<r8brain_rs::ResamplerQueue>> {
    assert!(source_num_channels > 0);
    assert!(native_sample_rate > 0);

    if native_sample_rate == source_rate {
      return None;
    }

    Some((0..source_num_channels).map(|_| {
                                   r8brain_rs::ResamplerQueue::new(source_rate as f64,
                                                                   native_sample_rate as f64,
                                                                   8192,
                                                                   0.2,
                                                                   PrecisionProfile::Bits32)
                                 })
                                 .collect())
  }

  fn push_to_resamplers(input: &mut [&mut [f32]], num_read: usize, resamplers: &mut Vec<r8brain_rs::ResamplerQueue>) {
    resamplers.iter_mut().zip(input.iter()).for_each(|(resampler, buffer)| {
                                             let mut resample_buffer = [0.0; BUF_SIZE];
                                             fill_slice(&mut resample_buffer[..num_read],
                                                        buffer[..num_read].into_iter().map(cast_sample_ref()));

                                             resampler.push(&resample_buffer[..num_read]);
                                           });
  }

  fn pull_from_resamplers(node_buffers: &NodeBuffers,
                          start: usize,
                          remaining: usize,
                          resamplers: &mut Vec<r8brain_rs::ResamplerQueue>)
                          -> usize {
    let mut resampler_read = 0;

    for (output, resampler) in node_buffers.outputs().zip(resamplers.iter_mut()) {
      resampler_read = resampler.pull(&mut output[start..start + remaining]);
    }

    resampler_read
  }

  fn prepare_to_play_with_resamplers(&mut self) -> Result {
    let channels = self.reader.get_channel_count() as usize;
    let mut buffers = crate::buffers8!(self);
    let resamplers = self.resamplers.as_mut().unwrap();

    for _ in 0..PRELOAD_BUFFER_COUNT {
      let num_read = self.reader.read_samples(&mut buffers[..channels],
                                              self.play_head.position as i64,
                                              self.play_head.buffer_size as i32);

      if num_read <= 0 {
        break;
      }

      let num_read = num_read as usize;

      Self::push_to_resamplers(&mut buffers[..channels], num_read, resamplers);

      self.play_head = self.play_head.advance_position();
    }

    Ok(())
  }

  fn prepare_to_play_no_resamplers(&mut self) -> Result {
    let channels = self.reader.get_channel_count() as usize;
    let mut buffers = crate::buffers8!(self);
    let mut play = self.play_head;

    for _ in 0..PRELOAD_BUFFER_COUNT {
      let num_read = self.reader.read_samples(&mut buffers[..channels],
                                              self.play_head.position as i64,
                                              self.play_head.buffer_size as i32);

      if num_read <= 0 {
        break;
      }

      play = play.advance_position();
    }

    Ok(())
  }

  fn process_with_resamplers(&mut self, node_buffers: &NodeBuffers) -> Result {
    let channels = self.reader.get_channel_count() as usize;
    let mut buffers = crate::buffers8!(self);
    let mut total_read = 0;
    let buffer_size = self.play_head.buffer_size as usize;
    let resamplers = self.resamplers.as_mut().unwrap();

    while total_read < buffer_size {
      let remaining = buffer_size - total_read;

      let num_read = self.reader.read_samples(&mut buffers[..channels],
                                              self.play_head.position as i64,
                                              self.play_head.buffer_size as i32);

      if num_read < 0 {
        return Err(anyhow!("Error reading samples from file"));
      } else if num_read == 0 {
        // we read what we can, but we're done
        break;
      }

      let num_read = num_read as usize;

      if resamplers[0].available_for_reading() < remaining {
        // push to the queue ...
        Self::push_to_resamplers(&mut buffers[..channels], num_read, resamplers);
      }

      // pull from the queue ...
      let num_resampled = Self::pull_from_resamplers(node_buffers, total_read, remaining, resamplers);

      total_read += num_resampled;
    }

    Ok(())
  }

  fn process_without_resamplers(&mut self, node_buffers: &NodeBuffers) -> Result {
    let channels = self.reader.get_channel_count() as usize;
    let mut buffers = crate::buffers8!(self);
    let mut total_read = 0;
    let buffer_size = self.play_head.buffer_size as usize;

    while total_read < buffer_size {
      let remaining = buffer_size - total_read;

      let num_read = self.reader
                         .read_samples(&mut buffers[..channels], self.play_head.position as i64, remaining as i32);

      if num_read < 0 {
        return Err(anyhow!("Error reading samples from file"));
      } else if num_read == 0 {
        // we read what we can, but we're done
        break;
      }

      let num_read = num_read as usize;

      for (output, buffer) in node_buffers.outputs().zip(buffers[..channels].iter()) {
        fill_slice(&mut output[total_read..total_read + num_read],
                   buffer[..num_read].into_iter().map(cast_sample_ref()));
      }

      total_read += num_read;
      self.play_head = self.play_head.advance_position_by(num_read);
    }

    Ok(())
  }
}

impl Node for JuceSourceReaderNode {
  fn get_node_info(&self, _play: PlayHead) -> NodeInfo {
    NodeInfo { num_outputs: self.reader.get_channel_count() as usize,
               ..Default::default() }
  }

  fn prepare_to_play(&mut self, play: PlayHead, _accumulated_latency: usize) -> Result {
    self.play_head = play;
    self.resamplers = Self::make_resamplers(play.sample_rate, self.info.num_outputs, self.reader.get_sample_rate() as u32);

    if self.resamplers.is_some() {
      self.prepare_to_play_with_resamplers()
    } else {
      self.prepare_to_play_no_resamplers()
    }
  }

  fn process(&mut self,
             _play: PlayHead,
             _device_buffers: DevicesBuffers,
             node_buffers: NodeBuffers,
             _deadline: Instant,
             events: &mut Vec<NodeEvent>)
             -> Result {
    if self.resamplers.is_some() {
      self.process_with_resamplers(&node_buffers)?;
    } else {
      self.process_without_resamplers(&node_buffers)?;
    }

    events.extend(node_buffers.outputs()
                              .map(|s| slice_peak_level_db(s as &_))
                              .enumerate()
                              .map(make_report(reports::PEAK_LEVEL, 0)));

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use std::time::{Duration, Instant};

  use api::task::player::PlayHead;

  use crate::buffer::{DevicesBuffers, NodeBuffers};
  use crate::juce::JuceAudioReader;
  use crate::Node;

  use super::*;

  #[test]
  fn test_io_perf() {
    let mut buffer0 = [0.0; BUF_SIZE];

    let src = JuceAudioReader::new("../../test-files/StarWars3.wav").expect("Failed to open file");

    let start = Instant::now();
    let length = 10_000 * BUF_SIZE as i64;
    let mut read = 0;
    let duration = test_duration();
    let channels = src.get_channel_count() as usize;
    let mut pos = 0;

    while start.elapsed() < duration {
      let mut buffers = [&mut buffer0[..]];
      if src.read_samples(&mut buffers[..channels], pos, BUF_SIZE as i32) < BUF_SIZE as i32 {
        pos = 0;
        continue;
      }

      read += 1;
      pos += BUF_SIZE as i64;

      if pos >= length {
        pos = 0;
      }
    }

    println!("Direct: Read {read} blocks in {:?}", start.elapsed());
    println!("Equals {0:.1} MB/s",
             (read as f64 * BUF_SIZE as f64 * 2 as f64 / (1024f64 * 1024f64)) / start.elapsed().as_secs_f64());
  }

  fn test_duration() -> Duration {
    Duration::from_secs(5)
  }

  #[test]
  fn test_node_perf() {
    let mut play_head = PlayHead::default();
    play_head.play_region.end = (BUF_SIZE * 1_000) as u64;
    play_head.play_region.looping = true;
    play_head.sample_rate = 22_050;
    play_head.buffer_size = BUF_SIZE as u32;

    test_node_perf_with_play_head("../../test-files/StarWars3.wav", "native", play_head);
  }

  #[test]
  fn test_node_perf_src() {
    let mut play_head = PlayHead::default();
    play_head.play_region.end = (BUF_SIZE * 1_000) as u64;
    play_head.play_region.looping = true;
    play_head.sample_rate = 192_000;
    play_head.buffer_size = BUF_SIZE as u32;

    test_node_perf_with_play_head("../../test-files/StarWars3.wav", "192k upsample", play_head);
  }

  #[test]
  fn test_node_perf_surround() {
    let mut play_head = PlayHead::default();
    play_head.play_region.end = (BUF_SIZE * 1_000) as u64;
    play_head.play_region.looping = true;
    play_head.sample_rate = 48000;
    play_head.buffer_size = BUF_SIZE as u32;

    test_node_perf_with_play_head("../../test-files/Nums_7dot1_24_48000.wav", "native", play_head);
  }

  #[test]
  fn test_node_perf_surround_src() {
    let mut play_head = PlayHead::default();
    play_head.play_region.end = (BUF_SIZE * 1_000) as u64;
    play_head.play_region.looping = true;
    play_head.sample_rate = 192_000;
    play_head.buffer_size = BUF_SIZE as u32;

    test_node_perf_with_play_head("../../test-files/Nums_7dot1_24_48000.wav", "192k upsample", play_head);
  }

  fn test_node_perf_with_play_head(file_name: &str, label: &str, play_head: PlayHead) {
    let mut node = JuceSourceReaderNode::new(file_name, play_head, 2).expect("Failed to open file");

    let info = node.get_node_info(play_head);

    node.prepare_to_play(play_head, 0).expect("Failed to prepare to play");
    let device_buffers = DevicesBuffers::default();

    let start = Instant::now();
    let duration = test_duration();

    let node_buffers = NodeBuffers::allocate(&info, BUF_SIZE);
    let mut read = 0;
    let mut events = vec![];

    while start.elapsed() < duration {
      let deadline = Instant::now() + Duration::from_millis(10);

      node.process(play_head, device_buffers.clone(), node_buffers.clone(), deadline, &mut events)
          .expect("Failed to process");

      events.clear();

      // assert!(events.len() >= 1);
      read += 1;
    }

    println!("Node: Read {read} blocks in {:?} from {file_name} ({label})", start.elapsed());
    println!("Equals {0:.1} MB/s",
             (read as f64 * play_head.buffer_size as f64 * info.num_outputs as f64 / (1024f64 * 1024f64)) / start.elapsed().as_secs_f64());
  }
}
