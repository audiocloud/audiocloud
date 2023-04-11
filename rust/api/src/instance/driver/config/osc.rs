use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::instance::driver::config::{Clamp, Remap, Rescale};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OscDriverConfig {
  pub host:       String,
  pub port:       u16,
  #[serde(default)]
  pub use_tcp:    bool,
  #[serde(default)]
  pub parameters: HashMap<String, Vec<OscParameterConfig>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OscParameterConfig {
  #[serde(default = "default_osc_type")]
  pub osc_type:  String,
  pub address:   String,
  #[serde(default)]
  pub transform: Option<String>,
  #[serde(default)]
  pub rescale:   Option<Rescale>,
  #[serde(default)]
  pub remap:     Option<Remap>,
  #[serde(default)]
  pub clamp:     Option<Clamp>,
}

fn default_osc_type() -> String {
  "f".to_owned()
}
