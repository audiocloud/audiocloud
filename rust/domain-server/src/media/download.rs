use std::path::PathBuf;

use anyhow::anyhow;
use hex::ToHex;
use lazy_static::lazy_static;
use reqwest::Client;
use sha2::digest::FixedOutput;
use sha2::Digest;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::task::spawn_blocking;

use api::media::spec::{MediaDownloadSpec, MediaSpec};

use crate::media::{probe, InternalEvent};

use super::Result;

lazy_static! {
  static ref HTTP_CLIENT: Client = Client::new();
}

pub async fn download_file(id: String,
                           spec: MediaDownloadSpec,
                           media_root: PathBuf,
                           native_sample_rate: u32,
                           sender: Sender<InternalEvent>)
                           -> Result<MediaSpec> {
  // create a temp file
  let mut temp_file = NamedTempFile::new()?;
  let temp_path = temp_file.path().to_owned();
  let mut out = tokio::fs::File::open(&temp_path).await?;
  let mut sha = sha2::Sha256::default();

  // download the file in chunks
  let mut request = HTTP_CLIENT.get(&spec.from_url).send().await?;
  let mut progress = 0.0;
  let mut read = 0;

  while let Some(chunk) = request.chunk().await? {
    sha.update(&chunk);
    out.write_all(&chunk).await?;

    read += chunk.len();
    let new_progress = (read as f32 / spec.size as f32 * 100.0).round();
    if new_progress != progress {
      progress = new_progress;
      let id = id.clone();
      let _ = sender.send(InternalEvent::DownloadProgress { id, progress }).await;
    }
  }

  // check hash
  let sha256 = sha.finalize_fixed().encode_hex::<String>();
  if &sha256 != &spec.sha256 {
    return Err(anyhow!("hash mismatch: got {} != expected {}, download is faulty or hash is wrong",
                       sha256,
                       spec.sha256));
  }

  // convert the file to the native sample rate if needed
  let sample_rate = probe::get_sample_rate(&temp_path).await?;

  if sample_rate != native_sample_rate {
    let resampled_file = NamedTempFile::new()?;
    let resampled_path = resampled_file.path().to_owned();

    let resampling_status = Command::new("ffmpeg").args(["-i",
                                                         temp_path.to_str().unwrap(),
                                                         "-af",
                                                         "aresample=resampler=soxr",
                                                         "-precision",
                                                         "33",
                                                         "-ar",
                                                         native_sample_rate.to_string().as_str(),
                                                         "-f",
                                                         "wav",
                                                         resampled_path.to_str().unwrap()].into_iter())
                                                  .kill_on_drop(true)
                                                  .spawn()?
                                                  .wait()
                                                  .await?;

    if !resampling_status.success() {
      return Err(anyhow!("resampling failed with exit code: {}", resampling_status.code().unwrap_or(-1)));
    }

    temp_file = resampled_file;
  }

  // move the file to the final location
  let persistent_path = media_root.join(&id);
  spawn_blocking(move || temp_file.persist(&persistent_path)).await??;

  Ok(MediaSpec { id, sha256 })
}
