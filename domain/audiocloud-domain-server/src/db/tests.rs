

use maplit::{hashset};
use nanoid::nanoid;
use serde_json::json;

use audiocloud_api::{
    now, AppId, AppMediaObjectId, DownloadFromDomain, MediaChannels, MediaDownload, MediaJobState, MediaMetadata, MediaObject,
    MediaObjectId, MediaUpload, Model, ModelId, TrackMediaFormat, UploadToDomain,
};

use crate::db::DataOpts;
use crate::media::{DownloadJobId, UploadJobId};

#[actix::test]
async fn test_media_create() -> anyhow::Result<()> {
    let db = super::init(DataOpts::temporary()).await?;

    let media_id = new_random_test_media_id();

    let media_metadata = test_media_metadata();

    let media = db.create_initial_media(&media_id, Some(media_metadata.clone()), None).await?;

    let loaded = db.fetch_media_by_id(&media_id).await?;

    assert_eq!(loaded.as_ref(), Some(&media));
    assert_eq!(loaded.as_ref().and_then(|media| media.metadata.as_ref()), Some(&media_metadata));
    assert_eq!(loaded.as_ref().and_then(|media| media.path.as_ref()), None);

    Ok(())
}

#[actix::test]
async fn test_create_download_job() -> anyhow::Result<()> {
    let db = super::init(DataOpts::temporary()).await?;

    let media_id = new_random_test_media_id();

    let job_id = new_random_download_job_id();

    let upload_settings = test_media_download_settings();

    let initial_state = not_completed_job_state();

    let download = MediaDownload { media_id:   { media_id.clone() },
                                   download:   { upload_settings },
                                   state:      { initial_state },
                                   created_at: { now() }, };

    db.create_initial_media(&media_id, None, None).await?;

    db.save_download_job(&job_id, &download).await?;

    let download_jobs = db.fetch_pending_download_jobs(1).await?;

    assert_eq!(download_jobs.len(), 1);
    assert_eq!(download_jobs.keys().next(), Some(&job_id));
    assert_eq!(download_jobs.values().next().map(|download| &download.state), Some(&download.state));
    assert_eq!(download_jobs.values().next().map(|download| &download.download),
               Some(&download.download));

    Ok(())
}

#[actix::test]
async fn test_create_upload_job() -> anyhow::Result<()> {
    let db = super::init(DataOpts::temporary()).await?;

    let media_id = new_random_test_media_id();

    let job_id = new_random_upload_job_id();

    let upload_settings = test_media_upload_settings();

    let initial_state = not_completed_job_state();

    let upload = MediaUpload { media_id:   { media_id.clone() },
                               upload:     { upload_settings.clone() },
                               state:      { initial_state },
                               created_at: { now() }, };

    db.create_initial_media(&media_id, None, None).await?;

    db.save_upload_job(&job_id, &upload).await?;

    db.debug_jobs().await?;

    let upload_jobs = db.fetch_pending_upload_jobs(1).await?;

    assert_eq!(upload_jobs.len(), 1);
    assert_eq!(upload_jobs.keys().next(), Some(&job_id));
    assert_eq!(upload_jobs.values().next().map(|upload| &upload.state), Some(&upload.state));
    assert_eq!(upload_jobs.values().next().map(|upload| &upload.upload), Some(&upload.upload));

    Ok(())
}

#[actix::test]
async fn test_sys_props() -> anyhow::Result<()> {
    let db = super::init(DataOpts::temporary()).await?;

    db.set_sys_prop("test", &"value".to_owned()).await?;
    let value: Option<String> = db.get_sys_prop("test").await?;

    assert_eq!(value, Some("value".to_owned()));

    Ok(())
}

#[actix::test]
async fn test_models_get_set() -> anyhow::Result<()> {
    let db = super::init(DataOpts::temporary()).await?;

    let a_id = ModelId { manufacturer: "distopik".to_owned(),
                         name:         "a".to_owned(), };
    let b_id = ModelId { manufacturer: "distopik".to_owned(),
                         name:         "b".to_owned(), };

    let model_a = Model { resources:    Default::default(),
                          inputs:       vec![],
                          outputs:      vec![],
                          parameters:   Default::default(),
                          reports:      Default::default(),
                          media:        false,
                          capabilities: Default::default(), };

    let model_b = Model { resources:    Default::default(),
                          inputs:       vec![],
                          outputs:      vec![],
                          parameters:   Default::default(),
                          reports:      Default::default(),
                          media:        true,
                          capabilities: Default::default(), };

    assert_eq!(db.get_model(&a_id).await?, None);

    db.set_model(&a_id, &model_a).await?;
    db.set_model(&b_id, &model_b).await?;

    assert_eq!(db.get_model(&a_id).await?, Some(model_a.clone()));
    assert_eq!(db.get_model(&b_id).await?, Some(model_b.clone()));

    db.delete_all_models_except(&hashset! { b_id.clone() }).await?;

    assert_eq!(db.get_model(&a_id).await?, None);
    assert_eq!(db.get_model(&b_id).await?, Some(model_b.clone()));

    Ok(())
}

fn test_media_object(media_id: &AppMediaObjectId, media_metadata: &MediaMetadata) -> MediaObject {
    MediaObject { id:        media_id.clone(),
                  metadata:  Some(media_metadata.clone()),
                  path:      Some(format!("random-path/{media_id}")),
                  last_used: None,
                  revision:  2, }
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
                    in_progress: false,
                    updated_at:  now(), }
}

fn new_random_test_media_id() -> AppMediaObjectId {
    AppMediaObjectId::new(AppId::test(), MediaObjectId::new(uuid::Uuid::new_v4().to_string()))
}

fn new_random_download_job_id() -> DownloadJobId {
    DownloadJobId::new(nanoid!())
}

fn new_random_upload_job_id() -> UploadJobId {
    UploadJobId::new(nanoid!())
}
