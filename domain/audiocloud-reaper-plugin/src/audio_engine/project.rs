use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::{CStr, CString};
use std::fs;
use std::path::PathBuf;
use std::ptr::null_mut;

use anyhow::anyhow;
use askama::Template;
use cstr::cstr;

use lazy_static::lazy_static;
use reaper_medium::ProjectContext::CurrentProject;
use reaper_medium::{
    AutoSeekBehavior, ChunkCacheHint, CommandId, EditMode, MediaTrack, PlayState, PositionInSeconds, ProjectContext, ProjectRef,
    ReaProject, Reaper, ReaperPanValue, ReaperVolumeValue, SetEditCurPosOptions, TimeRangeType, TrackAttributeKey, TrackSendCategory,
    TrackSendRef,
};
use tempdir::TempDir;
use tracing::*;

use audiocloud_api::audio_engine::event::EngineEvent;
use audiocloud_api::cloud::domains::FixedInstanceRouting;
use audiocloud_api::common::change::{ModifyTaskSpec, UpdateTaskPlay};
use audiocloud_api::common::media::{PlayId, RenderId, RequestPlay, RequestRender};

use audiocloud_api::common::task::{ConnectionValues, FixedInstanceNode, MixerNode, NodeConnection, TaskSpec, TimeSegment, TrackNode};
use audiocloud_api::common::time::Timestamped;
use audiocloud_api::newtypes::{
    AppMediaObjectId, AppTaskId, FixedInstanceId, FixedInstanceNodeId, MixerNodeId, NodeConnectionId, TrackNodeId,
};
use audiocloud_api::{InputPadId, NodePadId, OutputPadId, PadMetering};

use crate::audio_engine::fixed_instance::EngineFixedInstance;
use crate::audio_engine::media_track::EngineMediaTrack;
use crate::audio_engine::mixer::AudioMixer;
use crate::audio_engine::{EngineStatus, PluginRegistry};

#[derive(Debug, Clone)]
pub enum ProjectPlayState {
    PreparingToPlay(RequestPlay),
    Playing(RequestPlay),
    Rendering(RequestRender),
    Stopped,
}

#[derive(Debug)]
pub struct EngineProject {
    id:                    AppTaskId,
    project:               ReaProject,
    tracks:                HashMap<TrackNodeId, EngineMediaTrack>,
    fixed_instances:       HashMap<FixedInstanceNodeId, EngineFixedInstance>,
    mixers:                HashMap<MixerNodeId, AudioMixer>,
    spec:                  TaskSpec,
    local_media_root:      PathBuf,
    shared_media_root:     PathBuf,
    pub play_state:        Timestamped<ProjectPlayState>,
    pub temp_dir:          TempDir,
    pub session_path:      PathBuf,
    pub reaper_play_state: Timestamped<PlayState>,
    pub events:            VecDeque<EngineEvent>,
}

#[derive(Debug, Clone)]
pub struct EngineProjectTemplateSnapshot {
    context:     ProjectContext,
    connections: HashMap<NodeConnectionId, NodeConnection>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ReaperChunkId {
    pub pad_id:          NodePadId,
    pub include_inserts: bool,
}

impl From<InputPadId> for ReaperChunkId {
    fn from(input_pad_id: InputPadId) -> Self {
        Self { pad_id:          { NodePadId::from(input_pad_id) },
               include_inserts: { false }, }
    }
}

impl From<OutputPadId> for ReaperChunkId {
    fn from(output_pad_id: OutputPadId) -> Self {
        Self { pad_id:          { NodePadId::from(output_pad_id) },
               include_inserts: { false }, }
    }
}

impl EngineProjectTemplateSnapshot {
    pub fn context(&self) -> ProjectContext {
        self.context
    }

    fn get_track_index_for_pad(&self, pad_id: &NodePadId) -> Option<usize> {
        let reaper = Reaper::get();
        let mut index = 0;
        let track_name = pad_id.to_string();

        while let Some(track) = reaper.get_track(self.context, index) {
            let matches = unsafe { reaper.get_set_media_track_info_get_name(track, |name| name.to_str() == track_name.as_str()) };

            if matches.unwrap_or(false) {
                return Some(index as usize);
            }

            index += 1;
        }

        None
    }

    pub fn track_index(&self, id: &NodePadId) -> Option<usize> {
        self.get_track_index_for_pad(id)
    }

    pub fn fixed_input_track_index(&self, fixed_id: &FixedInstanceNodeId) -> Option<usize> {
        self.track_index(&NodePadId::FixedInstanceInput(fixed_id.clone()))
    }

    pub fn mixer_input_track_index(&self, mixer_id: &MixerNodeId) -> Option<usize> {
        self.track_index(&NodePadId::MixerInput(mixer_id.clone()))
    }

    pub fn flows_to<'a>(&'a self, flow: &'a InputPadId) -> impl Iterator<Item = (&NodeConnectionId, &NodeConnection)> + 'a {
        self.connections.iter().filter(move |(_, conn)| &conn.to == flow)
    }
}

// NOTE: requires SWS extensions
lazy_static! {
    static ref CMD_REC_MODE_SET_TIME_RANGE_AUTO_PUNCH: CommandId = CommandId::new(40076);
    static ref CMD_CREATE_PROJECT_TAB: CommandId = CommandId::new(40859);
    static ref CMD_CLOSE_CURRENT_PROJECT_TAB: CommandId = CommandId::new(40860);
    static ref CMD_SWITCH_TO_NEXT_PROJECT_TAB: CommandId = CommandId::new(40861);
    static ref CMD_TRANSPORT_RECORD: CommandId = CommandId::new(1013);
    static ref CMD_TRANSPORT_STOP_AND_SAVE_MEDIA: CommandId = CommandId::new(40667);
    static ref CMD_TRANSPORT_STOP_AND_DELETE_MEDIA: CommandId = CommandId::new(40668);
}

#[derive(Template)]
#[template(path = "audio_engine/project.txt")]
struct EngineProjectTemplate<'a> {
    spec:       &'a TaskSpec,
    session_id: &'a AppTaskId,
    media_root: String,
}

impl EngineProject {
    #[instrument(skip_all, err)]
    pub fn new(id: AppTaskId,
               temp_dir: TempDir,
               shared_media_root: PathBuf,
               session_spec: TaskSpec,
               instances: HashMap<FixedInstanceId, FixedInstanceRouting>,
               media: HashMap<AppMediaObjectId, String>)
               -> anyhow::Result<Self> {
        let reaper = Reaper::get();

        // this is not OK we need to configure it
        let local_media_root = temp_dir.path().join("media");
        let session_path = temp_dir.path().join("session.rpp");

        fs::write(&session_path,
                  EngineProjectTemplate { spec:       &session_spec,
                                          session_id: &id,
                                          media_root: local_media_root.to_string_lossy().to_string(), }.render()?)?;

        reaper.main_on_command_ex(*CMD_CREATE_PROJECT_TAB, 0, CurrentProject);

        unsafe {
            let path_as_cstr = CString::new(format!("noprompt:{}", session_path.to_string_lossy()))?;
            reaper.low().Main_openProject(path_as_cstr.as_ptr());
        }

        let project = reaper.enum_projects(ProjectRef::Current, 0)
                            .ok_or_else(|| anyhow!("No current project even though we just opened one"))?
                            .project;

        let context = ProjectContext::Proj(project);

        reaper.main_on_command_ex(*CMD_REC_MODE_SET_TIME_RANGE_AUTO_PUNCH, 0, context);

        let tracks = Default::default();
        let fixed_instances = Default::default();
        let mixers = Default::default();
        let spec = Default::default();
        let play_state = ProjectPlayState::Stopped.into();
        let reaper_play_state = Timestamped::from(Reaper::get().get_play_state_ex(context));
        let events = VecDeque::new();

        let mut rv = Self { id,
                            project,
                            tracks,
                            fixed_instances,
                            mixers,
                            spec,
                            local_media_root,
                            shared_media_root,
                            temp_dir,
                            session_path,
                            play_state,
                            reaper_play_state,
                            events };

        rv.set_spec(session_spec, instances, media)?;

        Ok(rv)
    }

    pub fn template_snapshot(&self) -> EngineProjectTemplateSnapshot {
        EngineProjectTemplateSnapshot { context:     self.context(),
                                        connections: self.spec.connections.clone(), }
    }

    pub fn play_ready(&mut self, play_id: PlayId) {
        if let ProjectPlayState::PreparingToPlay(play) = self.play_state.get_ref() {
            if play.play_id == play_id {
                self.play_state = ProjectPlayState::Playing(play.clone()).into();
                Reaper::get().on_play_button_ex(self.context());
            }
        }
    }

    #[instrument(skip_all, err, fields(id = % self.id))]
    pub fn run(&mut self) -> anyhow::Result<()> {
        let context = self.context();

        let reaper = Reaper::get();
        let new_play_state = reaper.get_play_state_ex(context);
        let cur_pos = reaper.get_play_position_ex(context).get();

        match self.play_state.get_ref().clone() {
            ProjectPlayState::PreparingToPlay(play) => {
                debug!(play_id = %play.play_id, "waiting for plugin to be ready...");
                if self.play_state.elapsed().num_seconds() > 1 {
                    self.play_state = ProjectPlayState::Stopped.into();
                    self.events.push_back(EngineEvent::Error { task_id: self.id.clone(),
                                                               error:   format!("Timed out preparing resampling or compression"), });
                }
            }
            ProjectPlayState::Playing(play) => {
                debug!(cur_pos, end = play.segment.end(), "playing...");
                if !new_play_state.is_playing && self.reaper_play_state.get_ref().is_playing {
                    debug!(play_id = %play.play_id, "reached end of play");
                    self.clean_up_end_of_play(play.play_id);
                }
            }
            ProjectPlayState::Rendering(render) => {
                debug!(cur_pos, end = render.segment.end(), "rendering...");
                if cur_pos >= render.segment.end() {
                    debug!(render_id = %render.render_id, "reached end of render");
                    self.clean_up_end_of_render(render.mixer_id.clone(), render.render_id);
                }
            }
            _ => {}
        }

        self.reaper_play_state = Timestamped::from(new_play_state);

        Ok(())
    }

    fn clean_up_end_of_render(&mut self, mixer_id: MixerNodeId, render_id: RenderId) {
        let reaper = Reaper::get();
        let context = self.context();

        reaper.main_on_command_ex(*CMD_TRANSPORT_STOP_AND_SAVE_MEDIA, 0, context);

        if let Some(mixer) = self.mixers.get_mut(&mixer_id) {
            if let Some(path) = mixer.clear_render() {
                self.events.push_back(EngineEvent::RenderingFinished { task_id: self.id.clone(),
                                                                       render_id,
                                                                       path });
            } else {
                // we did not get a path
                self.events.push_back(EngineEvent::RenderingFailed { task_id: self.id.clone(),
                                                                     render_id,
                                                                     error: format!("Rendered file not found") });
            }
        }

        self.play_state = ProjectPlayState::Stopped.into();
    }

    fn clean_up_end_of_play(&mut self, play_id: PlayId) {
        self.clear_mixer_master_sends();
        // a plugin flush is not critical, so we are fine with discarding the error
        let _ = PluginRegistry::flush(&self.id, play_id);

        self.play_state = ProjectPlayState::Stopped.into();
    }

    pub fn context(&self) -> ProjectContext {
        ProjectContext::Proj(self.project)
    }

    pub fn shared_media_root_dir(&self) -> PathBuf {
        self.shared_media_root.clone()
    }

    pub fn get_peak_meters(&self) -> HashMap<NodePadId, PadMetering> {
        let mut peaks = HashMap::new();

        for track in self.tracks.values() {
            track.fill_peak_meters(&mut peaks);
        }

        for mixer in self.mixers.values() {
            mixer.fill_peak_meters(&mut peaks);
        }

        for instance in self.fixed_instances.values() {
            instance.fill_peak_meters(&mut peaks);
        }

        peaks
    }

    #[instrument(skip_all, err)]
    pub fn focus(&self) -> anyhow::Result<()> {
        let reaper = Reaper::get();
        let mut first = None;
        loop {
            if let Some(enumerated) = reaper.enum_projects(ProjectRef::Current, 0) {
                match first {
                    None => first = Some(enumerated.project),
                    Some(x) => {
                        if x == enumerated.project {
                            return Err(anyhow!("Project not found"));
                        }
                    }
                }

                if enumerated.project != self.project {
                    reaper.main_on_command_ex(*CMD_SWITCH_TO_NEXT_PROJECT_TAB, 0, CurrentProject);
                } else {
                    break;
                }
            }
        }

        Ok(())
    }
}

impl Drop for EngineProject {
    fn drop(&mut self) {
        if self.project.as_ptr() != null_mut() {
            let reaper = Reaper::get();
            if let Ok(_) = self.focus() {
                debug!(id = %self.id, "Closing project");
                reaper.main_on_command_ex(*CMD_CLOSE_CURRENT_PROJECT_TAB, 0, self.context());
            } else {
                warn!("Project could not be focused for closing");
            }
        }
    }
}

impl EngineProject {
    pub fn add_track(&mut self, id: TrackNodeId, spec: TrackNode, media: &HashMap<AppMediaObjectId, String>) -> anyhow::Result<()> {
        self.tracks.insert(id.clone(),
                           EngineMediaTrack::new(self, self.id.app_id.clone(), id.clone(), spec, media)?);

        Ok(())
    }

    pub fn delete_track(&mut self, id: &TrackNodeId) {
        if let Some(track) = self.tracks.remove(id) {
            track.delete(self.context());
        }
    }

    pub fn add_fixed_instance(&mut self,
                              fixed_id: FixedInstanceNodeId,
                              spec: FixedInstanceNode,
                              instances: &HashMap<FixedInstanceId, FixedInstanceRouting>)
                              -> anyhow::Result<()> {
        let routing = instances.get(&spec.instance_id).cloned();

        self.fixed_instances
            .insert(fixed_id.clone(), EngineFixedInstance::new(self, fixed_id.clone(), spec, routing)?);

        Ok(())
    }

    pub fn delete_fixed_instance(&mut self, fixed_id: FixedInstanceNodeId) -> anyhow::Result<()> {
        if let Some(fixed) = self.fixed_instances.remove(&fixed_id) {
            fixed.delete(self.context());
        }

        Ok(())
    }

    pub fn add_mixer(&mut self, mixer_id: MixerNodeId, spec: MixerNode) -> anyhow::Result<()> {
        self.mixers.insert(mixer_id.clone(), AudioMixer::new(self, mixer_id.clone(), spec)?);

        Ok(())
    }

    fn delete_mixer(&mut self, mixer_id: &MixerNodeId) {
        if let Some(mixer) = self.mixers.remove(mixer_id) {
            mixer.delete(self.context());
        }
    }

    pub fn get_status(&self) -> anyhow::Result<EngineStatus> {
        Ok(EngineStatus { plugin_ready:         PluginRegistry::has(&self.id)?,
                          is_transport_playing: self.reaper_play_state.get_ref().is_playing || self.reaper_play_state.get_ref().is_recording,
                          is_playing:           if let ProjectPlayState::Playing(play) = self.play_state.get_ref() {
                              Some(play.play_id.clone())
                          } else {
                              None
                          },
                          is_rendering:         if let ProjectPlayState::Rendering(render) = self.play_state.get_ref() {
                              Some(render.render_id.clone())
                          } else {
                              None
                          },
                          position:             Reaper::get().get_play_position_ex(self.context()).get(), })
    }

    pub fn render(&mut self, render: RequestRender) -> anyhow::Result<()> {
        let reaper = Reaper::get();

        self.stop()?;
        self.clear_mixer_master_sends();
        self.set_time_range_markers(render.segment);
        self.clear_all_project_markers();
        self.set_looping(false);
        self.set_play_position((render.segment.start - 0.125).max(0.0), false);

        if let Some(mixer) = self.mixers.get_mut(&render.mixer_id) {
            mixer.prepare_render(&render);
        }

        self.play_state = ProjectPlayState::Rendering(render).into();

        reaper.main_on_command_ex(*CMD_TRANSPORT_RECORD, 0, self.context());

        Ok(())
    }

    pub fn play(&mut self, play: RequestPlay) -> anyhow::Result<()> {
        let reaper = Reaper::get();

        self.stop()?;

        for (mixer_id, mixer) in &mut self.mixers {
            mixer.set_master_send(mixer_id == &play.mixer_id);
        }

        self.clear_all_project_markers();
        self.set_time_range_markers(play.segment);
        self.set_play_position(play.start_at, false);
        self.set_looping(play.looping);

        PluginRegistry::play(&self.id, play.clone(), self.context())?;

        self.play_state = ProjectPlayState::PreparingToPlay(play).into();

        Ok(())
    }

    pub fn update_play(&mut self, update: UpdateTaskPlay) -> anyhow::Result<()> {
        let reaper = Reaper::get();

        if let Some(new_mixer_id) = update.mixer_id {
            for (mixer_id, mixer) in self.mixers.iter_mut() {
                mixer.set_master_send(mixer_id == &new_mixer_id);
            }
        }

        if let Some(segment) = update.segment {
            self.set_time_range_markers(segment);
        }

        if let Some(looping) = update.looping {
            self.set_looping(looping);
        }

        if let Some(start_at) = update.start_at {
            self.set_play_position(start_at, true);
        }

        Ok(())
    }

    pub fn stop_render(&mut self, render_id: RenderId) -> anyhow::Result<()> {
        self.stop()
    }

    pub fn stop_play(&mut self, play_id: PlayId) -> anyhow::Result<()> {
        self.stop()
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        let reaper = Reaper::get();
        let context = self.context();

        match self.play_state.get_ref() {
            ProjectPlayState::Rendering(render) => {
                // this is an incomplete render...
                reaper.main_on_command_ex(*CMD_TRANSPORT_STOP_AND_DELETE_MEDIA, 0, context);
                if let Some(mixer) = self.mixers.get_mut(&render.mixer_id) {
                    let _ = mixer.clear_render();
                }

                self.events.push_back(EngineEvent::RenderingFailed { task_id:   self.id.clone(),
                                                                     render_id: render.render_id,
                                                                     error:     format!("Rendering stopped prematurely"), });
            }
            _ => {
                reaper.on_stop_button_ex(context);
            }
        }

        self.play_state = ProjectPlayState::Stopped.into();

        Ok(())
    }

    #[instrument(skip_all, err)]
    pub fn set_spec(&mut self,
                    spec: TaskSpec,
                    instances: HashMap<FixedInstanceId, FixedInstanceRouting>,
                    media: HashMap<AppMediaObjectId, String>)
                    -> anyhow::Result<()> {
        if &self.spec == &spec {
            debug!("incoming spec is the same, not changing anything");
            return Ok(());
        }

        debug!(?spec, "new spec");

        self.stop()?;
        self.clear();

        for (track_id, track_spec) in spec.tracks.clone() {
            self.add_track(track_id, track_spec, &media)?;
        }

        for (fixed_id, fixed_spec) in spec.fixed.clone() {
            self.add_fixed_instance(fixed_id, fixed_spec, &instances)?;
        }

        // skip dynamic instances, not supported yet

        for (mixer_id, mixer_spec) in spec.mixers.clone() {
            self.add_mixer(mixer_id, mixer_spec)?;
        }

        self.spec = spec;

        self.update_all_state_chunks()?;

        Ok(())
    }

    fn update_all_state_chunks(&mut self) -> anyhow::Result<()> {
        let snapshot = self.template_snapshot();

        for track in self.tracks.values() {
            track.update_state_chunk(&snapshot)?;
        }

        for instance in self.fixed_instances.values() {
            instance.update_state_chunk(&snapshot)?;
        }

        for mixer in self.mixers.values() {
            mixer.update_state_chunk(&snapshot)?;
        }

        Ok(())
    }

    pub fn modify_spec(&mut self,
                       transaction: Vec<ModifyTaskSpec>,
                       instances: HashMap<FixedInstanceId, FixedInstanceRouting>,
                       media_ready: HashMap<AppMediaObjectId, String>)
                       -> anyhow::Result<()> {
        let current_spec = self.spec.clone();
        let mut dirty_chunks = HashSet::new();

        for item in transaction {
            if let Err(err) = self.modify_spec_one(item, &instances, &media_ready, &mut dirty_chunks) {
                warn!(%err, "failed to execute transaction, rolling back");
                return Ok(self.set_spec(current_spec, instances, media_ready)?);
            }
        }

        for chunk_id in dirty_chunks {
            self.update_track_chunk(&chunk_id.pad_id, chunk_id.include_inserts)?;
        }

        Ok(())
    }

    fn update_track_chunk(&self, chunk_id: &NodePadId, include_inserts: bool) -> anyhow::Result<()> {
        let snapshot = self.template_snapshot();
        let chunk = match chunk_id {
            NodePadId::MixerInput(mixer_id) => self.mixers
                                                   .get(mixer_id)
                                                   .ok_or_else(|| anyhow!("Mixer {mixer_id} not found"))?
                                                   .get_input_state_chunk(&snapshot)?,
            NodePadId::MixerOutput(mixer_id) => self.mixers
                                                    .get(mixer_id)
                                                    .ok_or_else(|| anyhow!("Mixer {mixer_id} not found"))?
                                                    .get_output_state_chunk(&snapshot)?,
            NodePadId::FixedInstanceInput(fixed_id) => self.fixed_instances
                                                           .get(fixed_id)
                                                           .ok_or_else(|| anyhow!("Fixed {fixed_id} not found"))?
                                                           .get_send_state_chunk(&snapshot)?,
            NodePadId::FixedInstanceOutput(fixed_id) => self.fixed_instances
                                                            .get(fixed_id)
                                                            .ok_or_else(|| anyhow!("Fixed {fixed_id} not found"))?
                                                            .get_return_state_chunk(&snapshot)?,
            NodePadId::DynamicInstanceInput(_) => {
                return Err(anyhow!("Dynamic instances not supported yet"));
            }
            NodePadId::DynamicInstanceOutput(_) => {
                return Err(anyhow!("Dynamic instances not supported yet"));
            }
            NodePadId::TrackOutput(track_id) => self.tracks
                                                    .get(track_id)
                                                    .ok_or_else(|| anyhow!("Track {track_id} not found"))?
                                                    .get_state_chunk(&snapshot)?,
        };

        self.set_track_state_chunk(chunk_id, chunk)?;

        Ok(())
    }

    fn modify_spec_one(&mut self,
                       item: ModifyTaskSpec,
                       instances: &HashMap<FixedInstanceId, FixedInstanceRouting>,
                       media: &HashMap<AppMediaObjectId, String>,
                       dirty: &mut HashSet<ReaperChunkId>)
                       -> anyhow::Result<()> {
        match item {
            ModifyTaskSpec::AddTrack { track_id, channels } => {
                self.add_track(track_id.clone(),
                               TrackNode { channels,
                                           media: HashMap::new() },
                               media)?;

                dirty.insert(ReaperChunkId { pad_id:          NodePadId::TrackOutput(track_id),
                                             include_inserts: true, });
            }
            ModifyTaskSpec::AddTrackMedia { track_id, media_id, spec } => {
                if let Some(track) = self.tracks.get_mut(&track_id) {
                    if track.add_media(media_id, spec, media)? {
                        dirty.insert(ReaperChunkId::from(track.get_output_pad_id().clone()));
                    }
                } else {
                    return Err(anyhow!("track {track_id} not found"));
                }
            }
            ModifyTaskSpec::UpdateTrackMedia { track_id,
                                               media_id,
                                               update, } => {
                if let Some(track) = self.tracks.get_mut(&track_id) {
                    if track.set_media_values(media_id, update, media)? {
                        dirty.insert(ReaperChunkId::from(track.get_output_pad_id().clone()));
                    }
                } else {
                    return Err(anyhow!("track {track_id} not found"));
                }
            }
            ModifyTaskSpec::DeleteTrackMedia { track_id, media_id } => {
                if let Some(track) = self.tracks.get_mut(&track_id) {
                    if track.delete_media(&media_id)? {
                        dirty.insert(ReaperChunkId::from(track.get_output_pad_id().clone()));
                    }
                } else {
                    return Err(anyhow!("track {track_id} not found"));
                }
            }
            ModifyTaskSpec::DeleteTrack { track_id } => {
                self.delete_track(&track_id);
            }
            ModifyTaskSpec::AddFixedInstance { fixed_id, spec: process } => {
                self.add_fixed_instance(fixed_id.clone(), process, instances)?;

                dirty.insert(ReaperChunkId::from(InputPadId::FixedInstanceInput(fixed_id.clone())));
                dirty.insert(ReaperChunkId::from(OutputPadId::FixedInstanceOutput(fixed_id.clone())));
            }
            ModifyTaskSpec::AddDynamicInstance { .. } => {
                // not supported, silently ignore
            }
            ModifyTaskSpec::AddMixer { mixer_id, spec: mixer } => {
                self.add_mixer(mixer_id.clone(), mixer)?;

                dirty.insert(ReaperChunkId::from(InputPadId::MixerInput(mixer_id.clone())));
                dirty.insert(ReaperChunkId::from(OutputPadId::MixerOutput(mixer_id.clone())));
            }
            ModifyTaskSpec::DeleteMixer { mixer_id } => {
                self.delete_mixer(&mixer_id);
            }
            ModifyTaskSpec::DeleteFixedInstance { fixed_id } => {
                self.delete_fixed_instance(fixed_id)?;
            }
            ModifyTaskSpec::DeleteDynamicInstance { .. } => {}
            ModifyTaskSpec::DeleteConnection { connection_id } => {
                if let Some(connection) = self.spec.connections.remove(&connection_id) {
                    dirty.insert(ReaperChunkId::from(connection.to.clone()));
                }
            }
            ModifyTaskSpec::AddConnection { to, .. } => {
                dirty.insert(ReaperChunkId::from(to));
            }
            ModifyTaskSpec::SetConnectionParameterValues { connection_id, values } => {
                if let Some(connection) = self.spec.connections.get(&connection_id) {
                    self.set_connection_parameter_values(&connection.to, &connection_id, values)?;
                } else {
                    return Err(anyhow!("connection {connection_id} not found"));
                }
            }
            ModifyTaskSpec::SetFixedInstanceParameterValues { .. } => {}
            ModifyTaskSpec::SetDynamicInstanceParameterValues { .. } => {}
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        let reaper = Reaper::get();

        while let Some(track) = reaper.get_track(self.context(), 0) {
            unsafe {
                reaper.delete_track(track);
            }
        }

        self.tracks.clear();
        self.fixed_instances.clear();
        self.mixers.clear();
    }

    pub fn on_media_updated(&mut self, available: &HashMap<AppMediaObjectId, String>) -> anyhow::Result<()> {
        let snapshot = self.template_snapshot();
        for track in self.tracks.values_mut() {
            if track.on_media_updated(available) {
                track.update_state_chunk(&snapshot)?;
            }
        }

        Ok(())
    }

    pub fn on_instances_updated(&mut self, instances: &HashMap<FixedInstanceId, FixedInstanceRouting>) -> anyhow::Result<()> {
        let snapshot = self.template_snapshot();

        for fixed_instance in self.fixed_instances.values_mut() {
            if fixed_instance.on_instances_updated(instances) {
                fixed_instance.update_state_chunk(&snapshot)?;
            }
        }

        Ok(())
    }

    pub fn set_track_state_chunk(&self, pad_id: &NodePadId, chunk: String) -> anyhow::Result<()> {
        let reaper = Reaper::get();
        let index = self.template_snapshot()
                        .track_index(pad_id)
                        .ok_or_else(|| anyhow!("Track not found"))?;

        let track = reaper.get_track(self.context(), index as u32)
                          .ok_or_else(|| anyhow!("Track could not be loaded"))?;

        unsafe {
            reaper.set_track_state_chunk(track, chunk.as_str(), ChunkCacheHint::NormalMode)?;
        }

        Ok(())
    }

    fn set_connection_parameter_values(&self, target: &InputPadId, id: &NodeConnectionId, values: ConnectionValues) -> anyhow::Result<()> {
        let track = match target {
                        InputPadId::MixerInput(mixer_id) => self.mixers.get(mixer_id).map(|mixer| mixer.get_input_track()),
                        InputPadId::FixedInstanceInput(fixed_id) => self.fixed_instances
                                                                        .get(fixed_id)
                                                                        .map(|fixed_instance| fixed_instance.get_input_track()),
                        other => return Err(anyhow!("Unsupported target {other}")),
                    }.ok_or_else(|| anyhow!("Connection target {target} not found"))?;

        let index = get_track_receive_index(track, id).ok_or_else(|| anyhow!("Connection not found on target {target} input track"))?;

        let reaper = Reaper::get();

        // TODO: if we need any dB conversions, now is a good time :)
        let index = TrackSendRef::Receive(index as u32);

        if let Some(volume) = values.volume {
            unsafe {
                reaper.set_track_send_ui_vol(track, index, ReaperVolumeValue::new(volume), EditMode::NormalTweak)?;
            }
        }

        if let Some(pan) = values.pan {
            unsafe {
                reaper.set_track_send_ui_pan(track, index, ReaperPanValue::new(pan), EditMode::NormalTweak)?;
            }
        }

        Ok(())
    }

    fn clear_mixer_master_sends(&mut self) {
        for (mixer_id, mixer) in &mut self.mixers {
            mixer.set_master_send(false);
        }
    }

    fn set_time_range_markers(&mut self, segment: TimeSegment) {
        Reaper::get().get_set_loop_time_range_2_set(self.context(),
                                                    TimeRangeType::TimeSelection,
                                                    PositionInSeconds::new(segment.start),
                                                    PositionInSeconds::new(segment.end()),
                                                    AutoSeekBehavior::DenyAutoSeek);
    }

    fn set_play_position(&mut self, position: f64, and_play: bool) {
        Reaper::get().set_edit_curs_pos_2(self.context(),
                                          PositionInSeconds::new(position),
                                          SetEditCurPosOptions { seek_play: and_play,
                                                                 move_view: true, });
    }

    fn set_looping(&mut self, looping: bool) {
        Reaper::get().get_set_repeat_ex_set(self.context(), looping);
    }

    fn clear_all_project_markers(&mut self) {
        let reaper = Reaper::get();
        for _ in 0..reaper.count_project_markers(self.context()).total_count {
            unsafe {
                reaper.low().DeleteProjectMarkerByIndex(self.project.as_ptr(), 0);
            }
        }
    }
}

fn get_track_receive_index(track: MediaTrack, id: &NodeConnectionId) -> Option<usize> {
    const P_EXT_ID: &'static CStr = cstr!("P_EXT:ID");

    let reaper = Reaper::get();

    for i in 0..unsafe { reaper.get_track_num_sends(track, TrackSendCategory::Receive) } {
        let mut buffer = [0i8; 256];
        unsafe {
            if reaper.low().GetSetTrackSendInfo_String(track.as_ptr(),
                                                       TrackSendCategory::Receive.to_raw(),
                                                       i as i32,
                                                       P_EXT_ID.as_ptr(),
                                                       buffer.as_mut_ptr(),
                                                       false)
            {
                let ext_id = CStr::from_ptr(buffer.as_ptr()).to_string_lossy();

                if ext_id == id.as_str() {
                    return Some(i as usize);
                }
            }
        }
    }

    None
}

pub fn set_track_master_send(track: MediaTrack, mut send: bool) {
    unsafe {
        Reaper::get().get_set_media_track_info(track, TrackAttributeKey::MainSend, &mut send as *mut _ as _);
    }
}

pub fn get_track_peak_meters(track: MediaTrack, channels: usize) -> PadMetering {
    let reaper = Reaper::get();
    let mut rv = PadMetering { volume: vec![] };

    for i in 0..channels {
        let value = 0.0f64;
        rv.volume.push(unsafe { reaper.track_get_peak_info(track, i as u32) }.into());
    }

    rv
}
