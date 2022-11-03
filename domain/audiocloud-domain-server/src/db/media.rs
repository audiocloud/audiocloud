#![allow(non_snake_case)]

use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use futures::TryFutureExt;
use maplit::btreemap;
use prisma_client_rust::Direction;
use serde::{Deserialize, Serialize};

use audiocloud_api::{now, AppMediaObjectId, MediaDownload, MediaMetadata, MediaObject, MediaUpload, Timestamp};

use crate::db::{prisma, Db};
use crate::media::{DownloadJobId, UploadJobId};

pub type UploadJobs = HashMap<UploadJobId, MediaUpload>;
pub type DownloadJobs = HashMap<DownloadJobId, MediaDownload>;

const DOWNLOAD_KIND: &str = "DOWNLOAD";
const UPLOAD_KIND: &str = "UPLOAD";

impl Db {
    pub async fn clear_in_progress_for_all_jobs(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn fetch_pending_download_jobs(&self, limit: usize) -> anyhow::Result<DownloadJobs> {
        let mut rv = DownloadJobs::new();
        let rows = self.db
                       .media_job()
                       .find_many(vec![prisma::media_job::kind::equals(DOWNLOAD_KIND.to_owned()),
                                       prisma::media_job::in_progress::equals(false),])
                       .order_by(prisma::media_job::created_at::order(Direction::Asc))
                       .take(limit as i64)
                       .exec()
                       .await?;

        for row in rows {
            rv.insert(DownloadJobId::new(row.id),
                      MediaDownload { media_id:   AppMediaObjectId::from_str(&row.media_id)?,
                                      download:   serde_json::from_str(&row.config)?,
                                      state:      serde_json::from_str(&row.status)?,
                                      created_at: row.created_at.into(), });
        }

        Ok(rv)
    }

    pub async fn fetch_pending_upload_jobs(&self, limit: usize) -> anyhow::Result<UploadJobs> {
        let mut rv = UploadJobs::new();
        let rows = self.db
                       .media_job()
                       .find_many(vec![prisma::media_job::kind::equals(UPLOAD_KIND.to_owned()),
                                       prisma::media_job::in_progress::equals(false),])
                       .order_by(prisma::media_job::created_at::order(Direction::Asc))
                       .take(limit as i64)
                       .exec()
                       .await?;

        for row in rows {
            rv.insert(UploadJobId::new(row.id),
                      MediaUpload { media_id:   AppMediaObjectId::from_str(&row.media_id)?,
                                    upload:     serde_json::from_str(&row.config)?,
                                    state:      serde_json::from_str(&row.status)?,
                                    created_at: row.created_at.into(), });
        }

        Ok(rv)
    }

    pub async fn fetch_media_by_id(&self, id: &AppMediaObjectId) -> anyhow::Result<Option<MediaObject>> {
        match self.db
                  .media_file()
                  .find_unique(prisma::media_file::id::equals(id.to_string()))
                  .exec()
                  .await?
        {
            None => Ok(None),
            Some(media_file) => {
                let metadata = media_file.metadata
                                         .as_ref()
                                         .map(String::as_str)
                                         .map(serde_json::from_str)
                                         .map_or(Ok(None), |r| r.map(Some))?;

                Ok(Some(MediaObject { metadata,
                                      path: media_file.path,
                                      id: id.clone(),
                                      revision: media_file.revision as u64,
                                      last_used: media_file.last_used.map(|last_used| last_used.with_timezone(&Utc)) }))
            }
        }
    }

    pub async fn create_initial_media(&self,
                                      id: &AppMediaObjectId,
                                      metadata: Option<MediaMetadata>,
                                      path: Option<String>)
                                      -> anyhow::Result<MediaObject> {
        let metadata = metadata.as_ref()
                               .map(serde_json::to_string_pretty)
                               .map_or(Ok(None), |r| r.map(Some))?;

        self.db
            .media_file()
            .create(id.to_string(),
                    vec![prisma::media_file::path::set(path), prisma::media_file::metadata::set(metadata)])
            .exec()
            .await?;

        Ok(self.fetch_media_by_id(id)
               .await?
               .ok_or_else(|| anyhow!("Failed to create media object"))?)
    }

    pub async fn update_media(&self, media: MediaObject) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn save_download_job(&self, id: &DownloadJobId, download: &MediaDownload) -> anyhow::Result<()> {
        let state = serde_json::to_string_pretty(&download.state)?;
        let config = serde_json::to_string_pretty(&download.download)?;

        self.db
            .media_job()
            .upsert(prisma::media_job::id::equals(id.to_string()),
                    prisma::media_job::create(id.to_string(),
                                              state.clone(),
                                              config.clone(),
                                              DOWNLOAD_KIND.to_owned(),
                                              prisma::media_file::id::equals(download.media_id.to_string()),
                                              vec![prisma::media_job::created_at::set(download.created_at.into())]),
                    vec![prisma::media_job::status::set(state),
                         prisma::media_job::config::set(config),
                         prisma::media_job::created_at::set(download.created_at.into())])
            .exec()
            .await?;

        Ok(())
    }

    pub async fn save_upload_job(&self, id: &UploadJobId, upload: &MediaUpload) -> anyhow::Result<()> {
        let state = serde_json::to_string_pretty(&upload.state)?;
        let config = serde_json::to_string_pretty(&upload.upload)?;

        self.db
            .media_job()
            .upsert(prisma::media_job::id::equals(id.to_string()),
                    prisma::media_job::create(id.to_string(),
                                              state.clone(),
                                              config.clone(),
                                              UPLOAD_KIND.to_owned(),
                                              prisma::media_file::id::equals(upload.media_id.to_string()),
                                              vec![prisma::media_job::created_at::set(upload.created_at.into())]),
                    vec![prisma::media_job::status::set(state),
                         prisma::media_job::config::set(config),
                         prisma::media_job::created_at::set(upload.created_at.into())])
            .exec()
            .await?;

        Ok(())
    }

    pub async fn delete_upload(&self, id: &UploadJobId) -> anyhow::Result<()> {
        self.delete_job(id.as_str()).await
    }

    pub async fn delete_download(&self, id: &DownloadJobId) -> anyhow::Result<()> {
        self.delete_job(id.as_str()).await
    }

    async fn delete_job(&self, id: &str) -> anyhow::Result<()> {
        self.db
            .media_job()
            .delete(prisma::media_job::id::equals(id.to_owned()))
            .exec()
            .await?;

        Ok(())
    }

    pub async fn debug_jobs(&self) -> anyhow::Result<()> {
        for job in self.db.media_job().find_many(vec![]).exec().await? {
            println!("{}: {} ({})", job.id, job.kind, job.status);
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct WithId<K, V> {
    id:    K,
    #[serde(flatten)]
    value: V,
}
