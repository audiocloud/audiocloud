use actix::{Context, Handler};
use actix_broker::BrokerSubscribe;
use tracing::*;

use crate::tasks::supervisor::TasksSupervisor;
use crate::tasks::NotifyEngineEvent;

impl Handler<NotifyEngineEvent> for TasksSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyEngineEvent, ctx: &mut Self::Context) -> Self::Result {
        let task_id = msg.event.task_id();
        match self.tasks.get(task_id).and_then(|task| task.actor.as_ref()) {
            Some(session) => {
                session.do_send(msg);
            }
            None => {
                warn!(%task_id, "Dropping audio engine event for unknown / inactive task");
            }
        }
    }
}

impl TasksSupervisor {
    pub(crate) fn subscribe_engine_events(&self, ctx: &mut Context<Self>) {
        self.subscribe_system_async::<NotifyEngineEvent>(ctx);
    }
}
