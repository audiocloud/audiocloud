use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::instance::driver::config::InstanceDriverConfig;
use crate::instance::model::InstanceModel;
use crate::instance::{DesiredInstancePlayState, DesiredInstancePowerState, InstancePlayStateTransition};
use crate::BucketKey;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InstanceSpec {
  pub model:      InstanceModel,
  pub host:       String,
  pub power:      Option<InstancePowerSpec>,
  pub media:      Option<InstanceMediaSpec>,
  pub attachment: Option<InstanceAttachment>,
  pub driver:     InstanceDriverConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstanceAttachment {
  pub device:  String,
  pub inputs:  Vec<usize>,
  pub outputs: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePowerSpec {
  pub power_controller:   String,
  pub power_on:           SetParameterCommand,
  pub power_off:          SetParameterCommand,
  pub warm_up_ms:         u64,
  pub cool_down_ms:       u64,
  pub idle_ms:            u64,
  /// if true, the instance will not be reachable by a driver until it is powered on.
  pub driver_needs_power: bool,
}

impl InstancePowerSpec {
  pub fn get_command(&self, desired: DesiredInstancePowerState) -> &SetParameterCommand {
    match desired {
      | DesiredInstancePowerState::On => &self.power_on,
      | DesiredInstancePowerState::Off => &self.power_off,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstanceMediaSpec {
  pub duration_ms:         u64,
  #[serde(default = "default_position_report")]
  pub position_report:     String,
  pub play_state_triggers: Vec<PlayStateReportTrigger>,
  pub play:                SetParameterCommand,
  pub stop:                SetParameterCommand,
  pub rewind:              SetParameterCommand,
}

impl InstanceMediaSpec {
  pub fn get_command(&self, desired: DesiredInstancePlayState, remaining: f64) -> &SetParameterCommand {
    match desired {
      | DesiredInstancePlayState::Stop => &self.stop,
      | DesiredInstancePlayState::Play { duration, .. } =>
        if duration > remaining {
          &self.rewind
        } else {
          &self.play
        },
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetParameterCommand {
  pub parameter: String,
  #[serde(default)]
  pub channel:   usize,
  pub value:     f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayStateReportTrigger {
  pub report:       String,
  #[serde(default)]
  pub equals:       Option<f64>,
  #[serde(default)]
  pub greater_than: Option<f64>,
  #[serde(default)]
  pub less_than:    Option<f64>,
  pub then:         InstancePlayStateTransition,
}

impl PlayStateReportTrigger {
  pub fn is_triggered(&self, report_id: &str, value: f64) -> bool {
    if report_id != self.report {
      return false;
    }
    if let Some(equals) = self.equals {
      if value == equals {
        return true;
      }
    }
    if let Some(greater_than) = self.greater_than {
      if value > greater_than {
        return true;
      }
    }
    if let Some(less_than) = self.less_than {
      if value < less_than {
        return true;
      }
    }
    false
  }
}

fn default_position_report() -> String {
  "position".to_owned()
}

pub fn instance_spec_key<T: ToString>(instance_id: &T) -> BucketKey<String, InstanceSpec> {
  instance_id.to_string().into()
}

pub fn schema() -> RootSchema {
  schema_for!(InstanceSpec)
}
