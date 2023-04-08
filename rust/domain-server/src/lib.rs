use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

pub mod instance;
pub mod nats;
pub mod request_tracker;
pub mod tasks;
pub mod rest_api;
pub mod service;

pub type Result<T = ()> = anyhow::Result<T>;

pub struct ServiceRegistry {
  driver: JoinHandle<()>,
}
