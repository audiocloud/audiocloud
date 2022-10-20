use audiocloud_api::{Model, ModelId};

use crate::db::Db;

impl Db {
    pub async fn delete_all_models(&self) -> anyhow::Result<()> {
        sqlx::query!(r#"DELETE FROM model WHERE true"#).execute(&self.pool).await?;

        Ok(())
    }

    pub async fn set_model(&self, model_id: ModelId, model: Model) -> anyhow::Result<()> {
        let spec = serde_json::to_string(&model)?;
        let model_id = model_id.to_string();

        sqlx::query!(r#"INSERT OR REPLACE INTO model (id, spec) VALUES (?, ?)"#, model_id, spec).execute(&self.pool)
                                                                                                .await?;

        Ok(())
    }

    pub async fn get_model(&self, model_id: &ModelId) -> anyhow::Result<Option<Model>> {
        let model_id = model_id.to_string();
        let result = sqlx::query!(r#"SELECT spec FROM model WHERE id=?"#, model_id).fetch_optional(&self.pool)
                                                                                   .await?;
        Ok(match result {
            None => None,
            Some(model) => Some(serde_json::from_str(&model.spec)?),
        })
    }
}
