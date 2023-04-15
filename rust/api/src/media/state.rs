use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::media::spec::MediaSpec;
use crate::{BucketKey, IntoBucketKey, Timestamp};

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

pub fn media_download_state_key<T: ToString>(media_id: &T) -> BucketKey<MediaDownloadState> {
  media_id.to_bucket_key()
}
