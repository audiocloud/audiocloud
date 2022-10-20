//! Types used to communicate with the instance_driver

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::OpenApi;

use crate::common::instance::{DesiredInstancePlayState, InstancePlayState};
use crate::common::media::{PlayId, RenderId};
use crate::common::task::InstanceReports;
use crate::newtypes::FixedInstanceId;
use crate::{merge_schemas, Request, SerializableResult};

/// A command that can be sent to the instance driver
#[derive(PartialEq, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDriverCommand {
    CheckConnection,
    Stop,
    Play { play_id: PlayId },
    Render { length: f64, render_id: RenderId },
    Rewind { to: f64 },
    SetParameters(serde_json::Value),
    SetPowerChannel { channel: usize, power: bool },
}

impl Request for InstanceDriverCommand {
    type Response = SerializableResult<(), InstanceDriverError>;
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SetInstanceParameters {
    pub parameters: serde_json::Value,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug, Error, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDriverError {
    #[error("Instance {0} does not exist")]
    InstanceNotFound(FixedInstanceId),

    #[error("Parameter {error} does not exist")]
    ParameterDoesNotExist { error: String },

    #[error("Parameters are malformed: {error}")]
    ParametersMalformed { error: String },

    #[error("Reports are malformed: {error}")]
    ReportsMalformed { error: String },

    #[error("Media is not present, can't play, record or rewind")]
    MediaNotPresent,

    #[error("Instance is not a power controller")]
    NotPowerController,

    #[error("Driver can't guarantee that playback won't be interrupted")]
    NotInterruptable,

    #[error("Remote call failed: {error}")]
    RPC { error: String },
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDriverEvent {
    /// Sent when the instance_driver has started
    Started,

    /// If an I/O error happened during communication with device
    IOError { error: String },

    /// Driver lost connection to the hardware
    ConnectionLost,

    /// Driver connected to the hardware
    Connected,

    /// Received metering updates from the hardware
    Reports { reports: InstanceReports },

    /// Playing; media current position reported
    PlayState {
        desired: DesiredInstancePlayState,
        current: InstancePlayState,
        media: Option<f64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct InstanceWithStatus {
    pub id: FixedInstanceId,
    pub play_state: Option<InstancePlayState>,
}

pub type InstanceWithStatusList = Vec<InstanceWithStatus>;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceParametersUpdated {
    Updated { id: FixedInstanceId },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceCommandAccepted {
    Updated { id: FixedInstanceId },
}

mod instance {
    /// Set desired play state
    ///
    /// If the instance has media capabilities, set an instance's desired play state.
    #[utoipa::path(
     put,
     request_body = InstanceDriverCommand,
     path = "/v1/instances/{manufacturer}/{name}/{instance}/play-state",
     responses(
      (status = 200, description = "Success", body = InstanceCommandAccepted),
      (status = 404, description = "Not found", body = InstanceDriverError),
     ),
    params(
     ("manufacturer" = String, Path, description = "Model manufacturer"),
     ("name" = String, Path, description = "Model product name"),
     ("instance" = String, Path, description = "Unique instance identifier"),
    ))]
    fn accept_command() {}

    /// Set parameters
    ///
    /// Update instance parameter values.
    #[utoipa::path(
     patch,
     request_body = SetInstanceParameters,
     path = "/v1/instances/{manufacturer}/{name}/{instance}/parameters",
     responses(
      (status = 200, description = "Success", body = InstanceParametersUpdated),
      (status = 404, description = "Not found", body = InstanceDriverError),
     ), params(
      ("manufacturer" = String, Path, description = "Model manufacturer"),
      ("name" = String, Path, description = "Model product name"),
      ("instance" = String, Path, description = "Unique instance identifier"),
    ))]
    fn set_parameters() {}
}

mod driver {
    /// List running instances
    ///
    /// List instances running on this driver enddpoint.
    #[utoipa::path(
     get,
     path = "/v1/instances",
     responses(
      (status = 200, description = "Success", body = InstanceWithStatusList),
     ))]
    fn list_instances() {}
}

#[derive(OpenApi)]
#[openapi(paths(
    instance::accept_command,
    instance::set_parameters,
    driver::list_instances
))]
pub struct InstanceDriverApi;

pub fn schemas() -> RootSchema {
    merge_schemas(
        [
            schema_for!(InstanceDriverError),
            schema_for!(InstanceDriverCommand),
            schema_for!(InstanceCommandAccepted),
            schema_for!(InstanceParametersUpdated),
            schema_for!(SetInstanceParameters),
            schema_for!(InstanceWithStatusList),
        ]
        .into_iter(),
    )
}
