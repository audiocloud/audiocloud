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

use crate::media::download;
use crate::nats::{Nats, WatchStream};

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
          self.media_upload_spec_changed(media_id, maybe_upload_spec);
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

    let state = MediaDownloadState { updated_at: Utc::now(),
                                     progress:   0.0,
                                     done:       None,
                                     error:      None, };

    let task = spawn({
      let tx_internal = self.tx_internal.clone();
      let media_root = self.media_root.clone();
      let native_sample_rate = self.native_sample_rate;
      let spec = spec.clone();
      let media_id = media_id.clone();

      async move {
        let result = download::download_file(media_id.clone(), spec, media_root, native_sample_rate, tx_internal.clone()).await;
        let _ = tx_internal.send(InternalEvent::DownloadComplete { media_id, result }).await;
      }
    });

    self.downloads.insert(media_id, Download { spec, state, task });
  }

  fn media_upload_spec_changed(&mut self, media_id: String, maybe_upload_spec: Option<MediaUploadSpec>) {
    // TODO: ...
  }

  #[instrument(skip(self), err)]
  async fn internal_event(&mut self, event: InternalEvent) -> crate::Result {
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
    }

    Ok(())
  }
}

#[derive(Debug)]
pub enum InternalEvent {
  DownloadProgress {
    media_id: String,
    progress: f64,
  },
  DownloadComplete {
    media_id: String,
    result:   crate::Result<MediaSpec>,
  },
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
