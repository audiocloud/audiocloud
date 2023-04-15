pub mod instance;
pub mod media;
pub mod nats;
pub mod request_tracker;
pub mod rest_api;
pub mod service;
pub mod tasks;

pub type Result<T = ()> = anyhow::Result<T>;

pub mod meta {
  pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
}
