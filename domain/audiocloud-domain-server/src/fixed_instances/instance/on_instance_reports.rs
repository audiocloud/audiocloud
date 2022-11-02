/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::NotifyFixedInstanceReports;

impl Handler<NotifyFixedInstanceReports> for FixedInstanceActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyFixedInstanceReports, ctx: &mut Self::Context) -> Self::Result {
        if let Some(power) = &mut self.power {
            if power.power_instance_id() == &msg.instance_id {
                power.on_instance_power_channels_changed(msg);
            }
        }
    }
}
