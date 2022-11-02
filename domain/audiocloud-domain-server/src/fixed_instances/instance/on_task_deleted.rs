/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::tasks::NotifyTaskDeleted;

impl Handler<NotifyTaskDeleted> for FixedInstanceActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskDeleted, ctx: &mut Self::Context) {
        if self.spec.get_ref().as_ref().map(|prev_notify| &prev_notify.task_id == &msg.task_id) == Some(true) {
            self.spec = None.into();
        }
    }
}
