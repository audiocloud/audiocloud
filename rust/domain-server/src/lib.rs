use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

pub mod instance;
pub mod nats_utils;
pub mod request_tracker;
pub mod tasks;

pub type Result<T = ()> = anyhow::Result<T>;

pub struct ServiceRegistry {
  driver: JoinHandle<()>,
}
