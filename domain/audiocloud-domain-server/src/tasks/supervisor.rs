#![allow(unused_variables)]

use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};
use opentelemetry::global;
use opentelemetry::metrics::ObservableGauge;
use tracing::*;

use audiocloud_api::cloud::domains::{DomainConfig, DomainEngineConfig, FixedInstanceRoutingMap};
use audiocloud_api::common::change::TaskState;
use audiocloud_api::newtypes::{AppTaskId, EngineId};
use audiocloud_api::{
    DomainId, FixedInstanceId, PlayId, StreamingPacket, Task, TaskReservation, TaskSecurity,
    TaskSpec, Timestamped,
};

use crate::db::Db;

use crate::tasks::messages::BecomeOnline;
use crate::tasks::task::TaskActor;
use crate::tasks::TaskOpts;

mod cancel_render;
mod create_task;
mod delete_task;
mod get_task;
mod handle_engine_events;
mod handle_instance_events;
mod handle_media_events;
mod handle_task_events;
mod list_tasks;
mod modify_task;
mod packets;
mod play_task;
mod render_task;
mod seek_task;
mod stop_play;
mod task_timers;

pub struct TasksSupervisor {
    db: Db,
    opts: TaskOpts,
    domain_config: DomainConfig,
    tasks: HashMap<AppTaskId, SupervisedTask>,
    engines: HashMap<EngineId, ReferencedEngine>,
    fixed_instance_membership: HashMap<FixedInstanceId, AppTaskId>,
    fixed_instance_routing: FixedInstanceRoutingMap,
    num_tasks: ObservableGauge<u64>,
    num_active_tasks: ObservableGauge<u64>,
    online: bool,
}

struct SupervisedTask {
    pub domain_id: DomainId,
    pub reservations: TaskReservation,
    pub spec: TaskSpec,
    pub security: TaskSecurity,
    pub state: TaskState,
    pub actor: Option<Addr<TaskActor>>,
    pub packet_cache: HashMap<PlayId, HashMap<u64, Timestamped<StreamingPacket>>>,
}

struct ReferencedEngine {
    config: DomainEngineConfig,
}

impl TasksSupervisor {
    pub fn new(
        db: Db,
        opts: &TaskOpts,
        cfg: &DomainConfig,
        routing: FixedInstanceRoutingMap,
    ) -> anyhow::Result<Self> {
        let meter = global::meter("audiocloud.io/tasks_total");
        let num_tasks = meter
            .u64_observable_gauge("tasks")
            .with_description("Total number of tasks")
            .init();

        let num_active_tasks = meter
            .u64_observable_gauge("active_tasks")
            .with_description("Total number of active tasks")
            .init();

        let tasks = cfg.tasks
            .iter()
            .filter(|(id, task)| {
                if &task.domain_id != &cfg.domain_id {
                    warn!(%id, domain_id = %cfg.domain_id, other_domain_id = %task.domain_id, "Configuration time task is for another domain, skipping");
                    true
                } else {
                    false
                }
            })
            .map(Self::create_task_actor)
            .collect();

        let engines = cfg
            .engines
            .iter()
            .map(|(id, config)| {
                (
                    id.clone(),
                    ReferencedEngine {
                        config: config.clone(),
                    },
                )
            })
            .collect();

        Ok(Self {
            db: { db },
            opts: { opts.clone() },
            domain_config: { cfg.clone() },
            fixed_instance_membership: { HashMap::new() },
            fixed_instance_routing: { routing },
            tasks: { tasks },
            engines: { engines },
            num_tasks: { num_tasks },
            num_active_tasks: { num_active_tasks },
            online: { false },
        })
    }

    fn create_task_actor((id, task): (&AppTaskId, &Task)) -> (AppTaskId, SupervisedTask) {
        (
            id.clone(),
            SupervisedTask {
                domain_id: { task.domain_id.clone() },
                reservations: { task.reservations.clone() },
                spec: { task.spec.clone() },
                security: { task.security.clone() },
                state: { Default::default() },
                actor: { None },
                packet_cache: { Default::default() },
            },
        )
    }

    fn allocate_engine(&self, id: &AppTaskId, spec: &TaskSpec) -> Option<EngineId> {
        // TODO: we know we only have one engine, so we always pick the first
        let engine_id = self.engines.keys().next().cloned();
        info!(?engine_id, %id, "Allocated engine for task");
        engine_id
    }
}

impl Actor for TasksSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_task_events(ctx);
        self.subscribe_instance_events(ctx);
        self.subscribe_media_events(ctx);
        self.subscribe_engine_events(ctx);

        self.register_task_timers(ctx);
        self.register_packet_cache_cleanup(ctx);
    }
}

impl Handler<BecomeOnline> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: BecomeOnline, ctx: &mut Self::Context) -> Self::Result {
        self.online = true;
    }
}
