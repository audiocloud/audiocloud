/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use anyhow::anyhow;
use tracing::*;

use audiocloud_api::cloud::domains::{DomainCommandSource, DomainEventSink};
pub use messages::*;

mod log_events;
mod noop_events;

#[cfg(kafka)]
mod kafka;
mod messages;

#[instrument(skip_all, err)]
pub async fn init(commands: DomainCommandSource, events: DomainEventSink) -> anyhow::Result<()> {
    match commands {
        DomainCommandSource::Disabled => {
            // nothing to do
        }
        DomainCommandSource::Kafka { topic: _,
                                     brokers: _,
                                     username: _,
                                     password: _,
                                     offset: _, } => {
            #[cfg(kafka)]
            {
                kafka::commands::init(topic, brokers, username, password, offset).await?;
            }
            #[cfg(not(kafka))]
            {
                return Err(anyhow!("Kafka command source support is not enabled"));
            }
        }
        DomainCommandSource::JetStream { url: _, topic: _ } => {
            return Err(anyhow!("JetStream command source is not yet supported"));
        }
    }

    match events {
        DomainEventSink::Disabled => {
            noop_events::init().await?;
        }
        DomainEventSink::Log => {
            log_events::init().await?;
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
