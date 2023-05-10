use std::time::Instant;

use dasp::Sample;
use ebur128::{EbuR128, Mode};

use api::instance::spec::SetParameterCommand;
use api::task::player::{NodeEvent, PlayHead};

use crate::buffer::{fill_slice, DevicesBuffers, NodeBuffers};
use crate::events::make_report;
use crate::{Node, NodeInfo, Result};

use super::{parameters, reports};

pub struct MonitorSinkNode {
  device_id:        String,
  sample_rate:      u32,
  outputs:          Vec<usize>,
  measurements:     EbuR128,
  measure_position: u64,
  measure_interval: u64,
  gain:             Vec<f64>,
  info:             NodeInfo,
}

impl MonitorSinkNode {
  pub fn new(device_id: String, outputs: Vec<usize>, sample_rate: u32) -> Self {
    let measurements = EbuR128::new(outputs.len() as u32, sample_rate, Mode::I | Mode::TRUE_PEAK).unwrap();

    let measure_position = 0;
    let measure_interval = (sample_rate as f64 * reports::MEASURE_LUFS_FACTOR).floor() as u64;

    let gain = vec![1.0; outputs.len()];

    let num_channels = outputs.len();

    let info = NodeInfo { num_inputs: num_channels,
                          reports: reports::create(num_channels),
                          parameters: parameters::create(num_channels),
                          ..Default::default() };

    Self { device_id,
           sample_rate,
           outputs,
           measurements,
           measure_position,
           measure_interval,
           gain,
           info }
  }
}

impl Node for MonitorSinkNode {
  fn set_parameter(&mut self, p: &SetParameterCommand) {
    match (&p.parameter[..], p.channel, p.value) {
      | (parameters::GAIN, ch, value) if ch < self.gain.len() => {
        self.gain[ch] = value;
      }
      | _ => {}
    }
  }

  fn get_node_info(&self, _play: PlayHead) -> NodeInfo {
    self.info.clone()
  }

  fn prepare_to_play(&mut self, _play: PlayHead, _accumulated_latency: usize) -> Result {
    self.measure_position = 0;

    Ok(())
  }

  fn process(&mut self,
             play: PlayHead,
             device_buffers: DevicesBuffers,
             node_buffers: NodeBuffers,
             _deadline: Instant,
             events: &mut Vec<NodeEvent>)
             -> Result {
    let channels = self.info.num_inputs;
    let device_buffers = device_buffers.device(&self.device_id)?;

    for (index, output_id) in self.outputs.iter().enumerate() {
      let device_out = device_buffers.output_plane(*output_id);
      let node_in = node_buffers.input_plane(index);
      let gain = self.gain.get(index).copied().unwrap_or(1.0);

      fill_slice(device_out, node_in.iter().map(|src| f32::from_sample(*src * gain)));
    }

    let buffers = (0..channels).map(|i| node_buffers.input_plane(i) as &_).collect::<Vec<_>>();

    self.measurements.add_frames_planar_f64(buffers.as_slice())?;

    events.extend((0..channels).map(|i| (i, self.measurements.true_peak(i as u32).unwrap_or_default()))
                               .map(make_report(reports::PEAK_LEVEL, 0)));

    self.measure_position += play.buffer_size as u64;
    while self.measure_position >= self.measure_interval {
      events.push(make_report(reports::LUFS_LEVEL, 0)((0, self.measurements.loudness_momentary()?)));
      self.measure_position -= self.measure_interval;
    }

    Ok(())
  }
}
