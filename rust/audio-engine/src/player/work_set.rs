use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::Arc;
use std::time::Instant;

use anyhow::bail;
use tokio::spawn;
use tokio::sync::{mpsc, RwLock};
use tokio::task::block_in_place;

use api::task::graph::NodeId;
use api::task::player::{NodeEvent, PlayHead};

use crate::audio_device::DeviceCommand;
use crate::buffer::{DeviceBuffers, DevicesBuffers, NodeBuffers};
use crate::player::{GraphPlayer, InternalTaskEvent, PlayerNodeState};
use crate::{BoxedNode, Node, Result};

#[derive(Debug)]
pub struct WorkSet {
  pub(crate) play_head:             PlayHead,
  pub(crate) nodes_to_execute:      HashSet<NodeId>,
  pub(crate) nodes_executing:       HashSet<NodeId>,
  pub(crate) nodes_executed:        HashSet<NodeId>,
  pub(crate) device_flips_started:  HashMap<String, DeviceBuffers>,
  pub(crate) device_flips_finished: HashSet<String>,
  pub(crate) deadline:              Option<Instant>,
}

impl WorkSet {
  fn devices_buffers(&self) -> DevicesBuffers {
    DevicesBuffers(self.device_flips_started.clone())
  }

  fn is_empty(&self) -> bool {
    self.nodes_executing.is_empty() && self.nodes_to_execute.is_empty()
  }
}

impl From<PlayHead> for WorkSet {
  fn from(current_play_head: PlayHead) -> Self {
    Self { play_head:             current_play_head,
           nodes_to_execute:      Default::default(),
           nodes_executing:       Default::default(),
           nodes_executed:        Default::default(),
           device_flips_started:  Default::default(),
           device_flips_finished: Default::default(),
           deadline:              None, }
  }
}

impl GraphPlayer {
  pub(crate) fn execute_work_sets(&mut self) {
    Self::execute_work_set(&mut self.current_work_set, &mut self.node_state, &self.node_apis, &self.tx_tasks);

    for work_set in &mut self.partial_work_sets {
      Self::execute_work_set(work_set, &mut self.node_state, &self.node_apis, &self.tx_tasks);
    }

    self.partial_work_sets.retain(|ws| !ws.is_empty());

    if self.partial_work_sets.is_empty() && self.current_work_set.is_empty() {
      self.apply_pending_structure_changes();
      self.current_work_set_finished();
    }
  }

  fn execute_work_set(work_set: &mut WorkSet,
                      node_states: &mut HashMap<NodeId, PlayerNodeState>,
                      node_apis: &HashMap<NodeId, Arc<RwLock<BoxedNode>>>,
                      tx_tasks: &mpsc::Sender<InternalTaskEvent>) {
    'next_node: for ref node_id in work_set.nodes_to_execute.clone() {
      let Some(node) = node_states.get_mut(node_id) else { continue };
      if node.processing.is_some() {
        continue;
      };

      // are all nodes we depend on done executing?
      for requirement_id in &node.node_requirements {
        if !work_set.nodes_executed.contains(requirement_id) {
          continue 'next_node;
        }
      }

      // have all devices that we depend on started flipping?
      for required_device in &node.audio_device_requirements {
        if !work_set.device_flips_started.contains_key(required_device) {
          continue 'next_node;
        }
      }

      // node is executable
      let Some(node_api) = node_apis.get(node_id) else { continue };

      work_set.nodes_to_execute.remove(node_id);

      node.processing = Some(work_set.play_head.generation);
      spawn(execute_node(*node_id,
                         node_api.clone(),
                         work_set.devices_buffers(),
                         node.buffers.clone(),
                         work_set.play_head,
                         work_set.deadline.expect("WorkSet Deadline not set"),
                         tx_tasks.clone()));
    }
  }

  pub(crate) fn task_completed(&mut self, task_id: NodeId, generation: u64, result: Result<Vec<NodeEvent>>) -> Result {
    if let Err(err) = result {
      bail!("Task {task_id} generation {generation} failed: {err}");
    }

    let Some(node) = self.node_state.get_mut(&task_id) else { bail!("Task {task_id} generation {generation} completed but node not found") };

    if node.processing != Some(generation) {
      bail!("Task {task_id} generation {generation} completed but node is not processing");
    }

    node.processing = None;

    self.current_work_set.nodes_executed.insert(task_id);

    self.check_device_flip_finished();

    if self.current_work_set.nodes_to_execute.is_empty() {
      self.current_work_set_finished();
    }

    Ok(())
  }

  fn check_device_flip_finished(&mut self) {
    // if all nodes requiring device dev finished flipping...
    let mut required_devices = HashSet::new();

    for node_id in &self.current_work_set.nodes_to_execute {
      let Some(node) = self.node_state.get(node_id) else { continue };
      if !node.audio_device_requirements.is_empty() {
        required_devices.extend(node.audio_device_requirements.iter().cloned());
      }
    }

    for (device_id, buffers) in &self.current_work_set.device_flips_started {
      if !self.current_work_set.device_flips_finished.contains(device_id) && !required_devices.contains(device_id) {
        self.current_work_set.device_flips_finished.insert(device_id.clone());
        self.audio_devices
            .send_command(device_id, DeviceCommand::FlipFinished { client_id:  self.client_id.clone(),
                                                                   generation: buffers.generation, })
            .expect("Failed to send device flip finished command");
      }
    }
  }

  fn current_work_set_finished(&mut self) {
    // create a new current WorkSet
    self.play_head = self.play_head.advance_position();
    let prev_work_set = mem::replace(&mut self.current_work_set, self.play_head.into());

    // store partial WorkSet if it is non-empty
    if !prev_work_set.nodes_to_execute.is_empty() || !prev_work_set.nodes_executing.is_empty() {
      self.partial_work_sets.push_back(prev_work_set);
    }
  }
}

async fn execute_node(id: NodeId,
                      node_api: Arc<RwLock<BoxedNode>>,
                      devices: DevicesBuffers,
                      buffers: NodeBuffers,
                      play_head: PlayHead,
                      deadline: Instant,
                      tx_tasks: mpsc::Sender<InternalTaskEvent>) {
  let mut source = node_api.write().await;
  let result = block_in_place(|| source.process(play_head, devices, buffers, deadline));

  tx_tasks.send(InternalTaskEvent::Completed { node_id: id,
                                               generation: play_head.generation,
                                               result })
          .await
          .expect("Failed to send Task completion");
}
