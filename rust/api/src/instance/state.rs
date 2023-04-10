use schemars::schema::RootSchema;
use schemars::schema_for;
use schemars_zod::merge_schemas;

use crate::instance::{InstancePlayState, InstancePowerState};
use crate::BucketKey;

pub fn instance_power_state(instance_id: impl ToString) -> BucketKey<InstancePowerState> {
  BucketKey::new(instance_id)
}

pub fn instance_play_state(instance_id: impl ToString) -> BucketKey<InstancePlayState> {
  BucketKey::new(instance_id)
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(InstancePowerState), schema_for!(InstancePlayState)].into_iter())
}
