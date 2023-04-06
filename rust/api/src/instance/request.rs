use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::instance::driver::config::InstanceDriverConfig;
use crate::instance::spec::{InstanceMediaSpec, InstancePowerSpec};
use crate::Request;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterOrUpdateInstanceRequest {
  pub id:            String,
  pub model_id:      String,
  pub driver_id:     String,
  pub power_spec:    Option<InstancePowerSpec>,
  pub play_spec:     Option<InstanceMediaSpec>,
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

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(RegisterOrUpdateInstanceRequest),
                 schema_for!(RegisterOrUpdateInstanceResponse)].into_iter())
}
