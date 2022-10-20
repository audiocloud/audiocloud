use std::collections::{HashMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppMediaObjectId, AppTaskId, CreateTaskReservation, CreateTaskSecurity, CreateTaskSpec,
    FixedInstanceId, InstancePlayState, MediaObject, ModifyTaskSpec, TaskPlayState, TaskSpec,
};
pub use crate::audio_engine::{
    TaskPlaying, TaskPlayStopped, TaskRenderCancelled, TaskRendering, TaskSought,
};

/// A summary of a task
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TaskSummary {
    /// Task Id
    pub task_id: AppTaskId,
    /// Current play sate
    pub play_state: TaskPlayState,
    /// List of instances that are blocking play state change
    pub waiting_for_instances: HashSet<FixedInstanceId>,
    /// List of media that are blocking or influencing completeness of play state change
    pub waiting_for_media: HashSet<AppMediaObjectId>,
}

/// A more complete information about a task
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TaskWithStatusAndSpec {
    /// Task Id
    pub task_id: AppTaskId,
    /// Current play state
    pub play_state: TaskPlayState,
    /// State of attatched fixed instances
    pub instances: HashMap<FixedInstanceId, InstancePlayState>,
    /// State of attached media objects
    pub media: HashMap<AppMediaObjectId, MediaObject>,
    /// The current specification of the task
    pub spec: TaskSpec,
}

pub type TaskSummaryList = Vec<TaskSummary>;

/// Create a task on the domain
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CreateTask {
    /// The new app id
    pub task_id: AppTaskId,
    /// Task reservations
    pub reservations: CreateTaskReservation,
    /// Task specification
    pub spec: CreateTaskSpec,
    /// Security keys and associateds permissions
    pub security: CreateTaskSecurity,
}

/// Response to creating a task on the domain
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskCreated {
    /// Created normally
    Created {
        /// Task Id
        task_id: AppTaskId,
    },
}

/// Request to modify a task on the domain
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ModifyTask {
    /// A list of modifications to apply
    pub modify_spec: Vec<ModifyTaskSpec>,
}

/// Response to modifying a task on the domain
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskUpdated {
    /// Updated normally
    Updated {
        /// Task Id
        task_id: AppTaskId,
        /// New version to be used with `If-Matches` when submitting further modifications
        revision: u64,
    },
    /// Did not update because a newer revision was specified and update is optional
    Ignored {
        /// Task Id
        task_id: AppTaskId,
        /// Current version to be used with `If-Matches` when submitting further modifications
        revision: u64,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskDeleted {
    Deleted { id: AppTaskId },
}

