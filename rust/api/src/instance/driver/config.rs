use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

pub mod osc;
pub mod serial;
pub mod usb_hid;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InstanceDriverConfig {
  #[serde(rename = "USBHID")]
  USBHID(usb_hid::UsbHidDriverConfig),
  #[serde(rename = "serial")]
  Serial(serial::SerialDriverConfig),
  #[serde(rename = "OSC")]
  OSC(osc::OscDriverConfig),
}

fn read_interval_ms_default() -> i32 {
  10
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OscParameterConfig {}

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

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(InstanceDriverConfig)].into_iter())
}
