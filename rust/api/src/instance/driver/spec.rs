use std::collections::HashSet;

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::BucketKey;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct DriverServiceSpec {
  driver_id:    String,
  instance_ids: HashSet<String>,
}

pub fn driver_spec<T: ToString>(driver_id: &T) -> BucketKey<String, DriverServiceSpec> {
  driver_id.to_string().into()
}

pub fn schema() -> RootSchema {
  schema_for!(DriverServiceSpec)
}
