use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BucketKey;

use super::{DesiredInstancePlayState, DesiredInstancePowerState};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePowerControl {
  pub desired:         DesiredInstancePowerState,
  pub max_duration_ms: u32,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePlayControl {
  pub desired:         DesiredInstancePlayState,
  pub max_duration_ms: u32,
}

pub fn instance_power_control(instance_id: impl ToString) -> BucketKey<InstancePowerControl> {
  BucketKey::new(instance_id)
}

pub fn instance_play_control(instance_id: impl ToString) -> BucketKey<InstancePlayControl> {
  BucketKey::new(instance_id)
}
