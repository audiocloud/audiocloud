/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

//! API definitions for the Cloud

use std::collections::HashSet;

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::change::ModifyTaskError;
use crate::common::model::ResourceId;
use crate::{
    merge_schemas, AppId, AppMediaObjectId, AppTaskId, ChannelMask, DomainId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId,
    MixerNodeId, ModelId, NodeConnectionId, TrackNodeId,
};

pub mod apps;
pub mod domains;
pub mod media;
pub mod models;
pub mod tasks;

#[derive(Serialize, Deserialize, Debug, Clone, Error, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum CloudError {
    #[error("API Key not found")]
    ApiKeyNotFound,

    #[error("App file {id} not found")]
    AppFileNotFound { id: AppMediaObjectId },

    #[error("App {id} not found")]
    AppNotFound { id: AppId },

    #[error("{task_id} is an invalid app task ID")]
    InvalidAppTaskId { task_id: String },

    #[error("{object_id} is an invalid app media object ID")]
    InvalidAppMediaObjectId { object_id: String },

    #[error("At least a segment of a reservation needs to be in the future")]
    OnlyFutureReservations,

    #[error("Task time must be well-formed")]
    TimeMalformed,

    #[error("Task requested duration {requested} is smaller than domain minimum task duration time {minimum} ms")]
    DurationTooShort { minimum: f64, requested: f64 },

    #[error("Too many overlapping tasks reserved on domain, maximum is {max}")]
    TooManyTasks { max: usize },

    #[error("Detected internal inconsistency: {message}")]
    InternalInconsistency { message: String },

    #[error("Instances overlapping: {instance_ids:?}")]
    OverlappingFixedInstances { instance_ids: HashSet<FixedInstanceId> },

    #[error("Connection error: {connection_id}: {error}")]
    ConnectionError {
        connection_id: NodeConnectionId,
        error:         Box<CloudError>,
    },

    #[error("Channel mask {mask:?} is invalid for channel count {channels}")]
    ChannelMaskIncompatible { mask: ChannelMask, channels: usize },

    #[error("Mixer instance node not found: {mixer_node_id}")]
    MixerNodeNotFound { mixer_node_id: MixerNodeId },

    #[error("Mixer instance node not found: {track_node_id}")]
    TrackNodeNotFound { track_node_id: TrackNodeId },

    #[error("Fixd instance node not found: {fixed_node_id}")]
    FixedInstanceNodeNotFound { fixed_node_id: FixedInstanceNodeId },

    #[error("Dynamic instance node not found: {dynamic_node_id}")]
    DynamicInstanceNodeNotFound { dynamic_node_id: DynamicInstanceNodeId },

    #[error("Domain {domain_id} unknown")]
    DomainNotFound { domain_id: DomainId },

    #[error("Instance {instance_id} unknown")]
    InstanceNotFound { instance_id: FixedInstanceId },

    #[error("Model {model_id} unknown")]
    ModelNotFound { model_id: ModelId },

    #[error("Model {model_id} of a dynamic instance required by node {node_id} is not supported on domain {domain_id}")]
    DynamicInstanceNotSupported {
        node_id:   DynamicInstanceNodeId,
        domain_id: DomainId,
        model_id:  ModelId,
    },

    #[error("Fixed instance {instance_id} required by fixed instance node {node_id} is not supported on domain {domain_id}")]
    FixedInstanceNotSupported {
        node_id:     FixedInstanceNodeId,
        domain_id:   DomainId,
        instance_id: FixedInstanceId,
    },

    #[error("Fixed instance {instance_id} required by fixed instance node {node_id} is not avaialble to app {app_id} on domain {domain_id}")]
    FixedInstanceAccessDenied {
        node_id:     FixedInstanceNodeId,
        domain_id:   DomainId,
        instance_id: FixedInstanceId,
        app_id:      AppId,
    },

    #[error("Out of {resource} resource. Requested {requested} available {available}")]
    OutOfResource {
        resource:  ResourceId,
        available: f64,
        requested: f64,
    },

    #[error("Task {task_id} was not found")]
    TaskNotFound { task_id: AppTaskId },

    #[error("Task could not be modified: {error}")]
    TaskModification {
        #[from]
        error: ModifyTaskError,
    },

    #[error("Database error: {message}")]
    Database { message: String },

    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    #[error("Authorization failed: {message}")]
    Authorization { message: String },

    #[error("All retries exhausted while trying to obtain a lock")]
    BlockingLock,
}

pub fn schemas() -> RootSchema {
    merge_schemas([schema_for!(CloudError),
                   schema_for!(crate::ModifyTaskError),
                   schema_for!(crate::AppId),
                   schema_for!(crate::DomainId),
                   schema_for!(crate::TaskId),
                   schema_for!(crate::TimeRange),
                   schema_for!(crate::TrackNode),
                   schema_for!(crate::MixerNode),
                   schema_for!(crate::DynamicInstanceNode),
                   schema_for!(crate::FixedInstanceNode),
                   schema_for!(crate::NodeConnection),
                   schema_for!(crate::TaskPermissions),
                   schema_for!(crate::TrackMedia),
                   schema_for!(crate::TaskSpec),
                   schema_for!(crate::ModifyTaskSpec),
                   schema_for!(crate::ModifyTask),
                   schema_for!(crate::Model),
                   schema_for!(crate::MediaJobState),
                   schema_for!(crate::UploadToDomain),
                   schema_for!(crate::DownloadFromDomain),
                   schema_for!(apps::GetAppResponse),
                   schema_for!(apps::UpdateApp),
                   schema_for!(apps::AppUpdated),
                   schema_for!(tasks::CreateTask),
                   schema_for!(tasks::TaskCreated),
                   schema_for!(tasks::TaskUpdated),
                   schema_for!(tasks::TaskDeleted),
                   schema_for!(tasks::AdjustTaskTime),
                   schema_for!(tasks::ModifyTaskList),
                   schema_for!(domains::DomainMediaInstanceConfig),
                   schema_for!(domains::DomainPowerInstanceConfig),
                   schema_for!(domains::GetDomainResponse),
                   schema_for!(domains::DomainConfig),
                   schema_for!(domains::DomainUpdated),
                   schema_for!(domains::AddMaintenance),
                   schema_for!(domains::ClearMaintenance),
                   schema_for!(domains::Maintenance),
                   schema_for!(domains::AppFixedInstance),
                   schema_for!(domains::FixedInstanceConfig),
                   schema_for!(domains::DynamicInstanceLimits),
                   schema_for!(domains::EngineConfig),
                   schema_for!(media::DownloadCreated),
                   schema_for!(media::UploadCreated),
                   schema_for!(media::MediaObjectDeleted),
                   schema_for!(media::ReportMediaJobProgress)].into_iter())
}
