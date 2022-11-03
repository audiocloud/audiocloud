/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{ContextFutureSpawner, Handler, WrapFuture};
use futures::FutureExt;

use crate::fixed_instances::{FixedInstancesSupervisor, NotifyFixedInstanceReports};

impl Handler<NotifyFixedInstanceReports> for FixedInstancesSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyFixedInstanceReports, ctx: &mut Self::Context) -> Self::Result {
        for instance in self.instances.values() {
            if let Some(power_config) = instance.config.power.as_ref() {
                if &power_config.instance == &msg.instance_id {
                    instance.address.send(msg.clone()).map(drop).into_actor(self).spawn(ctx);
                }
            }
        }
    }
}
