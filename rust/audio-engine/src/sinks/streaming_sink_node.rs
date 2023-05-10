use std::collections::VecDeque;
use std::ffi::{c_void, CStr};
use std::ptr::null_mut;
use std::time::Instant;

use anyhow::{anyhow, bail};
use dasp::Sample;
use ebur128::{EbuR128, Mode};
use libflac_sys::*;
use r8brain_rs::PrecisionProfile;

use api::task::player::{NodeEvent, NodeInfo, PlayHead};

use crate::buffer::{fill_slice, DevicesBuffers, NodeBuffers};
use crate::events::make_report;
use crate::{Node, Result};

use super::{parameters, reports};

pub struct StreamingSinkNode {
  info:             NodeInfo,
  shared:           Box<Shared>,
  encoder:          *mut FLAC__StreamEncoder,
  bits_per_sample:  usize,
  resampler:        Vec<r8brain_rs::ResamplerQueue>,
  input_buffers:    Vec<Vec<i32>>,
  gain:             [f64; 8],
  measurements:     EbuR128,
  measure_position: u64,
  measure_interval: u64,
}

unsafe impl Send for StreamingSinkNode {}
unsafe impl Sync for StreamingSinkNode {}

#[derive(Default)]
struct Shared {
  buffer: VecDeque<bytes::Bytes>,
}

impl Drop for StreamingSinkNode {
  fn drop(&mut self) {
    unsafe {
      if self.encoder != null_mut() {
        FLAC__stream_encoder_delete(self.encoder);
        self.encoder = null_mut();
      }
    }
  }
}

unsafe extern "C" fn write_callback(encoder: *const FLAC__StreamEncoder,
                                    buffer: *const FLAC__byte,
                                    bytes: usize,
                                    samples: u32,
                                    current_frame: u32,
                                    client_data: *mut c_void)
                                    -> FLAC__StreamEncoderWriteStatus {
  let shared = &mut *(client_data as *mut Shared);

  shared.buffer
        .push_back(bytes::Bytes::copy_from_slice(std::slice::from_raw_parts(buffer, bytes)));

  FLAC__STREAM_ENCODER_WRITE_STATUS_OK
}

impl StreamingSinkNode {
  pub fn new(channels: usize, sample_rate: u32, native_sample_rate: u32, bits_per_sample: usize) -> Result<Self> {
    if channels > 8 {
      bail!("StreamingSinkNode supports up to 8 channels");
    }

    if bits_per_sample != 16 && bits_per_sample != 32 {
      bail!("StreamingSinkNode supports only 16 or 32 bits per sample");
    }

    let encoder = unsafe { FLAC__stream_encoder_new() };
    let mut shared = Box::new(Shared::default());

    unsafe {
      if FLAC__stream_encoder_set_bits_per_sample(encoder, bits_per_sample as u32) != 1 {
        bail!("FLAC__stream_encoder_set_bits_per_sample failed: {bits_per_sample}")
      }

      if FLAC__stream_encoder_set_channels(encoder, channels as u32) != 1 {
        bail!("FLAC__stream_encoder_set_channels failed: {channels}")
      }

      if FLAC__stream_encoder_set_sample_rate(encoder, sample_rate as u32) != 1 {
        bail!("FLAC__stream_encoder_set_sample_rate failed: {sample_rate}")
      }
    };

    let init_rv = unsafe {
      FLAC__stream_encoder_init_stream(encoder,
                                       Some(write_callback),
                                       None,
                                       None,
                                       None,
                                       shared.as_mut() as *mut Shared as *mut c_void)
    };

    if init_rv != FLAC__STREAM_ENCODER_INIT_STATUS_OK {
      return Err(anyhow!("FLAC__stream_encoder_init_stream failed: {init_rv}"));
    }

    let info = NodeInfo { num_inputs:  channels,
                          num_outputs: 0,
                          latency:     0,
                          reports:     reports::create(channels),
                          parameters:  parameters::create(channels), };

    let input_buffers = (0..channels).map(|_| Vec::new()).collect();

    let measure_position = 0;
    let measure_interval = (sample_rate as f64 * reports::MEASURE_LUFS_FACTOR).floor() as u64;

    let gain = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    let resampler =
      (0..channels).map(|_| {
                     r8brain_rs::ResamplerQueue::new(native_sample_rate as f64, sample_rate as f64, 8192, 2.0, PrecisionProfile::Bits32)
                   })
                   .collect();

    let measurements = EbuR128::new(channels as u32, sample_rate as u32, Mode::TRUE_PEAK | Mode::I)?;

    let rv = Self { info,
                    shared,
                    encoder,
                    input_buffers,
                    bits_per_sample,
                    resampler,
                    measurements,
                    measure_position,
                    measure_interval,
                    gain };

    Ok(rv)
  }
}

impl Node for StreamingSinkNode {
  fn get_node_info(&self, _play: PlayHead) -> NodeInfo {
    self.info.clone()
  }

  fn process(&mut self,
             _play: PlayHead,
             _device_buffers: DevicesBuffers,
             node_buffers: NodeBuffers,
             _deadline: Instant,
             events: &mut Vec<NodeEvent>)
             -> Result {
    let mut channel0 = [0.0; 8192];
    let mut channel1 = [0.0; 8192];
    let mut channel2 = [0.0; 8192];
    let mut channel3 = [0.0; 8192];
    let mut channel4 = [0.0; 8192];
    let mut channel5 = [0.0; 8192];
    let mut channel6 = [0.0; 8192];
    let mut channel7 = [0.0; 8192];

    let num_channels = self.info.num_inputs;
    let mut num_samples = channel0.len();

    for (i, ((resampler, source), buffer)) in self.resampler
                                                  .iter_mut()
                                                  .zip(node_buffers.inputs())
                                                  .zip(self.input_buffers.iter_mut())
                                                  .enumerate()
    {
      let target = match i {
        | 0 => &mut channel0[..],
        | 1 => &mut channel1[..],
        | 2 => &mut channel2[..],
        | 3 => &mut channel3[..],
        | 4 => &mut channel4[..],
        | 5 => &mut channel5[..],
        | 6 => &mut channel6[..],
        | 7 => &mut channel7[..],
        | _ => unreachable!("too many channels"),
      };

      resampler.push(source);
      let num_resampled = resampler.pull(&mut target[..]);

      num_samples = num_samples.min(num_resampled);
      if num_samples == 0 {
        continue;
      }

      buffer.resize(num_resampled as usize, 0);
      match self.bits_per_sample {
        | 16 => {
          fill_slice(&mut buffer[..num_resampled],
                     target[..num_resampled].iter().map(|x| i16::from_sample(*x) as i32));
        }
        | 32 => {
          fill_slice(&mut buffer[..num_resampled],
                     target[..num_resampled].iter().map(|x| i32::from_sample(*x)));
        }
        | _ => {}
      }
    }

    if num_samples > 0 {
      let channels = [&channel0[..num_samples],
                      &channel1[..num_samples],
                      &channel2[..num_samples],
                      &channel3[..num_samples],
                      &channel4[..num_samples],
                      &channel5[..num_samples],
                      &channel6[..num_samples],
                      &channel7[..num_samples]];

      self.measurements.add_frames_planar_f64(&channels[..num_channels])?;
      events.extend((0..num_channels).map(|i| (i, self.measurements.true_peak(i as u32).unwrap_or_default()))
                                     .map(make_report(reports::PEAK_LEVEL, 0)));

      self.measure_position += num_samples as u64;
      while self.measure_position > self.measure_interval {
        events.push(make_report(reports::LUFS_LEVEL, 0)((0, self.measurements.loudness_momentary()?)));
        self.measure_position -= self.measure_interval;
      }

      let buffers = self.input_buffers.iter().map(|buf| buf.as_ptr()).collect::<Vec<_>>();
      let buffers = buffers.as_ptr();

      unsafe {
        if FLAC__stream_encoder_process(self.encoder, buffers, num_samples as u32) == 0 {
          bail!("FLAC__stream_encoder_process failed: {}",
                CStr::from_ptr(FLAC__stream_encoder_get_resolved_state_string(self.encoder)).to_str()
                                                                                            .unwrap_or_default());
        }
      }
    }

    Ok(())
  }
}
