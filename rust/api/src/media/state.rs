use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::media::spec::MediaSpec;
use crate::Timestamp;

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
