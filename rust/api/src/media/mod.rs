use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Timestamp;

pub mod buckets {
  pub const DOWNLOAD_SPEC: &'static str = "audiocloud_media_download_spec";
  pub const DOWNLOAD_STATE: &'static str = "audiocloud_media_download_state";
  pub const UPLOAD_SPEC: &'static str = "audiocloud_media_upload_spec";
  pub const UPLOAD_STATE: &'static str = "audiocloud_media_upload_state";
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaUploadState {
  pub updated_at: Timestamp,
  pub uploaded:   bool,
  pub progress:   f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaDownloadState {
  pub updated_at: Timestamp,
  pub progress:   f64,
  pub done:       Option<MediaSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaDownloadSpec {
  pub from_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaUploadSpec {
  pub to_url: String,
}

// once downloaded, media is in the database
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaSpec {
  pub id: String,
}
