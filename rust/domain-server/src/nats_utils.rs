use std::str::FromStr;

use anyhow::anyhow;
use async_nats::jetstream::kv;
use serde::de::DeserializeOwned;
use serde_json::from_slice;

pub(crate) trait ExtractValue {
  fn extract<K: FromStr, V: DeserializeOwned>(&self) -> anyhow::Result<(K, V)>;
}

impl ExtractValue for kv::Entry {
  fn extract<K: FromStr, V: DeserializeOwned>(&self) -> anyhow::Result<(K, V)> {
    let key = K::from_str(&self.key).map_err(|e| anyhow!("Failed to parse key"))?;
    let value = from_slice(self.value.as_slice())?;

    Ok((key, value))
  }
}

pub struct EncodedParameterId {
  pub instance_id: String,
  pub parameter:   String,
  pub channel:     usize,
}

impl FromStr for EncodedParameterId {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut s = s.split('.');
    let instance_id = s.next().ok_or(anyhow!("Missing instance id"))?.to_string();
    let parameter = s.next().ok_or(anyhow!("Missing parameter"))?.to_string();
    let channel = s.next().ok_or(anyhow!("Missing channel"))?.parse::<usize>()?;

    Ok(Self { instance_id,
              parameter,
              channel })
  }
}
