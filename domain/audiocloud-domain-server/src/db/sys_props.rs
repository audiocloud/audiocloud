use anyhow::anyhow;
use maplit::btreemap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{json, Strand, Value};
use surrealdb::Session;

use crate::db::Db;

impl Db {
    pub async fn get_sys_prop<T: DeserializeOwned>(&self, prop_id: &str) -> anyhow::Result<Option<T>> {
        let ses = Session::for_db("audiocloud", "domain");
        let ast = r#"SELECT value FROM sys_prop WHERE id = type::thing("sys_prop", $prop_id);"#;
        let vars = btreemap! {"prop_id".to_owned() => Value::from(Strand::from(prop_id.to_owned()))};

        let res = self.db.execute(ast, &ses, Some(vars), false).await?;

        let response = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let result = response.result?;

        #[derive(Deserialize)]
        struct QueryResult<T> {
            value: T,
        }

        let value: Vec<QueryResult<T>> = serde_json::from_value(serde_json::to_value(&result)?)?;
        Ok(value.into_iter().next().map(|qr| qr.value))
    }

    pub async fn set_sys_prop<T: Serialize>(&self, prop_id: &str, value: &T) -> anyhow::Result<()> {
        let value = json(&serde_json::to_string(value)?)?;

        let ses = Session::for_db("audiocloud", "domain");
        let ast = r#"UPDATE type::thing("sys_prop", $prop_id) CONTENT {value: $value};"#;
        let vars = btreemap! {
            "prop_id".to_owned() => Value::from(Strand::from(prop_id.to_owned())),
            "value".to_owned() => value,
        };

        let res = self.db.execute(ast, &ses, Some(vars), false).await?;

        // we just need to know that the query was successful
        let _ = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;

        Ok(())
    }
}
