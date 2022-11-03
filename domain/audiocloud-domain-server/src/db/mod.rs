use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use clap::Args;

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tracing::*;

mod media;
mod models;
mod prisma;
mod sys_props;
mod tasks;
#[cfg(test)]
mod tests;
mod utils;

#[derive(Clone)]
pub struct Db {
    db:        Arc<prisma::PrismaClient>,
    temp_file: Arc<Option<NamedTempFile>>,
}

impl Debug for Db {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Db @ {:p}", self))
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct TaskInfo {}

#[derive(Args)]
pub struct DataOpts {
    /// Sqlite database file where data for media and session cache will be stored. Use :memory: for an in-memory store
    #[clap(long, env, default_value = "file:domain.sqlite")]
    pub database_url: String,

    /// Use write-ahead logging as journal mode to speed up writes
    #[clap(long, env)]
    pub database_use_wal: bool,

    #[clap(skip)]
    pub temp_file: Option<NamedTempFile>,
}

impl DataOpts {
    pub fn temporary() -> Self {
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        Self { database_url:     format!("file:{}", temp_file.path().display()),
               database_use_wal: false,
               temp_file:        Some(temp_file), }
    }
}

#[instrument(skip_all, err)]
pub async fn init(cfg: DataOpts) -> anyhow::Result<Db> {
    let database_url = &cfg.database_url;
    debug!(?database_url, "Initializing database");

    let db = prisma::PrismaClient::_builder().with_url(database_url.to_owned()).build().await?;

    if cfg!(debug_assertions) {
        debug!("pushing database");
        db._db_push().accept_data_loss().force_reset().await?;
    } else {
        debug!("executing migrations");
        db._migrate_deploy().await?;
        debug!("migrations done!");
    }

    Ok(Db { db:        Arc::new(db),
            temp_file: Arc::new(cfg.temp_file), })
}
