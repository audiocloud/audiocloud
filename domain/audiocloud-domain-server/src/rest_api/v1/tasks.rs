/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::convert::identity;
use std::str::FromStr;

use actix_web::http::header::IfMatch;
use actix_web::web::{Header, Json};
use actix_web::{delete, get, post, web};

use web::Path;

use audiocloud_api::audio_engine::{TaskPlayStopped, TaskPlaying, TaskRenderCancelled, TaskRendering, TaskSought};
use audiocloud_api::domain::tasks::{
    CreateTask, ModifyTask, TaskCreated, TaskDeleted, TaskSummaryList, TaskUpdated, TaskWithStatusAndSpec,
};
use audiocloud_api::domain::DomainError;
use audiocloud_api::{RequestCancelRender, RequestPlay, RequestRender, RequestSeek, RequestStopPlay};

use crate::rest_api::{ApiResponder, ApiResponse, AppTaskIdPath};
use crate::tasks::{get_tasks_supervisor, messages, ListTasks};
use crate::{rest_api, DomainResult, DomainSecurity};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_tasks)
       .service(create_task)
       .service(get_task)
       .service(modify_task)
       .service(delete_task)
       .service(render_task)
       .service(play_task)
       .service(seek_task)
       .service(cancel_render_task)
       .service(stop_play_task);
}

fn not_implemented_yet<T>(call: &'static str) -> Result<T, DomainError> {
    Err(DomainError::NotImplemented { call:   call.to_string(),
                                      reason: "Not implemented yet".to_string(), })
}

#[get("")]
async fn list_tasks(responder: ApiResponder) -> ApiResponse<TaskSummaryList> {
    responder.respond(async move { get_tasks_supervisor().send(ListTasks).await.map_err(rest_api::bad_gateway) })
             .await
}

#[post("")]
async fn create_task(responder: ApiResponder, create: Json<CreateTask>) -> ApiResponse<TaskCreated> {
    let create = messages::CreateTask { task_id:      create.0.task_id,
                                        reservations: create.0.reservations,
                                        spec:         create.0.spec,
                                        security:     create.0.security, };

    responder.respond(async move {
                 get_tasks_supervisor().send(create)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[get("/{app_id}/{task_id}")]
async fn get_task(responder: ApiResponder, task_id: Path<AppTaskIdPath>) -> ApiResponse<TaskWithStatusAndSpec> {
    let task_id = task_id.into_inner().into();
    let get = messages::GetTaskWithStatusAndSpec { task_id };
    responder.respond(async move {
                 get_tasks_supervisor().send(get)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[post("/{app_id}/{task_id}/modify")]
async fn modify_task(responder: ApiResponder,
                     security: DomainSecurity,
                     task_id: Path<AppTaskIdPath>,
                     modify: Json<ModifyTask>,
                     if_match: Header<IfMatch>)
                     -> ApiResponse<TaskUpdated> {
    let task_id = task_id.into_inner().into();

    responder.respond(async move {
                 let modify = messages::ModifyTask { task_id:     { task_id },
                                                     modify_spec: { modify.into_inner().modify_spec },
                                                     revision:    { get_revision(if_match)? },
                                                     security:    { security },
                                                     optional:    { false }, };

                 get_tasks_supervisor().send(modify)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[delete("/{app_id}/{task_id}")]
async fn delete_task(responder: ApiResponder,
                     security: DomainSecurity,
                     task_id: Path<AppTaskIdPath>,
                     if_match: Header<IfMatch>)
                     -> ApiResponse<TaskDeleted> {
    let task_id = task_id.into_inner().into();

    responder.respond(async move {
                 let delete = messages::DeleteTask { task_id:  { task_id },
                                                     revision: { get_revision(if_match)? },
                                                     security: { security }, };

                 get_tasks_supervisor().send(delete)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[post("/{app_id}/{task_id}/transport/render")]
async fn render_task(responder: ApiResponder,
                     task_id: Path<AppTaskIdPath>,
                     render: Json<RequestRender>,
                     if_match: Header<IfMatch>,
                     security: DomainSecurity)
                     -> ApiResponse<TaskRendering> {
    let task_id = task_id.into_inner().into();

    responder.respond(async move {
                 let render = messages::RenderTask { task_id:  { task_id },
                                                     render:   { render.into_inner() },
                                                     security: { security },
                                                     revision: { get_revision(if_match)? }, };

                 get_tasks_supervisor().send(render)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[post("/{app_id}/{task_id}/transport/play")]
async fn play_task(responder: ApiResponder,
                   task_id: Path<AppTaskIdPath>,
                   play: Json<RequestPlay>,
                   if_match: Header<IfMatch>,
                   security: DomainSecurity)
                   -> ApiResponse<TaskPlaying> {
    let task_id = task_id.into_inner().into();

    responder.respond(async move {
                 let render = messages::PlayTask { task_id:  { task_id },
                                                   play:     { play.into_inner() },
                                                   security: { security },
                                                   revision: { get_revision(if_match)? }, };

                 get_tasks_supervisor().send(render)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[post("/{app_id}/{task_id}/transport/seek")]
async fn seek_task(responder: ApiResponder,
                   task_id: Path<AppTaskIdPath>,
                   seek: Json<RequestSeek>,
                   if_match: Header<IfMatch>,
                   security: DomainSecurity)
                   -> ApiResponse<TaskSought> {
    responder.respond(async move {
                 let seek = messages::SeekTask { task_id:  { task_id.into_inner().into() },
                                                 seek:     { seek.into_inner() },
                                                 security: { security },
                                                 revision: { get_revision(if_match)? }, };

                 get_tasks_supervisor().send(seek)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[post("/{app_id}/{task_id}/transport/cancel")]
async fn cancel_render_task(responder: ApiResponder,
                            task_id: Path<AppTaskIdPath>,
                            cancel: Json<RequestCancelRender>,
                            if_match: Header<IfMatch>,
                            security: DomainSecurity)
                            -> ApiResponse<TaskRenderCancelled> {
    responder.respond(async move {
                 let cancel = messages::CancelRenderTask { task_id:  { task_id.into_inner().into() },
                                                           cancel:   { cancel.into_inner() },
                                                           security: { security },
                                                           revision: { get_revision(if_match)? }, };

                 get_tasks_supervisor().send(cancel)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[post("/{app_id}/{task_id}/transport/stop")]
async fn stop_play_task(responder: ApiResponder,
                        task_id: Path<AppTaskIdPath>,
                        stop: Json<RequestStopPlay>,
                        if_match: Header<IfMatch>,
                        security: DomainSecurity)
                        -> ApiResponse<TaskPlayStopped> {
    responder.respond(async move {
                 let stop = messages::StopPlayTask { task_id:  { task_id.into_inner().into() },
                                                     stop:     { stop.into_inner() },
                                                     security: { security },
                                                     revision: { get_revision(if_match)? }, };

                 get_tasks_supervisor().send(stop)
                                       .await
                                       .map_err(rest_api::bad_gateway)
                                       .and_then(identity)
             })
             .await
}

fn get_revision(header: Header<IfMatch>) -> DomainResult<u64> {
    use DomainError::TaskRevisionMalformed;
    match header.into_inner() {
        IfMatch::Any => Err(TaskRevisionMalformed { error: format!("Task revision must be specified"), }),
        IfMatch::Items(items) => match items.get(0) {
            Some(item) if items.len() == 1 => Ok(u64::from_str(item.tag()).map_err(|err| {
                                                                              TaskRevisionMalformed {
                        error: format!("Task revision is not a number: {err}"),
                    }
                                                                          })?),
            _ => Err(TaskRevisionMalformed { error: format!("Exactly one revision must be specified"), }),
        },
    }
}
