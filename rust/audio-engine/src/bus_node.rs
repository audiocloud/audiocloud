use std::time::Instant;

use api::task::graph::{BusId, MidSideMode};
use api::task::player::PlayHead;

use crate::buffer::{fill_slice, zero_slice, DeviceBuffers};
use crate::{Node, NodeInfo};

pub struct BusNode {
  id:            BusId,
  info:          NodeInfo,
  mid_side_mode: Option<MidSideMode>,
  input_buffer:  Vec<f64>,
}

impl BusNode {
  pub fn new(id: BusId, num_inputs: usize, num_outputs: usize, mid_side_mode: Option<MidSideMode>) -> Self {
    let latency = 0;

    let info = NodeInfo { latency,
                          num_inputs,
                          num_outputs };

    let input_buffer = vec![0.0; 0];

    Self { id,
           info,
           mid_side_mode,
           input_buffer }
  }

  fn reset_buffers(&mut self, size: usize) {
    self.input_buffer.resize(size, 0.0);
    zero_slice(&mut self.input_buffer);
  }

  fn stereo_unwrap(&mut self, source: &[f64], left: &mut [f64], right: &mut [f64], play: PlayHead) {
    fill_slice(left, source.iter().copied());
    fill_slice(right, source.iter().copied());
  }

  fn stereo_collapse(&mut self, left: &[f64], right: &[f64], target: &mut [f64], play: PlayHead) {
    fill_slice(target, left.into_iter().zip(right.into_iter()).map(|(l, r)| *l + *r));
  }

  fn copy(&mut self, source: &[&[f64]], target: &mut [&mut [f64]], play: PlayHead) {
    for (src, dest) in source.iter().zip(target.iter_mut()) {
      fill_slice(dest, src.iter().copied());
    }
  }
}

impl Node for BusNode {
  fn get_node_info(&self, play: PlayHead) -> NodeInfo {
    self.info
  }

  fn process(&mut self,
             play: PlayHead,
             _device_buffers: DeviceBuffers,
             inputs: &[&[f64]],
             outputs: &mut [&mut [f64]],
             _deadline: Instant)
             -> crate::Result {
    self.reset_buffers(play.buffer_size as usize);

    match (inputs, outputs) {
      | ([input], [left, right]) => {
        self.stereo_unwrap(input, left, right, play);
      }
      | ([left, right], [output]) => {
        self.stereo_collapse(left, right, output, play);
      }
      | (inputs, outputs) => {
        // TODO: mid/side
        self.copy(inputs, outputs, play);
      }
    }

    Ok(())
  }
}
