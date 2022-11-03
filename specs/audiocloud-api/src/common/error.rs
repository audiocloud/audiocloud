/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::fmt::Debug;

use anyhow::anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SerializableResult<T, E> {
    Ok(T),
    Error(E),
}

impl<T, E> Into<anyhow::Result<T>> for SerializableResult<T, E> where E: Debug
{
    fn into(self) -> anyhow::Result<T> {
        match self {
            SerializableResult::Ok(t) => Ok(t),
            SerializableResult::Error(err) => Err(anyhow!("Error {err:?}")),
        }
    }
}

impl<T, E> From<anyhow::Result<T>> for SerializableResult<T, E> where E: From<anyhow::Error>
{
    fn from(res: anyhow::Result<T>) -> Self {
        match res {
            Ok(ok) => Self::Ok(ok),
            Err(err) => Self::Error(err.into()),
        }
    }
}
