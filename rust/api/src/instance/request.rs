use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::instance_driver::config::InstanceDriverConfig;
use crate::instance::spec::{InstancePlaySpec, InstancePowerSpec};
use crate::Request;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterOrUpdateInstanceRequest {
  pub id:            String,
  pub model_id:      String,
  pub driver_id:     String,
  pub power_spec:    Option<InstancePowerSpec>,
  pub play_spec:     Option<InstancePlaySpec>,
  pub driver_config: InstanceDriverConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum RegisterOrUpdateInstanceResponse {
  Success,
}

pub fn register_or_update_instance_request() -> Request<RegisterOrUpdateInstanceRequest, RegisterOrUpdateInstanceResponse> {
  Request::new("audiocloud_instance_register_or_update")
}
