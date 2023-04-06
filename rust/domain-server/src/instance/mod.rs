use api::instance::request::RegisterOrUpdateInstanceRequest;

use crate::ServiceRegistry;

pub mod driver;
pub mod service;

pub type Result<T = ()> = anyhow::Result<T>;

impl ServiceRegistry {
  pub fn register_instance(&self, _instance: RegisterOrUpdateInstanceRequest) -> Result {
    Ok(())
  }
}
