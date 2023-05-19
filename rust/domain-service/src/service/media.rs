use api::media::buckets::{media_download_spec_key, media_upload_spec_key, media_upload_state_key};
use api::media::spec::{MediaDownloadSpec, MediaId, MediaUploadSpec};
use api::media::state::{media_download_state_key, MediaDownloadState, MediaUploadState};
use api::BucketKey;

use crate::nats::WatchStream;

use super::{Result, Service};

impl Service {
  pub fn watch_all_media_download_specs(&self) -> WatchStream<MediaId, MediaDownloadSpec> {
    self.nats.media_download_spec.watch(BucketKey::all())
  }

  pub async fn set_media_download_spec(&self, media_id: &MediaId, spec: MediaDownloadSpec) -> Result {
    self.nats.media_download_spec.put(media_download_spec_key(&media_id), spec).await?;

    Ok(())
  }

  pub fn watch_media_download_state(&self, media_id: &MediaId) -> WatchStream<MediaId, MediaDownloadState> {
    self.nats.media_download_state.watch(media_download_state_key(&media_id))
  }

  pub async fn set_media_download_state(&self, media_id: &MediaId, state: MediaDownloadState) -> Result {
    self.nats
        .media_download_state
        .put(media_download_state_key(&media_id), state)
        .await?;

    Ok(())
  }

  pub fn watch_all_media_upload_specs(&self) -> WatchStream<MediaId, MediaUploadSpec> {
    self.nats.media_upload_spec.watch(BucketKey::all())
  }

  pub async fn get_media_upload_spec(&self, media_id: &MediaId) -> Result<Option<MediaUploadSpec>> {
    Ok(self.nats.media_upload_spec.get(media_upload_spec_key(&media_id)).await?)
  }

  pub async fn set_media_upload_spec(&self, media_id: &MediaId, spec: MediaUploadSpec) -> Result {
    self.nats.media_upload_spec.put(media_upload_spec_key(&media_id), spec).await?;

    Ok(())
  }

  pub fn watch_media_upload_state(&self, media_id: &MediaId) -> WatchStream<MediaId, MediaUploadState> {
    self.nats.media_upload_state.watch(media_upload_state_key(&media_id))
  }

  pub async fn set_media_upload_state(&self, media_id: &MediaId, state: MediaUploadState) -> Result {
    self.nats.media_upload_state.put(media_upload_state_key(&media_id), state).await?;

    Ok(())
  }

  pub async fn get_media_download_state(&self, media_id: &MediaId) -> Result<Option<MediaDownloadState>> {
    Ok(self.nats.media_download_state.get(media_download_state_key(&media_id)).await?)
  }
}
