use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{Clamp, Remap, Rescale};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerialDriverConfig {
  #[serde(default)]
  pub vendor_id:                      Option<u16>,
  #[serde(default)]
  pub product_id:                     Option<u16>,
  #[serde(default = "default_baud_rate")]
  pub baud_rate:                      u32,
  #[serde(default)]
  pub flow_control:                   Option<SerialFlowControl>,
  #[serde(default)]
  pub serial_number:                  Option<String>,
  #[serde(default)]
  pub serial_port:                    Option<String>,
  #[serde(default)]
  pub line_handler:                   Option<String>,
  #[serde(default = "default_line_terminator")]
  pub send_line_terminator:           String,
  #[serde(default = "default_line_terminator")]
  pub receive_line_terminator:        String,
  #[serde(default)]
  pub parameters:                     HashMap<String, Vec<SerialParameterConfig>>,
  #[serde(default)]
  pub reports:                        HashMap<String, Vec<SerialReportConfig>>,
  #[serde(default)]
  pub comments_start_with:            Vec<String>,
  #[serde(default)]
  pub errors_start_with:              Vec<String>,
  #[serde(default)]
  pub read_response_after_every_send: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SerialFlowControl {
  XonXoff,
  RtsCts,
}

fn default_baud_rate() -> u32 {
  // from mixanalog v2
  460_800
}

fn default_receive_time_out() -> u64 {
  1000
}

fn default_line_terminator() -> String {
  "\r\n".to_string()
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerialParameterConfig {
  #[serde(default)]
  pub format_string:   Option<String>,
  #[serde(default)]
  pub transform:       Option<String>,
  #[serde(default)]
  pub to_string:       Option<String>,
  #[serde(default)]
  pub rescale:         Option<Rescale>,
  #[serde(default)]
  pub remap:           Option<Remap>,
  #[serde(default)]
  pub clamp:           Option<Clamp>,
  #[serde(default)]
  pub line_terminator: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerialReportConfig {
  pub matcher:       SerialReportMatcher,
  #[serde(default)]
  pub value:         SerialReportValueInterpretation,
  #[serde(default)]
  pub rescale:       Option<Rescale>,
  #[serde(default)]
  pub remap:         Option<Remap>,
  #[serde(default)]
  pub clamp:         Option<Clamp>,
  #[serde(default)]
  pub request_timer: Option<SerialRequestTimer>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerialRequestTimer {
  pub line:        String,
  pub interval_ms: u64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SerialReportMatcher {
  StringPrefix { prefix: String },
  Matches { regex: String },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SerialReportValueInterpretation {
  ParseFloat,
  ParseDateTimeToSeconds { format: String },
  ParseInteger { base: u8 },
  Custom { function: String },
}

impl Default for SerialReportValueInterpretation {
  fn default() -> Self {
    Self::ParseFloat
  }
}
