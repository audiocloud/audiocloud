use std::collections::HashSet;

use maplit::hashmap;
use serde_json::json;

use audiocloud_api::{
    now, AppId, AppMediaObjectId, DownloadFromDomain, MediaChannels, MediaDownload, MediaJobState, MediaMetadata, MediaObject,
    MediaObjectId, MediaUpload, TrackMediaFormat, UploadToDomain,
};

use crate::db::DataOpts;
use crate::media::{DownloadJobId, UploadJobId};

#[actix::test]
async fn test_migrations() -> anyhow::Result<()> {
    let db = super::init(DataOpts::memory()).await?;
    let mut conn = db.pool.acquire().await?;
    let res = sqlx::query!("SELECT name FROM sqlite_master WHERE type='table'").fetch_all(&mut conn)
                                                                               .await?;
    assert_eq!(res.len(), 5);
    let set = res.into_iter().filter_map(|r| r.name).collect::<HashSet<_>>();

    assert_eq!(set,
               ["_sqlx_migrations", "media_object", "sys_props", "model", "media_job"].into_iter()
                                                                                      .map(String::from)
                                                                                      .collect());

    Ok(())
}

#[actix::test]
async fn test_media_create() -> anyhow::Result<()> {
    let db = super::init(DataOpts::memory()).await?;

    let media_id = new_random_test_media_id();

    let media_metadata = test_media_metadata();

    let media = test_media_object(&media_id, &media_metadata);

    db.save_media(media.clone()).await?;

    let loaded = db.fetch_media_by_id(&media_id).await?;

    assert_eq!(loaded.as_ref(), Some(&media));

    Ok(())
}

#[actix::test]
async fn test_create_download_job() -> anyhow::Result<()> {
    let db = super::init(DataOpts::memory()).await?;

    let media_id = new_random_test_media_id();

    let job_id = new_random_download_job_id();

    let upload_settings = test_media_download_settings();

    let initial_state = not_completed_job_state();

    let download = MediaDownload { media_id: media_id.clone(),
                                   download: upload_settings,
                                   state:    initial_state, };

    let media = MediaObject { id:       media_id.clone(),
                              metadata: None,
                              path:     None,
                              download: None,
                              upload:   None,
                              revision: 0, };

    db.save_media(media).await?;

    db.save_download_job(&job_id, &download).await?;

    let download_jobs = db.fetch_pending_download_jobs(1).await?;

    assert_eq!(download_jobs, hashmap! { job_id.clone() => download.clone()});

    Ok(())
}

#[actix::test]
async fn test_create_upload_job() -> anyhow::Result<()> {
    let db = super::init(DataOpts::memory()).await?;

    let media_id = new_random_test_media_id();

    let job_id = new_random_upload_job_id();

    let upload_settings = test_media_upload_settings();

    let initial_state = not_completed_job_state();

    let upload = MediaUpload { media_id: media_id.clone(),
                               upload:   upload_settings.clone(),
                               state:    initial_state, };

    let media = MediaObject { id:       media_id.clone(),
                              metadata: None,
                              path:     None,
                              download: None,
                              upload:   None,
                              revision: 0, };

    db.save_media(media).await?;

    db.save_upload_job(&job_id, &upload).await?;

    let upload_jobs = db.fetch_pending_upload_jobs(1).await?;

    assert_eq!(upload_jobs, hashmap! { job_id.clone() => upload.clone()});

    Ok(())
}

fn test_media_object(media_id: &AppMediaObjectId, media_metadata: &MediaMetadata) -> MediaObject {
    MediaObject { id:       media_id.clone(),
                  metadata: Some(media_metadata.clone()),
                  path:     Some(format!("random-path/{media_id}")),
                  download: None,
                  upload:   None,
                  revision: 2, }
}

fn test_media_metadata() -> MediaMetadata {
    MediaMetadata { channels:    MediaChannels::Mono,
                    format:      TrackMediaFormat::Wave,
                    seconds:     69.0,
                    sample_rate: 44_100,
                    bytes:       5294831, }
}

fn test_media_download_settings() -> DownloadFromDomain {
    DownloadFromDomain { url:        "http://test.local/file.wav".to_string(),
                         notify_url: Some("http://test.local/api/notify".to_string()),
                         context:    Some(json!({"test_int": 123, "test_bool": true})), }
}

fn test_media_upload_settings() -> UploadToDomain {
    UploadToDomain { channels:    MediaChannels::Mono,
                     format:      TrackMediaFormat::Wave,
                     seconds:     10.0,
                     sample_rate: 44_1000,
                     bytes:       234582,
                     url:         "http://test.local/file.wav".to_string(),
                     notify_url:  Some("http://test.local/api/notify".to_string()),
                     context:     Some(json!({"test_int": 123, "test_bool": true})), }
}

fn not_completed_job_state() -> MediaJobState {
    MediaJobState { progress:    0.0,
                    retry:       1,
                    error:       None,
                    in_progress: true,
                    updated_at:  now(), }
}

fn new_random_test_media_id() -> AppMediaObjectId {
    AppMediaObjectId::new(AppId::test(), MediaObjectId::new(uuid::Uuid::new_v4().to_string()))
}

fn new_random_download_job_id() -> DownloadJobId {
    DownloadJobId::new()
}

fn new_random_upload_job_id() -> UploadJobId {
    UploadJobId::new()
}
