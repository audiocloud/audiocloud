use std::io;
use std::path::PathBuf;

use actix::{Actor, ActorContext, ActorFutureExt, Context, ContextFutureSpawner, WrapFuture};
use actix_broker::BrokerIssue;
use futures::TryStreamExt;
use reqwest::Client;
use serde_json::json;
use tokio::fs::File;
use tokio_util::io::StreamReader;
use tracing::*;

use audiocloud_api::common::media::MediaJobState;
use audiocloud_api::common::time::now;
use audiocloud_api::MediaUpload;

use crate::db::Db;
use crate::media::messages::NotifyUploadProgress;
use crate::media::UploadJobId;

#[derive(Debug)]
pub struct Uploader {
    db:          Db,
    job_id:      UploadJobId,
    upload:      MediaUpload,
    destination: PathBuf,
    client:      Client,
    state:       MediaJobState,
}

impl Uploader {
    pub fn new(db: Db, job_id: UploadJobId, client: Client, destination: PathBuf, upload: MediaUpload) -> anyhow::Result<Self> {
        let state = MediaJobState::default();

        Ok(Self { db,
                  job_id,
                  upload,
                  destination,
                  client,
                  state })
    }

    fn upload(&mut self, ctx: &mut Context<Self>) {
        let destination = self.destination.clone();
        let client = self.client.clone();
        let media_id = self.upload.media_id.clone();
        let upload = self.upload.clone();
        let db = self.db.clone();

        async move {
            if let Some(media) = db.fetch_media_by_id(&media_id).await? {
                match (media.path.as_ref(), media.metadata.as_ref()) {
                    (Some(path), Some(metadata)) => {
                        // TODO: more checks, for example media hash?

                        let fs_metadata_bytes = tokio::fs::metadata(path).await.map(|m| m.len()).unwrap_or_default();
                        let upload_bytes = upload.upload.bytes;
                        if metadata.bytes == upload_bytes && fs_metadata_bytes == upload_bytes {
                            debug!(%media_id, "Media already uploaded");
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }

            let mut file = File::create(&destination).await?;

            let stream = client.get(&upload.upload.url)
                               .send()
                               .await?
                               .bytes_stream()
                               .map_err(|err| io::Error::new(io::ErrorKind::Other, err));

            let mut stream = StreamReader::new(stream);

            tokio::io::copy(&mut stream, &mut file).await?;

            if let Some(notify_url) = upload.upload.notify_url {
                client.post(&notify_url)
                      .json(&json!({
                                "context": &upload.upload.context,
                                "media_id": &media_id,
                            }))
                      .send()
                      .await?;
            }

            Ok::<_, anyhow::Error>(())
        }.into_actor(self)
         .map(|res, actor, ctx| match res {
             Ok(_) => {
                 actor.state.error = None;
                 actor.state.in_progress = false;

                 actor.notify_supervisor();

                 ctx.stop();
             }
             Err(err) => {
                 warn!(%err, "upload failed");

                 actor.started(ctx);
             }
         })
         .spawn(ctx);
    }

    fn notify_supervisor(&mut self) {
        self.state.updated_at = now();
        self.issue_system_async(NotifyUploadProgress { job_id: self.job_id.clone(),
                                                       upload: self.upload.clone(), });
    }
}

impl Actor for Uploader {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        if self.state.retry > 5 {
            debug!("final failure");

            self.notify_supervisor();

            ctx.stop();
        } else {
            self.state.retry += 1;

            self.notify_supervisor();

            self.upload(ctx);
        }
    }
}
