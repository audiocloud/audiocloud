/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn to_json_str<T: Serialize>(t: &T) -> serde_json::Result<String> {
    serde_json::to_string_pretty(t)
}

pub fn to_json_vec<T: Serialize>(t: &T) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec_pretty(t)
}

pub fn from_json_str<T: DeserializeOwned>(s: &str) -> serde_json::Result<T> {
    serde_json::from_str(s)
}

pub fn from_json_slice<T: DeserializeOwned>(v: &[u8]) -> serde_json::Result<T> {
    serde_json::from_slice(v)
}

pub fn to_msgpack<T: Serialize>(t: &T) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    rmp_serde::encode::to_vec_named(t)
}

pub fn from_msgpack_slice<T: DeserializeOwned>(v: &[u8]) -> Result<T, rmp_serde::decode::Error> {
    rmp_serde::decode::from_slice(v)
}

pub trait Codec: Clone {
    type SerializeError: Error + Send + Sync + 'static;
    type DeserializeError: Error + Send + Sync + 'static;

    fn serialize<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, Self::SerializeError>;
    fn deserialize<T: DeserializeOwned>(&self, v: &[u8]) -> Result<T, Self::DeserializeError>;
}

#[derive(Clone, Copy)]
pub struct Json;

impl Codec for Json {
    type SerializeError = serde_json::Error;
    type DeserializeError = serde_json::Error;

    fn serialize<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, Self::SerializeError> {
        to_json_vec(t)
    }

    fn deserialize<T: DeserializeOwned>(&self, v: &[u8]) -> Result<T, Self::DeserializeError> {
        from_json_slice(v)
    }
}

#[derive(Clone, Copy)]
pub struct MsgPack;

impl Codec for MsgPack {
    type SerializeError = rmp_serde::encode::Error;
    type DeserializeError = rmp_serde::decode::Error;

    fn serialize<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, Self::SerializeError> {
        to_msgpack(t)
    }

    fn deserialize<T: DeserializeOwned>(&self, v: &[u8]) -> Result<T, Self::DeserializeError> {
        from_msgpack_slice(v)
    }
}

pub trait Transferable {
    type Codec: Codec;
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::api::codec::{Codec, Json};

    #[test]
    pub fn test_err_json() {
        let value = Err::<String, _>(format!("Error message"));
        let msg = Json.serialize(&value).expect("serialize");
        let jsvalue: serde_json::Value = serde_json::from_slice(&msg).expect("deserialize");
        assert_eq!(jsvalue,
                   json!({
                       "Err": "Error message"
                   }));
    }

    #[test]
    pub fn test_ok_json() {
        let value = Ok::<_, String>(18);
        let msg = Json.serialize(&value).expect("serialize");
        let jsvalue: serde_json::Value = serde_json::from_slice(&msg).expect("deserialize");
        assert_eq!(jsvalue,
                   json!({
                       "Ok": 18
                   }));
    }
}
