use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use clap::Args;
use serde::{Deserialize, Serialize};
use surrealdb::Datastore;
use tracing::*;

mod media;
mod models;
mod sys_props;
mod tasks;
#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Db {
    db: Arc<Datastore>,
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
    #[clap(long, env, default_value = "file://domain.db")]
    pub database_url: String,
}

impl DataOpts {
    pub fn memory() -> Self {
        Self { database_url: "memory".to_string() }
    }
}

#[instrument(skip_all, err)]
pub async fn init(cfg: DataOpts) -> anyhow::Result<Db> {
    let database_url = &cfg.database_url;
    debug!(?database_url, "Initializing database");

    let pool = Datastore::new(database_url).await?;

    // debug!("Running migrations");

    // sqlx::migrate!("src/db/migrations").run(&pool).await?;

    // debug!("Migrations done");

    Ok(Db { db: Arc::new(pool) })
}
