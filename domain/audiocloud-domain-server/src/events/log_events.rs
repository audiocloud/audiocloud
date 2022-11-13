/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use async_trait::async_trait;
use coerce::actor::context::ActorContext;
use coerce::actor::message::Handler;
use coerce::actor::scheduler::{start_actor, ActorType};
use coerce::actor::{new_actor_id, Actor, ActorId};
use tracing::debug;

use audiocloud_actors::subscribe;

use crate::events::{subscribe_domain_events, NotifyDomainEvent};

pub async fn init(id: ActorId) -> anyhow::Result<()> {
    start_actor(LogEventsActor, id, ActorType::Tracked, None, None, None);

    Ok(())
}

struct LogEventsActor;

#[async_trait]
impl Actor for LogEventsActor {
    async fn started(&mut self, ctx: &mut ActorContext) {
        subscribe(ctx.actor_ref(), subscribe_domain_events());
    }
}

#[async_trait]
impl Handler<NotifyDomainEvent> for LogEventsActor {
    async fn handle(&mut self, message: NotifyDomainEvent, _ctx: &mut ActorContext) {
        debug!(event = ?message.event, "Event!");
    }
}
