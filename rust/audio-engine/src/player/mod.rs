use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{BitOr, BitOrAssign};
use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, RwLock};
use tokio::{select, spawn};

use api::instance::spec::SetParameterCommand;
use api::media::spec::MediaId;
use api::task::graph::modify::AudioGraphModification;
use api::task::graph::{AudioGraphSpec, InputId, NodeId, OutputId};
use api::task::player::{GraphPlayerEvent, NodeEvent, PlayHead, PlayId};
use api::task::{DesiredTaskPlayState, PlayRequest};

use crate::audio_device::{AudioDevices, DeviceClientCommand};
use crate::buffer::NodeBuffers;
use crate::connection::Connection;
use crate::player::work_set::WorkSet;
use crate::BoxedNode;
use crate::{NodeInfo, Result};

mod device;
mod error;
mod init;
mod structure;
mod work_set;

pub trait MediaResolver: Send + Sync {
  fn resolve(&self, media_id: &MediaId) -> Result<String>;
}
pub type BoxedMediaResolver = Box<dyn MediaResolver>;

pub trait DeviceInstanceResolver: Send + Sync {
  fn resolve(&self, instance_id: &str) -> Result<DeviceInstanceAttachment>;
}
pub type BoxedDeviceInstanceResolver = Box<dyn DeviceInstanceResolver>;

pub struct DeviceInstanceAttachment {
  pub device_id:          String,
  pub sends:              Vec<u32>,
  pub returns:            Vec<u32>,
  pub additional_latency: usize,
}

pub struct GraphPlayer {
  /// The client id used for device subscriptions
  pub(crate) client_id:                String,
  /// current specifications
  pub(crate) specs:                    AudioGraphSpec,
  /// materialized nodes
  pub(crate) node_apis:                HashMap<NodeId, Arc<RwLock<BoxedNode>>>,
  /// connection buffers
  pub(crate) connections:              HashMap<(OutputId, InputId), Connection>,
  /// Receive control messages
  pub(crate) rx_control:               Receiver<PlayerControlCommand>,
  /// Send device commands (clone when subscribing to device)
  pub(crate) tx_device:                Sender<DeviceClientCommand>,
  /// Receive device commands
  pub(crate) rx_device:                Receiver<DeviceClientCommand>,
  /// Receive parameter updates
  pub(crate) rx_params:                Receiver<PlayerParameterCommand>,
  /// Send internal updates from spawned tasks
  pub(crate) tx_tasks:                 Sender<InternalTaskEvent>,
  /// Receive internal updates from spawned tasks
  pub(crate) rx_tasks:                 Receiver<InternalTaskEvent>,
  /// Send player events
  pub(crate) tx_events:                Sender<GraphPlayerEvent>,
  /// Play head
  pub(crate) play_head:                PlayHead,
  /// Node info and buffers
  pub(crate) node_state:               HashMap<NodeId, PlayerNodeState>,
  /// Audio devices to send commands to
  pub(crate) audio_devices:            AudioDevices,
  /// Current work set
  pub(crate) current_work_set:         WorkSet,
  /// Partial work sets that have pending
  pub(crate) partial_work_sets:        VecDeque<WorkSet>,
  /// Pending structural changes
  pub(crate) pending_changes:          VecDeque<AudioGraphModification>,
  /// Media resolver
  pub(crate) media_resolver:           Box<dyn MediaResolver>,
  /// Device instance resolver
  pub(crate) device_instance_resolver: Box<dyn DeviceInstanceResolver>,
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
  pub(crate) node_inputs:               HashMap<InputId, Vec<OutputId>>,
  /// Accumulated latency
  pub(crate) accumulated_latency:       usize,
}

#[derive(Debug)]
pub struct GraphPlayerHandle {
  play_id:    PlayId,
  tx_params:  Sender<PlayerParameterCommand>,
  tx_control: Sender<PlayerControlCommand>,
  rx_events:  Receiver<GraphPlayerEvent>,
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

  pub fn new(devices: AudioDevices,
             media_resolver: BoxedMediaResolver,
             device_instance_resolver: BoxedDeviceInstanceResolver,
             spec: AudioGraphSpec)
             -> Result<Self> {
    let (tx_control, rx_control) = mpsc::channel(0x100);
    let (tx_params, rx_params) = mpsc::channel(0x100);
    let (tx_events, rx_events) = mpsc::channel(0x100);

    let mut rv = GraphPlayer::new(devices,
                                  media_resolver,
                                  device_instance_resolver,
                                  spec,
                                  rx_control,
                                  rx_params,
                                  tx_events)?;

    let play_id = PlayId::default();

    spawn(rv.run());

    Ok(Self { play_id,
              tx_params,
              tx_control,
              rx_events })
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
          self.handle_device_cmd(device_msg).await;
        },
        Some(params_msg) = self.rx_params.recv() => {
          self.handle_params_cmd(params_msg);
        }
        Some(task_msg) = self.rx_tasks.recv() => {
          self.handle_task_msg(task_msg).await;
        }
        else => break,
      }
    }
  }

  async fn handle_device_cmd(&mut self, cmd: DeviceClientCommand) {
    match cmd {
      | DeviceClientCommand::Flip { device_id,
                                    buffers,
                                    generation,
                                    deadline, } =>
        if let Err(err) = self.device_flip_buffers(device_id, buffers, generation, deadline).await {
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

  fn handle_params_cmd(&mut self, PlayerParameterCommand { node, changes }: PlayerParameterCommand) {
    let Some(node) = self.node_apis.get(&node) else { return };
    let node = node.clone();

    spawn(async move {
      let mut node = node.write().await;
      for change in changes {
        node.set_parameter(&change);
      }
    });
  }

  async fn handle_task_msg(&mut self, cmd: InternalTaskEvent) {
    match cmd {
      | InternalTaskEvent::Completed { node_id,
                                       result,
                                       generation, } =>
        if let Err(err) = self.task_completed(node_id, generation, result).await {
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

pub struct PlayerParameterCommand {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlayerChangeOutcome {
  NoAction,
  ConnectionSync,
  NeedReset,
}

impl BitOrAssign for PlayerChangeOutcome {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl PlayerChangeOutcome {
  pub fn from_needs_reset(needs_reset: bool) -> Self {
    if needs_reset {
      Self::NeedReset
    } else {
      Self::NoAction
    }
  }
}

impl BitOr for PlayerChangeOutcome {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      | (Self::NoAction, Self::NoAction) => Self::NoAction,
      | (Self::NoAction, Self::ConnectionSync) | (Self::ConnectionSync, Self::NoAction) => Self::ConnectionSync,
      | (_, _) => Self::NeedReset,
    }
  }
}
