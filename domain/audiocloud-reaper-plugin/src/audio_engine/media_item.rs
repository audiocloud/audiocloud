use std::collections::HashMap;
use std::ffi::CStr;
use std::path::PathBuf;

use askama::Template;
use audiocloud_api::common::task::{TrackMedia, UpdateTaskTrackMedia};
use cstr::cstr;
use reaper_medium::{MediaItem, MediaItemTake, MediaTrack, Reaper};
use tracing::*;
use uuid::Uuid;

use crate::audio_engine::media_track::EngineMediaTrack;
use crate::audio_engine::project::EngineProjectTemplateSnapshot;
use audiocloud_api::newtypes::{AppId, AppMediaObjectId, TrackMediaId};

#[derive(Debug)]
pub struct EngineMediaItem {
    media_id:  TrackMediaId,
    object_id: AppMediaObjectId,
    item_id:   Uuid,
    take_id:   Uuid,
    track:     MediaTrack,
    item:      MediaItem,
    take:      MediaItemTake,
    spec:      TrackMedia,
    path:      Option<String>,
}

impl EngineMediaItem {
    #[instrument(skip_all, err)]
    pub fn new(track: MediaTrack,
               media_root: &PathBuf,
               app_id: &AppId,
               media_id: TrackMediaId,
               spec: TrackMedia,
               media: &HashMap<AppMediaObjectId, String>)
               -> anyhow::Result<Self> {
        let object_id = spec.object_id.clone().for_app(app_id.clone());

        debug!(%object_id, "object_id");

        let path = match media.get(&object_id) {
            Some(path) => Some(media_root.join(path).canonicalize()?.to_string_lossy().to_string()),
            None => None,
        };

        debug!(?path, "path");

        let reaper = Reaper::get();

        let item = unsafe { reaper.add_media_item_to_track(track)? };
        let take = unsafe { reaper.add_take_to_media_item(item)? };

        debug!(?item, ?take, "craated item and take");

        let item_id = get_media_item_uuid(item)?;
        debug!(?item_id, "item_id is");

        let take_id = get_media_item_take_uuid(take)?;
        debug!(?take_id, "take_id is");

        Ok(Self { media_id,
                  object_id,
                  item_id,
                  take_id,
                  track,
                  item,
                  take,
                  spec,
                  path })
    }

    #[instrument(skip_all, err)]
    pub fn delete(self) -> anyhow::Result<()> {
        debug!("enter");
        let reaper = Reaper::get();
        unsafe {
            reaper.delete_track_media_item(self.track, self.item)?;
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn on_media_updated(&mut self, root_dir: &PathBuf, available: &HashMap<AppMediaObjectId, String>) -> bool {
        debug!(?root_dir, ?available, "entered");

        if self.path.is_none() {
            if let Some(path) = available.get(&self.object_id) {
                let new_path = Some(root_dir.join(path).to_str().unwrap().to_string());
                if &new_path != &self.path {
                    self.path = new_path;
                    debug!("our path is replaced, queue to sync");
                    return true;
                }
            }
        }

        debug!("no need to sync");

        false
    }

    pub fn update(&mut self, update: UpdateTaskTrackMedia) {
        self.spec.update(update.clone());
    }
}

#[instrument(skip_all, err)]
fn get_media_item_uuid(media_item: MediaItem) -> anyhow::Result<Uuid> {
    let reaper = Reaper::get();
    let param: &'static CStr = cstr!("GUID");
    let mut buffer = [0i8; 1024];

    unsafe {
        if !reaper.low()
                  .GetSetMediaItemInfo_String(media_item.as_ptr(), param.as_ptr(), buffer.as_mut_ptr(), false)
        {
            return Err(anyhow::anyhow!("Failed to get media item GUID"));
        }

        let str = CStr::from_ptr(buffer.as_mut_ptr() as *mut i8).to_str()?;

        Ok(Uuid::try_parse(&str[1..str.len() - 1])?)
    }
}

#[instrument(skip_all, err)]
fn get_media_item_take_uuid(media_item_take: MediaItemTake) -> anyhow::Result<Uuid> {
    let reaper = Reaper::get();
    let param: &'static CStr = cstr!("GUID");
    let mut buffer = [0i8; 1024];

    unsafe {
        if !reaper.low()
                  .GetSetMediaItemTakeInfo_String(media_item_take.as_ptr(), param.as_ptr(), buffer.as_mut_ptr(), false)
        {
            return Err(anyhow::anyhow!("Failed to get media item take GUID"));
        }

        let str = CStr::from_ptr(buffer.as_mut_ptr() as *mut i8).to_str()?;

        Ok(Uuid::try_parse(&str[1..str.len() - 1])?)
    }
}

#[derive(Template)]
#[template(path = "audio_engine/media_item.txt")]
pub struct EngineMediaItemTemplate<'a> {
    media:   &'a EngineMediaItem,
    track:   &'a EngineMediaTrack,
    project: &'a EngineProjectTemplateSnapshot,
}

impl<'a> EngineMediaItemTemplate<'a> {
    pub fn new(media: &'a EngineMediaItem, track: &'a EngineMediaTrack, project: &'a EngineProjectTemplateSnapshot) -> Self {
        Self { media, track, project }
    }
}
