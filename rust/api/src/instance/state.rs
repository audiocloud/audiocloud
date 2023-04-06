use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::instance::{InstancePlayState, InstancePowerState};
use crate::BucketKey;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InstanceState {
  pub power: Option<InstancePowerState>,
  pub play:  Option<InstancePlayState>,
}

pub fn instance_state(instance_id: impl ToString) -> BucketKey<InstanceState> {
  BucketKey::new(instance_id)
}
