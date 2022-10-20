use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::change::ModifyTask;
use crate::time::Timestamp;
use crate::{AppId, CreateTaskReservation, CreateTaskSecurity, CreateTaskSpec, DomainId, TaskId};

/// Create a task
///
/// Tasks describe graphs of media operations that may execute in real time or unattended as a render.
/// They are allocated to a domain and an engine within that domain. Operations are executed with
/// the help of instances, which are fixed hardware blocks or dynamically instanced software
/// components.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct CreateTask {
    /// Domain that will be executing the task
    pub domain_id: DomainId,
    /// Task reservations
    pub reservations: CreateTaskReservation,
    /// Task specification
    pub spec: CreateTaskSpec,
    /// Security keys and associateds permissions
    pub security: CreateTaskSecurity,
    /// When true, do not actually create a task, just validate the process
    pub dry_run: bool,
}

/// Task created successfully
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskCreated {
    /// Created normally
    Created {
        /// App creating the task
        app_id: AppId,
        /// Task Id
        task_id: TaskId,
    },
    /// Validated successfully, but not created
    DryRun {
        /// App creating the task
        app_id: AppId,
        /// Task Id
        task_id: TaskId,
    },
}

/// Task was updated successfully
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskUpdated {
    /// Updated normally
    Updated {
        /// App creating the task
        app_id: AppId,
        /// Task Id
        task_id: TaskId,
        /// New version to be used with `If-Matches` when submitting further modifications
        version: u64,
    },
}

/// Task was deleted successfully
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskDeleted {
    /// Deleted normally
    Deleted {
        /// App creating the task
        app_id: AppId,
        /// Task Id
        task_id: TaskId,
        /// Version when deleted
        version: u64,
    },
}

/// Adjust the task time
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct AdjustTaskTime {
    /// If not null, overwrite the starting time
    pub from: Option<Timestamp>,
    /// If not null, overwrite the ending time
    pub to: Option<Timestamp>,
}

/// A list of tasks
pub type ModifyTaskList = Vec<ModifyTask>;

/// Create a task
///
/// The task will be checked against exclusivity with other tasks, as well as resources and other
/// limits imposed by the domain configuration.
#[utoipa::path(
post,
path = "/v1/apps/{app_id}/tasks",
request_body = CreateTask,
responses(
(status = 200, description = "Success", body = TaskCreated),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "App not found", body = CloudError),
(status = 409, description = "Overlapping task exists", body = CloudError),
),
params(
("app_id" = AppId, Path, description = "The app for which we are creating a task")
))]
pub(crate) fn create_task() {}

/// Modify existing task spec
///
/// Submit modifications to the task. This generic request can be used to update most aspects of the
/// session: adjusting parameters, creating, deleting, reconnecting nodes, changing media, etc.
#[utoipa::path(
put,
path = "/v1/apps/{app_id}/tasks/{task_id}/spec",
request_body = ModifyTaskList,
responses(
(status = 200, description = "Success", body = TaskUpdated),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "App or task not found", body = CloudError),
),
params(
("app_id" = AppId, Path, description = "App owning the task"),
("task_id" = TaskId, Path, description = "Task to be updated"),
("If-Match" = u64, Header, description = "The task version for"),
))]
pub(crate) fn modify_task_spec() {}

/// Modify existing task time
///
/// Submit modifications to the task reservation time. Can be used to extend, move start or end early.
#[utoipa::path(
put,
path = "/v1/apps/{app_id}/tasks/{task_id}/time",
request_body = AdjustTaskTime,
responses(
(status = 200, description = "Success", body = TaskUpdated),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "App or task not found", body = CloudError),
(status = 409, description = "Overlapping task exists", body = CloudError),
),
params(
("app_id" = AppId, Path, description = "App owning the task"),
("task_id" = TaskId, Path, description = "Task to be updated"),
("If-Match" = u64, Header, description = "The task version for"),
))]
pub(crate) fn adjust_task_time() {}

/// Delete a task
///
/// Delete a task and release all referenced resources.
#[utoipa::path(
delete,
path = "/v1/apps/{app_id}/tasks/{task_id}",
responses(
(status = 200, description = "Success", body = TaskDeleted),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "App not found", body = CloudError),
),
params(
("app_id" = AppId, Path, description = "App owning the task"),
("task_id" = TaskId, Path, description = "Task to be deleted"),
))]
pub(crate) fn delete_task() {}
