/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::path::PathBuf;

use anyhow::anyhow;
use tracing::*;

use audiocloud_api::cloud::domains::DomainConfig;

#[instrument(skip_all, err)]
pub async fn get_config(path: PathBuf) -> anyhow::Result<DomainConfig> {
    Ok(serde_yaml::from_slice(std::fs::read(&path).map_err(|err| {
                                                      anyhow!("Could not open config file: {path:?}: {err}")
                                                  })?
                                                  .as_slice())?)
}
