use std::collections::HashMap;

use anyhow::anyhow;
use tracing::*;

use audiocloud_api::cloud::domains::{DomainConfig, DomainModelSource};
use audiocloud_api::ModelId;

use crate::db::Db;

#[instrument(skip_all, err)]
pub async fn init(cfg: &DomainConfig, db: Db) -> anyhow::Result<()> {
    let models = match &cfg.models {
        DomainModelSource::Inline { models } => models.clone(),
        DomainModelSource::Local { path } => {
            let mut rv = HashMap::new();
            for model_path in globwalk::GlobWalkerBuilder::from_patterns(path, &["*.yaml", "*.yml"]).max_depth(4)
                                                                                                    .follow_links(true)
                                                                                                    .build()?
                                                                                                    .into_iter()
                                                                                                    .filter_map(Result::ok)
            {
                let model_path = model_path.path();
                let model_file_stem = model_path.file_stem()
                                                .ok_or_else(|| anyhow!("missing stem"))?
                                                .to_string_lossy()
                                                .to_string();

                let (manufacturer, name) =
                    model_file_stem.split_at(model_file_stem.find('_').ok_or_else(|| anyhow!("missing '_' character"))?);
                let name = &name[1..];

                let text = tokio::fs::read_to_string(model_path).await?;
                let model = serde_yaml::from_str(&text)?;

                rv.insert(ModelId::new(manufacturer.to_owned(), name.to_owned()), model);
            }

            rv
        }
        DomainModelSource::Remote { url, .. } => reqwest::get(url).await?.json().await?,
    };

    db.delete_all_models().await?;

    for (id, model) in models {
        debug!(%id, "registering model");
        db.set_model(id, model).await?;
    }

    Ok(())
}
