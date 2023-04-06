use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::instance::driver::events::InstanceDriverReportEvent;

use super::config::InstanceDriverConfig;
use super::requests::SetInstanceParameterRequest;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WsDriverRequest {
  SetParameter(SetInstanceParameterRequest),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WsDriverEvent {
  Report(InstanceDriverReportEvent),
  Config { config: InstanceDriverConfig },
  KeepAlive,
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(WsDriverRequest), schema_for!(WsDriverEvent)].into_iter())
}
