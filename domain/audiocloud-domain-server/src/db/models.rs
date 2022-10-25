use std::collections::HashSet;
use std::iter::repeat;

use anyhow::anyhow;
use futures::stream::iter;
use maplit::btreemap;
use rbs::Value;

use audiocloud_api::{Model, ModelId};

use crate::db::Db;
use crate::Deserialize;

impl Db {
    pub async fn delete_all_models_except(&self, ids: &HashSet<ModelId>) -> anyhow::Result<()> {
        let id_keys = repeat("?").take(ids.len()).collect::<Vec<_>>().join(", ");
        let sql = format!("delete from models where id not in ({id_keys})");

        self.db
            .exec(&sql,
                  ids.iter().cloned().map(|model_id| Value::from(model_id.to_string())).collect())
            .await?;

        Ok(())
    }

    pub async fn set_model(&self, model_id: &ModelId, model: &Model) -> anyhow::Result<()> {
        let model = serde_json::to_string(model)?;
        self.db
            .exec("insert or replace into models (id, spec) values (?1, ?2)",
                  vec![Value::from(model_id.to_string()), Value::from(model)])
            .await?;

        Ok(())
    }

    pub async fn get_model(&self, model_id: &ModelId) -> anyhow::Result<Option<Model>> {
        let model: Option<String> = self.db
                                        .fetch_decode("select spec from models where id = ?1", vec![Value::from(model_id.to_string())])
                                        .await?;

        Ok(match model {
            None => None,
            Some(spec) => serde_json::from_str(&spec)?,
        })
    }
}
