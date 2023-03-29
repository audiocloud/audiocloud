use serde::{Deserialize, Serialize};

pub mod instance_driver;
pub mod nats_utils;
pub mod request_tracker;

pub type Result<T = ()> = anyhow::Result<T>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "service", rename_all = "camelCase")]
pub enum ServiceId {
  Instances(String),
  Driver(String),
  Tasks(String),
  Media(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "id", rename_all = "camelCase")]
pub enum ServiceRef {
  Local(ServiceId),
  Remote(ServiceId),
}

#[derive(Clone)]
pub struct ServiceRegistry {}
