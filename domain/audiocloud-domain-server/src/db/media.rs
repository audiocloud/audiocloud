#![allow(non_snake_case)]

use std::collections::HashMap;

use anyhow::anyhow;
use maplit::btreemap;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use surrealdb::{sql, sql::Value};

use audiocloud_api::{AppMediaObjectId, MediaDownload, MediaMetadata, MediaObject, MediaUpload};

use crate::db::Db;
use crate::media::{DownloadJobId, UploadJobId};

pub type UploadJobs = HashMap<UploadJobId, MediaUpload>;
pub type DownloadJobs = HashMap<DownloadJobId, MediaDownload>;

impl Db {
    pub async fn clear_in_progress_for_all_jobs(&self) -> anyhow::Result<()> {
        self.execute_no_params("UPDATE job SET state.in_progress = false").await
    }

    pub async fn fetch_pending_download_jobs(&self, limit: usize) -> anyhow::Result<DownloadJobs> {
        let values: Vec<WithId<DownloadJobId, MediaDownload>> = self.query_multi(
            &format!("SELECT *, media.id AS media_id FROM job WHERE download != null AND state.in_progress == false ORDER BY created_at ASC LIMIT {limit}"),
            None
        ).await?;

        Ok(values.into_iter().map(|v| (v.id, v.value)).collect())
    }

    pub async fn fetch_pending_upload_jobs(&self, limit: usize) -> anyhow::Result<UploadJobs> {
        let values: Vec<WithId<UploadJobId, MediaUpload>> = self.query_multi(
            &format!("SELECT *, media.id AS media_id FROM job WHERE upload != null AND state.in_progress == false ORDER BY created_at ASC LIMIT {limit}"),
            None
        ).await?;

        Ok(values.into_iter().map(|v| (v.id, v.value)).collect())
    }

    pub async fn fetch_media_by_id(&self, id: &AppMediaObjectId) -> anyhow::Result<Option<MediaObject>> {
        self.query_one(r#"SELECT id, metadata, path FROM type::thing("media", $id) FETCH upload, download"#,
                       Some(btreemap! {"id".to_owned() => Value::from(id.to_string())}))
            .await
    }

    pub async fn create_initial_media(&self,
                                      media_id: &AppMediaObjectId,
                                      metadata: Option<MediaMetadata>,
                                      path: Option<String>)
                                      -> anyhow::Result<MediaObject> {
        let media = MediaObject { id:       { media_id.clone() },
                                  metadata: { metadata },
                                  path:     { path },
                                  download: { None },
                                  upload:   { None },
                                  revision: { 0 }, };

        let ast = r#"UPDATE type::thing("media", $id) CONTENT $media"#;
        let vars = btreemap! {
            "id".to_owned() => Value::from(media_id.to_string()),
            "media".to_owned() => sql::json(&serde_json::to_string(&media)?)?
        };

        self.execute(ast, Some(vars)).await?;

        Ok(media)
    }

    pub async fn save_media(&self, media: MediaObject) -> anyhow::Result<()> {
        let ast = r#"UPDATE type::thing("media", $id) SET revision = revision + 1, metadata = $metadata, path = $path"#;
        let vars = btreemap! {
            "id".to_owned() => Value::from(media.id.to_string()),
            "metadata".to_owned() => sql::json(&serde_json::to_string(&media.metadata)?)?,
            "path".to_owned() => Value::from(media.path),
        };

        self.execute(ast, Some(vars)).await
    }

    pub async fn save_download_job(&self, id: &DownloadJobId, download: &MediaDownload) -> anyhow::Result<()> {
        let ast = r#"UPDATE type::thing("job", $id) SET state = $state, download = $download, created_at = $created_at, media = type::thing("media", $media_id)"#;
        let vars = btreemap! {
            "id".to_owned() => Value::from(id.to_string()),
            "state".to_owned() => sql::json(&serde_json::to_string(&download.state)?)?,
            "download".to_owned() => sql::json(&serde_json::to_string(&download.download)?)?,
            "media_id".to_owned() => Value::from(download.media_id.to_string()),
            "created_at".to_owned() => Value::Datetime(Datetime::from(download.created_at)),
        };

        self.execute(ast, Some(vars)).await
    }

    pub async fn save_upload_job(&self, id: &UploadJobId, upload: &MediaUpload) -> anyhow::Result<()> {
        let ast = r#"UPDATE type::thing("job", $id) SET state = $state, upload = $upload, created_at = $created_at, media = type::thing("media", $media_id)"#;
        let vars = btreemap! {
            "id".to_owned() => Value::from(id.to_string()),
            "state".to_owned() => sql::json(&serde_json::to_string(&upload.state)?)?,
            "upload".to_owned() => sql::json(&serde_json::to_string(&upload.upload)?)?,
            "media_id".to_owned() => Value::from(upload.media_id.to_string()),
            "created_at".to_owned() => Value::Datetime(Datetime::from(upload.created_at)),
        };

        self.execute(ast, Some(vars)).await
    }

    pub async fn delete_upload(&self, id: &UploadJobId) -> anyhow::Result<()> {
        self.delete_job(id.as_str()).await
    }

    pub async fn delete_download(&self, id: &DownloadJobId) -> anyhow::Result<()> {
        self.delete_job(id.as_str()).await
    }

    async fn delete_job(&self, id: &str) -> anyhow::Result<()> {
        let ses = Self::session();
        let ast = r#"DELETE type::thing("job", $id)"#;
        let vars = btreemap! {"id".to_owned() => Value::from(id)};

        let res = self.db.execute(&ast, &ses, Some(vars), false).await?;
        let res = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let ___ = res.result?;

        Ok(())
    }

    pub async fn debug_jobs(&self) -> anyhow::Result<()> {
        let multi = self.query_multi::<serde_json::Value>(r#"SELECT * FROM job"#, None).await?;
        println!("{}", serde_json::to_string_pretty(&multi)?);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct WithId<K, V> {
    id:    K,
    #[serde(flatten)]
    value: V,
}
