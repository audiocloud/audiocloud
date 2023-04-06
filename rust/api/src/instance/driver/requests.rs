use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::Request;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstanceParameterRequest {
  pub parameter: String,
  pub channel:   usize,
  pub value:     f64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SetInstanceParameterResponse {
  Success,
  ParameterNotFound,
  ChannelNotFound,
}

pub fn set_instance_parameters_request(instance_id: impl AsRef<str>)
                                       -> Request<Vec<SetInstanceParameterRequest>, SetInstanceParameterResponse> {
  Request::new(format!("audiocloud_driver_{}_set_parameters", instance_id.as_ref()))
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(SetInstanceParameterRequest), schema_for!(SetInstanceParameterResponse)].into_iter())
}
