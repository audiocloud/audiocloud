#![allow(unused_variables)]

use std::collections::HashMap;
use std::time::Duration;

use actix::{Actor, ActorFutureExt, AsyncContext, Context, ContextFutureSpawner, WrapFuture};
use actix_broker::{BrokerIssue, BrokerSubscribe};
use tracing::*;

use audiocloud_api::audio_engine::{EngineCommand, EngineError};
use audiocloud_api::cloud::domains::FixedInstanceRouting;
use audiocloud_api::{
    AppMediaObjectId, AppTaskId, DomainId, EngineId, FixedInstanceId, SerializableResult, StreamingPacket, TaskReservation, TaskSecurity,
    TaskSpec,
};

use crate::config::NotifyFixedInstanceRouting;
use crate::fixed_instances::{get_instance_supervisor, GetMultipleFixedInstanceState};
use crate::nats;
use crate::tasks::task_engine::TaskEngine;
use crate::tasks::{NotifyTaskActivated, NotifyTaskReservation, NotifyTaskSecurity, NotifyTaskSpec, TaskOpts};

use super::task_fixed_instance::TaskFixedInstances;
use super::task_media_objects::TaskMediaObjects;

mod cancel_render;
mod handle_engine_events;
mod handle_instance_events;
mod handle_media_events;
mod modify_task;
mod packet_handling;
mod play_task;
mod render_task;
mod seek_task;
mod stop_play;

pub struct TaskActor {
    id:                     AppTaskId,
    opts:                   TaskOpts,
    engine_id:              EngineId,
    domain_id:              DomainId,
    reservations:           TaskReservation,
    spec:                   TaskSpec,
    security:               TaskSecurity,
    engine_command_subject: String,
    fixed_instance_routing: HashMap<FixedInstanceId, FixedInstanceRouting>,
    fixed_instances:        TaskFixedInstances,
    media_objects:          TaskMediaObjects,
    engine:                 TaskEngine,
    packet:                 StreamingPacket,
}

impl Actor for TaskActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.notify_task_spec();

        self.notify_task_security();

        self.notify_task_reservation();

        self.issue_system_async(NotifyTaskActivated { task_id: self.id.clone() });

        // subscribe to routing changes
        self.subscribe_system_async::<NotifyFixedInstanceRouting>(ctx);

        // inform the engine that we want to start a task
        self.set_engine_spec(ctx);

        ctx.run_interval(Duration::from_millis(30), Self::update);
    }
}

impl TaskActor {
    pub fn new(id: AppTaskId,
               opts: TaskOpts,
               domain_id: DomainId,
               engine_id: EngineId,
               reservations: TaskReservation,
               spec: TaskSpec,
               security: TaskSecurity,
               routing: HashMap<FixedInstanceId, FixedInstanceRouting>)
               -> anyhow::Result<Self> {
        let engine_command_subject = engine_id.engine_command_subject();

        Ok(Self { id:                     { id.clone() },
                  engine_id:              { engine_id },
                  domain_id:              { domain_id },
                  opts:                   { opts },
                  reservations:           { reservations },
                  spec:                   { spec },
                  security:               { security },
                  engine_command_subject: { engine_command_subject },
                  fixed_instance_routing: { routing },
                  fixed_instances:        { TaskFixedInstances::default() },
                  media_objects:          { TaskMediaObjects::default() },
                  engine:                 { TaskEngine::new(id.clone()) },
                  packet:                 { Default::default() }, })
    }

    fn update(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.engine.set_instances_are_ready(self.fixed_instances.update(&self.spec));

        if let Some(engine_cmd) = self.engine.update() {
            nats::request_msgpack(self.engine_command_subject.clone(), engine_cmd).into_actor(self)
                                                                                  .map(Self::handle_engine_response)
                                                                                  .spawn(ctx)
        }
    }

    fn set_engine_spec(&mut self, ctx: &mut Context<TaskActor>) {
        let cmd = EngineCommand::SetSpec { task_id:     { self.id.clone() },
                                           spec:        { self.spec.clone() },
                                           instances:   { self.engine_fixed_instance_routing() },
                                           media_ready: { self.engine_media_paths() }, };

        nats::request_msgpack(self.engine_command_subject.clone(), cmd).into_actor(self)
                                                                       .map(Self::handle_engine_response)
                                                                       .spawn(ctx);
    }

    fn handle_engine_response(res: anyhow::Result<SerializableResult<(), EngineError>>, actor: &mut Self, ctx: &mut Context<Self>) {
        match res {
            Ok(SerializableResult::Error(error)) => {
                error!(%error, id = %actor.id, "Engine command failed");
            }
            Err(error) => {
                error!(%error, id = %actor.id, "Failed to deliver command to engine");
            }
            _ => {}
        }
    }

    fn update_fixed_instance_state(&self, ctx: &mut <Self as Actor>::Context) {
        get_instance_supervisor().send(GetMultipleFixedInstanceState { instance_ids: self.reservations.fixed_instances.clone(), })
                                 .into_actor(self)
                                 .map(|res, actor, ctx| {
                                     if let Ok(state) = res {
                                         actor.update_fixed_instance_state_inner(state, ctx);
                                     }
                                 })
                                 .spawn(ctx);
    }

    fn engine_fixed_instance_routing(&self) -> HashMap<FixedInstanceId, FixedInstanceRouting> {
        self.fixed_instance_routing
            .iter()
            .filter_map(|(id, routing)| {
                if self.reservations.fixed_instances.contains(id) {
                    Some((id.clone(), routing.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn engine_media_paths(&self) -> HashMap<AppMediaObjectId, String> {
        self.media_objects.ready_for_engine()
    }

    fn notify_task_spec(&mut self) {
        self.issue_system_async(NotifyTaskSpec { task_id: self.id.clone(),
                                                 spec:    self.spec.clone(), });
    }

    fn notify_task_security(&mut self) {
        self.issue_system_async(NotifyTaskSecurity { task_id:  self.id.clone(),
                                                     security: self.security.clone(), });
    }

    fn notify_task_reservation(&mut self) {
        self.issue_system_async(NotifyTaskReservation { task_id:     self.id.clone(),
                                                        reservation: self.reservations.clone(), });
    }
}
