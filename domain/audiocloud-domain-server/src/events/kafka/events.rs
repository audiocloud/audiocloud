/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Actor, ActorContext, Addr, Context, Handler};
use actix_broker::BrokerSubscribe;
use anyhow::anyhow;
use once_cell::sync::OnceCell;
use rdkafka::config::FromClientConfigAndContext;
use rdkafka::producer::{BaseProducer, BaseRecord, DefaultProducerContext};
use tracing::*;

use audiocloud_api::{Codec, Json};

use crate::events::messages::NotifyDomainEvent;

static KAFKA_DOMAIN_EVENTS_SINK: OnceCell<Addr<KafkaDomainEventsSink>> = OnceCell::new();

pub async fn init(topic: String, brokers: String, username: String, password: String) -> anyhow::Result<()> {
    KAFKA_DOMAIN_EVENTS_SINK.set(KafkaDomainEventsSink { topic,
                                                         brokers,
                                                         username,
                                                         password,
                                                         producer: None }.start())
                            .map_err(|_| anyhow!("KAFKA_DOMAIN_EVENTS_SINK already initialized"))?;

    Ok(())
}

pub struct KafkaDomainEventsSink {
    topic:    String,
    brokers:  String,
    username: String,
    password: String,
    producer: Option<BaseProducer>,
}

impl KafkaDomainEventsSink {
    #[instrument(skip_all)]
    fn init(&mut self, ctx: &mut Context<Self>) {
        self.subscribe_system_async::<NotifyDomainEvent>(ctx);

        let config = super::create_config(&self.brokers, &self.username, &self.password);

        self.producer = Some(BaseProducer::from_config_and_context(&config, DefaultProducerContext).expect("create producer"));
    }
}

impl Actor for KafkaDomainEventsSink {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.init(ctx);
    }
}

impl Handler<NotifyDomainEvent> for KafkaDomainEventsSink {
    type Result = ();

    #[instrument(skip_all, name = "handle_notify_domain_event")]
    fn handle(&mut self, msg: NotifyDomainEvent, ctx: &mut Self::Context) -> Self::Result {
        match self.producer.as_mut() {
            Some(producer) => match Json.serialize(&msg.event) {
                Ok(encoded) => {
                    let key = msg.event.key();
                    if let Err(error) = producer.send(BaseRecord::to(&self.topic).key(&key).payload(&encoded[..])) {
                        warn!(?error, "Failed to send domain event to Kafka")
                    }
                }
                Err(error) => {
                    warn!(?error, "Failed to serialize event");
                }
            },
            None => {
                error!("Kafka producer not initialized");
                ctx.stop();
            }
        }
    }
}
