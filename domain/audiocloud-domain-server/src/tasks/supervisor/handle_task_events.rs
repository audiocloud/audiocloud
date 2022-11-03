/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Context, Handler};
use actix_broker::BrokerSubscribe;

use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::{NotifyTaskReservation, NotifyTaskSecurity, NotifyTaskSpec, NotifyTaskState};

impl Handler<NotifyTaskState> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskState, ctx: &mut Self::Context) -> Self::Result {
        if let Some(task) = self.tasks.get_mut(&msg.task_id) {
            task.state = msg.state;
        }
    }
}

impl Handler<NotifyTaskSpec> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskSpec, ctx: &mut Self::Context) -> Self::Result {
        // clear all previous associations with the same task ID
        self.fixed_instance_membership.retain(|_, task_id| task_id != &msg.task_id);

        // associate task ID with all the current fixed instance IDs
        for fixed_instance_id in msg.spec.get_fixed_instance_ids() {
            self.fixed_instance_membership
                .insert(fixed_instance_id.clone(), msg.task_id.clone());
        }

        if let Some(task) = self.tasks.get_mut(&msg.task_id) {
            task.spec = msg.spec;
        }
    }
}

impl Handler<NotifyTaskReservation> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskReservation, ctx: &mut Self::Context) -> Self::Result {
        if let Some(task) = self.tasks.get_mut(&msg.task_id) {
            task.reservations = msg.reservation;
        }
    }
}

impl Handler<NotifyTaskSecurity> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskSecurity, ctx: &mut Self::Context) -> Self::Result {
        if let Some(task) = self.tasks.get_mut(&msg.task_id) {
            task.security = msg.security;
        }
    }
}

impl TasksSupervisor {
    pub(crate) fn subscribe_task_events(&self, ctx: &mut Context<Self>) {
        self.subscribe_system_async::<NotifyTaskSpec>(ctx);
        self.subscribe_system_async::<NotifyTaskState>(ctx);
        self.subscribe_system_async::<NotifyTaskReservation>(ctx);
        self.subscribe_system_async::<NotifyTaskSecurity>(ctx);
    }
}
