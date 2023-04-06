use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OscDriverConfig {
  pub host:       String,
  pub port:       u16,
  #[serde(default)]
  pub use_tcp:    bool,
  pub parameters: HashMap<String, Vec<OscParameterConfig>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OscParameterConfig {
  pub osc_type:      String,
  pub path_template: String,
  #[serde(default)]
  pub transform:     Option<String>,
}
