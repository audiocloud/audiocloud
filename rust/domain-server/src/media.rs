use futures::StreamExt;
use tokio::select;
use tokio::sync::mpsc;

use api::media::spec::MediaDownloadSpec;

use crate::nats::{Nats, WatchStream};

pub struct MediaService {
  nats:                 Nats,
  native_sample_rate:   u32,
  watch_download_specs: WatchStream<MediaDownloadSpec>,
  tx_internal:          mpsc::Sender<InternalEvent>,
  rx_internal:          mpsc::Receiver<InternalEvent>,
}

enum InternalEvent {}

impl MediaService {
  pub fn new(nats: Nats, native_sample_rate: u32) -> Self {
    let watch_download_specs = nats.media_download_spec.watch_all();

    Self { nats,
           native_sample_rate,
           watch_download_specs }
  }

  pub async fn run(mut self) {
    loop {
      select! {
        Some((media_id, maybe_download_spec)) = self.watch_download_specs.next() => {
          self.media_download_spec_changed(media_id, maybe_download_spec);
        },
        Some(event) = self.rx_internal.recv() => {
          self.internal_event(event);
        }
      }
    }
  }

  fn media_download_spec_changed(&mut self, media_id: String, maybe_download_spec: Option<MediaDownloadSpec>) {
    // TODO: ...
  }

  fn internal_event(&mut self, event: InternalEvent) {
    // TODO: ...
  }
}
