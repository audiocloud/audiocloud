use std::collections::{HashMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::audio_engine::{
    TaskPlayStopped, TaskPlaying, TaskRenderCancelled, TaskRendering, TaskSought,
};
use crate::{
    AppMediaObjectId, AppTaskId, CreateTaskReservation, CreateTaskSecurity, CreateTaskSpec,
    FixedInstanceId, InstancePlayState, MediaObject, ModifyTaskSpec, TaskPlayState, TaskSpec,
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

/// List tasks
///
/// Return a list of all current tasks and their status.
#[utoipa::path(
  get,
  path = "/v1/tasks",
  responses(
    (status = 200, description = "Success", body = TaskSummaryList),
    (status = 401, description = "Not authorized", body = DomainError),
  ))]
pub(crate) fn list_tasks() {}

/// Get task details
///
/// Get details of a task, including dependent media and instance statuses
#[utoipa::path(
  get,
  path = "/v1/tasks/{app_id}/{task_id}",
  responses(
    (status = 200, description = "Success", body = TaskWithStatusAndSpec),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn get_task() {}

/// Create a task
///
/// In standalone mode, the task will be checked for mutual exclusivity with other tasks, otherwise
/// it will be created. This call could also fail if the referenced resources (such as fixed
/// instances) do not exist.
#[utoipa::path(
  post,
  path = "/v1/tasks",
  request_body = CreateTask,
  responses(
    (status = 200, description = "Success", body = TaskCreated),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
    (status = 409, description = "Overlapping task exists", body = DomainError),
  ))]
pub(crate) fn create_task() {}

/// Modify existing task
///
/// Submit modifications to the task. This generic request can be used to update most aspects of the
/// session: adjusting parameters, creating, deleting, reconnecting nodes, changing media, etc.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/modify",
  request_body = ModifyTask,
  responses(
    (status = 200, description = "Success", body = TaskUpdated),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
    (status = 409, description = "Not allowed to change instances", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version to be changed"),
  ))]
pub(crate) fn modify_task() {}

/// Delete a task
///
/// Delete a task and release all referenced resources.
#[utoipa::path(
  delete,
  path = "/v1/tasks/{app_id}/{task_id}",
  responses(
    (status = 200, description = "Success", body = TaskDeleted),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn delete_task() {}

/// Render a task to a new file
///
/// The domain will check that
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/render",
  request_body = RequestRender,
  responses(
    (status = 200, description = "Success", body = TaskRendering),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn render_task() {}

/// Start playing a task
///
/// Start playing a task that is stopped. The request will return when the task has started to play
/// or with an error.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/play",
  request_body = RequestPlay,
  responses(
    (status = 200, description = "Success", body = TaskPlaying),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version"),
  ))]
pub(crate) fn play_task() {}

/// Seek while task is playing
///
/// If the task is playing, change the playing position.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/seek",
  request_body = RequestSeek,
  responses(
    (status = 200, description = "Success", body = TaskSought),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn seek_task() {}

/// Cancel rendering a task
///
/// Request to stop (cancel) rendering if the task is rendering.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/cancel",
  request_body = RequestCancelRender,
  responses(
    (status = 200, description = "Success", body = TaskRenderCancelled),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version"),
  ))]
pub(crate) fn cancel_render_task() {}

/// Stop playing a task
///
/// Request to stop a track if the task is playing.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/stop",
  request_body = RequestStopPlay,
  responses(
    (status = 200, description = "Success", body = TaskPlayStopped),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version"),
  ))]
pub(crate) fn stop_playing_task() {}
