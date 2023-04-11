use std::collections::{HashMap, HashSet};

use schemars::_serde_json::Value;
use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
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
  #[serde(default)]
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
  #[serde(default)]
  pub range:    ValueRange,
  #[serde(default)]
  pub step:     Option<f64>,
  #[serde(default)]
  pub unit:     Option<String>,
  #[serde(default = "default_parameter_model_channels")]
  pub channels: usize,
  #[serde(default)]
  pub metadata: HashMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ValueRange {
  Toggle,
  Bounded {
    #[serde(default)]
    min:  f64,
    #[serde(default = "default_bounded_max")]
    max:  f64,
    #[serde(default)]
    step: Option<f64>,
  },
  Values {
    values: Vec<f64>,
  },
}

impl Default for ValueRange {
  fn default() -> Self {
    Self::Bounded { min:  0.0,
                    max:  default_bounded_max(),
                    step: None, }
  }
}

fn default_bounded_max() -> f64 {
  1.0
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReportModel {
  #[serde(default)]
  pub range:    ValueRange,
  #[serde(default)]
  pub unit:     Option<String>,
  #[serde(default = "default_report_model_channels")]
  pub channels: usize,
  #[serde(default)]
  pub metadata: HashMap<String, Value>,
}

fn default_report_model_channels() -> usize {
  1
}

pub fn schema() -> RootSchema {
  schema_for!(InstanceModel)
}
