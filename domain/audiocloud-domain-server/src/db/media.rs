use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;
use sqlx::prelude::*;

use audiocloud_api::{
    now, AppMediaObjectId, MediaDownload, MediaJobState, MediaMetadata, MediaObject, MediaUpload,
    Timestamp,
};

use crate::db::Db;
use crate::media::{DownloadJobId, UploadJobId};

#[derive(Debug, FromRow)]
struct MediaJobRow {
    id: String,
    media_id: String,
    kind: String,
    spec: sqlx::types::Json<serde_json::Value>,
    state: sqlx::types::Json<MediaJobState>,
    last_modified: Timestamp,
    active: i64,
}

impl TryInto<MediaDownload> for MediaJobRow {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MediaDownload, Self::Error> {
        let Self {
            media_id,
            kind,
            spec,
            state,
            ..
        } = self;
        if kind != KIND_DOWNLOAD {
            return Err(anyhow!("Job is not download"));
        }

        Ok(MediaDownload {
            media_id: { AppMediaObjectId::from_str(&media_id)? },
            download: { serde_json::from_value(spec.0)? },
            state: { state.0 },
        })
    }
}

impl TryInto<MediaUpload> for MediaJobRow {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MediaUpload, Self::Error> {
        let Self {
            media_id,
            kind,
            spec,
            state,
            ..
        } = self;
        if kind != KIND_UPLOAD {
            return Err(anyhow!("Job is not upload"));
        }

        Ok(MediaUpload {
            media_id: { AppMediaObjectId::from_str(&media_id)? },
            upload: { serde_json::from_value(spec.0)? },
            state: { state.0 },
        })
    }
}

#[derive(Debug, FromRow)]
struct MediaObjectRow {
    id: String,
    path: Option<String>,
    metadata: Option<sqlx::types::Json<MediaMetadata>>,
    revision: i64,
    last_used: Timestamp,
}

const KIND_DOWNLOAD: &str = "download";
const KIND_UPLOAD: &str = "upload";

pub type UploadJobs = HashMap<UploadJobId, MediaUpload>;
pub type DownloadJobs = HashMap<DownloadJobId, MediaDownload>;

impl Db {
    pub async fn fetch_pending_download_jobs(&self, limit: usize) -> anyhow::Result<DownloadJobs> {
        let mut rv = HashMap::new();
        for row in self.fetch_jobs(false, KIND_DOWNLOAD, limit).await? {
            let job_id = DownloadJobId::from_str(&row.id)?;

            rv.insert(job_id, row.try_into()?);
        }

        Ok(rv)
    }

    pub async fn fetch_pending_upload_jobs(&self, limit: usize) -> anyhow::Result<UploadJobs> {
        let mut rv = HashMap::new();
        for row in self.fetch_jobs(false, KIND_UPLOAD, limit).await? {
            let job_id = UploadJobId::from_str(&row.id)?;
            rv.insert(job_id, row.try_into()?);
        }

        Ok(rv)
    }

    async fn fetch_jobs(
        &self,
        active: bool,
        kind: &'static str,
        limit: usize,
    ) -> anyhow::Result<Vec<MediaJobRow>> {
        let query = r#"SELECT * FROM media_job WHERE active = ? AND kind = ? AND media_id IS NOT NULL ORDER BY media_id LIMIT ?"#;

        Ok(sqlx::query_as(query)
            .bind(active)
            .bind(kind)
            .bind(limit as u32)
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn fetch_media_by_id(
        &self,
        id: &AppMediaObjectId,
    ) -> anyhow::Result<Option<MediaObject>> {
        let opt: Option<MediaObjectRow> =
            sqlx::query_as(r#"SELECT * FROM media_object WHERE id = ?"#)
                .bind(id.to_string())
                .fetch_optional(&self.pool)
                .await?;

        Ok(match opt {
            None => None,
            Some(MediaObjectRow {
                id,
                path,
                metadata,
                revision,
                ..
            }) => Some(MediaObject {
                id: { AppMediaObjectId::from_str(&id)? },
                metadata: { metadata.map(|json| json.0) },
                path: { path },
                download: { None },
                upload: { None },
                revision: { revision } as u64,
            }),
        })
    }

    pub async fn save_media(&self, media: MediaObject) -> anyhow::Result<()> {
        let MediaObject {
            id,
            metadata,
            path,
            revision,
            ..
        } = media;

        let query = r#"INSERT OR REPLACE INTO media_object (id, path, metadata, revision, last_used) VALUES (?, ?, ?, ?, ?)"#;

        sqlx::query(query)
            .bind(id.to_string())
            .bind(path)
            .bind(metadata.map(|m| sqlx::types::Json(m)))
            .bind(revision as i64)
            .bind(now())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_upload(&self, id: &UploadJobId) -> anyhow::Result<()> {
        self.delete_job(id.to_string()).await
    }

    pub async fn save_download_job(
        &self,
        id: &DownloadJobId,
        download: &MediaDownload,
    ) -> anyhow::Result<()> {
        sqlx::query(r#"INSERT OR REPLACE INTO media_job (id, kind, spec, state, last_modified, active, media_id) VALUES(?, ?, ?, ?, ?, ?, ?)"#)
      .bind(id.to_string())
      .bind(KIND_DOWNLOAD.to_string())
      .bind(serde_json::to_string(&download.download)?)
      .bind(serde_json::to_string(&download.state)?)
      .bind(now())
      .bind(!download.state.in_progress)
      .bind(download.media_id.to_string())
      .execute(&self.pool).await?;

        Ok(())
    }

    pub async fn save_upload_job(
        &self,
        id: &UploadJobId,
        upload: &MediaUpload,
    ) -> anyhow::Result<()> {
        sqlx::query(r#"INSERT OR REPLACE INTO media_job (id, kind, spec, state, last_modified, active, media_id) VALUES(?, ?, ?, ?, ?, ?, ?)"#)
            .bind(id.to_string())
            .bind(KIND_UPLOAD.to_string())
            .bind(serde_json::to_string(&upload.upload)?)
            .bind(serde_json::to_string(&upload.state)?)
            .bind(now())
            .bind(!upload.state.in_progress)
            .bind(upload.media_id.to_string())
            .execute(&self.pool).await?;

        Ok(())
    }

    pub async fn delete_download(&self, id: &DownloadJobId) -> anyhow::Result<()> {
        self.delete_job(id.to_string()).await
    }

    async fn delete_job(&self, id: String) -> anyhow::Result<()> {
        let id = id.to_string();
        sqlx::query!(r#"DELETE FROM media_job WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
