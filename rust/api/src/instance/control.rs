use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::{BucketKey, IntoBucketKey, Timestamp};

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

pub fn instance_power_control_key<T: ToString>(instance_id: &T) -> BucketKey<InstancePowerControl> {
  instance_id.to_bucket_key()
}

pub fn instance_play_control_key<T: ToString>(instance_id: &T) -> BucketKey<InstancePlayControl> {
  instance_id.to_bucket_key()
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(InstancePowerControl), schema_for!(InstancePlayControl)].into_iter())
}
