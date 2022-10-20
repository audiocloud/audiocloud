use actix::{Context, Handler};
use actix_broker::BrokerSubscribe;

use crate::fixed_instances::NotifyFixedInstanceReports;
use crate::tasks::supervisor::TasksSupervisor;

impl Handler<NotifyFixedInstanceReports> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyFixedInstanceReports, ctx: &mut Self::Context) -> Self::Result {
        if let Some(task_id) = self.fixed_instance_membership.get(&msg.instance_id) {
            if let Some(actor_addr) = self.tasks.get(task_id).and_then(|task| task.actor.as_ref()) {
                actor_addr.do_send(msg);
            }
        }
    }
}

impl TasksSupervisor {
    pub(crate) fn subscribe_instance_events(&self, ctx: &mut Context<Self>) {
        self.subscribe_system_async::<NotifyFixedInstanceReports>(ctx);
    }
}
