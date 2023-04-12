use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaDownloadSpec {
  pub from_url: String,
  pub size:     u64,
  pub sha256:   String,
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
  pub id:     String,
  pub sha256: String,
}
