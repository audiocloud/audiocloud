use std::path::PathBuf;
use std::sync::mpsc::Sender;

use api::media::spec::MediaUploadSpec;
use crate::media::service::InternalEvent;

use super::Result;

pub async fn upload_file(id: String, spec: MediaUploadSpec, media_root: PathBuf, sender: Sender<InternalEvent>) -> Result {
  let source_file = tokio::fs::File::open(media_root.join(&id)).await?;
  let mut request = super::HTTP_CLIENT.put(&spec.to_url).body(source_file).send().await?;

  while let Some(_) = request.chunk().await? {}

  Ok(())
}
