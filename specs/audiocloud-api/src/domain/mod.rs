/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::collections::HashMap;

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::audio_engine::EngineError;
use crate::common::change::{DesiredTaskPlayState, ModifyTaskSpec};
use crate::common::task::TaskPermissions;
use crate::common::task::TaskSpec;
use crate::instance_driver::InstanceDriverError;
use crate::newtypes::{AppTaskId, SecureKey};
use crate::{
    merge_schemas, AppId, AppMediaObjectId, ClientSocketId, EngineId, FixedInstanceId, InstanceEvent, ModifyTaskError, PlayId, RequestId,
    SocketId, Task, TaskEvent, TaskId, TaskPlayStateSummary,
};

pub mod streaming;
pub mod tasks;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainCommand {
    Create {
        app_session_id: AppTaskId,
        task:           Task,
    },
    SetSpec {
        app_session_id: AppTaskId,
        version:        u64,
        spec:           TaskSpec,
    },
    SetSecurity {
        app_session_id: AppTaskId,
        version:        u64,
        security:       HashMap<SecureKey, TaskPermissions>,
    },
    Modify {
        app_session_id: AppTaskId,
        version:        u64,
        modifications:  Vec<ModifyTaskSpec>,
    },
    SetDesiredPlayState {
        app_session_id:     AppTaskId,
        version:            u64,
        desired_play_state: DesiredTaskPlayState,
    },
    Delete {
        app_session_id: AppTaskId,
    },
}

impl DomainCommand {
    pub fn get_session_id(&self) -> &AppTaskId {
        match self {
            DomainCommand::Create { app_session_id, .. } => app_session_id,
            DomainCommand::SetSpec { app_session_id, .. } => app_session_id,
            DomainCommand::SetSecurity { app_session_id, .. } => app_session_id,
            DomainCommand::Modify { app_session_id, .. } => app_session_id,
            DomainCommand::SetDesiredPlayState { app_session_id, .. } => app_session_id,
            DomainCommand::Delete { app_session_id, .. } => app_session_id,
        }
    }

    pub fn get_kind(&self) -> &'static str {
        match self {
            DomainCommand::Create { .. } => "create",
            DomainCommand::SetSpec { .. } => "set_spec",
            DomainCommand::SetSecurity { .. } => "set_security",
            DomainCommand::Modify { .. } => "modify",
            DomainCommand::SetDesiredPlayState { .. } => "set_desired_play_state",
            DomainCommand::Delete { .. } => "delete",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainEvent {
    FixedInstance {
        instance_id: FixedInstanceId,
        event:       InstanceEvent,
    },
    Task {
        task_id: AppTaskId,
        event:   TaskEvent,
    },
}

impl DomainEvent {
    pub fn key(&self) -> String {
        match self {
            DomainEvent::FixedInstance { instance_id, .. } => instance_id.to_string(),
            DomainEvent::Task { task_id, .. } => task_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Error)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum DomainError {
    #[error("Instance driver for instance {instance_id}: {error}")]
    InstanceDriver {
        instance_id: FixedInstanceId,
        error:       InstanceDriverError,
    },

    #[error("Engine {engine_id} raised an error: {error}")]
    Engine { engine_id: EngineId, error: EngineError },

    #[error("Engine {engine_id} not found")]
    EngineNotFound { engine_id: EngineId },

    #[error("Socket {socket_id} not found")]
    SocketNotFound { socket_id: ClientSocketId },

    #[error("Socket {socket_id} already exists")]
    SocketExists { socket_id: ClientSocketId },

    #[error("Task {task_id} not found")]
    TaskNotFound { task_id: AppTaskId },

    #[error("Task {task_id} stream {play_id} not found")]
    TaskStreamNotFound { task_id: AppTaskId, play_id: PlayId },

    #[error("Task {task_id} packet {serial} of stream {play_id} not found")]
    TaskPacketNotFound { task_id: AppTaskId, play_id: PlayId, serial: u64 },

    #[error("Task {task_id} already exists")]
    TaskExists { task_id: AppTaskId },

    #[error("Task {task_id} revision {revision} cannot safely apply the update")]
    TaskModificationRevisionOutOfDate { task_id: AppTaskId, revision: u64 },

    #[error("Task {task_id} failed to modify: {error}")]
    TaskModification { task_id: AppTaskId, error: ModifyTaskError },

    #[error("Instance {instance_id} not found")]
    InstanceNotFound { instance_id: FixedInstanceId },

    #[error("Instance {instance_id} does not support operation {operation}")]
    InstanceNotCapable { instance_id: FixedInstanceId, operation: String },

    #[error("Media {media_object_id} not found")]
    MediaNotFound { media_object_id: AppMediaObjectId },

    #[error("Error during serialization: {error}")]
    Serialization { error: String },

    #[error("This feature or service call {call} is not implemented: {reason}")]
    NotImplemented { call: String, reason: String },

    #[error("The service call failed or timed out: {error}")]
    BadGateway { error: String },

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Task revision is malformed: {error}")]
    TaskRevisionMalformed { error: String },

    #[error("You are not authorized to access task {task_id}, required permissions {required:?}")]
    TaskAuthtorizationFailed { task_id: AppTaskId, required: TaskPermissions },

    #[error("Task {task_id} is in an incorrect state: state")]
    TaskIllegalPlayState { task_id: AppTaskId, state: TaskPlayStateSummary },

    #[error("WebRTC error: {error}")]
    WebRTCError { error: String },

    #[error("Remote procedure call failed: {error}")]
    RPC { error: String },
}

impl DomainError {
    pub fn status_code(&self) -> u16 {
        use DomainError::*;

        match self {
            AuthenticationFailed => 401,
            TaskAuthtorizationFailed { .. } => 403,
            EngineNotFound { .. } | SocketNotFound { .. } | TaskNotFound { .. } | InstanceNotFound { .. } | MediaNotFound { .. } => 404,
            NotImplemented { .. } => 500,
            BadGateway { .. } => 502,
            _ => 400,
        }
    }
}

pub fn schemas() -> RootSchema {
    merge_schemas([schema_for!(DomainError),
                   schema_for!(DomainCommand),
                   schema_for!(DomainEvent),
                   schema_for!(AppId),
                   schema_for!(TaskId),
                   schema_for!(SocketId),
                   schema_for!(RequestId),
                   schema_for!(streaming::StreamStats),
                   schema_for!(streaming::DomainServerMessage),
                   schema_for!(streaming::DomainClientMessage),
                   schema_for!(tasks::TaskSummaryList),
                   schema_for!(tasks::TaskWithStatusAndSpec),
                   schema_for!(tasks::CreateTask),
                   schema_for!(tasks::ModifyTask),
                   schema_for!(tasks::TaskCreated),
                   schema_for!(tasks::TaskDeleted),
                   schema_for!(tasks::TaskUpdated),
                   schema_for!(tasks::TaskPlayStopped),
                   schema_for!(tasks::TaskPlaying),
                   schema_for!(tasks::TaskRenderCancelled),
                   schema_for!(tasks::TaskRendering),
                   schema_for!(tasks::TaskSought),
                   schema_for!(crate::StreamingPacket),
                   schema_for!(crate::RequestPlay),
                   schema_for!(crate::RequestSeek),
                   schema_for!(crate::RequestChangeMixer),
                   schema_for!(crate::RequestStopPlay),
                   schema_for!(crate::RequestCancelRender)].into_iter())
}
