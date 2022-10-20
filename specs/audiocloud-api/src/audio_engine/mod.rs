//! The API to the audio engine (from the domain side)

use std::collections::HashMap;

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use command::*;
pub use event::*;

use crate::common::media::PlayId;
use crate::{
    merge_schemas, AppId, AppMediaObjectId, AppTaskId, FixedInstanceId, MediaObject, ModifyTaskError, RenderId, TaskId, TaskPlayState,
    TaskSpec,
};

pub mod command;
pub mod event;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CompressedAudio {
    pub play_id:      PlayId,
    pub timeline_pos: f64,
    pub stream_pos:   u64,
    pub buffer:       bytes::Bytes,
    pub num_samples:  usize,
    pub last:         bool,
}

#[derive(Debug, Clone, Error, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EngineError {
    #[error("Track {0} not found")]
    TrackNotFound(usize),

    #[error("Item {0} on track {1} not found")]
    ItemNotFound(usize, usize),

    #[error("Task {0} failed to modify: {1}")]
    ModifyTask(AppTaskId, ModifyTaskError),

    #[error("Internal sound engine error: {0}")]
    InternalError(String),

    #[error("Remote call failed: {0}")]
    RPC(String),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskReplaced {
    Updated { task_id: AppTaskId },
    Created { task_id: AppTaskId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskModified {
    Modified { task_id: AppTaskId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskDeleted {
    Deleted { task_id: AppTaskId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskSought {
    Sought { task_id: AppTaskId, play_id: PlayId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaUpdated {
    Updated { added: usize, replaced: usize, deleted: usize },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EngineFixedInstance {
    pub input_start:  u32,
    pub output_start: u32,
    pub num_inputs:   u32,
    pub num_outputs:  u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetInstances {
    instances: HashMap<FixedInstanceId, EngineFixedInstance>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetMedia {
    media: HashMap<AppMediaObjectId, MediaObject>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstancesUpdated {
    Updated { added: usize, replaced: usize, deleted: usize },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TaskWithStatus {
    pub id:         AppTaskId,
    pub spec:       TaskSpec,
    pub play_state: TaskPlayState,
}

pub type TaskWithStatusList = Vec<TaskWithStatus>;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskRendering {
    Rendering { task_id: AppTaskId, render_id: RenderId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskPlaying {
    Playing { task_id: AppTaskId, play_id: PlayId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskMixerChanged {
    Changed { task_id: AppTaskId, play_id: PlayId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskPlayStopped {
    Stopped { task_id: AppTaskId, play_id: PlayId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskRenderCancelled {
    Cancelled { task_id: AppTaskId, render_id: RenderId },
}

pub fn schemas() -> RootSchema {
    merge_schemas([schema_for!(EngineError),
                   schema_for!(TaskReplaced),
                   schema_for!(TaskDeleted),
                   schema_for!(TaskModified),
                   schema_for!(TaskPlaying),
                   schema_for!(TaskSought),
                   schema_for!(TaskPlayStopped),
                   schema_for!(TaskRendering),
                   schema_for!(TaskRenderCancelled),
                   schema_for!(MediaUpdated),
                   schema_for!(InstancesUpdated),
                   schema_for!(EngineFixedInstance),
                   schema_for!(SetInstances),
                   schema_for!(SetMedia),
                   schema_for!(TaskWithStatusList),
                   schema_for!(TaskWithStatus),
                   schema_for!(SetMedia),
                   schema_for!(SetInstances),
                   schema_for!(AppId),
                   schema_for!(TaskId),
                   schema_for!(crate::RequestPlay),
                   schema_for!(crate::RequestSeek),
                   schema_for!(crate::RequestChangeMixer),
                   schema_for!(crate::RequestStopPlay),
                   schema_for!(crate::RequestCancelRender),
                   schema_for!(crate::ModifyTaskSpec),
                   schema_for!(crate::TaskSpec)].into_iter())
}
