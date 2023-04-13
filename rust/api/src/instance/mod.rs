use std::fmt::{Display, Formatter};

use schemars::schema::RootSchema;
use schemars::JsonSchema;
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::task::spec::PlayId;
use crate::Timestamp;

pub mod control;
pub mod driver;
pub mod model;
pub mod request;
pub mod spec;
pub mod state;

pub mod buckets {
  use crate::instance::control::{InstancePlayControl, InstancePowerControl};
  use crate::instance::spec::InstanceSpec;
  use crate::instance::{InstanceConnectionState, InstancePlayState, InstancePowerState};
  use crate::BucketName;

  pub const INSTANCE_POWER_CONTROL: BucketName<InstancePowerControl> = BucketName::new("audiocloud_instance_power_control");
  pub const INSTANCE_PLAY_CONTROL: BucketName<InstancePlayControl> = BucketName::new("audiocloud_instance_play_control");
  pub const INSTANCE_CONNECTION_STATE: BucketName<InstanceConnectionState> = BucketName::new("audiocloud_instance_connection_state");
  pub const INSTANCE_POWER_STATE: BucketName<InstancePowerState> = BucketName::new("audiocloud_instance_power_state");
  pub const INSTANCE_PLAY_STATE: BucketName<InstancePlayState> = BucketName::new("audiocloud_instance_play_state");
  pub const INSTANCE_SPEC: BucketName<InstanceSpec> = BucketName::new("audiocloud_instance_spec");
}

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
  pub driver_id:   String,
  pub power_state: InstancePowerStateSummary,
  pub play_state:  InstancePlayStateSummary,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InstancePowerState {
  Off,
  CoolingDown,
  On,
  WarmingUp,
}

impl InstancePowerState {
  pub fn is_on(&self) -> bool {
    matches!(self, Self::On)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InstanceConnectionState {
  Connected,
  Disconnected,
}

impl InstanceConnectionState {
  pub fn is_connected(&self) -> bool {
    matches!(self, Self::Connected)
  }
}

impl Display for InstanceConnectionState {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      | InstanceConnectionState::Connected => write!(f, "connected"),
      | InstanceConnectionState::Disconnected => write!(f, "disconnected"),
    }
  }
}

impl InstancePowerState {
  pub fn is_in_progress(&self) -> bool {
    matches!(self, Self::CoolingDown | Self::WarmingUp)
  }
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
  Idle,
  Busy,
  Playing { play_id: PlayId, duration: f64 },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum InstancePlayStateTransition {
  SetRewinding,
  SetIdle,
  SetBusy,
  SetPlaying,
}

impl Default for InstancePlayState {
  fn default() -> Self {
    Self::Idle
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
      | (InstancePlayState::Idle, DesiredInstancePlayState::Stop) => true,
      | (InstancePlayState::Playing { play_id: play_id_a,
                                      duration: duration_a, },
         DesiredInstancePlayState::Play { play_id: play_id_b,
                                          duration: duration_b, }) => play_id_a == play_id_b && duration_a == duration_b,
      | _ => false,
    }
  }
}

pub fn schema() -> RootSchema {
  merge_schemas([driver::schema(),
                 spec::schema(),
                 model::schema(),
                 request::schema(),
                 state::schema(),
                 control::schema()].into_iter())
}
