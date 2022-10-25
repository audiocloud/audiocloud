use anyhow::anyhow;
use maplit::btreemap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::db::Db;

impl Db {
    pub async fn get_sys_prop<T: DeserializeOwned>(&self, prop_id: &str) -> anyhow::Result<Option<T>> {
        let value: Option<String> = self.db
                                        .fetch_decode("select value from sys_props where id = ?1", vec![prop_id.into()])
                                        .await?;

        Ok(match value {
            None => None,
            Some(value) => serde_json::from_str(&value)?,
        })
    }

    pub async fn set_sys_prop<T: Serialize>(&self, prop_id: &str, value: &T) -> anyhow::Result<()> {
        let value = serde_json::to_string(value)?;

        self.db
            .exec("insert or replace into sys_props (id, value) values (?1, ?2)",
                  vec![prop_id.into(), value.into()])
            .await?;

        Ok(())
    }
}
