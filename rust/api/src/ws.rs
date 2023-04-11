use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::instance::control::{InstancePlayControl, InstancePowerControl};
use crate::instance::driver::events::InstanceDriverEvent;
use crate::instance::driver::requests::{SetInstanceParameterResponse, SetInstanceParametersRequest};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WsRequest {
  pub request_id: String,
  pub command:    WsCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WsCommand {
  #[serde(rename_all = "camelCase")]
  SetInstancePowerControl {
    instance_id: String,
    power:       InstancePowerControl,
  },
  #[serde(rename_all = "camelCase")]
  SetInstancePlayControl {
    instance_id: String,
    play:        InstancePlayControl,
  },
  #[serde(rename_all = "camelCase")]
  SetInstanceParameters(SetInstanceParametersRequest),
  #[serde(rename_all = "camelCase")]
  SubscribeToInstanceEvents { instance_id: String },
  #[serde(rename_all = "camelCase")]
  UnsubscribeFromInstanceEvents { instance_id: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WsEvent {
  #[serde(rename_all = "camelCase")]
  SetInstancePowerControl { success: bool, request_id: String },
  #[serde(rename_all = "camelCase")]
  SetInstancePlayControl { success: bool, request_id: String },
  #[serde(rename_all = "camelCase")]
  SetInstanceParameters {
    response:   SetInstanceParameterResponse,
    request_id: String,
  },
  #[serde(rename_all = "camelCase")]
  InstanceDriverEvent {
    instance_id: String,
    event:       InstanceDriverEvent,
  },
  #[serde(rename_all = "camelCase")]
  SubscribeToInstanceEvents { success: bool, request_id: String },
  #[serde(rename_all = "camelCase")]
  UnsubscribeFromInstanceEvents { success: bool, request_id: String },
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(WsRequest), schema_for!(WsEvent)].into_iter())
}
