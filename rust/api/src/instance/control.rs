use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::{BucketKey, Timestamp};

use super::{DesiredInstancePlayState, DesiredInstancePowerState};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePowerControl {
  pub desired: DesiredInstancePowerState,
  pub until:   Timestamp,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePlayControl {
  pub desired: DesiredInstancePlayState,
  pub until:   Timestamp,
}

pub fn instance_power_control(instance_id: impl ToString) -> BucketKey<InstancePowerControl> {
  BucketKey::new(instance_id)
}

pub fn instance_play_control(instance_id: impl ToString) -> BucketKey<InstancePlayControl> {
  BucketKey::new(instance_id)
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(InstancePowerControl), schema_for!(InstancePlayControl)].into_iter())
}
