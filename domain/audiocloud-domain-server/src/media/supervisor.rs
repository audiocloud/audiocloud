/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use actix::{Actor, Addr, AsyncContext, Context};
use futures::executor::block_on;
use reqwest::Client;
use tracing::*;

use audiocloud_api::{AppMediaObjectId, MediaUpload};

use crate::db::Db;
use crate::media::download::Downloader;
use crate::media::upload::Uploader;
use crate::media::{DownloadJobId, MediaOpts, UploadJobId};

pub struct MediaSupervisor {
    db:         Db,
    downloads:  HashMap<DownloadJobId, Addr<Downloader>>,
    uploads:    HashMap<UploadJobId, Addr<Uploader>>,
    client:     Client,
    opts:       MediaOpts,
    media_root: PathBuf,
}

impl MediaSupervisor {
    pub fn new(opts: MediaOpts, db: Db) -> anyhow::Result<Self> {
        let media_root = opts.media_root.clone();
        Ok(Self { db:         { db },
                  opts:       { opts },
                  downloads:  { Default::default() },
                  uploads:    { Default::default() },
                  client:     { Default::default() },
                  media_root: { media_root }, })
    }

    #[instrument(skip_all)]
    fn load_pending_downloads(&mut self, ctx: &mut Context<Self>) {
        match block_on(self.db.fetch_pending_download_jobs(self.opts.max_downloads_batch)) {
            Ok(downloads) => {
                let mut created = 0;
                for (id, download) in downloads {
                    if self.downloads.contains_key(&id) {
                        continue;
                    }

                    created += 1;

                    match Downloader::new(self.db.clone(), id.clone(), self.client.clone(), self.media_root.clone(), download) {
                        Ok(downloader) => {
                            self.downloads.insert(id, downloader.start());
                        }
                        Err(error) => {
                            warn!( % error, % id, "Failed to start downloader");
                        }
                    }
                }

                if created > 0 {
                    info!(%created, "Created new downloaders");
                }
            }
            Err(error) => {
                error!(%error, "Failed to load pending downloads");
            }
        }
    }

    #[instrument(skip_all)]
    fn load_pending_uploads(&mut self, ctx: &mut Context<Self>) {
        let rv = block_on(self.db.fetch_pending_download_jobs(self.opts.max_uploads_batch));
    }

    #[instrument(skip_all)]
    fn process_pending_uploads(uploads: anyhow::Result<HashMap<UploadJobId, MediaUpload>>, actor: &mut Self, ctx: &mut Context<Self>) {
        match uploads {
            Ok(uploads) => {
                let created = 0;
                for (id, upload) in uploads {
                    if actor.uploads.contains_key(&id) {
                        continue;
                    }

                    let path = actor.get_local_path(&upload.media_id);

                    match Uploader::new(actor.db.clone(), id.clone(), actor.client.clone(), actor.media_root.clone(), upload) {
                        Ok(uploader) => {
                            actor.uploads.insert(id, uploader.start());
                        }
                        Err(error) => {
                            warn!(%error, %id, "Failed to start uploader");
                        }
                    }
                }

                if created > 0 {
                    info!(%created, "Created new uploaders");
                }
            }
            Err(error) => {
                error!(%error, "Failed to load pending uploads");
            }
        }
    }

    fn get_local_path(&self, path: &AppMediaObjectId) -> PathBuf {
        let mut rv = self.media_root.clone();
        rv.push(path.app_id.as_str());
        rv.push(path.media_id.as_str());

        rv
    }

    #[instrument(skip_all)]
    fn update(&mut self, ctx: &mut Context<Self>) {
        self.clear_stale_downloads(ctx);
        self.clear_stale_uploads(ctx);

        self.load_pending_uploads(ctx);
        self.load_pending_downloads(ctx);
    }

    fn clear_stale_uploads(&mut self, ctx: &mut Context<Self>) {
        self.uploads.retain(|id, uploader| {
                        if uploader.connected() {
                            true
                        } else {
                            warn!(%id, "Uploader dropped");
                            if let Err(error) = block_on(self.db.delete_upload(&id)) {
                                error!(%error, %id, "Failed to delete upload");
                            }
                            false
                        }
                    });
    }

    fn clear_stale_downloads(&mut self, ctx: &mut Context<Self>) {
        self.downloads.retain(|id, downloader| {
                          if downloader.connected() {
                              true
                          } else {
                              warn!(%id, "Downloader dropped");

                              if let Err(error) = block_on(self.db.delete_download(&id)) {
                                  error!(%error, %id, "Failed to delete download");
                              }

                              false
                          }
                      });
    }
}

impl Actor for MediaSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1), Self::update);
    }
}
