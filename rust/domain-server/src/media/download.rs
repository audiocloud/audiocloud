use std::path::PathBuf;
use std::process::Stdio;

use anyhow::anyhow;
use hex::ToHex;
use reqwest::Url;
use sha2::digest::FixedOutput;
use sha2::Digest;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::task::{block_in_place, spawn_blocking};

use api::media::spec::{MediaDownloadSpec, MediaId, MediaSpec};

use crate::media::probe;
use crate::media::service::InternalEvent;

use super::Result;

pub async fn download_file(id: MediaId,
                           spec: MediaDownloadSpec,
                           media_root: PathBuf,
                           native_sample_rate: u32,
                           sender: Sender<InternalEvent>)
                           -> Result<MediaSpec> {
  // create a temp file
  let mut temp_file = block_in_place(|| NamedTempFile::new())?;
  let temp_path = temp_file.path().to_owned();
  let mut out = tokio::fs::File::create(&temp_path).await?;
  let mut sha = sha2::Sha256::default();

  // download the file in chunks
  let parsed_url = Url::parse(&spec.from_url)?;

  let mut request = super::HTTP_CLIENT.get(parsed_url).send().await?;
  let mut progress = 0.0;
  let mut read = 0;

  while let Some(chunk) = request.chunk().await? {
    sha.update(&chunk);
    out.write_all(&chunk).await?;

    read += chunk.len();

    if spec.size != 0 {
      let new_progress = (read as f64 / spec.size as f64 * 100.0).round();
      if new_progress != progress {
        progress = new_progress;
        let id = id.clone();
        let _ = sender.send(InternalEvent::DownloadProgress { media_id: id, progress }).await;
      }
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

    let resampling_status = Command::new("ffmpeg").args(["-y",
                                                         "-i",
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
                                                  .stdin(Stdio::null())
                                                  .spawn()?
                                                  .wait()
                                                  .await?;

    if !resampling_status.success() {
      return Err(anyhow!("resampling failed with exit code: {}", resampling_status.code().unwrap_or(-1)));
    }

    temp_file = resampled_file;
  }

  // move the file to the final location
  let folder_path = id.to_folder_path(media_root.clone());

  if !folder_path.try_exists()? {
    tokio::fs::create_dir_all(&folder_path).await?;
  }

  let persistent_path = id.to_path(media_root.clone());

  spawn_blocking(move || temp_file.persist(&persistent_path)).await??;

  Ok(MediaSpec { id, sha256 })
}
