use tracing::*;

use audiocloud_api::cloud::domains::{DomainCommandSource, DomainEventSink};
pub use messages::*;

mod log_events;
mod noop_events;

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
            kafka::commands::init(topic, brokers, username, password, offset).await?;
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
            kafka::events::init(topic, brokers, username, password).await?;
        }
    }

    Ok(())
}
