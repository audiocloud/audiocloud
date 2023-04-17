use schemars::schema::RootSchema;
use schemars::schema_for;
use schemars_zod::merge_schemas;
use serde::{Deserialize, Serialize};

use crate::media::spec::{MediaDownloadSpec, MediaUploadSpec};
use crate::media::state::{MediaDownloadState, MediaUploadState};

pub mod request;
pub mod spec;
pub mod state;

pub mod buckets {
  use crate::media::spec::{MediaDownloadSpec, MediaId, MediaUploadSpec};
  use crate::media::state::{MediaDownloadState, MediaUploadState};
  use crate::{BucketKey, BucketName};

  pub const DOWNLOAD_SPEC: BucketName<MediaDownloadSpec> = BucketName::new("audiocloud_media_download_spec");
  pub const DOWNLOAD_STATE: BucketName<MediaDownloadState> = BucketName::new("audiocloud_media_download_state");
  pub const UPLOAD_SPEC: BucketName<MediaUploadSpec> = BucketName::new("audiocloud_media_upload_spec");
  pub const UPLOAD_STATE: BucketName<MediaUploadState> = BucketName::new("audiocloud_media_upload_state");

  pub fn media_download_spec_key(media_id: &MediaId) -> BucketKey<MediaId, MediaDownloadSpec> {
    media_id.into()
  }

  pub fn media_download_state_key(media_id: &MediaId) -> BucketKey<MediaId, MediaDownloadState> {
    media_id.into()
  }

  pub fn media_upload_spec_key(media_id: &MediaId) -> BucketKey<MediaId, MediaUploadSpec> {
    media_id.into()
  }

  pub fn media_upload_state_key(media_id: &MediaId) -> BucketKey<MediaId, MediaUploadState> {
    media_id.into()
  }
}

pub fn schema() -> RootSchema {
  merge_schemas([schema_for!(MediaDownloadSpec),
                 schema_for!(MediaDownloadState),
                 schema_for!(MediaUploadSpec),
                 schema_for!(MediaUploadState)].into_iter())
}
