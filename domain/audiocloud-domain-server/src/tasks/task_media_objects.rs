use std::collections::{HashMap, HashSet};

use audiocloud_api::common::media::MediaObject;
use audiocloud_api::newtypes::AppMediaObjectId;

#[derive(Default)]
pub struct TaskMediaObjects {
    pub media: HashMap<AppMediaObjectId, MediaObject>,
}

impl TaskMediaObjects {
    pub fn waiting_for_media(&self) -> HashSet<AppMediaObjectId> {
        let mut rv = HashSet::new();

        for (object_id, object) in &self.media {
            if object.path.is_none() {
                rv.insert(object_id.clone());
            }
        }

        rv
    }

    pub fn update_media(
        &mut self,
        media_service_objects: HashMap<AppMediaObjectId, MediaObject>,
    ) -> bool {
        // make a backup of pending items
        let prev = self.waiting_for_media();

        // this will keep any local pending media
        self.media.extend(media_service_objects.into_iter());

        self.waiting_for_media() != prev
    }

    pub fn ready_for_engine(&self) -> HashMap<AppMediaObjectId, String> {
        self.media
            .iter()
            .filter_map(|(_, object)| {
                object
                    .path
                    .as_ref()
                    .map(|path| (object.id.clone(), path.clone()))
            })
            .collect()
    }

    pub fn any_waiting(&self) -> bool {
        self.media.values().any(|object| object.path.is_none())
    }
}
