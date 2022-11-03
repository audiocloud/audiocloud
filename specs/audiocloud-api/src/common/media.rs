//! Communication with the on-site media library

use std::collections::HashSet;

use derive_more::{Constructor, Display, From, Into};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::task::{MediaChannels, TrackMediaFormat};
use crate::common::time::{now, Timestamp};
use crate::newtypes::{AppMediaObjectId, AppTaskId};
use crate::{MixerNodeId, TimeSegment};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct MediaJobState {
    pub progress:    f64,
    pub retry:       usize,
    pub error:       Option<String>,
    pub in_progress: bool,
    pub updated_at:  Timestamp,
}

impl Default for MediaJobState {
    fn default() -> Self {
        Self { progress:    0.0,
               retry:       0,
               error:       None,
               in_progress: false,
               updated_at:  now(), }
    }
}

impl MediaJobState {
    pub fn is_finished_ok(&self) -> bool {
        !self.in_progress && self.error.is_none()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MediaMetadata {
    pub channels:    MediaChannels,
    pub format:      TrackMediaFormat,
    pub seconds:     f64,
    pub sample_rate: usize,
    pub bytes:       u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UploadToDomain {
    pub channels:    MediaChannels,
    pub format:      TrackMediaFormat,
    pub seconds:     f64,
    pub sample_rate: usize,
    pub bytes:       u64,
    pub url:         String,
    pub notify_url:  Option<String>,
    // typescript: any
    pub context:     Option<Value>,
}

impl UploadToDomain {
    pub fn metadata(&self) -> MediaMetadata {
        MediaMetadata { channels:    self.channels,
                        format:      self.format,
                        seconds:     self.seconds,
                        sample_rate: self.sample_rate,
                        bytes:       self.bytes, }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DownloadFromDomain {
    pub url:        String,
    pub notify_url: Option<String>,
    // typescript: any
    pub context:    Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ImportToDomain {
    pub path:        String,
    pub channels:    MediaChannels,
    pub format:      TrackMediaFormat,
    pub seconds:     f64,
    pub sample_rate: usize,
    pub bytes:       u64,
}

impl ImportToDomain {
    pub fn metadata(&self) -> MediaMetadata {
        MediaMetadata { channels:    self.channels,
                        format:      self.format,
                        seconds:     self.seconds,
                        sample_rate: self.sample_rate,
                        bytes:       self.bytes, }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MediaDownload {
    pub media_id:   AppMediaObjectId,
    pub download:   DownloadFromDomain,
    pub state:      MediaJobState,
    pub created_at: Timestamp,
}

impl MediaDownload {
    pub fn completed_successfully(&self) -> bool {
        self.state.error.is_none() && !self.state.in_progress
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MediaUpload {
    pub media_id:   AppMediaObjectId,
    pub upload:     UploadToDomain,
    pub state:      MediaJobState,
    pub created_at: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MediaObject {
    pub id:        AppMediaObjectId,
    pub metadata:  Option<MediaMetadata>,
    pub path:      Option<String>,
    pub last_used: Option<Timestamp>,
    pub revision:  u64,
}

impl MediaObject {
    pub fn new(id: &AppMediaObjectId) -> Self {
        Self { id:        id.clone(),
               metadata:  None,
               path:      None,
               last_used: None,
               revision:  0, }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateMediaSession {
    pub media_objects: HashSet<AppMediaObjectId>,
    pub ends_at:       Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MediaServiceCommand {
    SetSessionMedia {
        session_id: AppTaskId,
        media:      HashSet<AppMediaObjectId>,
    },
    DeleteSession {
        session_id: AppTaskId,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SampleRate {
    #[serde(rename = "192")]
    SR192,
    #[serde(rename = "96")]
    SR96,
    #[serde(rename = "88.2")]
    SR88_2,
    #[serde(rename = "48")]
    SR48,
    #[serde(rename = "44.1")]
    SR44_1,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlayBitDepth {
    #[serde(rename = "24")]
    PD24,
    #[serde(rename = "16")]
    PD16,
}

impl Into<usize> for PlayBitDepth {
    fn into(self) -> usize {
        match self {
            PlayBitDepth::PD24 => 24,
            PlayBitDepth::PD16 => 16,
        }
    }
}

impl Into<usize> for SampleRate {
    fn into(self) -> usize {
        match self {
            SampleRate::SR192 => 192_000,
            SampleRate::SR96 => 96_000,
            SampleRate::SR88_2 => 88_200,
            SampleRate::SR48 => 48_000,
            SampleRate::SR44_1 => 44_100,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestPlay {
    pub play_id:     PlayId,
    pub mixer_id:    MixerNodeId,
    pub segment:     TimeSegment,
    pub start_at:    f64,
    pub looping:     bool,
    pub sample_rate: SampleRate,
    pub bit_depth:   PlayBitDepth,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestChangeMixer {
    pub play_id:  PlayId,
    pub mixer_id: MixerNodeId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestSeek {
    pub play_id:  PlayId,
    pub segment:  TimeSegment,
    pub start_at: f64,
    pub looping:  bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestStopPlay {
    pub play_id: PlayId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestCancelRender {
    pub render_id: RenderId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestRender {
    pub render_id: RenderId,
    pub mixer_id:  MixerNodeId,
    pub segment:   TimeSegment,
    pub object_id: AppMediaObjectId,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct PlayId(u64);

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct RenderId(u64);
