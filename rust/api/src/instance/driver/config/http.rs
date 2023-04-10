use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HttpDriverConfig {
  pub base_url:   String,
  #[serde(default)]
  pub parameters: HashMap<String, HttpDriverParameter>,
  #[serde(default)]
  pub reports:    HashMap<String, HttpDriverReport>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HttpDriverParameter {
  pub url:     String,
  #[serde(default)]
  pub method:  HttpMethod,
  #[serde(default)]
  pub body:    Option<String>,
  #[serde(default)]
  pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
  GET,
  PUT,
  POST,
}

impl Default for HttpMethod {
  fn default() -> Self {
    HttpMethod::POST
  }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HttpDriverReport {
  pub path:         String,
  #[serde(default)]
  pub method:       HttpMethod,
  #[serde(default)]
  pub body:         Option<String>,
  pub response:     String,
  #[serde(default = "default_report_poll_time")]
  pub poll_time_ms: u64,
}

fn default_report_poll_time() -> u64 {
  5000
}
