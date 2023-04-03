use std::collections::HashMap;

use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type SourceId = u64;
pub type InsertId = u64;
pub type SinkId = u64;
pub type BusId = u64;
pub type PlayId = u64;

/// Specification of a graph, which can be later be created or modified
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioGraphSpec {
  pub sources: HashMap<SourceId, SourceSpec>,
  pub inserts: HashMap<InsertId, InsertSpec>,
  pub busses:  HashMap<BusId, BusSpec>,
}

/// Reference to an output channel of a graph
#[derive(Debug, Display, Serialize, Deserialize, JsonSchema, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum OutputId {
  #[display(fmt = "source {_0}, output channel {_1}")]
  Source(SourceId, usize),
  #[display(fmt = "insert {_0}, output channel {_1}")]
  Insert(InsertId, usize),
  #[display(fmt = "bus {_0}, output channel {_1}")]
  Bus(BusId, usize),
}

/// Reference to an input channel of a graph
#[derive(Debug, Display, Serialize, Deserialize, JsonSchema, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum InputId {
  #[display(fmt = "insert {_0}, input channel {_1}")]
  Insert(InsertId, usize),
  #[display(fmt = "bus {_0}, input channel {_1}")]
  Bus(BusId, usize),
  #[display(fmt = "sink {_0}, input channel {_1}")]
  Sink(SinkId, usize),
}

impl Into<NodeId> for OutputId {
  fn into(self) -> NodeId {
    match self {
      | OutputId::Source(id, _) => NodeId::Source(id),
      | OutputId::Insert(id, _) => NodeId::Insert(id),
      | OutputId::Bus(id, _) => NodeId::Bus(id),
    }
  }
}

impl Into<NodeId> for InputId {
  fn into(self) -> NodeId {
    match self {
      | InputId::Insert(id, _) => NodeId::Insert(id),
      | InputId::Bus(id, _) => NodeId::Bus(id),
      | InputId::Sink(id, _) => NodeId::Sink(id),
    }
  }
}

/// A component within the graph
#[derive(Debug, Display, Serialize, Deserialize, JsonSchema, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum NodeId {
  #[display(fmt = "source {_0}")]
  Source(SourceId),
  #[display(fmt = "insert {_0}")]
  Insert(InsertId),
  #[display(fmt = "bus {_0}")]
  Bus(BusId),
  #[display(fmt = "sink {_0}")]
  Sink(SinkId),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpec {
  pub start_at:     u64,
  pub source_url:   String,
  pub num_channels: usize,
}

/// Specification of a software summing bus
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BusSpec {
  pub mid_side_mode: Option<MidSideMode>,
  pub inputs:        Vec<Vec<InputSpec>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MidSideMode {
  EncodeToMidSide,
  DecodeToLeftRight,
}

/// Specification of an insert instance within the graph (e.g. an external hardware, or VST plugin) that can be connected to the graph
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsertSpec {
  pub inputs:      Vec<Vec<InputSpec>>,
  pub instance_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SinkSpec {
  pub inputs:      Vec<Vec<InputSpec>>,
  pub sample_rate: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayRegion {
  pub start:   u64,
  pub end:     u64,
  pub looping: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct InputSpec {
  pub source: OutputId,
  pub gain:   f64,
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
pub enum AudioGraphModification {
  #[display(fmt = "add source {source_id} with spec {source_spec:?}")]
  AddSource { source_id: SourceId, source_spec: SourceSpec },
  #[display(fmt = "add insert {insert_id} with spec {insert_spec:?}")]
  AddInsert { insert_id: InsertId, insert_spec: InsertSpec },
  #[display(fmt = "add bus {bus_id} with spec {bus_spec:?}")]
  AddBus { bus_id: BusId, bus_spec: BusSpec },
  #[display(fmt = "remove source {source_id}")]
  RemoveSource { source_id: SourceId },
  #[display(fmt = "remove insert {insert_id}")]
  RemoveInsert { insert_id: InsertId },
  #[display(fmt = "remove bus {bus_id}")]
  RemoveBus { bus_id: BusId },
  #[display(fmt = "connect component {component} input {input_channel} to {output}")]
  Connect {
    component:     NodeId,
    input_channel: usize,
    output:        OutputId,
  },
  #[display(fmt = "disconnect component {component} input {input_channel} from {output}")]
  Disconnect {
    component:     NodeId,
    input_channel: usize,
    output:        OutputId,
  },
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

/// Validation errors for the graph to be created or modified
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Error)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum GraphModificationError {
  /// The output to be linked does not exist
  #[error("component {component} input {input} references output {output} that does not exist")]
  InputSourceNotFound {
    component: NodeId,
    input:     usize,
    output:    OutputId,
  },
  /// Creating this connection would create a loop
  #[error("connecting component {component} input {input} to output {output} would create a loop")]
  LoopDetected {
    component: NodeId,
    input:     usize,
    output:    OutputId,
  },
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

/// Statistics of the graph playback
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphPlaybackStats {
  pub play_id:   PlayId,
  pub state:     GraphPlaybackState,
  pub sources:   HashMap<SourceId, SourceStats>,
  pub instances: HashMap<InsertId, InstanceStats>,
  pub busses:    HashMap<BusId, BusStats>,
  pub sinks:     HashMap<SinkId, SinkStats>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceStats {
  pub sample_rate: u64,
  pub peaks:       Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstanceStats {
  pub sample_rate: u64,
  pub inputs:      Vec<Vec<f64>>,
  pub outputs:     Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BusStats {
  pub sample_rate: u64,
  pub inputs:      Vec<Vec<f64>>,
  pub outputs:     Vec<Vec<f64>>,
}

/// Statistics emitted by a player while playing back a graph
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SinkStats {
  pub sample_rate:     u64,
  pub inputs:          Vec<Vec<f64>>,
  pub lufs_integrated: Vec<f64>,
  pub lufs_short_term: Vec<f64>,
  pub lufs_momentary:  Vec<f64>,
}

/// Audio data emitted by a player while playing back a graph
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphPlaybackAudio {
  pub play_id: PlayId,
  pub sinks:   HashMap<SinkId, Vec<u8>>,
}

/// Events emitted by a player while playing back a graph
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
pub enum GraphPlaybackEvent {
  Error(GraphPlaybackError),
  Stats(GraphPlaybackStats),
  Audio(GraphPlaybackAudio),
}
