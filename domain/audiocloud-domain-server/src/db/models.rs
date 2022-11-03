/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashSet;

use audiocloud_api::{Model, ModelId};

use crate::db::{prisma, Db};

impl Db {
    pub async fn delete_all_models_except(&self, ids: &HashSet<ModelId>) -> anyhow::Result<()> {
        self.db
            .model()
            .delete_many(vec![prisma::model::id::not_in_vec(ids.iter().map(|id| id.to_string()).collect())])
            .exec()
            .await?;

        Ok(())
    }

    pub async fn set_model(&self, model_id: &ModelId, model: &Model) -> anyhow::Result<()> {
        self.db
            .model()
            .upsert(prisma::model::id::equals(model_id.to_string()),
                    prisma::model::create(model_id.to_string(), serde_json::to_string_pretty(model)?, vec![]),
                    vec![prisma::model::spec::set(serde_json::to_string_pretty(model)?)])
            .exec()
            .await?;

        Ok(())
    }

    pub async fn get_model(&self, model_id: &ModelId) -> anyhow::Result<Option<Model>> {
        let model = self.db
                        .model()
                        .find_unique(prisma::model::id::equals(model_id.to_string()))
                        .exec()
                        .await?;

        Ok(model.map(|model| serde_json::from_str(&model.spec))
                .map_or(Ok(None), |result| result.map(Some))?)
    }
}
