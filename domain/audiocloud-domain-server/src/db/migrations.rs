use anyhow::anyhow;
use rbatis::Rbatis;
use tracing::*;

use crate::db::{DataOpts, Db};

#[instrument(skip_all, err)]
pub async fn execute(batis: &mut Rbatis) -> anyhow::Result<usize> {
    let user_version: usize = batis.fetch_decode("PRAGMA user_version", vec![]).await?;
    let migrations = vec![include_str!("migrations/2022-10-25T1045Z_init.sql")];

    let mut count = 0;

    info!(user_version, migrations_total = migrations.len(), "Init");

    for (id, migration) in migrations.into_iter().skip(user_version as usize).enumerate() {
        let id = id + user_version + 1;
        let mut txn = batis.acquire_begin().await?;

        txn.exec(migration, vec![]).await?;
        let pragma = format!("pragma user_version = {id};");
        txn.exec(&pragma, vec![]).await?;
        txn.commit().await?;

        count += 1;
    }

    Ok(count)
}
