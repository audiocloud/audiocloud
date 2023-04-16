use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use anyhow::anyhow;
use schemars::gen::SchemaGenerator;
use schemars::schema::Schema;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MediaId {
  pub app_id:   String,
  pub media_id: String,
}

impl JsonSchema for MediaId {
  fn schema_name() -> String {
    "MediaId".to_string()
  }

  fn json_schema(gen: &mut SchemaGenerator) -> Schema {
    String::json_schema(gen)
  }
}

impl<'de> Deserialize<'de> for MediaId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    let s = String::deserialize(deserializer)?;
    MediaId::from_str(&s).map_err(serde::de::Error::custom)
  }
}

impl Display for MediaId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.app_id, self.media_id)
  }
}

impl Debug for MediaId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.app_id, self.media_id)
  }
}

impl Serialize for MediaId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
  {
    serializer.serialize_str(&self.to_string())
  }
}

impl FromStr for MediaId {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split('.');

    let app_id = split.next()
                      .ok_or_else(|| anyhow!("invalid media id: needs to start with app_id"))?;

    let media_id = split.next()
                        .ok_or_else(|| anyhow!("invalid media id: needs to end with media_id after app_id and forward slash"))?;

    Ok(MediaId { app_id:   app_id.to_string(),
                 media_id: media_id.to_string(), })
  }
}

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
  pub id:     MediaId,
  pub sha256: String,
}
