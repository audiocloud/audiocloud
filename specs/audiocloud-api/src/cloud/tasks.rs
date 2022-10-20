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
    pub domain_id:    DomainId,
    /// Task reservations
    pub reservations: CreateTaskReservation,
    /// Task specification
    pub spec:         CreateTaskSpec,
    /// Security keys and associateds permissions
    pub security:     CreateTaskSecurity,
    /// When true, do not actually create a task, just validate the process
    pub dry_run:      bool,
}

/// Task created successfully
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskCreated {
    /// Created normally
    Created {
        /// App creating the task
        app_id:  AppId,
        /// Task Id
        task_id: TaskId,
    },
    /// Validated successfully, but not created
    DryRun {
        /// App creating the task
        app_id:  AppId,
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
        app_id:  AppId,
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
        app_id:  AppId,
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
    pub to:   Option<Timestamp>,
}

/// A list of tasks
pub type ModifyTaskList = Vec<ModifyTask>;
