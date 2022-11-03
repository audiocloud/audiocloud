/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use audiocloud_api::domain::tasks::TaskCreated;
use audiocloud_api::domain::DomainError;

use crate::tasks::supervisor::{SupervisedTask, TasksSupervisor};
use crate::tasks::CreateTask;
use crate::DomainResult;

impl Handler<CreateTask> for TasksSupervisor {
    type Result = DomainResult<TaskCreated>;

    fn handle(&mut self, msg: CreateTask, ctx: &mut Self::Context) -> Self::Result {
        if self.tasks.contains_key(&msg.task_id) {
            return Err(DomainError::TaskExists { task_id: msg.task_id });
        }

        self.tasks.insert(msg.task_id.clone(),
                          SupervisedTask { domain_id:    { self.domain_config.domain_id.clone() },
                                           reservations: { msg.reservations.into() },
                                           spec:         { msg.spec.into() },
                                           security:     { msg.security.into() },
                                           state:        { Default::default() },
                                           actor:        { None },
                                           packet_cache: { Default::default() }, });

        self.run_task_timers(ctx);

        Ok(TaskCreated::Created { task_id: msg.task_id.clone(), })
    }
}
