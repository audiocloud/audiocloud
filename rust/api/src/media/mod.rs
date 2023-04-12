use serde::{Deserialize, Serialize};

pub mod request;
pub mod spec;
pub mod state;

pub mod buckets {
  use crate::media::spec::{MediaDownloadSpec, MediaUploadSpec};
  use crate::media::state::{MediaDownloadState, MediaUploadState};
  use crate::BucketName;

  pub const DOWNLOAD_SPEC: BucketName<MediaDownloadSpec> = BucketName::new("audiocloud_media_download_spec");
  pub const DOWNLOAD_STATE: BucketName<MediaDownloadState> = BucketName::new("audiocloud_media_download_state");
  pub const UPLOAD_SPEC: BucketName<MediaUploadSpec> = BucketName::new("audiocloud_media_upload_spec");
  pub const UPLOAD_STATE: BucketName<MediaUploadState> = BucketName::new("audiocloud_media_upload_state");
}
