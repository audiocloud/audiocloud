use std::io::Read;
use std::time::Instant;

use anyhow::bail;

use api::instance::spec::SetParameterCommand;
use api::task::graph::BusId;
use api::task::player::PlayHead;

use crate::buffer::{fill_slice, DevicesBuffers, NodeBuffers};
use crate::{Node, NodeEvent, NodeInfo, Result};

mod parameters {
  use std::collections::HashMap;

  use maplit::hashmap;
  use serde_json::json;

  use api::instance::model::{ParameterModel, ValueRange};

  pub const INPUT_LEVEL: &'static str = "inputLevel";
  pub const OUTPUT_LEVEL: &'static str = "outputLevel";
  pub const MID_SIDE_MODE: &'static str = "midSideMode";

  fn mid_side_mode() -> ParameterModel {
    ParameterModel { range: ValueRange::List { values: vec![0.0, 1.0, 2.0], },
                     channels: 1,
                     metadata: hashmap! {
                       "labels".to_owned() => json!(["off", "encode", "decode"]),
                       "off".to_owned() => json!(0.0)
                     },
                     ..Default::default() }
  }

  fn io_level(num_channels: usize) -> ParameterModel {
    ParameterModel { range: ValueRange::volume(),
                     channels: num_channels,
                     metadata: Default::default(),
                     ..Default::default() }
  }

  pub fn create(num_inputs: usize, num_outputs: usize) -> HashMap<String, ParameterModel> {
    hashmap! {
      MID_SIDE_MODE.to_owned() => mid_side_mode(),
      INPUT_LEVEL.to_owned() => io_level(num_inputs),
      OUTPUT_LEVEL.to_owned() => io_level(num_outputs),
    }
  }
}

mod reports {
  use std::collections::HashMap;

  use maplit::hashmap;

  use api::instance::model::ReportModel;
  use api::task::player::NodeEvent;

  use crate::events::{make_report, slice_peak_level_db, slice_rms_level_db, volume_level_report};

  pub const INPUT_PEAK_LEVEL: &'static str = "inputPeakLevel";
  pub const OUTPUT_PEAK_LEVEL: &'static str = "outputPeakLevel";
  pub const INPUT_RMS_LEVEL: &'static str = "inputRmsLevel";
  pub const OUTPUT_RMS_LEVEL: &'static str = "outputRmsLevel";

  pub fn create(num_inputs: usize, num_outputs: usize) -> HashMap<String, ReportModel> {
    hashmap! {
      INPUT_PEAK_LEVEL.to_owned() => volume_level_report(num_inputs),
      OUTPUT_PEAK_LEVEL.to_owned() => volume_level_report(num_outputs),
      INPUT_RMS_LEVEL.to_owned() => volume_level_report(num_inputs),
      OUTPUT_RMS_LEVEL.to_owned() => volume_level_report(num_outputs),
    }
  }

  pub fn io_levels_peak<'a, Inputs, Outputs>(inputs: Inputs,
                                             outputs: Outputs,
                                             input_offset: usize,
                                             output_offset: usize)
                                             -> impl Iterator<Item = NodeEvent>
    where Inputs: Iterator<Item = &'a [f64]>,
          Outputs: Iterator<Item = &'a [f64]>
  {
    let peak_inputs = inputs.map(slice_peak_level_db)
                            .enumerate()
                            .map(make_report(INPUT_PEAK_LEVEL, input_offset));

    let peak_outputs = outputs.map(slice_peak_level_db)
                              .enumerate()
                              .map(make_report(OUTPUT_PEAK_LEVEL, output_offset));

    peak_inputs.chain(peak_outputs)
  }

  pub fn io_levels_rms<'a, Inputs, Outputs>(inputs: Inputs,
                                            outputs: Outputs,
                                            input_offset: usize,
                                            output_offset: usize)
                                            -> impl Iterator<Item = NodeEvent>
    where Inputs: Iterator<Item = &'a [f64]>,
          Outputs: Iterator<Item = &'a [f64]>
  {
    let rms_inputs = inputs.map(slice_rms_level_db)
                           .enumerate()
                           .map(make_report(INPUT_RMS_LEVEL, input_offset));

    let rms_outputs = outputs.map(slice_rms_level_db)
                             .enumerate()
                             .map(make_report(OUTPUT_RMS_LEVEL, output_offset));

    rms_inputs.chain(rms_outputs)
  }
}

pub struct BusNode {
  id:             BusId,
  info:           NodeInfo,
  mid_side_mode:  Option<MidSideMode>,
  input_volumes:  Vec<f64>,
  output_volumes: Vec<f64>,
}

enum MidSideMode {
  Encode,
  Decode,
}

impl BusNode {
  pub fn new(id: BusId, num_inputs: usize, num_outputs: usize) -> Result<Self> {
    let latency = 0;

    match (num_inputs, num_outputs) {
      | (1, 2) | (2, 1) => {}
      | (i, j) if i == j && i >= 1 => {}
      | (_, _) => bail!("Bus node must have either: 1 input and 2 outputs, 2 inputs and 1 output or the same number of inputs and outputs"),
    }

    let info = NodeInfo { latency,
                          num_inputs,
                          num_outputs,
                          parameters: parameters::create(num_inputs, num_outputs),
                          reports: reports::create(num_inputs, num_outputs) };

    let input_volumes = vec![1.0; num_inputs];
    let output_volumes = vec![1.0; num_outputs];

    let mid_side_mode = None;

    Ok(Self { id,
              info,
              mid_side_mode,
              input_volumes,
              output_volumes })
  }

  fn stereo_unwrap(&mut self, source: &mut [f64], left: &mut [f64], right: &mut [f64]) -> Result<Vec<NodeEvent>> {
    let input_vol = self.input_volumes.get(0).cloned().unwrap_or(1.0);

    for sample in source.iter_mut() {
      *sample *= input_vol;
    }

    let [left_vol, right_vol] = self.output_volumes.as_slice()[..2] else { bail!("mismatched channel volume counts") };

    fill_slice(left, source.iter().copied().map(|x| x * left_vol));
    fill_slice(right, source.iter().copied().map(|x| x * right_vol));

    Ok(reports::io_levels_peak([source as &_].into_iter(), [left as &_, right as &_].into_iter(), 0, 0).chain(
      reports::io_levels_rms([source as &_].into_iter(), [left as &_, right as &_].into_iter(), 0, 0)).collect())
  }

  fn stereo_collapse(&mut self, left: &mut [f64], right: &mut [f64], target: &mut [f64]) -> Result<Vec<NodeEvent>> {
    let [left_vol, right_vol] = self.input_volumes.as_slice()[..2] else { bail!("mismatched channel volume counts")};
    let output_vol = self.output_volumes.get(0).cloned().unwrap_or(1.0);

    for (l, r) in left.iter_mut().zip(right.iter_mut()) {
      *l *= left_vol;
      *r *= right_vol;
    }

    fill_slice(target, left.into_iter().zip(right.into_iter()).map(|(l, r)| (*l + *r) * output_vol));

    Ok(reports::io_levels_peak([left as &_, right as &_].into_iter(), [target as &_].into_iter(), 0, 0).chain(
      reports::io_levels_rms([left as &_, right as &_].into_iter(), [target as &_].into_iter(), 0, 0)).collect())
  }

  fn copy<'a, Sources, Targets>(&mut self, sources: Sources, targets: Targets) -> Result<Vec<NodeEvent>>
    where Sources: Iterator<Item = &'a mut [f64]>,
          Targets: Iterator<Item = &'a mut [f64]>
  {
    let mut levels = Vec::new();
    for (i, (src, dest)) in sources.zip(targets).enumerate() {
      let src_level = self.input_volumes.get(i).cloned().unwrap_or(1.0);
      let dst_level = self.output_volumes.get(i).cloned().unwrap_or(1.0);

      for src in &mut src[..] {
        *src *= src_level;
      }

      for (src, dst) in src.iter().zip(dest.iter_mut()) {
        *dst = *src * dst_level;
      }

      levels.extend(reports::io_levels_peak([src as &_].into_iter(), [dest as &_].into_iter(), i, i));
      levels.extend(reports::io_levels_rms([src as &_].into_iter(), [dest as &_].into_iter(), i, i));
    }

    Ok(levels)
  }
}

impl Node for BusNode {
  fn set_parameter(&mut self, p: &SetParameterCommand) {
    match (&p.parameter[..], p.channel, p.value) {
      | (parameters::MID_SIDE_MODE, 0, 0.0) => self.mid_side_mode = None,
      | (parameters::MID_SIDE_MODE, 0, 1.0) => self.mid_side_mode = Some(MidSideMode::Encode),
      | (parameters::MID_SIDE_MODE, 0, 2.0) => self.mid_side_mode = Some(MidSideMode::Decode),
      | (parameters::INPUT_LEVEL, ch, val) if ch < self.info.num_inputs => self.input_volumes[ch] = val,
      | (parameters::OUTPUT_LEVEL, ch, val) if ch < self.info.num_outputs => self.output_volumes[ch] = val,
      | _ => {}
    }
  }

  fn get_node_info(&self, _play: PlayHead) -> NodeInfo {
    self.info.clone()
  }

  fn process(&mut self,
             _play: PlayHead,
             _devices: DevicesBuffers,
             node_buffers: NodeBuffers,
             _deadline: Instant)
             -> Result<Vec<NodeEvent>> {
    match (node_buffers.num_inputs, node_buffers.num_outputs) {
      | (1, 2) => self.stereo_unwrap(node_buffers.input_plane(0),
                                     node_buffers.output_plane(0),
                                     node_buffers.output_plane(1)),
      | (2, 1) => self.stereo_collapse(node_buffers.input_plane(0),
                                       node_buffers.input_plane(1),
                                       node_buffers.output_plane(0)),
      | (_, _) => self.copy(node_buffers.inputs(), node_buffers.outputs()),
    }
  }
}
