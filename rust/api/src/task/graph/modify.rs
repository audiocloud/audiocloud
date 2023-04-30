use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{BusId, BusSpec, InsertId, InsertSpec, NodeId, OutputId, SourceId, SourceSpec};

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
  #[display(fmt = "set source {source_id} file path to {path}")]
  SetSourcePath { source_id: SourceId, path: String },
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
