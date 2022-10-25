use std::path::PathBuf;

use actix::{Actor, ActorContext, ActorFutureExt, Context, ContextFutureSpawner, WrapFuture};
use actix_broker::BrokerIssue;
use futures::executor::block_on;
use reqwest::Client;
use serde_json::json;
use tokio::fs::File;
use tracing::*;

use audiocloud_api::common::time::now;
use audiocloud_api::MediaDownload;

use crate::db::Db;
use crate::media::messages::NotifyDownloadProgress;
use crate::media::DownloadJobId;

#[derive(Debug)]
pub struct Downloader {
    db:       Db,
    job_id:   DownloadJobId,
    download: MediaDownload,
    source:   PathBuf,
    client:   Client,
}

impl Downloader {
    pub fn new(db: Db, job_id: DownloadJobId, client: Client, source: PathBuf, download: MediaDownload) -> anyhow::Result<Self> {
        Ok(Self { db,
                  job_id,
                  download,
                  source,
                  client })
    }

    #[instrument(skip_all)]
    fn download(&mut self, ctx: &mut Context<Self>) {
        let source = self.source.clone();
        let download = self.download.clone();
        let client = self.client.clone();
        let media_id = self.download.media_id.clone();
        let db = self.db.clone();

        debug!(?source, ?download, %media_id, "starting download");

        async move {
            client.put(&download.download.url).body(File::open(&source).await?).send().await?;

            if let Some(notify_url) = &download.download.notify_url {
                client.post(notify_url)
                      .json(&json!({
                                "context": &download.download.context,
                                "id": &media_id,
                            }))
                      .send()
                      .await?;
            }

            Ok::<_, anyhow::Error>(())
        }.into_actor(self)
         .map(|res, actor, ctx| match res {
             Ok(_) => {
                 actor.download.state.error = None;
                 actor.download.state.in_progress = false;

                 block_on(actor.save_and_notify());

                 ctx.stop();
             }
             Err(err) => {
                 warn!(%err, "download failed");

                 actor.download.state.error = Some(err.to_string());

                 block_on(actor.save_and_notify());

                 actor.started(ctx);
             }
         })
         .spawn(ctx);
    }

    async fn save_and_notify(&mut self) {
        self.download.state.updated_at = now();
        let _ = self.db.save_download_job(&self.job_id, &self.download).await;
        self.issue_system_async(NotifyDownloadProgress { job_id:   self.job_id.clone(),
                                                         download: self.download.clone(), });
    }
}

impl Actor for Downloader {
    type Context = Context<Self>;

    #[instrument(skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        self.download.state.progress = 0.0;

        if self.download.state.retry > 5 {
            warn!("final failure");

            self.download.state.in_progress = false;

            block_on(self.save_and_notify());

            ctx.stop();
        } else {
            self.download.state.retry += 1;

            block_on(self.save_and_notify());

            self.download(ctx);
        }
    }
}
