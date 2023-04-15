use std::collections::HashMap;
use std::convert::identity;
use std::path::PathBuf;
use std::time::Duration;

use chrono::Utc;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio::{select, spawn};
use tracing::{debug, info, instrument, warn};

use api::media::spec::{MediaDownloadSpec, MediaSpec, MediaUploadSpec};
use api::media::state::{MediaDownloadState, MediaUploadState};
use api::BucketKey;

use crate::media::{download, upload};
use crate::nats::{Nats, WatchStream};

use super::Result;

pub struct MediaService {
  nats:                 Nats,
  media_root:           PathBuf,
  native_sample_rate:   u32,
  watch_download_specs: WatchStream<MediaDownloadSpec>,
  watch_upload_specs:   WatchStream<MediaUploadSpec>,
  downloads:            HashMap<String, Download>,
  uploads:              HashMap<String, Upload>,
  tx_internal:          mpsc::Sender<InternalEvent>,
  rx_internal:          mpsc::Receiver<InternalEvent>,
}

impl MediaService {
  pub fn new(nats: Nats, media_root: PathBuf, native_sample_rate: u32) -> Self {
    let watch_download_specs = nats.media_download_spec.watch_all();
    let watch_upload_specs = nats.media_upload_spec.watch_all();

    let downloads = HashMap::new();
    let uploads = HashMap::new();

    let (tx_internal, rx_internal) = mpsc::channel(0xff);

    Self { nats,
           media_root,
           native_sample_rate,
           watch_download_specs,
           watch_upload_specs,
           downloads,
           uploads,
           tx_internal,
           rx_internal }
  }

  pub async fn run(mut self) {
    loop {
      select! {
        Some((media_id, maybe_download_spec)) = self.watch_download_specs.next() => {
          self.media_download_spec_changed(media_id, maybe_download_spec).await;
        },
        Some((media_id, maybe_upload_spec)) = self.watch_upload_specs.next() => {
          self.media_upload_spec_changed(media_id, maybe_upload_spec).await;
        },
        Some(event) = self.rx_internal.recv() => {
          let _ = self.internal_event(event).await;
        }
      }
    }
  }

  async fn media_download_spec_changed(&mut self, media_id: String, maybe_download_spec: Option<MediaDownloadSpec>) {
    match maybe_download_spec {
      | None => self.abort_download_if_exists(media_id).await,
      | Some(download) => self.create_or_update_download_if_not_completed(media_id, download).await,
    }
  }

  async fn abort_download_if_exists(&mut self, media_id: String) {
    if let Some(download) = self.downloads.remove(&media_id) {
      if !download.task.is_finished() {
        warn!(media_id, "Aborting download task");
        download.task.abort();
        let _ = timeout(Duration::from_millis(150), download.task).await;
      }
    }
  }

  async fn create_or_update_download_if_not_completed(&mut self, media_id: String, spec: MediaDownloadSpec) {
    let maybe_state = self.nats
                          .media_download_state
                          .get(BucketKey::new(&media_id))
                          .await
                          .ok()
                          .and_then(identity)
                          .and_then(|state| state.done);

    if maybe_state.map(|state| &state.sha256 != &spec.sha256).unwrap_or(true) {
      self.create_or_update_download(media_id, spec).await;
    } else {
      debug!(media_id, "Download already completed");
    }
  }

  async fn create_or_update_download(&mut self, media_id: String, spec: MediaDownloadSpec) {
    self.abort_download_if_exists(media_id.clone()).await;

    let state = MediaDownloadState::default();

    let task = spawn({
      let tx_internal = self.tx_internal.clone();
      let media_root = self.media_root.clone();
      let native_sample_rate = self.native_sample_rate;
      let media_id = media_id.clone();

      async move {
        let result = download::download_file(media_id.clone(), spec, media_root, native_sample_rate, tx_internal.clone()).await;
        let _ = tx_internal.send(InternalEvent::DownloadComplete { media_id, result }).await;
      }
    });

    self.downloads.insert(media_id, Download { state, task });
  }

  async fn media_upload_spec_changed(&mut self, media_id: String, maybe_upload_spec: Option<MediaUploadSpec>) {
    match maybe_upload_spec {
      | None => self.abort_upload_if_exists(media_id).await,
      | Some(upload) => self.create_or_update_upload_if_not_completed(media_id, upload).await,
    }
  }

  async fn abort_upload_if_exists(&mut self, media_id: String) {
    if let Some(upload) = self.uploads.remove(&media_id) {
      if !upload.task.is_finished() {
        warn!(media_id, "Aborting upload task");
        upload.task.abort();
        let _ = timeout(Duration::from_millis(150), upload.task).await;
      }
    }
  }

  async fn create_or_update_upload_if_not_completed(&mut self, media_id: String, spec: MediaUploadSpec) {
    let is_url_new = self.nats
                         .media_upload_spec
                         .get(BucketKey::new(&media_id))
                         .await
                         .ok()
                         .and_then(identity)
                         .map(|bucket_spec| &bucket_spec.to_url == &spec.to_url)
                         .unwrap_or(false);

    if !is_url_new {
      self.create_or_update_upload(media_id, spec).await;
    } else {
      debug!(media_id, "Upload already completed");
    }
  }

  async fn create_or_update_upload(&mut self, media_id: String, spec: MediaUploadSpec) {
    self.abort_upload_if_exists(media_id.clone()).await;

    let state = MediaUploadState::default();

    let task = spawn({
      let tx_internal = self.tx_internal.clone();
      let media_root = self.media_root.clone();
      let media_id = media_id.clone();

      async move {
        let result = upload::upload_file(media_id.clone(), spec, media_root, tx_internal.clone()).await;
        let _ = tx_internal.send(InternalEvent::UploadComplete { media_id, result }).await;
      }
    });

    self.uploads.insert(media_id, Upload { state, task });
  }

  #[instrument(skip(self), err)]
  async fn internal_event(&mut self, event: InternalEvent) -> Result {
    info!("Internal event: {event:?}");

    match event {
      | InternalEvent::DownloadProgress { media_id, progress } =>
        if let Some(download) = self.downloads.get_mut(&media_id) {
          download.state.progress = progress;
          download.state.updated_at = Utc::now();

          let _ = self.nats
                      .media_download_state
                      .put(BucketKey::new(&media_id), download.state.clone())
                      .await?;
        },
      | InternalEvent::DownloadComplete { media_id, result } => match result {
        | Ok(spec) =>
          if let Some(mut download) = self.downloads.remove(&media_id) {
            download.state.done = Some(spec);
            download.state.error = None;
            download.state.updated_at = Utc::now();
            download.state.progress = 100.0;

            info!(media_id, "Download completed");

            let _ = self.nats
                        .media_download_state
                        .put(BucketKey::new(&media_id), download.state)
                        .await?;
          },
        | Err(err) =>
          if let Some(mut download) = self.downloads.remove(&media_id) {
            download.state.done = None;
            download.state.error = Some(err.to_string());
            download.state.updated_at = Utc::now();
            download.state.progress = -100.0;

            info!(media_id, ?err, "Download failed: {err}");

            let _ = self.nats
                        .media_download_state
                        .put(BucketKey::new(&media_id), download.state)
                        .await?;
          },
      },
      | InternalEvent::UploadProgress { media_id, progress } =>
        if let Some(upload) = self.uploads.get_mut(&media_id) {
          upload.state.progress = progress;
          upload.state.updated_at = Utc::now();

          let _ = self.nats
                      .media_upload_state
                      .put(BucketKey::new(&media_id), upload.state.clone())
                      .await?;
        },
      | InternalEvent::UploadComplete { media_id, result } => match result {
        | Ok(_) =>
          if let Some(mut upload) = self.uploads.remove(&media_id) {
            upload.state.uploaded = true;
            upload.state.error = None;
            upload.state.updated_at = Utc::now();
            upload.state.progress = 100.0;

            info!(media_id, "Upload completed");

            let _ = self.nats.media_upload_state.put(BucketKey::new(&media_id), upload.state).await?;
          },
        | Err(err) =>
          if let Some(mut upload) = self.uploads.remove(&media_id) {
            upload.state.uploaded = false;
            upload.state.error = Some(err.to_string());
            upload.state.updated_at = Utc::now();
            upload.state.progress = -100.0;

            info!(media_id, ?err, "Upload failed: {err}");

            let _ = self.nats.media_upload_state.put(BucketKey::new(&media_id), upload.state).await?;
          },
      },
    }

    Ok(())
  }
}

#[derive(Debug)]
pub enum InternalEvent {
  DownloadProgress { media_id: String, progress: f64 },
  DownloadComplete { media_id: String, result: Result<MediaSpec> },
  UploadProgress { media_id: String, progress: f64 },
  UploadComplete { media_id: String, result: Result },
}

struct Download {
  state: MediaDownloadState,
  task:  JoinHandle<()>,
}

struct Upload {
  state: MediaUploadState,
  task:  JoinHandle<()>,
}
