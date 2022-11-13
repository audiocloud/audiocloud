/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::path::PathBuf;

use anyhow::anyhow;
use clap::{Args, ValueEnum};
use once_cell::sync::Lazy;
use reqwest::Url;
use tokio::sync::broadcast::Sender;
use tokio::{spawn, time};
use tracing::*;

use audiocloud_api::cloud::domains::DomainConfig;
pub use messages::*;

mod cloud;
mod file;
mod messages;

#[derive(Args, Debug, Clone)]
pub struct ConfigOpts {
    /// Source of the config
    #[clap(short, long, env, default_value = "file", value_enum)]
    pub config_source: ConfigSource,

    /// Path to the config file
    #[clap(long, env, default_value = "config.yaml", required_if_eq("config_source", "file"))]
    pub config_file: PathBuf,

    /// The base cloud URL to use for config retrieval
    #[clap(long, env, default_value = "https://api.audiocloud.io", required_if_eq("config_source", "cloud"))]
    pub cloud_url: Url,

    #[clap(long, env, required_if_eq("config_source", "cloud"))]
    pub api_key: Option<String>,

    #[clap(long, env, default_value = "3600")]
    pub config_refresh_seconds: usize,
}

impl ConfigOpts {
    pub fn describe(&self) -> String {
        match self.config_source {
            ConfigSource::File => format!("file:{}", self.config_file.display()),
            ConfigSource::Cloud => format!("cloud:{}", self.cloud_url),
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ConfigSource {
    /// Load the config from an cloud or orchestrator
    Cloud,
    /// Load the config from a local file
    File,
}

async fn load_config(cfg: ConfigOpts) -> anyhow::Result<DomainConfig> {
    match cfg.config_source {
        ConfigSource::Cloud => {
            Ok(cloud::get_config(cfg.cloud_url,
                                 cfg.api_key
                                    .ok_or_else(|| anyhow!("API key must be configured for cloud configuration"))?).await?)
        }
        ConfigSource::File => Ok(file::get_config(cfg.config_file).await?),
    }
}

static BROADCAST_CONFIG: Lazy<Sender<NotifyDomainConfiguration>> = Lazy::new(|| {
    let (tx, _) = tokio::sync::broadcast::channel(256);
    tx
});

pub fn get_broadcast_config() -> &'static Sender<NotifyDomainConfiguration> {
    &BROADCAST_CONFIG
}

#[instrument(skip_all, err)]
pub async fn init(cfg: ConfigOpts) -> anyhow::Result<DomainConfig> {
    let rv = load_config(cfg.clone()).await?;

    spawn({
        let mut rv = rv.clone();
        async move {
            loop {
                time::sleep(time::Duration::from_secs(cfg.config_refresh_seconds as u64)).await;
                rv = async {
                         debug!(source = cfg.describe(), "Reloading configuration");
                         match load_config(cfg.clone()).await {
                             Err(error) => {
                                 error!(%error, "Failed to reload config");
                                 rv
                             }
                             Ok(config) => {
                                 if &rv != &config {
                                     let _ = BROADCAST_CONFIG.send(NotifyDomainConfiguration { config: config.clone() });
                                     config
                                 } else {
                                     rv
                                 }
                             }
                         }
                     }.instrument(info_span!("config_reload"))
                      .await;
            }
        }
    });

    Ok(rv)
}
