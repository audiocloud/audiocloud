/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use core::panicking::assert_failed;
use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use derive_more::Deref;
use reaper_medium::{ChunkCacheHint, MediaTrack, ProjectContext, Reaper};
use uuid::Uuid;

use audiocloud_api::ChannelMask;
use error::Result;
use new_types::{ChunkId, InsertId, TrackId};
use track::Track;

use crate::project::error::ProjectError;

pub mod error;
pub mod insert;
pub mod new_types;
pub mod send;
pub mod track;

pub struct Project {
    reaper:  Reaper,
    tracks:  HashMap<TrackId, Track>,
    context: ProjectContext,
}

impl Project {
    pub fn get_track_and_index(&self, id: &TrackId) -> Result<Option<(u32, MediaTrack)>> {
        for i in 0..self.reaper.count_tracks(self.context) {
            if let Some(track) = self.reaper.get_track(self.context, i) {
                let track_id: TrackId = unsafe { self.reaper.get_set_media_track_info_get_guid(track) }.into();
                if &track_id == id {
                    return Ok(Some((i, track)));
                }
            }
        }

        Ok(None)
    }

    pub fn update_chunk(&self, chunk_id: ChunkId) -> Result {
        use ProjectError::*;

        let track_id = chunk_id.pad_id;

        let track = self.tracks.get(&track_id).ok_or(TrackNotFound { track_id })?;

        let (_, reaper_track) = self.get_track_and_index(&track_id)?
                                    .ok_or(TrackNotPresent { track_id })?;

        let chunk = track.get_chunk(chunk_id.include_inserts)?;

        unsafe {
            self.reaper
                .set_track_state_chunk(reaper_track, chunk.as_str(), ChunkCacheHint::UndoMode)
                .map_err(|err| ChunkUpdateFailed { chunk_id,
                                                   error: err.to_string() })?;
        }

        Ok(())
    }

    pub fn create_track(&mut self, track_id: TrackId, track: Track) -> Result {
        if !self.tracks.contains_key(&track_id) {
            self.tracks.insert(track_id, track);
            let chunk = track.get_chunk(true)?;
            self.append_track(chunk)?;

            Ok(())
        } else {
            Err(ProjectError::TrackAlreadyExists { track_id })
        }
    }

    fn append_track(&self, chunk: String) -> Result {
        let tracks = self.reaper.count_tracks(self.context);
        self.

        Ok(())
    }
}
