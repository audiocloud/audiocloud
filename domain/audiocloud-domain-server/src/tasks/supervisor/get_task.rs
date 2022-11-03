/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use audiocloud_api::domain::tasks::TaskWithStatusAndSpec;
use audiocloud_api::domain::DomainError;

use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::GetTaskWithStatusAndSpec;
use crate::DomainResult;

impl Handler<GetTaskWithStatusAndSpec> for TasksSupervisor {
    type Result = DomainResult<TaskWithStatusAndSpec>;

    fn handle(&mut self, msg: GetTaskWithStatusAndSpec, ctx: &mut Self::Context) -> Self::Result {
        use DomainError::*;

        if let Some(task) = self.tasks.get(&msg.task_id) {
            Ok(TaskWithStatusAndSpec { play_state: { task.state.play_state.get_ref().clone() },
                                       task_id:    { msg.task_id.clone() },
                                       instances:  { Default::default() },
                                       media:      { Default::default() },
                                       spec:       { Default::default() }, })
        } else {
            Err(TaskNotFound { task_id: msg.task_id })
        }
    }
}
