use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use schemars::gen::SchemaGenerator;
use schemars::schema::Schema;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::BucketKey;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MediaId {
  pub app_id:   String,
  pub media_id: String,
}

const MEDIA_ID_POSTFIX_LENGTH: usize = 2;

impl MediaId {
  pub fn new(app_id: impl ToString) -> Self {
    Self { app_id:   app_id.to_string(),
           media_id: cuid2::create_id(), }
  }

  pub fn to_path(&self, root: PathBuf) -> PathBuf {
    let mut path = root;

    path.push(&self.app_id);

    let postfix_chars = self.media_id[self.media_id.len() - MEDIA_ID_POSTFIX_LENGTH..].to_string();
    let other_chars = self.media_id[..self.media_id.len() - MEDIA_ID_POSTFIX_LENGTH].to_string();

    path.push(postfix_chars);

    if other_chars.len() > 0 {
      path.push(other_chars);
    }

    path
  }

  pub fn to_folder_path(&self, root: PathBuf) -> PathBuf {
    let mut path = root;

    path.push(&self.app_id);

    let postfix_chars = self.media_id[self.media_id.len() - MEDIA_ID_POSTFIX_LENGTH..].to_string();

    path.push(postfix_chars);

    path
  }
}

impl<'a, Content> Into<BucketKey<MediaId, Content>> for &'a MediaId {
  fn into(self) -> BucketKey<MediaId, Content> {
    BucketKey::from(self.to_string())
  }
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
    write!(f, "{}.{}", self.app_id, self.media_id)
  }
}

impl Debug for MediaId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}", self.app_id, self.media_id)
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

#[cfg(test)]
mod test {
  use std::collections::HashSet;
  use std::path::PathBuf;
  use std::time::Instant;

  use crate::media::spec::MediaId;

  #[test]
  fn test_media_id() {
    let media_id = super::MediaId::new("test");

    assert_eq!(media_id.app_id, "test");
    assert_eq!(media_id.media_id.len(), 24);
  }

  #[test]
  fn test_gen_100() {
    let mut set = HashSet::new();

    let start = Instant::now();
    for i in 0..100 {
      set.insert(MediaId::new("test"));
    }

    let elapsed = start.elapsed();

    assert_eq!(set.len(), 100);

    for media_id in set {
      println!("{media_id}: {}", media_id.to_path(PathBuf::from(".media")).to_str().unwrap());
    }

    println!("elapsed: {:?}", elapsed);
  }

  #[test]
  fn test_folder_path() {
    let media_id = super::MediaId { app_id:   "test".to_string(),
                                    media_id: "1234567890".to_string(), };

    let path = media_id.to_folder_path("/tmp".into());

    #[cfg(unix)]
    assert_eq!(path.to_str().unwrap(), "/tmp/test/90");

    let path = media_id.to_path("/tmp".into());

    #[cfg(unix)]
    assert_eq!(path.to_str().unwrap(), "/tmp/test/90/12345678");
  }
}
