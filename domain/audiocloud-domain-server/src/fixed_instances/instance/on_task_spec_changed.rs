/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::tasks::NotifyTaskSpec;

impl Handler<NotifyTaskSpec> for FixedInstanceActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskSpec, ctx: &mut Self::Context) {
        if msg.spec.get_fixed_instance_ids().contains(&self.id) {
            self.spec = Some(msg).into();
            self.update(ctx);
        }
    }
}
