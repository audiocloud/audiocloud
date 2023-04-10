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
  SetInstancePowerControl {
    instance_id: String,
    power:       InstancePowerControl,
  },
  SetInstancePlayControl {
    instance_id: String,
    play:        InstancePlayControl,
  },
  SetInstanceParameters(SetInstanceParametersRequest),
  SubscribeToInstanceEvents {
    instance_id: String,
  },
  UnsubscribeFromInstanceEvents {
    instance_id: String,
  },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WsEvent {
  SetInstancePowerControl {
    success:    bool,
    request_id: String,
  },
  SetInstancePlayControl {
    success:    bool,
    request_id: String,
  },
  SetInstanceParameters {
    response:   SetInstanceParameterResponse,
    request_id: String,
  },
  InstanceDriverEvent(InstanceDriverEvent),
  SubscribeToInstanceReports {
    success:    bool,
    request_id: String,
  },
  UnsubscribeFromInstanceReports {
    success:    bool,
    request_id: String,
  },
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(WsRequest), schema_for!(WsEvent)].into_iter())
}
