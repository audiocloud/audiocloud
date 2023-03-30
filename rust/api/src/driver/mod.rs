use std::collections::HashMap;
use std::time::Instant;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::instance::{DesiredInstancePlayState, DesiredInstancePowerState};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstancePowerRequest {
  pub channel: u32,
  pub power:   DesiredInstancePowerState,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstancePlayRequest {
  pub play: DesiredInstancePlayState,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInstanceParameterRequest {
  #[serde(flatten)]
  pub parameter: String,
  pub channel:   usize,
  pub value:     f64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InstanceDriverConfig {
  #[serde(rename = "USBHID")]
  USBHID(UsbHidDriverConfig),
  #[serde(rename = "serial")]
  Serial(SerialDriverConfig),
  #[serde(rename = "OSC")]
  OSC(OscDriverConfig),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidDriverConfig {
  #[serde(default)]
  pub vendor_id:         Option<u16>,
  #[serde(default)]
  pub product_id:        Option<u16>,
  #[serde(default)]
  pub serial_number:     Option<String>,
  #[serde(default = "read_interval_ms_default")]
  pub read_interval_ms:  i32,
  #[serde(default)]
  pub read_page_handler: Option<String>,
  #[serde(default)]
  pub parameters:        HashMap<String, Vec<UsbHidParameterConfig>>,
  #[serde(default)]
  pub reports:           HashMap<String, Vec<UsbHidReportConfig>>,
  #[serde(default)]
  pub parameter_pages:   Vec<UsbHidParameterPage>,
  #[serde(default)]
  pub report_pages:      Vec<UsbHidReportPage>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidParameterPage {
  pub page:                  u8,
  #[serde(default = "default_hid_page_size")]
  pub size:                  usize,
  #[serde(default)]
  pub copy_from_report_page: Option<u8>,
}

fn default_hid_page_size() -> usize {
  80
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidReportPage {
  pub page: u8,
  #[serde(default = "default_hid_page_size")]
  pub size: usize,
}

fn read_interval_ms_default() -> i32 {
  10
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidParameterConfig {
  pub position:  BinaryPosition,
  #[serde(default = "page_zero")]
  pub page:      u8,
  #[serde(default)]
  pub packing:   ValuePacking,
  #[serde(default)]
  pub transform: Option<String>,
  #[serde(default)]
  pub rescale:   Option<Rescale>,
  #[serde(default)]
  pub remap:     Option<Remap>,
  #[serde(default)]
  pub clamp:     Option<Clamp>,
}

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
pub struct OscDriverConfig {
  pub host:       String,
  pub port:       u16,
  #[serde(default)]
  pub use_tcp:    bool,
  pub parameters: HashMap<String, Vec<OscParameterConfig>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BinaryPosition {
  Byte(u32),
  Bytes(u32, u32),
  Bit(u32, u32),
  BitRange(Vec<(u32, u32)>),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Clamp {
  #[serde(default = "zero_f64")]
  pub min: f64,
  #[serde(default = "one_f64")]
  pub max: f64,
}

fn zero_f64() -> f64 {
  0f64
}
fn one_f64() -> f64 {
  1f64
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rescale {
  pub from: (f64, f64),
  pub to:   (f64, f64),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Remap {
  Linear { values: Vec<f64> },
  Pairs { pairs: Vec<(f64, f64)> },
}

fn page_zero() -> u8 {
  0
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ValuePacking {
  #[serde(rename = "uint8")]
  UInt8,
  #[serde(rename = "uint16le")]
  UInt16LE,
  #[serde(rename = "uint16be")]
  UInt16BE,
  #[serde(rename = "uint32le")]
  UInt32LE,
  #[serde(rename = "uint32be")]
  UInt32BE,
  #[serde(rename = "int8")]
  Int8,
  #[serde(rename = "int16le")]
  Int16LE,
  #[serde(rename = "int16be")]
  Int16BE,
  #[serde(rename = "int32le")]
  Int32LE,
  #[serde(rename = "int32be")]
  Int32BE,
  #[serde(rename = "float32le")]
  Float32LE,
  #[serde(rename = "float32be")]
  Float32BE,
  #[serde(rename = "float64le")]
  Float64LE,
  #[serde(rename = "float64be")]
  Float64BE,
}

impl Default for ValuePacking {
  fn default() -> Self {
    Self::UInt8
  }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidReportConfig {
  pub position:       BinaryPosition,
  #[serde(default = "page_zero")]
  pub page:           u8,
  #[serde(default)]
  pub packing:        ValuePacking,
  #[serde(default)]
  pub transform:      Option<String>,
  #[serde(default)]
  pub rescale:        Option<Rescale>,
  #[serde(default)]
  pub remap:          Option<Remap>,
  #[serde(default)]
  pub request_timers: Vec<SerialRequestTimer>,
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OscParameterConfig {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InstanceDriverEvent {
  Report(InstanceDriverReportEvent),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstanceDriverReportEvent {
  pub instance_id: String,
  pub report_id:   String,
  pub channel:     usize,
  pub value:       f64,
  #[serde(with = "serde_instant")]
  pub captured_at: Instant,
}

// we have up to three buckets for each service
// 1. service scoped specs bucket that lists the rarely changing spec of the objects it should manage
// 2. a globally scoped control bucket where we can manipulate desired object states, frequently
// 3. globally scoped state bucket where we can read the current state of the objects, frequently

pub mod buckets {
  pub fn instance_specs(service_id: &str) -> String {
    format!("audiocloud.driver.{}.specs", service_id)
  }

  pub const INSTANCE_CONTROL: &str = "audiocloud.instance.control";
  pub const INSTANCE_STATE: &str = "audiocloud.instance.state";
}

pub mod control_keys {
  pub fn instance_desired_power_state(instance_id: &str) -> String {
    format!("{}.power", instance_id)
  }

  pub fn instance_desired_play_state(instance_id: &str) -> String {
    format!("{}.play", instance_id)
  }

  pub fn instance_desired_parameter_value_wildcard(instance_id: &str) -> String {
    format!("{}.parameter.*", instance_id)
  }
}

pub mod status_keys {
  pub fn instance_power_state(instance_id: &str) -> String {
    format!("{instance_id}.power")
  }

  pub fn instance_play_state(instance_id: &str) -> String {
    format!("{instance_id}.play")
  }

  pub fn instance_report_value_wildcard(instance_id: &str) -> String {
    format!("{instance_id}.report.*")
  }

  pub fn instance_report_value(instance_id: &str, report_id: &str, channel: usize) -> String {
    format!("{instance_id}.report.{report_id}.{channel}")
  }
}

mod serde_instant {
  use std::time::{Duration, Instant};

  use serde::de::Error;
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
  {
    let duration = instant.elapsed();
    duration.serialize(serializer)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where D: Deserializer<'de>
  {
    let duration = Duration::deserialize(deserializer)?;
    let now = Instant::now();
    let instant = now.checked_sub(duration)
                     .ok_or_else(|| Error::custom("Invalid time manipulation, could not deserialize"))?;
    Ok(instant)
  }
}
