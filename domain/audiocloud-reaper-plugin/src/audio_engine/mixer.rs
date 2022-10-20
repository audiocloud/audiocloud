use std::collections::HashMap;
use std::ffi::CStr;

use askama::Template;
use reaper_medium::{MediaTrack, ProjectContext, Reaper, TrackAttributeKey};
use tracing::*;
use uuid::Uuid;

use audiocloud_api::common::media::RequestRender;
use audiocloud_api::common::task::{MixerNode, NodePadId};
use audiocloud_api::newtypes::MixerNodeId;
use audiocloud_api::{InputPadId, OutputPadId, PadMetering};

use crate::audio_engine::project::{get_track_peak_meters, set_track_master_send, EngineProject, EngineProjectTemplateSnapshot};
use crate::audio_engine::ConnectionTemplate;
use crate::audio_engine::{append_track, beautify_chunk, delete_track, set_track_chunk};

#[derive(Debug)]
pub struct AudioMixer {
    mixer_id:      MixerNodeId,
    input_pad_id:  InputPadId,
    output_pad_id: OutputPadId,
    input_id:      Uuid,
    output_id:     Uuid,
    input_track:   MediaTrack,
    output_track:  MediaTrack,
    spec:          MixerNode,
}

impl AudioMixer {
    pub(crate) fn delete(&self, context: ProjectContext) {
        delete_track(context, self.input_track);
        delete_track(context, self.output_track);
    }
}

impl AudioMixer {
    #[instrument(skip_all, err)]
    pub fn new(project: &EngineProject, mixer_id: MixerNodeId, spec: MixerNode) -> anyhow::Result<Self> {
        let input_pad_id = InputPadId::MixerInput(mixer_id.clone());
        let output_pad_id = OutputPadId::MixerOutput(mixer_id.clone());

        project.focus()?;

        let (input_track, input_id) = append_track(&input_pad_id.clone().into(), project.context())?;
        let (output_track, output_id) = append_track(&output_pad_id.clone().into(), project.context())?;

        Ok(Self { mixer_id:      { mixer_id },
                  input_pad_id:  { input_pad_id },
                  output_pad_id: { output_pad_id },
                  input_id:      { input_id },
                  output_id:     { output_id },
                  input_track:   { input_track },
                  output_track:  { output_track },
                  spec:          { spec }, })
    }

    pub fn get_input_track(&self) -> MediaTrack {
        self.input_track
    }

    pub fn get_input_state_chunk(&self, project: &EngineProjectTemplateSnapshot) -> anyhow::Result<String> {
        Ok(beautify_chunk(AudioMixerInputTemplate { project, mixer: self }.render()?))
    }

    pub fn get_output_state_chunk(&self, project: &EngineProjectTemplateSnapshot) -> anyhow::Result<String> {
        Ok(beautify_chunk(AudioMixerOutputTemplate { project, mixer: self }.render()?))
    }

    #[instrument(skip_all, err, fields(id = %self.mixer_id))]
    pub fn update_state_chunk(&self, project: &EngineProjectTemplateSnapshot) -> anyhow::Result<()> {
        set_track_chunk(project.context(), self.input_track, &self.get_input_state_chunk(project)?)?;

        set_track_chunk(project.context(), self.output_track, &self.get_output_state_chunk(project)?)?;

        Ok(())
    }

    pub fn fill_peak_meters(&self, peaks: &mut HashMap<NodePadId, PadMetering>) {
        peaks.insert(self.input_pad_id.clone().into(),
                     get_track_peak_meters(self.input_track, self.spec.input_channels));

        peaks.insert(self.output_pad_id.clone().into(),
                     get_track_peak_meters(self.output_track, self.spec.output_channels));
    }

    pub fn set_master_send(&mut self, master_send: bool) {
        set_track_master_send(self.output_track, master_send);
    }

    pub fn prepare_render(&mut self, render: &RequestRender) {
        let reaper = Reaper::get();
        use TrackAttributeKey::*;

        unsafe {
            // arm for recording
            reaper.get_set_media_track_info(self.output_track, RecArm, &mut 1i32 as *mut i32 as _);

            // set record mode to "output latency compensated"
            reaper.get_set_media_track_info(self.output_track, RecMode, &mut 3i32 as *mut i32 as _);

            // set record monitoring to off
            reaper.get_set_media_track_info(self.output_track, RecMon, &mut 0i32 as *mut i32 as _);
        }
    }

    pub fn clear_render(&mut self) -> Option<String> {
        use TrackAttributeKey::*;

        let reaper = Reaper::get();
        let mut rv = None;

        // iterate all media items
        unsafe {
            while let Some(media_item) = reaper.get_track_media_item(self.output_track, 0) {
                if let Some(take) = reaper.get_active_take(media_item) {
                    if let Some(source) = reaper.get_media_item_take_source(take) {
                        let mut path_name = [0i8; 1024];

                        reaper.low()
                              .GetMediaSourceFileName(source.as_ptr(), path_name.as_mut_ptr(), path_name.len() as i32);

                        rv = Some(CStr::from_ptr(path_name.as_ptr()).to_string_lossy().to_string());
                    }
                }

                if let Err(err) = reaper.delete_track_media_item(self.output_track, media_item) {
                    warn!(%err, "failed to delete media item");
                }
            }
        }

        unsafe {
            reaper.get_set_media_track_info(self.output_track, RecArm, &mut 0i32 as *mut i32 as _);
        }

        rv
    }
}

#[derive(Template)]
#[template(path = "audio_engine/mixer_track_input.txt")]
struct AudioMixerInputTemplate<'a> {
    project: &'a EngineProjectTemplateSnapshot,
    mixer:   &'a AudioMixer,
}

#[derive(Template)]
#[template(path = "audio_engine/mixer_track_output.txt")]
struct AudioMixerOutputTemplate<'a> {
    project: &'a EngineProjectTemplateSnapshot,
    mixer:   &'a AudioMixer,
}
