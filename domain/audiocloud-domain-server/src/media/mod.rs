#![allow(unused_variables)]

use std::path::PathBuf;
use std::str::FromStr;

use actix::{Actor, Addr};
use clap::Args;
use derive_more::{Display, From, FromStr};
use once_cell::sync::OnceCell;
use tracing::*;
use uuid::Uuid;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Hash, From, FromStr)]
#[repr(transparent)]
pub struct UploadJobId(Uuid);

impl UploadJobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<String> for UploadJobId {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Hash, From, FromStr)]
#[repr(transparent)]
pub struct DownloadJobId(Uuid);

impl From<String> for DownloadJobId {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

impl DownloadJobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

struct MediaJobs {
    download: Option<DownloadJobId>,
    upload: Option<UploadJobId>,
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

    let addr = MEDIA_SUPERVISOR
        .get_or_init(move || service.start())
        .clone();

    Ok(addr)
}

pub fn get_media_supervisor() -> &'static Addr<MediaSupervisor> {
    MEDIA_SUPERVISOR
        .get()
        .expect("Media supervisor not initialized")
}
