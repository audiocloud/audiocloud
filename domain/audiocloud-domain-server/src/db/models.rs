use std::collections::HashSet;

use anyhow::anyhow;
use maplit::btreemap;
use surrealdb::{Session, sql};
use surrealdb::sql::{Strand, Value};

use audiocloud_api::{Model, ModelId};

use crate::db::Db;
use crate::Deserialize;

fn slash(model_id: &ModelId) -> String {
    format!("{}/{}", model_id.manufacturer, model_id.name)
}

impl Db {
    pub async fn delete_all_models_except(&self, id: &HashSet<ModelId>) -> anyhow::Result<()> {
        let ses = Session::for_db("audiocloud", "domain");
        let conds = id.iter().map(|id| {
            let id = slash(id);
            format!(r#"id != type::thing("model", "{id}")"#)
        }).collect::<Vec<_>>().join(" AND ");
        let ast = format!(r#"DELETE FROM model WHERE {conds};"#);

        let res = self.db.execute(&ast, &ses, None, false).await?;

        // we just need to know that the query was successful
        let _ = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;

        Ok(())
    }

    pub async fn set_model(&self, model_id: &ModelId, model: &Model) -> anyhow::Result<()> {
        let ses = Session::for_db("audiocloud", "domain");
        let ast = r#"UPDATE type::thing("model", $model_id) CONTENT {"model": $model};"#;

        let vars = btreemap! {
            "model_id".to_owned() => Value::from(Strand::from(slash(model_id))),
            "model".to_owned() => sql::json(&serde_json::to_string(model)?)?,
        };

        let res = self.db.execute(&ast, &ses, Some(vars), false).await?;

        let _ = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;

        Ok(())
    }

    pub async fn get_model(&self, model_id: &ModelId) -> anyhow::Result<Option<Model>> {
        let ses = Session::for_db("audiocloud", "domain");
        let ast = r#"SELECT model FROM model WHERE id = type::thing("model", $model_id);"#;

        let vars = btreemap! {"model_id".to_owned() => Value::from(Strand::from(slash(model_id)))};

        #[derive(Deserialize)]
        struct QueryResult {
            model: Model,
        }

        let res = self.db.execute(&ast, &ses, Some(vars), false).await?;
        let response = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let result = response.result?;

        let value: Vec<QueryResult> = serde_json::from_value(serde_json::to_value(&result)?)?;
        Ok(value.into_iter().next().map(|qr| qr.model))
    }
}
