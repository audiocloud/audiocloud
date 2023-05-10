use std::time::Instant;

use api::task::player::{NodeEvent, PlayHead};

use crate::buffer::{cast_sample_ref, fill_slice, DevicesBuffers, NodeBuffers};
use crate::events::{slice_peak_level_db, slice_rms_level_db};
use crate::player::DeviceInstanceAttachment;
use crate::{Node, NodeInfo, Result};

mod reports {
  use std::collections::HashMap;

  use maplit::hashmap;

  use api::instance::model::ReportModel;

  use crate::events::volume_level_report;

  pub const SEND_PEAK_LEVEL: &'static str = "sendPeakLevel";
  pub const RETURN_PEAK_LEVEL: &'static str = "returnPeakLevel";
  pub const SEND_RMS_LEVEL: &'static str = "sendRmsLevel";
  pub const RETURN_RMS_LEVEL: &'static str = "returnRmsLevel";

  pub fn create(num_sends: usize, num_returns: usize) -> HashMap<String, ReportModel> {
    hashmap! {
      SEND_PEAK_LEVEL.to_owned() => volume_level_report(num_sends),
      RETURN_PEAK_LEVEL.to_owned() => volume_level_report(num_returns),
      SEND_RMS_LEVEL.to_owned() => volume_level_report(num_sends),
      RETURN_RMS_LEVEL.to_owned() => volume_level_report(num_returns),
    }
  }
}

mod parameters {
  use std::collections::HashMap;

  use maplit::hashmap;

  use api::instance::model::ParameterModel;

  pub fn create(num_sends: usize, num_returns: usize) -> HashMap<String, ParameterModel> {
    hashmap! {}
  }
}

pub struct AudioDeviceInsertNode {
  info:            NodeInfo,
  audio_device_id: String,
  sends:           Vec<u32>,
  returns:         Vec<u32>,
  send_gains:      Vec<f64>,
  return_gains:    Vec<f64>,
}

impl AudioDeviceInsertNode {
  pub fn new(spec: &DeviceInstanceAttachment, device_latency: usize) -> Result<Self> {
    let num_sends = spec.sends.len();
    let num_returns = spec.returns.len();
    let info = NodeInfo { latency:     spec.additional_latency + device_latency,
                          num_inputs:  num_sends,
                          num_outputs: num_returns,
                          parameters:  parameters::create(num_sends, num_returns),
                          reports:     reports::create(num_sends, num_returns), };

    let send_gains = vec![1.0; num_sends];
    let return_gains = vec![1.0; num_returns];
    let audio_device_id = spec.device_id.clone();
    let sends = spec.sends.clone();
    let returns = spec.returns.clone();

    Ok(Self { info,
              audio_device_id,
              sends,
              returns,
              send_gains,
              return_gains })
  }
}

impl Node for AudioDeviceInsertNode {
  fn get_node_info(&self, _play: PlayHead) -> NodeInfo {
    self.info.clone()
  }

  fn process(&mut self,
             _play: PlayHead,
             device_buffers: DevicesBuffers,
             node_buffers: NodeBuffers,
             _deadline: Instant,
             events: &mut Vec<NodeEvent>)
             -> Result {
    let device_buffers = device_buffers.device(&self.audio_device_id)?;

    for (index, send_channel) in self.sends.iter().copied().enumerate() {
      let device_plane = device_buffers.output_plane(send_channel as usize);
      let node_plane = node_buffers.input_plane(index);

      for s in &mut node_plane[..] {
        *s *= self.send_gains[index];
      }

      let peak_db = slice_peak_level_db(node_plane as &_);
      let rms_db = slice_rms_level_db(node_plane as &_);

      fill_slice(device_plane, node_plane.iter().map(cast_sample_ref()));

      events.push(NodeEvent::Report { name:    reports::SEND_PEAK_LEVEL.to_owned(),
                                      channel: index,
                                      value:   peak_db, });

      events.push(NodeEvent::Report { name:    reports::SEND_RMS_LEVEL.to_owned(),
                                      channel: index,
                                      value:   rms_db, });
    }

    for (index, return_channel) in self.returns.iter().copied().enumerate() {
      let device_plane = device_buffers.input_plane(return_channel as usize);
      let node_plane = node_buffers.output_plane(index);

      fill_slice(node_plane, device_plane.iter().map(cast_sample_ref()));

      for s in &mut node_plane[..] {
        *s *= self.return_gains[index];
      }

      let peak_db = slice_peak_level_db(node_plane);
      let rms_db = slice_rms_level_db(node_plane);

      events.push(NodeEvent::Report { name:    reports::RETURN_PEAK_LEVEL.to_owned(),
                                      channel: index,
                                      value:   peak_db, });

      events.push(NodeEvent::Report { name:    reports::RETURN_RMS_LEVEL.to_owned(),
                                      channel: index,
                                      value:   rms_db, });
    }

    Ok(())
  }
}
