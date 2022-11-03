/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use audiocloud_api::domain::tasks::TaskDeleted;

use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::DeleteTask;
use crate::DomainResult;

impl TasksSupervisor {}

impl Handler<DeleteTask> for TasksSupervisor {
    type Result = DomainResult<TaskDeleted>;

    fn handle(&mut self, msg: DeleteTask, ctx: &mut Self::Context) -> Self::Result {
        if self.tasks.contains_key(&msg.task_id) {}

        todo!()
    }
}
