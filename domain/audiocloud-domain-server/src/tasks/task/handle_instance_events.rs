use std::collections::HashMap;

use actix::{Context, Handler};

use audiocloud_api::domain::streaming::DiffStamped;
use audiocloud_api::FixedInstanceId;

use crate::config::NotifyFixedInstanceRouting;
use crate::fixed_instances::{NotifyFixedInstanceReports, NotifyInstanceState};
use crate::tasks::task::TaskActor;

impl Handler<NotifyFixedInstanceReports> for TaskActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyFixedInstanceReports, ctx: &mut Self::Context) -> Self::Result {
        self.packet
            .instance_metering
            .entry(msg.instance_id)
            .or_default()
            .push(DiffStamped::new(self.packet.created_at, msg.reports));
    }
}

impl Handler<NotifyFixedInstanceRouting> for TaskActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyFixedInstanceRouting, ctx: &mut Self::Context) -> Self::Result {
        self.fixed_instance_routing = msg.routing;
        // TODO: update engine?
    }
}

impl TaskActor {
    pub(crate) fn update_fixed_instance_state_inner(&mut self,
                                                    result: HashMap<FixedInstanceId, NotifyInstanceState>,
                                                    ctx: &mut Context<Self>) {
        for (id, notify) in result {
            self.fixed_instances.notify_instance_state_changed(notify);
        }

        self.update(ctx);
    }
}
