#![allow(non_snake_case)]

use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use anyhow::anyhow;
use derive_more::FromStr;
use maplit::btreemap;
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Value, Session};

use audiocloud_api::{
    now, AppMediaObjectId, DownloadFromDomain, MediaDownload, MediaJobState, MediaMetadata, MediaObject, MediaUpload, Timestamp,
    UploadToDomain,
};

use crate::db::Db;
use crate::media::{DownloadJobId, UploadJobId};

const KIND_DOWNLOAD: &str = "download";
const KIND_UPLOAD: &str = "upload";

pub type UploadJobs = HashMap<UploadJobId, MediaUpload>;
pub type DownloadJobs = HashMap<DownloadJobId, MediaDownload>;

impl Db {
    pub async fn clear_in_progress_for_all_jobs(&self) -> anyhow::Result<()> {
        let ses = Self::session();
        let ast = "UPDATE job SET state.in_progress = false";

        let res = self.db.execute(&ast, &ses, None, false).await?;
        let res = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let ___ = res.result?;

        Ok(())
    }

    pub async fn fetch_pending_download_jobs(&self, limit: usize) -> anyhow::Result<DownloadJobs> {
        let mut rv = HashMap::new();
        let ses = Self::session();
        let ast = "SELECT *, string::slice(media.id, 5, -1) AS media_id FROM job WHERE download != null AND state.in_progress != false ORDER BY created_at ASC LIMIT $limit";
        let vars = btreemap! {"limit".to_owned() => Value::from(limit)};

        let res = self.db.execute(&ast, &ses, Some(vars), false).await?;
        let res = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let result = res.result?;

        for mut job in serde_json::from_value::<Vec<WithId<DownloadJobId, MediaDownload>>>(serde_json::to_value(&result)?)? {
            job.value.media_id = normalize_id(job.value.media_id.as_str())?;
            rv.insert(normalize_id(job.id.as_str())?, job.value);
        }

        Ok(rv)
    }

    fn session() -> Session {
        Session::for_db("audiocloud", "domain")
    }

    pub async fn fetch_pending_upload_jobs(&self, limit: usize) -> anyhow::Result<UploadJobs> {
        let mut rv = HashMap::new();
        let ses = Self::session();
        let ast = "SELECT string::slice(id, 3, -1), state, created_at, download, string::slice(media.id, 5, -1) AS media_id FROM job WHERE download != null AND state.in_progress != false ORDER BY created_at ASC LIMIT $limit";
        let vars = btreemap! {"limit".to_owned() => Value::from(limit)};

        let res = self.db.execute(&ast, &ses, Some(vars), false).await?;
        let res = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let result = res.result?;

        for mut job in serde_json::from_value::<Vec<WithId<UploadJobId, MediaUpload>>>(serde_json::to_value(&result)?)? {
            job.value.media_id = normalize_id(job.value.media_id)?;
            rv.insert(normalize_id(job.id.as_str())?, job.value);
        }

        Ok(rv)
    }

    pub async fn fetch_media_by_id(&self, id: &AppMediaObjectId) -> anyhow::Result<Option<MediaObject>> {
        let ses = Self::session();
        let ast = r#"SELECT id, metadata, path, upload.*, download.* FROM type::thing("media", $id)"#;
        let vars = btreemap! {"id".to_owned() => Value::from(id.slashed())};

        let res = self.db.execute(&ast, &ses, Some(vars), false).await?;
        let res = res.into_iter().next().ok_or_else(|| anyhow!("No response"))?;
        let result = res.result?;

        #[derive(Deserialize)]
        struct Media {
            id:       String,
            metadata: Option<MediaMetadata>,
            path:     Option<String>,
            upload:   Option<MediaUpload>,
            download: Option<MediaDownload>,
            revision: u64,
        }

        let media = serde_json::from_value::<Vec<Media>>(serde_json::to_value(&result)?)?;
        let media = media.into_iter().next();

        Ok(match media {
            None => None,
            Some(media) => Some(MediaObject { id:       normalize_id(&media.id)?,
                                              metadata: media.metadata,
                                              path:     media.path,
                                              download: media.download,
                                              upload:   media.upload,
                                              revision: media.revision, }),
        })
    }

    pub async fn save_media(&self, media: MediaObject) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn delete_upload(&self, id: &UploadJobId) -> anyhow::Result<()> {
        self.delete_job(id.as_str()).await
    }

    pub async fn save_download_job(&self, id: &DownloadJobId, download: &MediaDownload) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn save_upload_job(&self, id: &UploadJobId, upload: &MediaUpload) -> anyhow::Result<()> {
        todo!()
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
}

fn normalize_id<'a, T>(id: &str) -> Result<T, <T as FromStr>::Err>
    where T: FromStr
{
    let mut id = match id.find(':') {
        None => id,
        Some(index) => &id[index + 1..],
    };

    if id.starts_with('⟨') {
        id = &id[1..];
    }

    if id.ends_with('⟩') {
        id = &id[..id.len() - 1];
    }

    T::from_str(id)
}

#[derive(Serialize, Deserialize)]
struct WithId<K, V> {
    id:    K,
    #[serde(flatten)]
    value: V,
}
