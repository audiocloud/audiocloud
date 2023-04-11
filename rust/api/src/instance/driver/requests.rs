use std::fmt::{Display, Formatter};

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::Request;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstanceParameter {
  pub parameter: String,
  pub channel:   usize,
  pub value:     f64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstanceParametersRequest {
  pub instance_id: String,
  pub changes:     Vec<SetInstanceParameter>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SetInstanceParameterResponse {
  Success,
  ParameterNotFound,
  ChannelNotFound,
  NotConnected,
  EncodingError,
  ConnectionError,
  RpcFailure,
}

impl Display for SetInstanceParameterResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      | SetInstanceParameterResponse::Success => {
        write!(f, "Success")
      }
      | SetInstanceParameterResponse::ParameterNotFound => {
        write!(f, "Parameter not found")
      }
      | SetInstanceParameterResponse::ChannelNotFound => {
        write!(f, "Channel not found")
      }
      | SetInstanceParameterResponse::NotConnected => {
        write!(f, "Not connected")
      }
      | SetInstanceParameterResponse::RpcFailure => {
        write!(f, "RPC failure")
      }
      | SetInstanceParameterResponse::EncodingError => {
        write!(f, "Encoding error")
      }
      | SetInstanceParameterResponse::ConnectionError => {
        write!(f, "Connection error")
      }
    }
  }
}

pub fn set_instance_parameters_request(instance_id: impl AsRef<str>) -> Request<Vec<SetInstanceParameter>, SetInstanceParameterResponse> {
  Request::new(format!("audiocloud_driver.{}.set_parameters", instance_id.as_ref()))
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(SetInstanceParameter), schema_for!(SetInstanceParameterResponse)].into_iter())
}
