use std::collections::HashMap;

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use graph::{modify::AudioGraphModification, AudioGraphSpec};
use player::{GraphPlayerEvent, PlayId};

use crate::instance::driver::events::InstanceDriverEvent;
use crate::task::graph::{SinkId, SinkSpec};
use crate::task::spec::TaskSpec;
use crate::Timestamp;

pub mod graph;
pub mod player;
pub mod spec;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
  pub app:       String,
  pub engine:    String,
  pub from:      Timestamp,
  pub to:        Timestamp,
  pub instances: HashMap<String, InstanceAllocationRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum CreateTaskResponse {
  Success { app_id: String, task_id: String },
  OverlappingTask,
  NoSuchInstance { instance_id: String },
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InstanceAllocationRequest {
  Fixed { instance_id: String },
  Dynamic { model_id: String },
}

pub type SetTaskGraphRequest = AudioGraphSpec;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SetTaskGraphResponse {
  Success,
  NotFound,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetTaskTimeRequest {
  pub from: Timestamp,
  pub to:   Timestamp,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SetTaskTimeResponse {
  Success,
  NotFound,
  Failure,
}

pub type ModifyTaskGraphRequest = Vec<AudioGraphModification>;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ModifyTaskGraphResponse {
  Success,
  NotFound,
  Failure,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum DeleteTaskResponse {
  Success,
  NotFound,
  Failure,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskSummary {}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum TaskId {
  All,
  Specific(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvent {
  play_id:         Option<String>,
  instance_events: Vec<InstanceDriverEvent>,
  player_events:   Vec<GraphPlayerEvent>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum DesiredTaskPlayState {
  Idle,
  Play(PlayRequest),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayRequest {
  pub play_id:    PlayId,
  pub start:      u64,
  pub end:        u64,
  pub start_from: u64,
  pub looping:    bool,
  pub sinks:      HashMap<SinkId, SinkSpec>,
}

impl Default for DesiredTaskPlayState {
  fn default() -> Self {
    Self::Idle
  }
}

impl DesiredTaskPlayState {
  pub fn is_playing(&self) -> bool {
    matches!(self, Self::Play { .. })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskListRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskListResponse {
  pub tasks: HashMap<String, TaskSummary>,
}

pub mod buckets {
  use crate::task::spec::TaskSpec;
  use crate::task::DesiredTaskPlayState;
  use crate::{BucketKey, BucketName};

  pub const TASK_SPEC: BucketName<TaskSpec> = BucketName::new("audiocloud_task_spec");
  pub const TASK_CONTROL: BucketName<DesiredTaskPlayState> = BucketName::new("audiocloud_task_control");
  pub const TASK_STATE: BucketName<()> = BucketName::new("audiocloud_task_state");

  pub fn task_spec_key(task_id: impl ToString) -> BucketKey<String, TaskSpec> {
    task_id.to_string().into()
  }

  pub fn task_control_key(task_id: impl ToString) -> BucketKey<String, DesiredTaskPlayState> {
    task_id.to_string().into()
  }

  pub fn task_state_key(task_id: impl ToString) -> BucketKey<String, ()> {
    task_id.to_string().into()
  }
}

pub mod subjects {
  use crate::task::{GetTaskListRequest, GetTaskListResponse, SetTaskGraphRequest, SetTaskGraphResponse};
  use crate::Request;

  pub fn get_task_list_req() -> Request<GetTaskListRequest, GetTaskListResponse> {
    Request::new("audiocloud_get_task_list")
  }
  pub fn set_task_graph_req() -> Request<SetTaskGraphRequest, SetTaskGraphResponse> {
    Request::new("audiocloud_set_task_graph")
  }
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(TaskSpec),
                 schema_for!(DesiredTaskPlayState),
                 schema_for!(TaskEvent),
                 schema_for!(CreateTaskRequest),
                 schema_for!(CreateTaskResponse)].into_iter())
}
