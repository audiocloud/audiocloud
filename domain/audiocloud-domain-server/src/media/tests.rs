/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */


use std::path::PathBuf;
use std::time::Duration;

use actix::Actor;
use nanoid::nanoid;
use serde_json::json;
use tempfile::NamedTempFile;
use tokio::time::sleep;

use audiocloud_api::{
    now, AppId, AppMediaObjectId, DownloadFromDomain, MediaChannels, MediaDownload, MediaObjectId, MediaUpload, TrackMediaFormat,
    UploadToDomain,
};

use crate::db;
use crate::db::DataOpts;
use crate::media::download::Downloader;
use crate::media::upload::Uploader;
use crate::media::{DownloadJobId, UploadJobId};

#[actix::test]
async fn test_download_success() -> anyhow::Result<()> {
    let db = db::init(DataOpts::temporary()).await?;

    let job_id = DownloadJobId::new(nanoid!());

    let client = reqwest::Client::new();

    let source = PathBuf::from("../README.md");

    let media_id = AppMediaObjectId::new(AppId::admin(), MediaObjectId::new("object-1".to_owned()));

    // monitor at https://requestbin.com/r/en1205p765d7rp

    let settings = DownloadFromDomain { url:        "https://en1205p765d7rp.x.pipedream.net".to_string(),
                                        notify_url: Some("https://en1205p765d7rp.x.pipedream.net".to_string()),
                                        context:    Some(json!({"context": "is here"})), };

    let download_info = MediaDownload { media_id:   media_id.clone(),
                                        download:   settings,
                                        state:      Default::default(),
                                        created_at: now(), };

    let upload = Downloader::new(db.clone(), job_id, client, source, download_info)?;

    let addr = upload.start();

    for i in 0..1000 {
        sleep(Duration::from_millis(50)).await;
        if !addr.connected() {
            println!("Endded at loop {i}");
            break;
        }
    }

    // load state
    let maybe_file = db.fetch_media_by_id(&media_id).await?;

    // TODO: validate file

    Ok(())
}

#[actix::test]
async fn test_upload_success() -> anyhow::Result<()> {
    let db = db::init(DataOpts::temporary()).await?;

    let source_url = "http://speedtest.ftp.otenet.gr/files/test100k.db".to_owned();

    let client = reqwest::Client::new();

    let media_id = AppMediaObjectId::new(AppId::admin(), MediaObjectId::new("object-1".to_owned()));

    let job_id = UploadJobId::new(nanoid!());

    // monitor at https://requestbin.com/r/en1205p765d7rp

    let settings = UploadToDomain { channels:    { MediaChannels::Mono },
                                    format:      { TrackMediaFormat::Wave },
                                    seconds:     { 10.0 },
                                    sample_rate: { 44100 },
                                    bytes:       { 100_000 },
                                    url:         { source_url },
                                    notify_url:  { Some("https://en1205p765d7rp.x.pipedream.net".to_string()) },
                                    context:     { Some(json!({"context": "is here"})) }, };

    let upload_info = MediaUpload { media_id:   media_id.clone(),
                                    upload:     settings,
                                    state:      Default::default(),
                                    created_at: now(), };

    let temp_file = NamedTempFile::new()?;

    let upload = Uploader::new(db.clone(), job_id, client, temp_file.path().to_path_buf(), upload_info)?;

    let addr = upload.start();

    for i in 0..1000 {
        sleep(Duration::from_millis(50)).await;
        if !addr.connected() {
            println!("Endded at loop {i}");
            break;
        }
    }

    // load state
    let maybe_file = db.fetch_media_by_id(&media_id).await?;

    Ok(())
}
