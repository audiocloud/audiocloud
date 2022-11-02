use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::media::{PlayId, RenderId};
use crate::common::time::Timestamped;


#[derive(PartialEq, Serialize, Deserialize, Copy, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstancePlayState {
    PreparingToPlay { play_id: PlayId },
    Playing { play_id: PlayId },
    PreparingToRender { length: f64, render_id: RenderId },
    Rendering { length: f64, render_id: RenderId },
    Rewinding { to: f64 },
    Stopping,
    Stopped { position: Option<f64> },
}

#[derive(PartialEq, Serialize, Deserialize, Copy, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesiredInstancePlayState {
    Playing { play_id: PlayId },
    Rendering { length: f64, render_id: RenderId },
    Stopped { position: Option<f64> },
}

impl InstancePlayState {
    pub fn satisfies(&self, required: &DesiredInstancePlayState) -> bool {
        match (self, required) {
            (Self::Playing { play_id }, DesiredInstancePlayState::Playing { play_id: desired_play_id }) => play_id == desired_play_id,
            (Self::Rendering { render_id, .. },
             DesiredInstancePlayState::Rendering { render_id: desired_render_id,
                                                   .. }) => render_id == desired_render_id,
            (Self::Stopped { position }, DesiredInstancePlayState::Stopped { position: desired_position, }) => {
                desired_position.map(|desired| Some(desired) == position.clone()).unwrap_or(true)
            }
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstancePowerState {
    PoweringUp,
    ShuttingDown,
    PoweredUp,
    ShutDown,
}

impl InstancePowerState {
    pub fn from_bool(power: bool) -> Self {
        match power {
            true => Self::PoweredUp,
            false => Self::ShutDown,
        }
    }

    pub fn satisfies(self, desired: DesiredInstancePowerState) -> bool {
        match (self, desired) {
            (Self::PoweredUp, DesiredInstancePowerState::PoweredUp) => true,
            (Self::ShutDown, DesiredInstancePowerState::ShutDown) => true,
            _ => false,
        }
    }

    pub fn is_powered_on(&self) -> bool {
        matches!(self, Self::PoweredUp)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesiredInstancePowerState {
    PoweredUp,
    ShutDown,
}

impl DesiredInstancePowerState {
    pub fn to_bool(self) -> bool {
        match self {
            DesiredInstancePowerState::PoweredUp => true,
            DesiredInstancePowerState::ShutDown => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReportInstancePowerState {
    pub desired: Timestamped<DesiredInstancePowerState>,
    pub actual:  Timestamped<InstancePowerState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReportInstancePlayState {
    pub desired: Timestamped<DesiredInstancePlayState>,
    pub actual:  Timestamped<InstancePlayState>,
    pub media:   Timestamped<Option<f64>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceEvent {
    State {
        power:     Option<ReportInstancePowerState>,
        play:      Option<ReportInstancePlayState>,
        connected: Timestamped<bool>,
    },
    Error {
        error: String,
    },
}

pub mod power {
    pub mod params {
        use crate::common::ParameterId;

        lazy_static::lazy_static! {
            pub static ref POWER: ParameterId = ParameterId::from("power");
        }
    }

    pub mod reports {
        use crate::common::ReportId;

        lazy_static::lazy_static! {
            pub static ref POWER: ReportId = ReportId::from("power");
            pub static ref CURRENT: ReportId = ReportId::from("current");
            pub static ref POWER_FACTOR: ReportId = ReportId::from("power_factor");
            pub static ref ENERGY: ReportId = ReportId::from("energy");
        }
    }
}
