use std::convert::identity;

use actix_web::{get, web};
use serde::Deserialize;

use audiocloud_api::domain::streaming::StreamStats;

use audiocloud_api::{AppId, AppTaskId, PlayId, StreamingPacket, TaskId};

use crate::rest_api::{bad_gateway, ApiResponder, ApiResponse};

use crate::tasks::{get_tasks_supervisor, GenerateStreamStats, GetStreamPacket};
use crate::DomainSecurity;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_stream_stats).service(get_stream_packet);
}

#[derive(Deserialize)]
pub struct AppTaskPlayIdPath {
    app_id:  AppId,
    task_id: TaskId,
    play_id: PlayId,
}

#[derive(Deserialize)]
pub struct AppTaskPlayIdPacketPath {
    app_id:  AppId,
    task_id: TaskId,
    play_id: PlayId,
    serial:  u64,
}

#[get("/{app_id}/{task_id}/{play_id}")]
pub async fn get_stream_stats(path: web::Path<AppTaskPlayIdPath>,
                              responder: ApiResponder,
                              security: DomainSecurity)
                              -> ApiResponse<StreamStats> {
    let path = path.into_inner();
    let task_id = AppTaskId::new(path.app_id, path.task_id);
    let play_id = path.play_id;

    responder.respond(async move {
                 get_tasks_supervisor().send(GenerateStreamStats { task_id:  { task_id },
                                                                   play_id:  { play_id },
                                                                   security: { security }, })
                                       .await
                                       .map_err(bad_gateway)
                                       .and_then(identity)
             })
             .await
}

#[get("/{app_id}/{task_id}/{play_id}/packet/{serial}")]
pub async fn get_stream_packet(path: web::Path<AppTaskPlayIdPacketPath>,
                               responder: ApiResponder,
                               security: DomainSecurity)
                               -> ApiResponse<StreamingPacket> {
    let path = path.into_inner();
    let task_id = AppTaskId::new(path.app_id, path.task_id);
    let play_id = path.play_id;
    let serial = path.serial;

    responder.respond(async move {
                 get_tasks_supervisor().send(GetStreamPacket { task_id:  { task_id },
                                                               play_id:  { play_id },
                                                               serial:   { serial },
                                                               security: { security }, })
                                       .await
                                       .map_err(bad_gateway)
                                       .and_then(identity)
             })
             .await
}
