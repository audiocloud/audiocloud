/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Context, Handler};
use actix_broker::BrokerSubscribe;
use tracing::*;

use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::NotifyMediaTaskState;

impl Handler<NotifyMediaTaskState> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyMediaTaskState, ctx: &mut Self::Context) -> Self::Result {
        let task_id = &msg.task_id;
        match self.tasks.get(task_id).and_then(|task| task.actor.as_ref()) {
            Some(task) => {
                task.do_send(msg);
            }
            None => {
                warn!(%task_id, "Dropping media service event for unknown / inactive task");
            }
        }
    }
}

impl TasksSupervisor {
    pub(crate) fn subscribe_media_events(&self, ctx: &mut Context<Self>) {
        self.subscribe_system_async::<NotifyMediaTaskState>(ctx);
    }
}
