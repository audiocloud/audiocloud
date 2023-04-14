use std::collections::HashMap;
use std::convert::identity;
use std::path::PathBuf;

use chrono::Utc;
use futures::StreamExt;
use lazy_static::lazy_static;
use reqwest::Client;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::{select, spawn};
use tracing::{info, instrument};

use api::media::spec::{MediaDownloadSpec, MediaSpec, MediaUploadSpec};
use api::media::state::{MediaDownloadState, MediaUploadState};
use api::BucketKey;

use crate::nats::{Nats, WatchStream};

pub use super::Result;

lazy_static! {
  pub(crate) static ref HTTP_CLIENT: Client = Client::new();
}

mod download;
mod probe;
mod upload;

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
    info!("Created");

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
          self.media_upload_spec_changed(media_id, maybe_upload_spec);
        },
        Some(event) = self.rx_internal.recv() => {
          let _ = self.internal_event(event).await;
        }
      }
    }
  }

  async fn media_download_spec_changed(&mut self, media_id: String, maybe_download_spec: Option<MediaDownloadSpec>) {
    info!(media_id, spec = ?maybe_download_spec, "Download spec");
    match maybe_download_spec {
      | None => self.abort_download_if_exists(media_id),
      | Some(download) => self.create_or_update_download_if_not_completed(media_id, download).await,
    }
  }

  fn abort_download_if_exists(&mut self, media_id: String) {
    if let Some(download) = self.downloads.remove(&media_id) {
      if !download.task.is_finished() {
        download.task.abort();
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

    info!("existing spec for media {media_id}: {maybe_state:?}");

    if maybe_state.map(|state| &state.sha256 != &spec.sha256).unwrap_or(true) {
      self.create_or_update_download(media_id, spec);
    }
  }

  fn create_or_update_download(&mut self, id: String, spec: MediaDownloadSpec) {
    self.abort_download_if_exists(id.clone());
    let state = MediaDownloadState { updated_at: Utc::now(),
                                     progress:   0.0,
                                     done:       None,
                                     error:      None, };

    let task = spawn({
      let tx_internal = self.tx_internal.clone();
      let media_root = self.media_root.clone();
      let native_sample_rate = self.native_sample_rate;
      let spec = spec.clone();
      let id = id.clone();

      async move {
        let result = download::download_file(id.clone(), spec, media_root, native_sample_rate, tx_internal.clone()).await;
        let _ = tx_internal.send(InternalEvent::DownloadComplete { id, result }).await;
      }
    });

    self.downloads.insert(id, Download { spec, state, task });
  }

  fn media_upload_spec_changed(&mut self, media_id: String, maybe_upload_spec: Option<MediaUploadSpec>) {
    // TODO: ...
  }

  #[instrument(skip(self), err)]
  async fn internal_event(&mut self, event: InternalEvent) -> Result {
    info!("Internal event: {event:?}");

    match event {
      | InternalEvent::DownloadProgress { id, progress } =>
        if let Some(download) = self.downloads.get_mut(&id) {
          download.state.progress = progress;
          download.state.updated_at = Utc::now();

          let _ = self.nats
                      .media_download_state
                      .put(BucketKey::new(&id), download.state.clone())
                      .await?;
        },
      | InternalEvent::DownloadComplete { id, result } => match result {
        | Ok(spec) =>
          if let Some(mut download) = self.downloads.remove(&id) {
            download.state.done = Some(spec);
            download.state.error = None;
            download.state.updated_at = Utc::now();
            download.state.progress = 100.0;

            let _ = self.nats.media_download_state.put(BucketKey::new(&id), download.state).await?;
          },
        | Err(err) =>
          if let Some(mut download) = self.downloads.remove(&id) {
            download.state.done = None;
            download.state.error = Some(err.to_string());
            download.state.updated_at = Utc::now();
            download.state.progress = -100.0;

            let _ = self.nats.media_download_state.put(BucketKey::new(&id), download.state).await?;
          },
      },
    }

    Ok(())
  }
}

#[derive(Debug)]
pub enum InternalEvent {
  DownloadProgress { id: String, progress: f64 },
  DownloadComplete { id: String, result: Result<MediaSpec> },
}

struct Download {
  spec:  MediaDownloadSpec,
  state: MediaDownloadState,
  task:  JoinHandle<()>,
}

struct Upload {
  state: MediaUploadState,
  task:  JoinHandle<()>,
}
