use std::collections::{HashMap, HashSet};

use actix::{Message, Recipient};
use actix_broker::{Broker, SystemBroker};
use serde::{Deserialize, Serialize};
use tracing::*;

use audiocloud_api::instance_driver::{InstanceDriverCommand, InstanceDriverError, InstanceDriverEvent};
use audiocloud_api::newtypes::FixedInstanceId;

pub mod distopik;
pub mod driver;
pub mod http_client;
pub mod nats;
pub mod netio;
pub mod rest_api;
pub mod supervisor;
pub mod utils;

pub type ConfigFile = HashMap<FixedInstanceId, DriverConfig>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DriverConfig {
    Distopik(distopik::Config),
    Netio(netio::Config),
}

impl InstanceConfig for DriverConfig {
    fn create(self, id: FixedInstanceId) -> anyhow::Result<Recipient<Command>> {
        match self {
            DriverConfig::Distopik(c) => c.create(id),
            DriverConfig::Netio(c) => c.create(id),
        }
    }
}

pub trait InstanceConfig {
    fn create(self, id: FixedInstanceId) -> anyhow::Result<Recipient<Command>>;
}

#[derive(Message)]
#[rtype(result = "Result<(), InstanceDriverError>")]
pub struct Command {
    pub instance_id: FixedInstanceId,
    pub command:     InstanceDriverCommand,
}

#[derive(Message)]
#[rtype(result = "Result<NotifyInstanceValues, InstanceDriverError>")]
pub struct GetValues {
    pub instance_id: FixedInstanceId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct NotifyInstanceValues {
    pub instance_id: FixedInstanceId,
    pub parameters:  serde_json::Value,
    pub reports:     serde_json::Value,
}

impl NotifyInstanceValues {
    pub fn new(instance_id: FixedInstanceId) -> Self {
        Self { instance_id,
               parameters: Default::default(),
               reports: Default::default() }
    }
}

#[derive(Message)]
#[rtype(result = "HashSet<FixedInstanceId>")]
pub struct GetInstances;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Event {
    pub instance_id: FixedInstanceId,
    pub event:       InstanceDriverEvent,
}

pub fn emit_event(instance_id: FixedInstanceId, event: InstanceDriverEvent) {
    info!(id = %instance_id, "{}", serde_json::to_string(&event).unwrap());
    Broker::<SystemBroker>::issue_async(Event { instance_id, event });
}
