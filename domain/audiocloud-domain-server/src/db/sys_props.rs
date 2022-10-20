use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::db::Db;

impl Db {
    pub async fn get_sys_prop<T: DeserializeOwned>(&self, prop_id: &str) -> anyhow::Result<Option<T>> {
        Ok(match sqlx::query!(r#"SELECT value FROM sys_props WHERE id=?"#, prop_id).fetch_optional(&self.pool)
                                                                                   .await?
           {
               None => None,
               Some(value) => Some(serde_json::from_str(&value.value)?),
           })
    }

    pub async fn set_sys_prop<T: Serialize>(&self, prop_id: &str, value: &T) -> anyhow::Result<()> {
        let value = serde_json::to_string(value)?;

        sqlx::query!(r#"INSERT OR REPLACE INTO sys_props (id, value) VALUES (?, ?)"#, prop_id, value).execute(&self.pool)
                                                                                                     .await?;

        Ok(())
    }
}
