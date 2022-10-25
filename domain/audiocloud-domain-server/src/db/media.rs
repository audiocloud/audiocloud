#![allow(non_snake_case)]

use std::collections::HashMap;

use anyhow::anyhow;
use maplit::btreemap;
use rbs::to_value;
use serde::{Deserialize, Serialize};

use audiocloud_api::{now, AppMediaObjectId, MediaDownload, MediaMetadata, MediaObject, MediaUpload, Timestamp};

use crate::db::Db;
use crate::media::{DownloadJobId, UploadJobId};

pub type UploadJobs = HashMap<UploadJobId, MediaUpload>;
pub type DownloadJobs = HashMap<DownloadJobId, MediaDownload>;

impl Db {
    pub async fn clear_in_progress_for_all_jobs(&self) -> anyhow::Result<()> {
        self.db.exec(include_str!("sql/clear_in_progress_for_all_jobs.sql"), vec![]).await?;

        Ok(())
    }

    pub async fn fetch_pending_download_jobs(&self, limit: usize) -> anyhow::Result<DownloadJobs> {
        let sql = include_str!("sql/fetch_pending_download_jobs.sql");

        #[derive(Deserialize)]
        struct DownloadJobRow {
            id:         DownloadJobId,
            media_id:   AppMediaObjectId,
            state:      String,
            download:   String,
            created_at: Timestamp,
        }

        let rows: Vec<DownloadJobRow> = self.db.fetch_decode(sql, vec![limit.into()]).await?;
        let mut rv = HashMap::new();

        for row in rows {
            rv.insert(row.id,
                      MediaDownload { media_id:   row.media_id,
                                      download:   serde_json::from_str(&row.download)?,
                                      state:      serde_json::from_str(&row.state)?,
                                      created_at: row.created_at, });
        }

        Ok(rv)
    }

    pub async fn fetch_pending_upload_jobs(&self, limit: usize) -> anyhow::Result<UploadJobs> {
        let sql = include_str!("sql/fetch_pending_upload_jobs.sql");

        #[derive(Deserialize)]
        struct UploadJobRow {
            id:         UploadJobId,
            media_id:   AppMediaObjectId,
            state:      String,
            upload:     String,
            created_at: Timestamp,
        }

        let rows: Vec<UploadJobRow> = self.db.fetch_decode(sql, vec![limit.into()]).await?;
        let mut rv = HashMap::new();

        for row in rows {
            rv.insert(row.id,
                      MediaUpload { media_id:   row.media_id,
                                    upload:     serde_json::from_str(&row.upload)?,
                                    state:      serde_json::from_str(&row.state)?,
                                    created_at: row.created_at, });
        }

        Ok(rv)
    }

    pub async fn fetch_media_by_id(&self, id: &AppMediaObjectId) -> anyhow::Result<Option<MediaObject>> {
        #[derive(Deserialize)]
        struct MediaRow {
            id:         AppMediaObjectId,
            metadata:   String,
            path:       Option<String>,
            created_at: Timestamp,
            last_used:  Timestamp,
            revision:   u64,
        }

        let sql = include_str!("sql/fetch_media_by_id.sql");
        let media: Option<MediaRow> = self.db.fetch_decode(sql, vec![to_value!(id.to_string())]).await?;

        Ok(match media {
            None => None,
            Some(row) => {
                let mut rv = MediaObject { id:        { row.id },
                                           metadata:  { serde_json::from_str(&row.metadata)? },
                                           path:      { row.path },
                                           last_used: { row.last_used },
                                           revision:  { row.revision }, };

                if rv.path.as_ref().map(|path| path.is_empty()).unwrap_or(false) {
                    rv.path = None;
                }

                Some(rv)
            }
        })
    }

    pub async fn create_initial_media(&self,
                                      id: &AppMediaObjectId,
                                      metadata: Option<MediaMetadata>,
                                      path: Option<String>)
                                      -> anyhow::Result<MediaObject> {
        let serialized_metadata = match metadata.as_ref() {
            None => None,
            Some(metadata) => Some(serde_json::to_string_pretty(metadata)?),
        };

        let last_used = now();
        let sql = include_str!("sql/create_initial_media.sql");
        let vars = vec![to_value!(id.to_string()),
                        to_value!(last_used),
                        to_value!(serialized_metadata),
                        to_value!(path.clone())];

        self.db.exec(sql, vars).await?;

        Ok(MediaObject { id:        { id.clone() },
                         metadata:  { metadata },
                         path:      { path },
                         last_used: { last_used },
                         revision:  { 0 }, })
    }

    pub async fn update_media(&self, media: MediaObject) -> anyhow::Result<()> {
        let serialized_metadata = match media.metadata.as_ref() {
            None => None,
            Some(metadata) => Some(serde_json::to_string_pretty(metadata)?),
        };

        let sql = include_str!("sql/save_media.sql");
        let vars = vec![to_value!(media.id.to_string()),
                        to_value!(serialized_metadata),
                        to_value!(&media.path),
                        to_value!(media.last_used.to_string())];

        if self.db.exec(sql, vars).await?.rows_affected != 1 {
            return Err(anyhow!("Failed to update media, incorrect nubmer of rows affected"));
        }

        Ok(())
    }

    pub async fn save_download_job(&self, id: &DownloadJobId, download: &MediaDownload) -> anyhow::Result<()> {
        let serialized_state = serde_json::to_string_pretty(&download.state)?;
        let serialized_download = serde_json::to_string_pretty(&download.download)?;

        let sql = include_str!("sql/save_download_job.sql");
        let vars = vec![to_value!(id.to_string()),
                        to_value!(download.media_id.to_string()),
                        to_value!(serialized_state),
                        to_value!(serialized_download),
                        to_value!(download.created_at)];

        self.db.exec(sql, vars).await?;

        Ok(())
    }

    pub async fn save_upload_job(&self, id: &UploadJobId, upload: &MediaUpload) -> anyhow::Result<()> {
        let serialized_state = serde_json::to_string_pretty(&upload.state)?;
        let serialized_upload = serde_json::to_string_pretty(&upload.upload)?;

        let sql = include_str!("sql/save_upload_job.sql");
        let vars = vec![to_value!(id.to_string()),
                        to_value!(upload.media_id.to_string()),
                        to_value!(serialized_state),
                        to_value!(serialized_upload),
                        to_value!(upload.created_at)];

        self.db.exec(sql, vars).await?;

        Ok(())
    }

    pub async fn delete_upload(&self, id: &UploadJobId) -> anyhow::Result<()> {
        self.delete_job(id.as_str()).await
    }

    pub async fn delete_download(&self, id: &DownloadJobId) -> anyhow::Result<()> {
        self.delete_job(id.as_str()).await
    }

    async fn delete_job(&self, id: &str) -> anyhow::Result<()> {
        let sql = include_str!("sql/delete_job.sql");
        let vars = vec![to_value!(id)];

        self.db.exec(sql, vars).await?;

        Ok(())
    }

    pub async fn debug_jobs(&self) -> anyhow::Result<()> {
        let sql = "select * from media_jobs";
        let rows = self.db.fetch(sql, vec![]).await?;
        eprintln!("jobs: {rows:?}");

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct WithId<K, V> {
    id:    K,
    #[serde(flatten)]
    value: V,
}
