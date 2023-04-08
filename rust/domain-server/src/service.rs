use std::collections::HashMap;

use api::instance::spec::InstanceSpec;

use crate::nats::Nats;

#[derive(Clone)]
pub struct Service {
  pub nats: Nats,
}

pub type Result<T = ()> = anyhow::Result<T>;

impl Service {
  pub async fn list_instances(&self, filter: String) -> Result<HashMap<String, InstanceSpec>> {
    Ok(self.nats.instance_spec.scan(filter.as_str()).await?)
  }
}
