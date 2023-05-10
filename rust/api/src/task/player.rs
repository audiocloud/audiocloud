use std::collections::HashMap;

use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::instance::model::{ParameterModel, ReportModel};
use crate::task::graph::{NodeId, SinkId, SinkSpec};

pub type PlayId = u64;

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayRegion {
  pub start:   u64,
  pub end:     u64,
  pub looping: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Play {
  pub region:   PlayRegion,
  pub start_at: u64,
  pub sinks:    HashMap<SinkId, SinkSpec>,
}

#[derive(Debug, Display, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum UpdatePlayer {
  /// Set the active play region and/or play position
  #[display(fmt = "set play region to {play_region:?} and position to {position:?} and looping to {looping:?}")]
  SetPlayRegion {
    /// If present, override the current play region
    play_region: Option<PlayRegion>,
    /// If present, override the looping flag on the player
    looping:     Option<bool>,
    /// If present, override the current play position
    position:    Option<u64>,
  },

  /// Stop the playback
  #[display(fmt = "stop")]
  Stop,
}

/// Validation errors for the graph to be played back
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Error)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum GraphPlaybackError {
  /// The sinks are using mutually incompatible settings or play configuration (looping, sample rate, ...)
  #[error("sink {sink} is not connected to any input: {error}")]
  IncompatibleSinks { sink: SinkId, error: String },

  /// The graph is already playing by another player. E_NO_EXCLUSIVE_ACCESS
  #[error("graph is already playing by player {play_id}")]
  GraphAlreadyPlaying { play_id: PlayId },
}

/// State of the graph playback
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "position")]
pub enum GraphPlaybackState {
  /// The graph is not playing, it is buffering from the inputs
  Buffering(u64),
  /// The graph is playing
  Playing(u64),
  /// The graph has reached end of play region and is not looping, so it stopped
  Stopped,
}

/// Audio data emitted by a player while playing back a graph
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphPlaybackAudio {
  pub play_id: PlayId,
  pub sinks:   Vec<(SinkId, bytes::Bytes)>,
}

/// Events emitted by a player while playing back a graph
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
pub enum GraphPlayerEvent {
  Error(GraphPlaybackError),
  GraphStateChanged {
    state: GraphPlaybackState,
  },
  GraphNodesPrepared {
    play_id: PlayId,
    nodes:   HashMap<NodeId, NodeInfo>,
  },
  GraphSinkCaptured {
    play_id:   PlayId,
    play_head: PlayHead,
    sink_id:   SinkId,
    data:      bytes::Bytes,
  },
  NodeEvents {
    play_id: PlayId,
    events:  Vec<(NodeId, NodeEvent)>,
  },
}

// Information about a playhead
#[derive(Copy, Clone, Debug, Default, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayHead {
  pub sample_rate: u32,
  pub buffer_size: u32,
  pub play_region: PlayRegion,
  pub play_id:     u64,
  pub generation:  u64,
  pub position:    u64,
}

impl PlayHead {
  pub fn advance_position(self) -> Self {
    self.advance_position_by(self.buffer_size as usize)
  }

  pub fn advance_position_by(self, len: usize) -> Self {
    let generation = self.generation + 1;
    let position_end = self.position + len as u64;
    let play_region = self.play_region;

    let position = if position_end > play_region.end {
      if play_region.looping {
        play_region.start + (position_end - play_region.end)
      } else {
        play_region.end
      }
    } else {
      position_end
    };

    Self { generation,
           position,
           ..self }
  }

  pub fn with_play_region(self, play_region: PlayRegion) -> Self {
    let generation = self.generation + 1;
    let play_id = self.play_id + 1;

    Self { play_region,
           generation,
           play_id,
           ..self }
  }

  pub fn playing_segment_size(&self) -> usize {
    ((self.play_region.end - self.position) as i64).min(self.buffer_size as i64).max(0) as usize
  }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
  pub latency:     usize,
  pub num_inputs:  usize,
  pub num_outputs: usize,
  #[serde(default)]
  pub parameters:  HashMap<String, ParameterModel>,
  #[serde(default)]
  pub reports:     HashMap<String, ReportModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum NodeEvent {
  #[serde(rename_all = "camelCase")]
  Report { name: String, channel: usize, value: f64 },
}
