use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

use api::media::spec::{MediaDownloadSpec, MediaUploadSpec};
use api::media::state::{MediaDownloadState, MediaUploadState};

use crate::{Db, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaData {
  pub id:       Thing,
  pub app:      String,
  pub sha256:   String,
  pub size:     u64,
  pub present:  bool,
  pub revision: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaDownloadTaskData {
  pub id:       Thing,
  pub spec:     MediaDownloadSpec,
  pub state:    MediaDownloadState,
  pub media:    String,
  pub revision: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaUploadTaskData {
  pub id:       Thing,
  pub spec:     MediaUploadSpec,
  pub state:    MediaUploadState,
  pub media:    String,
  pub revision: u64,
}

impl Db {
  pub async fn get_media_by_id(&self, id: &str) -> Result<Option<MediaData>> {
    Ok(self.db.select(("media", id)).await?)
  }

  pub async fn create_or_replace_media(&self, id: &str, app: &str, sha256: &str, size: u64, present: bool) -> Result<MediaData> {
    let received: MediaData = self.db
                                  .update(("media", id))
                                  .merge(json!({
                                           "app": app,
                                           "sha256": sha256,
                                           "size": size,
                                           "present": present,
                                           "revision": 0
                                         }))
                                  .await?;

    Ok(received)
  }

  pub async fn list_media(&self, app: Option<&str>, search: Option<&str>, offset: usize, limit: usize) -> Result<Vec<MediaData>> {
    let mut query = "SELECT * from media WHERE ".to_string();
    let mut conds = vec!["1=1"];

    if app.is_some() {
      conds.push("app = $app");
    }

    if search.is_some() {
      conds.push("id CONTAINS $search");
    }

    query.push_str(conds.join(" AND ").as_str());

    query.push_str(" ORDER BY id LIMIT $limit START $offset");

    Ok(self.db
           .query(query)
           .bind(("offset", offset))
           .bind(("limit", limit))
           .bind(("app", app))
           .bind(("search", search))
           .await?
           .take(0)?)
  }

  pub async fn create_download(&self, media_id: &str, spec: MediaDownloadSpec, state: MediaDownloadState) -> Result<MediaDownloadTaskData> {
    let media_id = Thing::from(("media", media_id));

    Ok(self.db
           .create("media_download_task")
           .content(json!({
                      "spec": spec,
                      "state": state,
                      "media": media_id,
                      "revision": 0
                    }))
           .await?)
  }

  pub async fn create_upload(&self, media_id: &str, spec: MediaUploadSpec, state: MediaUploadState) -> Result<MediaUploadTaskData> {
    let media_id = Thing::from(("media", media_id));

    Ok(self.db
           .create("media_upload_task")
           .content(json!({
                      "spec": spec,
                      "state": state,
                      "media": media_id,
                      "revision": 0
                    }))
           .await?)
  }

  pub async fn list_media_downloads(&self,
                                    state_done: Option<bool>,
                                    media_id: Option<&str>,
                                    offset: usize,
                                    limit: usize)
                                    -> Result<Vec<MediaDownloadTaskData>> {
    let mut query = "SELECT * from media_download_task WHERE ".to_string();
    let mut conds = vec!["1=1"];

    let media_id = media_id.map(|id| Thing::from(("media", id)));

    if state_done.is_some() {
      conds.push("state.done = $state_done");
    }

    if media_id.is_some() {
      conds.push("media = $media_id");
    }

    query.push_str(conds.join(" AND ").as_str());
    query.push_str(" ORDER BY id LIMIT $limit START $offset");

    Ok(self.db
           .query(query)
           .bind(("state_done", state_done))
           .bind(("offset", offset))
           .bind(("limit", limit))
           .bind(("media_id", media_id))
           .await?
           .take(0)?)
  }

  pub async fn list_media_uploads(&self,
                                  state_done: Option<bool>,
                                  media_id: Option<&str>,
                                  offset: usize,
                                  limit: usize)
                                  -> Result<Vec<MediaUploadTaskData>> {
    let mut query = "SELECT * from media_upload_task WHERE ".to_string();
    let mut conds = vec!["1=1"];

    let media_id = media_id.map(|id| Thing::from(("media", id)));

    if state_done.is_some() {
      conds.push("state.done = $state_done");
    }

    if media_id.is_some() {
      conds.push("media = $media_id");
    }

    query.push_str(conds.join(" AND ").as_str());
    query.push_str(" ORDER BY id LIMIT $limit START $offset");

    Ok(self.db
           .query(query)
           .bind(("state_done", state_done))
           .bind(("offset", offset))
           .bind(("limit", limit))
           .bind(("media_id", media_id))
           .await?
           .take(0)?)
  }

  pub async fn set_media_download_state(&self, id: &str, state: MediaDownloadState) -> Result<Option<MediaDownloadTaskData>> {
    Ok(self.db
           .query("UPDATE $media_download_task SET state = $state, revision = revision + 1")
           .bind(("media_download_task", Thing::from(("media_download_task", id))))
           .bind(("state", state))
           .await?
           .take(0)?)
  }

  pub async fn set_media_upload_state(&self, id: &str, state: MediaUploadState) -> Result<Option<MediaUploadTaskData>> {
    Ok(self.db
           .query("UPDATE $media_upload_task SET state = $state, revision = revision + 1")
           .bind(("media_upload_task", Thing::from(("media_upload_task", id))))
           .bind(("state", state))
           .await?
           .take(0)?)
  }
}

#[cfg(test)]
mod tests {
  use surrealdb::sql::Id;

  use super::*;

  #[tokio::test]
  async fn sanity() -> Result {
    let db = Db::new_in_mem().await?;

    let media = db.get_media_by_id("1").await?;
    assert!(media.is_none(), "media 1 should not exist after database init");

    let list = db.list_media(None, None, 0, 10).await?;
    assert_eq!(list.len(), 0, "media list should be empty after database init");

    Ok(())
  }

  #[tokio::test]
  async fn test_create_list() -> Result {
    let db = Db::new_in_mem().await?;

    let media = db.create_or_replace_media("1", "test", "sha256", 100, true).await?;
    assert_eq!(media.id.id, Id::from("1"), "media id should be 1");
    assert_eq!(media.app, "test", "media app should be test");
    assert_eq!(media.sha256, "sha256", "media sha256 should be sha256");
    assert_eq!(media.size, 100, "media size should be 100");
    assert_eq!(media.present, true, "media present should be true");

    let list = db.list_media(None, None, 0, 10).await?;
    assert_eq!(list.len(), 1, "media list should have 1 item");
    assert_eq!(list[0].id.id, Id::from("1"), "media list item 1 id should be 1");
    assert_eq!(list[0].app, "test", "media list item 1 app should be test");
    assert_eq!(list[0].sha256, "sha256", "media list item 1 sha256 should be sha256");
    assert_eq!(list[0].size, 100, "media list item 1 size should be 100");
    assert_eq!(list[0].present, true, "media list item 1 present should be true");

    let list = db.list_media(Some("other"), None, 0, 10).await?;
    assert_eq!(list.len(), 0, "media list should have 0 items when searching for app other");

    let list = db.list_media(Some("test"), None, 0, 10).await?;
    assert_eq!(list.len(), 1, "media list should have 1 item when searching for app test");

    let list = db.list_media(None, Some("1"), 0, 10).await?;
    assert_eq!(list.len(), 1, "media list should have 1 item when searching for id 1");

    Ok(())
  }

  #[tokio::test]
  async fn test_download() -> Result {
    let db = Db::new_in_mem().await?;

    let media = db.create_or_replace_media("1", "test", "sha256", 100, true).await?;
    assert_eq!(media.id.id, Id::from("1"), "media id should be 1");

    let download = db.create_download("1",
                                      MediaDownloadSpec { sha256:   "sha256".to_string(),
                                                          size:     100,
                                                          from_url: "http://example.com".to_string(), },
                                      MediaDownloadState::default())
                     .await?;

    assert_eq!(download.media,
               Thing::from(("media", "1")).to_raw(),
               "download media id should be 1");
    assert_eq!(download.spec.sha256, "sha256", "download spec sha256 should be sha256");
    assert_eq!(download.spec.size, 100, "download spec size should be 100");
    assert_eq!(download.spec.from_url, "http://example.com",
               "download spec from_url should be http://example.com");

    let downloads = db.list_media_downloads(None, Some("1"), 0, 10).await?;
    assert_eq!(downloads.len(), 1, "download list should have 1 item");
    assert_eq!(downloads[0].media,
               Thing::from(("media", "1")).to_raw(),
               "download list item 1 media id should be 1");

    Ok(())
  }
}
