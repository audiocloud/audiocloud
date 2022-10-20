use actix::Message;

use audiocloud_api::common::media::{DownloadFromDomain, ImportToDomain, UploadToDomain};
use audiocloud_api::newtypes::{AppMediaObjectId, AppTaskId};
use audiocloud_api::{MediaDownload, MediaUpload};

use crate::media::{DownloadJobId, UploadJobId};

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct RestartPendingUploadsDownloads;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct QueueDownload {
    pub job_id: DownloadJobId,
    pub media_id: AppMediaObjectId,
    pub download: DownloadFromDomain,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct QueueUpload {
    pub job_id: UploadJobId,
    pub session_id: Option<AppTaskId>,
    pub media_id: AppMediaObjectId,
    pub upload: Option<UploadToDomain>,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ImportMedia {
    pub media_id: AppMediaObjectId,
    pub import: ImportToDomain,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct NotifyDownloadProgress {
    pub job_id: DownloadJobId,
    pub download: MediaDownload,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct NotifyUploadProgress {
    pub job_id: UploadJobId,
    pub upload: MediaUpload,
}
