use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::graph::{AudioGraphModification, AudioGraphSpec};
use crate::Timestamp;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
  pub app_id:    String,
  pub from:      Timestamp,
  pub to:        Timestamp,
  pub instances: HashMap<String, InstanceAllocationRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InstanceAllocationRequest {
  Fixed { instance_id: String },
  Dynamic { model_id: String },
}

pub type SetTaskGraphRequest = AudioGraphSpec;

pub type ModifyTaskGraphRequest = Vec<AudioGraphModification>;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskSummary {}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum TaskId {
  All,
  Specific(String),
}
