use std::fmt::Debug;
use std::time::Instant;

use serde::de::DeserializeOwned;

use api::driver::InstanceDriverEvent;
use api::instance::RegisterInstanceRequest;

use crate::ServiceRegistry;

pub mod bin_page_utils;
pub mod run;
pub mod scripting;
pub mod serial;
pub mod service;
pub mod usb_hid;

pub type Result<T = ()> = anyhow::Result<T>;

pub trait Driver: Sized {
  type Config: DeserializeOwned + Debug + Clone;
  type Shared;

  fn create_shared() -> Result<Self::Shared>;

  fn new(instance_id: &str, shared: &mut Self::Shared, config: Self::Config) -> Result<Self>;

  fn set_parameter(&mut self, shared: &mut Self::Shared, parameter: &str, channel: usize, value: f64) -> Result<()>;

  fn poll(&mut self, shared: &mut Self::Shared, deadline: Instant) -> Result<Vec<InstanceDriverEvent>>;

  fn can_continue(&self) -> bool;
}

impl ServiceRegistry {
  pub fn register_instance(&self, _instance: RegisterInstanceRequest) -> Result {
    Ok(())
  }
}
