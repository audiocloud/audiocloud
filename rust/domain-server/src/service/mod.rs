use crate::nats::Nats;

pub mod instance;
pub mod media;

#[derive(Clone)]
pub struct Service {
  pub nats: Nats,
}

pub type Result<T = ()> = anyhow::Result<T>;
