use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::instance_driver::config::OscParameterConfig;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OscDriverConfig {
  pub host:       String,
  pub port:       u16,
  #[serde(default)]
  pub use_tcp:    bool,
  pub parameters: HashMap<String, Vec<OscParameterConfig>>,
}
