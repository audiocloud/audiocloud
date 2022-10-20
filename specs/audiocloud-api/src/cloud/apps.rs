//! Cloud APIs for apps

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AppId;

/// Returned information about an app
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetAppResponse {
    /// App Id
    pub id: AppId,
    /// If true, the app is enabled - it may make responses to the cloud API
    pub enabled: bool,
    /// App owner/administrator email
    pub admin_email: String,
    /// The URL used to resolve object IDs to media information
    pub media_url: String,
}

/// Request to update app
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct UpdateApp {
    /// If not null, enable or disable the app
    pub enabled: Option<bool>,
    /// If not null, overwrite the administrator's email
    pub admin_email: Option<String>,
    /// If not null, overwrite the URL used for resolving object IDs to media information
    pub media_url: Option<String>,
}

/// The App has been updated
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AppUpdated {
    /// Updated normally
    Updated(AppId),
}

/// Get app details
///
/// Get details of a registered app. Only administrators and app owners may do this.
#[utoipa::path(
  get,
  path = "/v1/apps/{app_id}", 
  responses(
    (status = 200, description = "Success", body = GetAppResponse),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App to get")
  ))]
pub(crate) fn get_app() {}

/// Update app details
///
/// Update details of a registered app. Only administrators and app owners may do this. If the media
/// URL is changed, it will only be used for newly submitted upload and download jobs.
#[utoipa::path(
  patch,
  path = "/v1/apps/{app_id}",
  request_body = UpdateApp,
  responses(
    (status = 200, description = "Success", body = AppUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App to update")
  )
)]
pub(crate) fn update_app() {}
