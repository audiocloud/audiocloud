use actix::{Actor, Context, Handler};
use actix_broker::BrokerSubscribe;
use tracing::debug;

use crate::events::NotifyDomainEvent;

pub async fn init() -> anyhow::Result<()> {
    Ok(())
}

struct LogEventsActor;

impl Actor for LogEventsActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<NotifyDomainEvent>(ctx);
    }
}

impl Handler<NotifyDomainEvent> for LogEventsActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyDomainEvent, _ctx: &mut Self::Context) -> Self::Result {
        debug!(event = ?msg.event, "Event!");
    }
}
