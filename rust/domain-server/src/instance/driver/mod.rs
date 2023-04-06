use std::fmt::Debug;
use std::time::Instant;

use serde::de::DeserializeOwned;

use crate::instance;

pub mod bin_page_utils;
pub mod run;
pub mod scripting;
pub mod serial;
pub mod service;
pub mod usb_hid;

pub type Result<T = ()> = super::Result<T>;

pub trait Driver: Sized {
  type Config: DeserializeOwned + Debug + Clone;
  type Shared;

  fn create_shared() -> instance::Result<Self::Shared>;

  fn new(instance_id: &str, shared: &mut Self::Shared, config: Self::Config) -> instance::Result<Self>;

  fn set_parameter(&mut self, shared: &mut Self::Shared, parameter: &str, channel: usize, value: f64) -> instance::Result<()>;

  fn poll(&mut self, shared: &mut Self::Shared, deadline: Instant) -> instance::Result<Vec<InstanceDriverEvent>>;

  fn can_continue(&self) -> bool;
}
