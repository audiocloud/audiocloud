/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;

use actix::Message;

use audiocloud_api::audio_engine::event::EngineEvent;
use audiocloud_api::common::change::TaskState;
use audiocloud_api::common::media::{MediaObject, RenderId};

use audiocloud_api::common::task::TaskSpec;
use audiocloud_api::domain::streaming::StreamStats;
use audiocloud_api::domain::tasks::{
    TaskCreated, TaskDeleted, TaskPlayStopped, TaskPlaying, TaskRenderCancelled, TaskRendering, TaskSought, TaskSummaryList, TaskUpdated,
    TaskWithStatusAndSpec,
};
use audiocloud_api::newtypes::{AppMediaObjectId, AppTaskId, EngineId};
use audiocloud_api::{
    CreateTaskReservation, CreateTaskSecurity, CreateTaskSpec, ModifyTaskSpec, PlayId, RequestCancelRender, RequestPlay, RequestRender,
    RequestSeek, RequestStopPlay, StreamingPacket, TaskReservation, TaskSecurity,
};

use crate::{DomainResult, DomainSecurity};

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskRendering>")]
pub struct RenderTask {
    pub task_id:  AppTaskId,
    pub render:   RequestRender,
    pub security: DomainSecurity,
    pub revision: u64,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskPlaying>")]
pub struct PlayTask {
    pub task_id:  AppTaskId,
    pub play:     RequestPlay,
    pub security: DomainSecurity,
    pub revision: u64,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskCreated>")]
pub struct CreateTask {
    pub task_id:      AppTaskId,
    pub reservations: CreateTaskReservation,
    pub spec:         CreateTaskSpec,
    pub security:     CreateTaskSecurity,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskDeleted>")]
pub struct DeleteTask {
    pub task_id:  AppTaskId,
    pub revision: u64,
    pub security: DomainSecurity,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyTaskDeleted {
    pub task_id: AppTaskId,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyTaskActivated {
    pub task_id: AppTaskId,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyTaskDeactivated {
    pub task_id: AppTaskId,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyTaskSecurity {
    pub task_id:  AppTaskId,
    pub security: TaskSecurity,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyTaskSpec {
    pub task_id: AppTaskId,
    pub spec:    TaskSpec,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyTaskReservation {
    pub task_id:     AppTaskId,
    pub reservation: TaskReservation,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyTaskState {
    pub task_id: AppTaskId,
    pub state:   TaskState,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyEngineEvent {
    pub engine_id: EngineId,
    pub event:     EngineEvent,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyMediaTaskState {
    pub task_id: AppTaskId,
    pub media:   HashMap<AppMediaObjectId, MediaObject>,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyRenderComplete {
    pub task_id:    AppTaskId,
    pub render_id:  RenderId,
    pub path:       String,
    pub object_id:  AppMediaObjectId,
    pub put_url:    String,
    pub notify_url: String,
    pub context:    String,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyRenderFailed {
    pub task_id:   AppTaskId,
    pub render_id: RenderId,
    pub error:     String,
    pub cancelled: bool,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct BecomeOnline;

#[derive(Message, Clone, Debug)]
#[rtype(result = "TaskSummaryList")]
pub struct ListTasks;

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskWithStatusAndSpec>")]
pub struct GetTaskWithStatusAndSpec {
    pub task_id: AppTaskId,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskUpdated>")]
pub struct ModifyTask {
    pub task_id:     AppTaskId,
    pub modify_spec: Vec<ModifyTaskSpec>,
    pub revision:    u64,
    pub security:    DomainSecurity,
    pub optional:    bool,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskSought>")]
pub(crate) struct SeekTask {
    pub task_id:  AppTaskId,
    pub seek:     RequestSeek,
    pub revision: u64,
    pub security: DomainSecurity,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskRenderCancelled>")]
pub struct CancelRenderTask {
    pub task_id:  AppTaskId,
    pub cancel:   RequestCancelRender,
    pub security: DomainSecurity,
    pub revision: u64,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<TaskPlayStopped>")]
pub struct StopPlayTask {
    pub task_id:  AppTaskId,
    pub stop:     RequestStopPlay,
    pub security: DomainSecurity,
    pub revision: u64,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct NotifyStreamingPacket {
    pub task_id: AppTaskId,
    pub packet:  StreamingPacket,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<StreamStats>")]
pub struct GenerateStreamStats {
    pub task_id:  AppTaskId,
    pub play_id:  PlayId,
    pub security: DomainSecurity,
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "DomainResult<StreamingPacket>")]
pub struct GetStreamPacket {
    pub task_id:  AppTaskId,
    pub play_id:  PlayId,
    pub serial:   u64,
    pub security: DomainSecurity,
}
