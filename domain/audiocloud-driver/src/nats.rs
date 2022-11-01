use std::collections::HashSet;

use actix::{Actor, AsyncContext, Context, Handler, Message};
use actix_broker::BrokerSubscribe;
use anyhow::anyhow;
use clap::Args;
use nats_aflowt::Connection;
use once_cell::sync::OnceCell;
use serde::Serialize;
use tokio::spawn;
use tracing::*;

use audiocloud_api::api::codec::{Codec, Json};
use audiocloud_api::common::error::SerializableResult;
use audiocloud_api::instance_driver::{InstanceDriverCommand, InstanceDriverEvent};
use audiocloud_api::newtypes::FixedInstanceId;

use crate::info;

#[derive(Args, Clone, Debug)]
pub struct NatsOpts {
    #[clap(env, long, default_value = "nats://localhost:4222")]
    pub nats_url: String,
}

static NATS: OnceCell<Connection> = OnceCell::new();

pub async fn init(opts: NatsOpts) -> anyhow::Result<()> {
    let connection = nats_aflowt::connect(opts.nats_url.as_str()).await?;

    NATS.set(connection).map_err(|_| anyhow!("State init already called!"))?;

    Ok(())
}

pub fn get_nats() -> &'static Connection {
    NATS.get().expect("NATS not initialized")
}

pub fn publish(instance_id: &FixedInstanceId, event: InstanceDriverEvent) {
    match serde_json::to_string(&event) {
        Ok(event) => {
            let subject = instance_id.driver_event_subject();
            spawn(async move { get_nats().publish(&subject, event.as_bytes()).await });
        }
        Err(error) => {
            error!(%error, ?event, "Failed to serialize event");
        }
    }
}
