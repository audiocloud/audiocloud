use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::task::graph::VirtualInsertSpec;

use super::{BusId, BusSpec, DeviceInsertSpec, InsertId, NodeId, OutputId, SourceId, SourceSpec};

#[derive(Debug, PartialEq, Display, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AudioGraphModification {
  #[display(fmt = "add source {source_id} with spec {source_spec:?}")]
  AddOrReplaceSource { source_id: SourceId, source_spec: SourceSpec },
  #[display(fmt = "add device insert {insert_id} with spec {insert_spec:?}")]
  AddOrReplaceDeviceInsert {
    insert_id:   InsertId,
    insert_spec: DeviceInsertSpec,
  },
  #[display(fmt = "add insert {insert_id} with spec {insert_spec:?}")]
  AddOrReplaceVirtualInsert {
    insert_id:   InsertId,
    insert_spec: VirtualInsertSpec,
  },
  #[display(fmt = "add bus {bus_id} with spec {bus_spec:?}")]
  AddOrReplaceBus { bus_id: BusId, bus_spec: BusSpec },
  #[display(fmt = "remove source {source_id}")]
  RemoveSource { source_id: SourceId },
  #[display(fmt = "remove device insert {insert_id}")]
  RemoveDeviceInsert { insert_id: InsertId },
  #[display(fmt = "remove virtual insert {insert_id}")]
  RemoveVirtualInsert { insert_id: InsertId },
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
