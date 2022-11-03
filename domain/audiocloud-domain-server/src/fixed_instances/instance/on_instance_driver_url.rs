/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::NotifyInstanceDriverUrl;

impl Handler<NotifyInstanceDriverUrl> for FixedInstanceActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyInstanceDriverUrl, ctx: &mut Self::Context) -> Self::Result {
        let NotifyInstanceDriverUrl { instance_id,
                                      base_url: new_base_url, } = msg;

        if &self.id == &instance_id {
            self.instance_client.set_url(new_base_url);
        }
    }
}
