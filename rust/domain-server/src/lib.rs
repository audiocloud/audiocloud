pub mod instance;
pub mod media;
pub mod nats;
pub mod request_tracker;
pub mod rest_api;
pub mod rtc_socket;
pub mod service;
pub mod tasks;
mod ws_socket;

pub type Result<T = ()> = anyhow::Result<T>;

pub mod meta {
  pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
}
