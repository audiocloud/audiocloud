//! Cloud APIs for apps

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AppId;

/// Returned information about an app
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetAppResponse {
    /// App Id
    pub id:          AppId,
    /// If true, the app is enabled - it may make responses to the cloud API
    pub enabled:     bool,
    /// App owner/administrator email
    pub admin_email: String,
    /// The URL used to resolve object IDs to media information
    pub media_url:   String,
}

/// Request to update app
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct UpdateApp {
    /// If not null, enable or disable the app
    pub enabled:     Option<bool>,
    /// If not null, overwrite the administrator's email
    pub admin_email: Option<String>,
    /// If not null, overwrite the URL used for resolving object IDs to media information
    pub media_url:   Option<String>,
}

/// The App has been updated
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AppUpdated {
    /// Updated normally
    Updated(AppId),
}
