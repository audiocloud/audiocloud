use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::instance::control::{InstancePlayControl, InstancePowerControl};
use crate::instance::driver::events::InstanceDriverEvent;
use crate::instance::driver::requests::{SetInstanceParameterResponse, SetInstanceParametersRequest};
use crate::instance::spec::InstanceSpec;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RtRequest {
  pub request_id: String,
  pub command:    RtCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum RtCommand {
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
  #[serde(rename_all = "camelCase")]
  CreatePeerConnection,
  #[serde(rename_all = "camelCase")]
  AcceptPeerConnection { offer: String },
  #[serde(rename_all = "camelCase")]
  OfferPeerConnectionCandidate { candidate: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum RtEvent {
  #[serde(rename_all = "camelCase")]
  SetInstancePowerControl {
    success:    bool,
    request_id: String,
  },
  #[serde(rename_all = "camelCase")]
  SetInstancePlayControl {
    success:    bool,
    request_id: String,
  },
  #[serde(rename_all = "camelCase")]
  SetInstanceSpec {
    instance_id: String,
    spec:        Option<InstanceSpec>,
  },
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
  SubscribeToInstanceEvents {
    success:    bool,
    request_id: String,
  },
  #[serde(rename_all = "camelCase")]
  UnsubscribeFromInstanceEvents {
    success:    bool,
    request_id: String,
  },
  OfferPeerConnection {
    offer: String,
  },
  #[serde(rename_all = "camelCase")]
  OfferPeerConnectionCandidate {
    candidate: String,
  },
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(RtRequest), schema_for!(RtEvent)].into_iter())
}
