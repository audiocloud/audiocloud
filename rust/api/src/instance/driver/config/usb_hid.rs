use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{BinaryPosition, Clamp, Remap, Rescale, ValuePacking};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidDriverConfig {
  #[serde(default)]
  pub vendor_id:         Option<u16>,
  #[serde(default)]
  pub product_id:        Option<u16>,
  #[serde(default)]
  pub serial_number:     Option<String>,
  #[serde(default = "super::read_interval_ms_default")]
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
  #[serde(default = "default_frame_mask")]
  pub frame_mask:        u8,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidParameterPage {
  pub page:                  u8,
  #[serde(default = "default_hid_page_size")]
  pub size:                  usize,
  #[serde(default)]
  pub copy_from_report_page: Option<u8>,
  #[serde(default)]
  pub header:                Vec<u8>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsbHidReportPage {
  pub page: u8,
  #[serde(default = "default_hid_page_size")]
  pub size: usize,
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
pub struct UsbHidReportConfig {
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
}

fn page_zero() -> u8 {
  0
}

fn default_hid_page_size() -> usize {
  80
}

fn default_frame_mask() -> u8 {
  0xFF
}
