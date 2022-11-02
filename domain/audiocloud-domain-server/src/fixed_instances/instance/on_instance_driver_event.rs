/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::StreamHandler;

use audiocloud_api::instance_driver::InstanceDriverEvent;

use crate::fixed_instances::instance::FixedInstanceActor;

impl StreamHandler<InstanceDriverEvent> for FixedInstanceActor {
    fn handle(&mut self, item: InstanceDriverEvent, ctx: &mut Self::Context) {
        match item {
            InstanceDriverEvent::Started => {}
            InstanceDriverEvent::IOError { .. } => {}
            InstanceDriverEvent::ConnectionLost => {
                self.connected = false.into();
            }
            InstanceDriverEvent::Connected => {
                self.connected = true.into();
                self.on_instance_driver_connected(ctx);
            }
            InstanceDriverEvent::Reports { reports } => {
                self.on_instance_driver_reports(reports);
            }
            InstanceDriverEvent::PlayState { desired,
                                             current,
                                             media: media_pos, } => self.on_instance_driver_play_state_changed(current, media_pos),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        self.subscribe_instance_driver_events(ctx);
    }
}
