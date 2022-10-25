use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use clap::Args;
use futures::lock::Mutex;
use rbatis::Rbatis;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use tracing::*;

mod media;
mod migrations;
mod models;
mod sys_props;
mod tasks;
#[cfg(test)]
mod tests;
mod utils;

#[derive(Clone)]
pub struct Db {
    db: Rbatis,
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
    #[clap(long, env, default_value = "sqlite://domain.sqlite")]
    pub database_url: String,

    /// Use write-ahead logging as journal mode to speed up writes
    #[clap(long, env)]
    pub database_use_wal: bool,
}

impl DataOpts {
    pub fn memory() -> Self {
        Self { database_url:     "sqlite://:memory:".to_string(),
               database_use_wal: false, }
    }
}

#[instrument(skip_all, err)]
pub async fn init(cfg: DataOpts) -> anyhow::Result<Db> {
    let database_url = &cfg.database_url;
    debug!(?database_url, "Initializing database");

    let mut db = Rbatis::new();
    db.init(SqliteDriver {}, database_url)?;

    if cfg.database_use_wal {
        db.exec("PRAGMA journal_mode = WAL", vec![]).await?;
    } else {
        debug!("Write ahead logging disabled");
    }

    debug!("Running migrations");

    let count = migrations::execute(&mut db).await?;

    debug!(count, "Migrations executed");

    Ok(Db { db })
}
