use schemars::schema::RootSchema;
use schemars_zod::merge_schemas;

pub mod config;
pub mod events;
pub mod requests;
pub mod spec;

pub mod buckets {
  use crate::instance::driver::spec::DriverServiceSpec;
  use crate::BucketName;

  pub const DRIVER_SPEC: BucketName<DriverServiceSpec> = BucketName::new("audiocloud_driver_spec");
}

pub fn schema() -> RootSchema {
  merge_schemas([config::schema(), requests::schema(), spec::schema(), events::schema()].into_iter())
}
