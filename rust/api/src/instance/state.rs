use schemars::schema::RootSchema;
use schemars::schema_for;
use schemars_zod::merge_schemas;

use crate::instance::{InstanceConnectionState, InstancePlayState, InstancePowerState};
use crate::{BucketKey, IntoBucketKey};

pub fn instance_power_state_key<T: ToString>(instance_id: &T) -> BucketKey<InstancePowerState> {
  instance_id.to_bucket_key()
}

pub fn instance_connection_state_key<T: ToString>(instance_id: &T) -> BucketKey<InstanceConnectionState> {
  instance_id.to_bucket_key()
}

pub fn instance_play_state_key<T: ToString>(instance_id: &T) -> BucketKey<InstancePlayState> {
  instance_id.to_bucket_key()
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(InstancePowerState), schema_for!(InstancePlayState)].into_iter())
}
