use std::collections::HashMap;

use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::media::spec::MediaId;

pub mod modify;

pub type SourceId = u64;
pub type InsertId = u64;
pub type SinkId = u64;
pub type BusId = u64;

/// Specification of a graph, which can be later be created or modified
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone, Default)]
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
  #[display(fmt = "device insert {_0}, output channel {_1}")]
  DeviceInsert(InsertId, usize),
  #[display(fmt = "virtual insert {_0}, output channel {_1}")]
  VirtualInsert(InsertId, usize),
  #[display(fmt = "bus {_0}, output channel {_1}")]
  Bus(BusId, usize),
}

/// Reference to an input channel of a graph
#[derive(Debug, Display, Serialize, Deserialize, JsonSchema, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum InputId {
  #[display(fmt = "device insert {_0}, input channel {_1}")]
  DeviceInsert(InsertId, usize),
  #[display(fmt = "virtual insert {_0}, input channel {_1}")]
  VirtualInsert(InsertId, usize),
  #[display(fmt = "bus {_0}, input channel {_1}")]
  Bus(BusId, usize),
  #[display(fmt = "device sink {_0}, input channel {_1}")]
  DeviceSink(SinkId, usize),
  #[display(fmt = "streaming sink {_0}, input channel {_1}")]
  StreamingSink(SinkId, usize),
}

impl Into<NodeId> for OutputId {
  fn into(self) -> NodeId {
    match self {
      | OutputId::Source(id, _) => NodeId::Source(id),
      | OutputId::DeviceInsert(id, _) => NodeId::DeviceInsert(id),
      | OutputId::VirtualInsert(id, _) => NodeId::VirtualInsert(id),
      | OutputId::Bus(id, _) => NodeId::Bus(id),
    }
  }
}

impl Into<NodeId> for InputId {
  fn into(self) -> NodeId {
    match self {
      | InputId::DeviceInsert(id, _) => NodeId::DeviceInsert(id),
      | InputId::VirtualInsert(id, _) => NodeId::VirtualInsert(id),
      | InputId::Bus(id, _) => NodeId::Bus(id),
      | InputId::DeviceSink(id, _) => NodeId::DeviceSink(id),
      | InputId::StreamingSink(id, _) => NodeId::StreamingSink(id),
    }
  }
}

/// A component within the graph
#[derive(Debug, Display, Serialize, Deserialize, JsonSchema, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum NodeId {
  #[display(fmt = "source {_0}")]
  Source(SourceId),
  #[display(fmt = "device insert {_0}")]
  DeviceInsert(InsertId),
  #[display(fmt = "virtual insert {_0}")]
  VirtualInsert(InsertId),
  #[display(fmt = "bus {_0}")]
  Bus(BusId),
  #[display(fmt = "device sink {_0}")]
  DeviceSink(SinkId),
  #[display(fmt = "streaming sink {_0}")]
  StreamingSink(SinkId),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpec {
  pub media_id:     MediaId,
  pub start_at:     u64,
  pub num_channels: usize,
}

/// Specification of a software summing bus
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BusSpec {
  pub inputs:      Vec<Vec<InputSpec>>,
  pub num_outputs: usize,
}

/// Specification of an insert instance within the graph (e.g. an external hardware, or VST plugin) that can be connected to the graph
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsertSpec {
  pub inputs:      Vec<Vec<InputSpec>>,
  pub instance_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SinkSpec {
  pub inputs:      Vec<Vec<InputSpec>>,
  pub sample_rate: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct InputSpec {
  pub source: OutputId,
  pub gain:   f64,
}
