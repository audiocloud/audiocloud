/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

extern crate core;

use derive_more::IsVariant;
use serde::{Deserialize, Serialize};

use audiocloud_api::domain::DomainError;
use audiocloud_api::{SecureKey, SerializableResult};

pub mod config;
pub mod db;
pub mod events;
pub mod fixed_instances;
pub mod media;
pub mod models;
pub mod nats;
pub mod o11y;
pub mod remote_value;
pub mod rest_api;
pub mod sockets;
pub mod tasks;
pub mod tracker;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, IsVariant)]
pub enum ResponseMedia {
    MsgPack,
    Json,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, IsVariant)]
pub enum DomainSecurity {
    Cloud,
    SecureKey(SecureKey),
}

pub type DomainResult<T = ()> = Result<T, DomainError>;

pub fn to_serializable<T>(result: DomainResult<T>) -> SerializableResult<T, DomainError> {
    match result {
        Ok(t) => SerializableResult::Ok(t),
        Err(err) => SerializableResult::Error(err),
    }
}
