use serde::{Deserialize, Serialize};

pub mod request;
pub mod spec;
pub mod state;

pub mod buckets {
  use crate::media::spec::{MediaDownloadSpec, MediaId, MediaUploadSpec};
  use crate::media::state::{MediaDownloadState, MediaUploadState};
  use crate::{BucketKey, BucketName, IntoBucketKey};

  pub const DOWNLOAD_SPEC: BucketName<MediaDownloadSpec> = BucketName::new("audiocloud_media_download_spec");
  pub const DOWNLOAD_STATE: BucketName<MediaDownloadState> = BucketName::new("audiocloud_media_download_state");
  pub const UPLOAD_SPEC: BucketName<MediaUploadSpec> = BucketName::new("audiocloud_media_upload_spec");
  pub const UPLOAD_STATE: BucketName<MediaUploadState> = BucketName::new("audiocloud_media_upload_state");

  pub fn media_download_spec_key(media_id: &MediaId) -> BucketKey<MediaDownloadSpec> {
    media_id.to_bucket_key()
  }

  pub fn media_download_state_key(media_id: &MediaId) -> BucketKey<MediaDownloadState> {
    media_id.to_bucket_key()
  }

  pub fn media_upload_spec_key(media_id: &MediaId) -> BucketKey<MediaUploadSpec> {
    media_id.to_bucket_key()
  }

  pub fn media_upload_state_key(media_id: &MediaId) -> BucketKey<MediaUploadState> {
    media_id.to_bucket_key()
  }
}
