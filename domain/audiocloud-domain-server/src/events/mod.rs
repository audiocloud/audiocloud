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
                return Err(anyhow!("Kafka command source support is not enabled"));
            }
        }
        DomainCommandSource::JetStream { url, topic } => {
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
        DomainEventSink::Kafka { topic,
                                 brokers,
                                 username,
                                 password, } => {
            #[cfg(kafka)]
            {
                kafka::events::init(topic, brokers, username, password).await?;
            }
            #[cfg(not(kafka))]
            {
                return Err(anyhow!("Kafka event sink support is not enabled"));
            }
        }
        DomainEventSink::JetStream { url, topic } => {
            return Err(anyhow!("JetStream event sink is not yet supported"));
        }
    }

    Ok(())
}
