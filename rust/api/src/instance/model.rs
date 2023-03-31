use std::collections::{HashMap, HashSet};

use schemars::JsonSchema;
use schemars::_serde_json::{json, Value};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InstanceModel {
  #[serde(default)]
  pub parameters:    HashMap<String, ParameterModel>,
  #[serde(default)]
  pub reports:       HashMap<String, ReportModel>,
  #[serde(default = "default_audio_io_count")]
  pub audio_inputs:  usize,
  #[serde(default = "default_audio_io_count")]
  pub audio_outputs: usize,
  pub supports:      HashSet<InstanceFeature>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum InstanceFeature {
  MediaTransport,
  MidiNoteOnOff,
  DigitalInputOutput,
  Routing,
}

fn default_audio_io_count() -> usize {
  2
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ParameterModel {
  #[serde(default = "default_parameter_model_min")]
  pub min:            f64,
  #[serde(default = "default_parameter_model_max")]
  pub max:            f64,
  #[serde(default)]
  pub allowed_values: Vec<f64>,
  #[serde(default)]
  pub step:           Option<f64>,
  #[serde(default)]
  pub unit:           Option<String>,
  #[serde(default = "default_parameter_model_channels")]
  pub channels:       usize,
  #[serde(default = "default_metadata")]
  pub metadata:       Value,
}

fn default_parameter_model_min() -> f64 {
  0.0
}

fn default_parameter_model_max() -> f64 {
  1.0
}

fn default_parameter_model_channels() -> usize {
  2
}

fn default_metadata() -> Value {
  json!({})
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReportModel {
  #[serde(default = "default_report_model_min")]
  pub min:      f64,
  #[serde(default = "default_report_model_max")]
  pub max:      f64,
  #[serde(default)]
  pub step:     Option<f64>,
  #[serde(default)]
  pub unit:     Option<String>,
  #[serde(default = "default_report_model_channels")]
  pub channels: usize,
  #[serde(default = "default_metadata")]
  pub metadata: Value,
}

fn default_report_model_min() -> f64 {
  0.0
}

fn default_report_model_max() -> f64 {
  1.0
}

fn default_report_model_channels() -> usize {
  1
}