use std::path::PathBuf;

use futures::TryStreamExt;
use reqwest::Body;
use tokio::sync::mpsc;
use tokio_util::io::ReaderStream;

use api::media::spec::MediaUploadSpec;

use crate::media::service::InternalEvent;

use super::Result;

pub async fn upload_file(media_id: String, spec: MediaUploadSpec, media_root: PathBuf, sender: mpsc::Sender<InternalEvent>) -> Result {
  let source_file = tokio::fs::File::open(media_root.join(&media_id)).await?;
  let size = source_file.metadata().await?.len();

  let mut read = 0;
  let mut progress = 0.0;

  let stream = ReaderStream::new(source_file);

  let stream = stream.inspect_ok(move |chunk| {
                       read += chunk.len();
                       let new_progress = (100.0 * read as f64 / size as f64).round();
                       if new_progress != progress {
                         let media_id = media_id.clone();
                         let event = InternalEvent::UploadProgress { media_id, progress };
                         let _ = sender.try_send(event);

                         progress = new_progress;
                       }
                     });

  let body = Body::wrap_stream(stream);

  let mut response = super::HTTP_CLIENT.put(&spec.to_url).body(body).send().await?;

  while let Some(_) = response.chunk().await? { /* no expected response chunks */ }

  Ok(())
}
