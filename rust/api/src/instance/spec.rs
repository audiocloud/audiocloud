use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::instance_driver::config::InstanceDriverConfig;
use crate::BucketKey;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InstanceSpec {
  pub id:            String,
  pub model_id:      String,
  pub driver_id:     String,
  pub power_spec:    Option<InstancePowerSpec>,
  pub play_spec:     Option<InstancePlaySpec>,
  pub driver_config: InstanceDriverConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePowerSpec {
  // instance id
  pub power_controller: String,
  pub channel:          u32,
  pub warm_up_ms:       u64,
  pub cool_down_ms:     u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePlaySpec {
  pub duration_ms: u64,
}

pub fn instance_spec(instance_id: impl ToString) -> BucketKey<InstanceSpec> {
  BucketKey::new(instance_id)
}
