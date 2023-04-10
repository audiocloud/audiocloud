use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::{Events, Timestamp};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InstanceDriverEvent {
  Connected { connected: bool },
  Report(InstanceDriverReportEvent),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstanceDriverReportEvent {
  pub instance_id: String,
  pub report_id:   String,
  pub channel:     usize,
  pub value:       f64,
  pub captured_at: Timestamp,
}

pub fn instance_driver_events(instance_id: impl AsRef<str>) -> Events<InstanceDriverEvent> {
  Events::new(format!("audiocloud_instance.{}.events", instance_id.as_ref()))
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(InstanceDriverEvent), schema_for!(InstanceDriverReportEvent)].into_iter())
}
