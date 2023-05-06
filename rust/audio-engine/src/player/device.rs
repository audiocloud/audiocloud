use std::collections::HashSet;
use std::time::Instant;

use anyhow::bail;

use crate::buffer::DeviceBuffers;
use crate::player::GraphPlayer;
use crate::Result;

impl GraphPlayer {
  pub(crate) fn device_flip_buffers(&mut self, device_id: String, buffers: DeviceBuffers, generation: u64, deadline: Instant) -> Result {
    if self.current_work_set.device_flips_started.contains_key(&device_id) {
      bail!("Device {} already has a flip in progress in the current WorkSet", device_id)
    }

    self.current_work_set.device_flips_started.insert(device_id.clone(), buffers);
    self.current_work_set.deadline = self.current_work_set
                                         .deadline
                                         .map(|prev_deadline| prev_deadline.min(deadline))
                                         .or(Some(deadline));

    // add nodes to execute - it will be monitors and inserts
    for (node_id, node) in &self.node_state {
      if node.audio_device_requirements.contains(&device_id) {
        self.current_work_set.nodes_to_execute.insert(*node_id);
      }
    }

    loop {
      let mut to_add = HashSet::new();

      for node_id in &self.current_work_set.nodes_to_execute {
        let Some(node) = self.node_state.get(node_id) else { continue };

        for input_id in &node.node_requirements {
          if !self.current_work_set.nodes_to_execute.contains(input_id) {
            to_add.insert(*input_id);
          }
        }
      }

      if to_add.is_empty() {
        break;
      } else {
        self.current_work_set.nodes_to_execute.extend(to_add);
      }
    }

    // we now have a work set
    self.execute_work_sets();

    Ok(())
  }
}
