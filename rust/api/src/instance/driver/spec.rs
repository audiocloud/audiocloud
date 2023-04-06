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

pub fn driver_spec(driver_id: impl ToString) -> BucketKey<DriverServiceSpec> {
  BucketKey::new(driver_id)
}

pub fn schema() -> RootSchema {
  schema_for!(DriverServiceSpec)
}
