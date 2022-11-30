/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use anyhow::anyhow;
use coerce::actor::ActorId;
use once_cell::sync::Lazy;
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::*;

use audiocloud_api::cloud::domains::{DomainCommandSource, DomainEventSink};
use audiocloud_api::domain::DomainEvent;
pub use messages::*;

mod log_events;
mod noop_events;

#[cfg(kafka)]
mod kafka;
mod messages;

static EVENTS_BROADCAST: Lazy<Sender<DomainEvent>> = Lazy::new(|| {
    let (sender, _) = tokio::sync::broadcast::channel(0xff);
    sender
});

pub fn subscribe_domain_events() -> Receiver<DomainEvent> {
    EVENTS_BROADCAST.subscribe()
}

pub fn notify_domain_event(event: DomainEvent) {
    let _ = EVENTS_BROADCAST.send(event);
}

#[instrument(skip_all, err)]
pub async fn init(commands: DomainCommandSource, events: DomainEventSink) -> anyhow::Result<()> {
    match commands {
        DomainCommandSource::Disabled => {
            // nothing to do
        }
        DomainCommandSource::Kafka { topic,
                                     brokers,
                                     username,
                                     password,
                                     offset, } => {
            #[cfg(kafka)]
            {
                kafka::commands::init(topic, brokers, username, password, offset).await?;
            }
            #[cfg(not(kafka))]
            {
                drop((topic, brokers, username, password, offset));
                return Err(anyhow!("Kafka command source support is not enabled"));
            }
        }
        DomainCommandSource::JetStream { url: _, topic: _ } => {
            return Err(anyhow!("JetStream command source is not yet supported"));
        }
    }

    let id: ActorId = "domain_event_sink".into();
    match events {
        DomainEventSink::Disabled => {
            noop_events::init(id).await?;
        }
        DomainEventSink::Log => {
            log_events::init(id).await?;
        }
        DomainEventSink::Kafka { topic: _,
                                 brokers: _,
                                 username: _,
                                 password: _, } => {
            #[cfg(kafka)]
            {
                kafka::events::init(topic, brokers, username, password).await?;
            }
            #[cfg(not(kafka))]
            {
                return Err(anyhow!("Kafka event sink support is not enabled"));
            }
        }
        DomainEventSink::JetStream { url: _, topic: _ } => {
            return Err(anyhow!("JetStream event sink is not yet supported"));
        }
    }

    Ok(())
}
