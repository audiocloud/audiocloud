use schemars::schema::RootSchema;
use schemars::schema_for;
use schemars_zod::merge_schemas;

use crate::instance::{InstanceConnectionState, InstancePlayState, InstancePowerState};
use crate::BucketKey;

pub fn instance_power_state_key<T: ToString>(instance_id: &T) -> BucketKey<String, InstancePowerState> {
  instance_id.to_string().into()
}

pub fn instance_connection_state_key<T: ToString>(instance_id: &T) -> BucketKey<String, InstanceConnectionState> {
  instance_id.to_string().into()
}

pub fn instance_play_state_key<T: ToString>(instance_id: &T) -> BucketKey<String, InstancePlayState> {
  instance_id.to_string().into()
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(InstancePowerState), schema_for!(InstancePlayState)].into_iter())
}
