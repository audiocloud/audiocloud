/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

#![allow(unused_variables)]

use std::path::PathBuf;
use std::str::FromStr;

use actix::{Actor, Addr};
use clap::Args;
use derive_more::{Constructor, Deref, Display, FromStr};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tracing::*;

pub use messages::*;
use supervisor::MediaSupervisor;

use crate::db::Db;

pub mod download;
pub mod messages;
mod supervisor;
#[cfg(test)]
mod tests;
pub mod upload;

static MEDIA_SUPERVISOR: OnceCell<Addr<MediaSupervisor>> = OnceCell::new();

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Hash, FromStr, Deref, Constructor)]
#[repr(transparent)]
pub struct UploadJobId(String);

impl From<String> for UploadJobId {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Hash, FromStr, Deref, Constructor)]
#[repr(transparent)]
pub struct DownloadJobId(String);

impl From<String> for DownloadJobId {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

struct MediaJobs {
    download: Option<DownloadJobId>,
    upload:   Option<UploadJobId>,
}

#[derive(Args)]
pub struct MediaOpts {
    #[clap(long, env, default_value = "media")]
    pub media_root: PathBuf,

    #[clap(long, env, default_value = "8")]
    pub max_uploads_batch: usize,

    #[clap(long, env, default_value = "8")]
    pub max_downloads_batch: usize,
}

#[instrument(skip_all, err)]
pub async fn init(cfg: MediaOpts, db: Db) -> anyhow::Result<Addr<MediaSupervisor>> {
    let service = MediaSupervisor::new(cfg, db)?;

    let addr = MEDIA_SUPERVISOR.get_or_init(move || service.start()).clone();

    Ok(addr)
}

pub fn get_media_supervisor() -> &'static Addr<MediaSupervisor> {
    MEDIA_SUPERVISOR.get().expect("Media supervisor not initialized")
}
