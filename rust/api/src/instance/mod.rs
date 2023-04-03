use std::str::FromStr;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::driver::InstanceDriverConfig;
use crate::graph::PlayId;
use crate::Timestamp;

pub mod model;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IdAndChannel {
  pub id:      String,
  pub channel: usize,
}

impl<T: AsRef<str>> From<(T, usize)> for IdAndChannel {
  fn from((name, channel): (T, usize)) -> Self {
    Self { id: name.as_ref().to_owned(),
           channel }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InstanceSummary {
  pub id:          String,
  pub model_id:    String,
  pub driver_id:   DriverId,
  pub power_state: InstancePowerStateSummary,
  pub play_state:  InstancePlayStateSummary,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum DriverId {
  Local,
  Remote(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInstanceRequest {
  pub id:            String,
  pub model_id:      String,
  pub driver_id:     DriverId,
  pub power_spec:    Option<InstancePowerSpec>,
  pub play_spec:     Option<InstancePlaySpec>,
  pub driver_config: InstanceDriverConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InstanceSpec {
  pub id:            String,
  pub model_id:      String,
  pub driver_id:     DriverId,
  pub power_spec:    Option<InstancePowerSpec>,
  pub play_spec:     Option<InstancePlaySpec>,
  pub driver_config: InstanceDriverConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePowerSpec {
  // instance id
  pub power_controller: String,
  pub channel:          u32,
  pub warm_up_ms:       u64,
  pub cool_down_ms:     u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePlaySpec {
  pub duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "update")]
pub enum UpdateInstanceRequest {
  SetPowerState(InstancePowerState),
  SetPlayState(InstancePlayState),
  PushReports(PushInstanceReports),
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PushInstanceReports {
  pub report_id: String,
  pub start_at:  Timestamp,
  pub values:    Vec<(f64, f64)>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePowerStateSummary {
  pub changed_at: Timestamp,
  pub state:      InstancePowerState,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePlayStateSummary {
  pub changed_at: Timestamp,
  pub state:      InstancePlayState,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InstancePowerState {
  Off,
  CoolingDown,
  On,
  WarmingUp,
}

impl Default for InstancePowerState {
  fn default() -> Self {
    Self::Off
  }
}

impl PartialEq<DesiredInstancePowerState> for InstancePowerState {
  fn eq(&self, other: &DesiredInstancePowerState) -> bool {
    match (self, other) {
      | (InstancePowerState::Off | InstancePowerState::CoolingDown, DesiredInstancePowerState::Off) => true,
      | (InstancePowerState::On | InstancePowerState::WarmingUp, DesiredInstancePowerState::On) => true,
      | _ => false,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum DesiredInstancePowerState {
  Off,
  On,
}

impl Default for DesiredInstancePowerState {
  fn default() -> Self {
    Self::Off
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum InstancePlayState {
  Rewinding,
  Stopped,
  Busy,
  Playing { play_id: PlayId, duration: f64 },
}

impl Default for InstancePlayState {
  fn default() -> Self {
    Self::Stopped
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum DesiredInstancePlayState {
  Stop,
  Play { play_id: PlayId, duration: f64 },
}

impl Default for DesiredInstancePlayState {
  fn default() -> Self {
    Self::Stop
  }
}

impl PartialEq<DesiredInstancePlayState> for InstancePlayState {
  fn eq(&self, other: &DesiredInstancePlayState) -> bool {
    match (self, other) {
      | (InstancePlayState::Stopped, DesiredInstancePlayState::Stop) => true,
      | (InstancePlayState::Playing { play_id: play_id_a,
                                      duration: duration_a, },
         DesiredInstancePlayState::Play { play_id: play_id_b,
                                          duration: duration_b, }) => play_id_a == play_id_b && duration_a == duration_b,
      | _ => false,
    }
  }
}
