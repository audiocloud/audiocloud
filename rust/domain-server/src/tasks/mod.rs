use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use api::graph::AudioGraphSpec;
use api::task::InstanceAllocationRequest;
use api::Timestamp;

pub mod run;

pub type Result<T = ()> = anyhow::Result<T>;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TaskSpec {
  pub app_id:     String,
  pub from:       Timestamp,
  pub to:         Timestamp,
  pub requests:   HashMap<String, InstanceAllocationRequest>,
  pub instances:  HashMap<String, String>, // this is the allocation bit
  pub graph_spec: AudioGraphSpec,
}