use std::sync::Arc;

use crate::nats::Nats;

pub mod instance;
pub mod media;
pub mod users;

#[derive(Clone)]
pub struct Service {
  pub nats:   Nats,
  pub config: Arc<ServiceConfig>,
}

pub struct ServiceConfig {
  pub jwt_secret: String,
}

pub type Result<T = ()> = anyhow::Result<T>;
