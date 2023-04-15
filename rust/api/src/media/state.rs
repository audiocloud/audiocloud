use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::media::spec::MediaSpec;
use crate::{BucketKey, Timestamp};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaUploadState {
  pub updated_at: Timestamp,
  pub uploaded:   bool,
  pub error:      Option<String>,
  pub progress:   f64,
}

impl Default for MediaUploadState {
  fn default() -> Self {
    Self { updated_at: Utc::now(),
           uploaded:   false,
           error:      None,
           progress:   0.0, }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaDownloadState {
  pub updated_at: Timestamp,
  pub progress:   f64,
  pub done:       Option<MediaSpec>,
  pub error:      Option<String>,
}

impl Default for MediaDownloadState {
  fn default() -> Self {
    Self { updated_at: Utc::now(),
           progress:   0.0,
           done:       None,
           error:      None, }
  }
}

pub fn media_download_state(media_id: impl ToString) -> BucketKey<MediaDownloadState> {
  BucketKey::new(media_id)
}
