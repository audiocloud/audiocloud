use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;

use api::instance::spec::SetParameterCommand;
use api::task::graph::modify::AudioGraphModification;
use api::task::graph::{AudioGraphSpec, InputId, NodeId, OutputId};
use api::task::player::{GraphPlaybackEvent, NodeEvent, PlayHead, PlayId};
use api::task::{DesiredTaskPlayState, PlayRequest};

use crate::audio_device::{AudioDevices, DeviceClientCommand};
use crate::buffer::NodeBuffers;
use crate::connection::Connection;
use crate::player::work_set::WorkSet;
use crate::BoxedNode;
use crate::{NodeInfo, Result};

mod device;
mod error;
mod structure;
mod work_set;

pub struct GraphPlayer {
  /// The client id used for device subscriptions
  pub(crate) client_id:         String,
  /// current specifications
  pub(crate) specs:             AudioGraphSpec,
  /// materialized sources
  pub(crate) node_apis:         HashMap<NodeId, Arc<RwLock<BoxedNode>>>,
  /// connection buffers
  pub(crate) connections:       HashMap<(OutputId, InputId), Connection>,
  /// Receive control messages
  pub(crate) rx_control:        Receiver<PlayerControlCommand>,
  /// Send device commands (clone when subscribing to device)
  pub(crate) tx_device:         Sender<DeviceClientCommand>,
  /// Receive device commands
  pub(crate) rx_device:         Receiver<DeviceClientCommand>,
  /// Receive parameter updates
  pub(crate) rx_params:         Receiver<UpdateParameterCommand>,
  /// Send internal updates from spawned tasks
  pub(crate) tx_tasks:          Sender<InternalTaskEvent>,
  /// Receive internal updates from spawned tasks
  pub(crate) rx_tasks:          Receiver<InternalTaskEvent>,
  /// Send player events
  pub(crate) tx_events:         Sender<GraphPlaybackEvent>,
  /// Play head
  pub(crate) play_head:         PlayHead,
  /// Node info and buffers
  pub(crate) node_state:        HashMap<NodeId, PlayerNodeState>,
  /// Audio devices to send commands to
  pub(crate) audio_devices:     AudioDevices,
  /// Current work set
  pub(crate) current_work_set:  WorkSet,
  /// Partial work sets that have pending
  pub(crate) partial_work_sets: VecDeque<WorkSet>,
  /// Pending structural changes
  pub(crate) pending_changes:   VecDeque<AudioGraphModification>,
}

#[derive(Debug)]
pub struct PlayerNodeState {
  /// Id
  pub(crate) id:                        NodeId,
  /// Info about the node
  pub(crate) info:                      NodeInfo,
  /// If Some(x) then currently processing task x
  pub(crate) processing:                Option<u64>,
  /// Pre-allocated Node I/O buffers
  pub(crate) buffers:                   NodeBuffers,
  /// Audio device dependencies (will only process within this device buffer flip context)
  pub(crate) audio_device_requirements: HashSet<String>,
  /// Nodes that depend on this node
  pub(crate) node_requirements:         HashSet<NodeId>,
  /// Actual source connection IDs
  pub(crate) node_inputs:               HashMap<InputId, HashSet<OutputId>>,
  /// Accumulated latency
  pub(crate) accumulated_latency:       usize,
}

#[derive(Clone, Debug)]
pub struct GraphPlayerHandle {
  play_id:    PlayId,
  tx_params:  Sender<UpdateParameterCommand>,
  tx_control: Sender<PlayerControlCommand>,
}

impl GraphPlayerHandle {
  pub async fn set_play(&mut self, request: PlayRequest) -> Result {
    self.play_id = request.play_id;
    let desired = DesiredTaskPlayState::Play(request);

    self.tx_control
        .send(PlayerControlCommand::SetDesiredPlaybackState { desired })
        .await?;

    Ok(())
  }

  pub async fn stop(&mut self) -> Result {
    self.tx_control
        .send(PlayerControlCommand::SetDesiredPlaybackState { desired: DesiredTaskPlayState::Idle, })
        .await?;

    Ok(())
  }

  pub async fn seek(&mut self, seek_to: u64) -> Result {
    self.tx_control
        .send(PlayerControlCommand::Seek { play_id: self.play_id,
                                           seek_to })
        .await?;

    Ok(())
  }
}

impl GraphPlayer {
  pub async fn run(mut self) {
    loop {
      select! {
        Some(control_msg) = self.rx_control.recv() => {
          self.handle_control_cmd(control_msg);
        },
        Some(device_msg) = self.rx_device.recv() => {
          self.handle_device_cmd(device_msg);
        },
        Some(params_msg) = self.rx_params.recv() => {
          self.handle_params_cmd(params_msg);
        }
        Some(task_msg) = self.rx_tasks.recv() => {
          self.handle_task_msg(task_msg);
        }
        else => break,
      }
    }
  }

  fn handle_device_cmd(&mut self, cmd: DeviceClientCommand) {
    match cmd {
      | DeviceClientCommand::Flip { device_id,
                                    buffers,
                                    generation,
                                    deadline, } =>
        if let Err(err) = self.device_flip_buffers(device_id, buffers, generation, deadline) {
          self.handle_error(err);
        },
      | DeviceClientCommand::Registered { device_id } => {}
      | DeviceClientCommand::Unregistered { device_id } => {}
    }
  }

  fn handle_control_cmd(&mut self, cmd: PlayerControlCommand) {
    match cmd {
      | PlayerControlCommand::SetDesiredPlaybackState { desired } => {}
      | PlayerControlCommand::ModifyGraph { modifications } => {
        self.pending_changes.extend(modifications.into_iter());
      }
      | PlayerControlCommand::Seek { play_id, seek_to } => {}
    }
  }

  async fn handle_params_cmd(&mut self, UpdateParameterCommand { node, changes }: UpdateParameterCommand) {
    let Some(node) = self.node_apis.get(&node) else { return };
    let mut node = node.write().await;
    for change in changes {
      node.set_parameter(&change);
    }
  }

  fn handle_task_msg(&mut self, cmd: InternalTaskEvent) {
    match cmd {
      | InternalTaskEvent::Completed { node_id,
                                       result,
                                       generation, } =>
        if let Err(err) = self.task_completed(node_id, generation, result) {
          self.handle_error(err);
        },
    }
  }
}

#[derive(Debug)]
pub enum PlayerControlCommand {
  SetDesiredPlaybackState { desired: DesiredTaskPlayState },
  Seek { play_id: PlayId, seek_to: u64 },
  ModifyGraph { modifications: Vec<AudioGraphModification> },
}

pub struct UpdateParameterCommand {
  node:    NodeId,
  changes: Vec<SetParameterCommand>,
}

#[derive(Debug)]
pub enum InternalTaskEvent {
  Completed {
    node_id:    NodeId,
    result:     Result<Vec<NodeEvent>>,
    generation: u64,
  },
}
