use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::task::graph::AudioGraphSpec;
use crate::task::InstanceAllocationRequest;
use crate::Timestamp;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TaskSpec {
  pub app_id:     String,
  pub host_id:    String,
  pub from:       Timestamp,
  pub to:         Timestamp,
  pub requests:   HashMap<String, InstanceAllocationRequest>,
  pub instances:  HashMap<String, String>,
  // this is the allocation bit
  pub graph_spec: AudioGraphSpec,
}
