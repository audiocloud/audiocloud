/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;

use askama::Template;
use itertools::Itertools;
use reaper_medium::{MediaTrack, ProjectContext};
use tracing::*;
use uuid::Uuid;

use audiocloud_api::cloud::domains::FixedInstanceRouting;
use audiocloud_api::common::task::{FixedInstanceNode, NodePadId};
use audiocloud_api::newtypes::{FixedInstanceId, FixedInstanceNodeId};
use audiocloud_api::{InputPadId, OutputPadId, PadMetering};

use crate::audio_engine::project::{get_track_peak_meters, EngineProject, EngineProjectTemplateSnapshot};
use crate::audio_engine::{append_track, beautify_chunk, delete_track, set_track_chunk, ConnectionTemplate};

#[derive(Debug)]
pub struct EngineFixedInstance {
    fixed_id:      FixedInstanceNodeId,
    send_pad_id:   InputPadId,
    return_pad_id: OutputPadId,
    send_id:       Uuid,
    return_id:     Uuid,
    reainsert_id:  Uuid,
    send_track:    MediaTrack,
    return_track:  MediaTrack,
    spec:          FixedInstanceNode,
    routing:       Option<FixedInstanceRouting>,
}

impl EngineFixedInstance {
    #[instrument(skip_all, err)]
    pub fn new(project: &EngineProject,
               fixed_id: FixedInstanceNodeId,
               spec: FixedInstanceNode,
               routing: Option<FixedInstanceRouting>)
               -> anyhow::Result<Self> {
        let send_flow_id = InputPadId::FixedInstanceInput(fixed_id.clone());
        let return_flow_id = OutputPadId::FixedInstanceOutput(fixed_id.clone());

        let reainsert_id = Uuid::new_v4();

        project.focus()?;

        let (send_track, send_id) = append_track(&send_flow_id.clone().into(), project.context())?;
        let (return_track, return_id) = append_track(&return_flow_id.clone().into(), project.context())?;

        Ok(Self { fixed_id:      { fixed_id },
                  send_pad_id:   { send_flow_id },
                  return_pad_id: { return_flow_id },
                  send_id:       { send_id },
                  return_id:     { return_id },
                  reainsert_id:  { reainsert_id },
                  spec:          { spec },
                  send_track:    { send_track },
                  return_track:  { return_track },
                  routing:       { routing }, })
    }

    pub(crate) fn delete(&self, context: ProjectContext) {
        delete_track(context, self.send_track);
        delete_track(context, self.return_track);
    }

    pub fn on_instances_updated(&mut self, instances: &HashMap<FixedInstanceId, FixedInstanceRouting>) -> bool {
        let routing = instances.get(&self.spec.instance_id).cloned();
        if &routing != &self.routing {
            self.routing = routing;
            true
        } else {
            false
        }
    }

    pub fn get_input_flow_id(&self) -> &InputPadId {
        &self.send_pad_id
    }

    pub fn get_output_flow_id(&self) -> &OutputPadId {
        &self.return_pad_id
    }

    pub fn get_input_track(&self) -> MediaTrack {
        self.send_track
    }

    fn use_reainsert(&self) -> bool {
        if let Some(routing) = &self.routing {
            routing.send_count <= 2 && routing.return_count <= 2
        } else {
            false
        }
    }

    pub fn get_send_state_chunk(&self, project: &EngineProjectTemplateSnapshot) -> anyhow::Result<String> {
        if self.use_reainsert() {
            Ok(beautify_chunk(ReaInsertSendTemplate { project, instance: self }.render()?))
        } else {
            Ok(beautify_chunk(HwOutSendTemplate { project, instance: self }.render()?))
        }
    }

    pub fn get_return_state_chunk(&self, project: &EngineProjectTemplateSnapshot) -> anyhow::Result<String> {
        if self.use_reainsert() {
            Ok(beautify_chunk(ReaInsertReturnTemplate { project, instance: self }.render()?))
        } else {
            Ok(beautify_chunk(HwOutReturnTemplate { project, instance: self }.render()?))
        }
    }

    fn send_count(&self) -> usize {
        match self.routing {
            None => 2,
            Some(routing) => routing.send_count,
        }
    }

    fn return_count(&self) -> usize {
        match self.routing {
            None => 2,
            Some(routing) => routing.return_count,
        }
    }

    #[instrument(skip_all, err, fields(id = %self.fixed_id))]
    pub fn update_state_chunk(&self, project: &EngineProjectTemplateSnapshot) -> anyhow::Result<()> {
        set_track_chunk(project.context(), self.send_track, &self.get_send_state_chunk(project)?)?;
        set_track_chunk(project.context(), self.return_track, &self.get_return_state_chunk(project)?)?;

        Ok(())
    }

    pub fn fill_peak_meters(&self, peaks: &mut HashMap<NodePadId, PadMetering>) {
        if let Some(routing) = self.routing {
            peaks.insert(self.send_pad_id.clone().into(),
                         get_track_peak_meters(self.send_track, routing.send_count));

            peaks.insert(self.return_pad_id.clone().into(),
                         get_track_peak_meters(self.return_track, routing.return_count));
        }
    }
}

#[derive(Template)]
#[template(path = "audio_engine/reainsert_send.txt")]
struct ReaInsertSendTemplate<'a> {
    project:  &'a EngineProjectTemplateSnapshot,
    instance: &'a EngineFixedInstance,
}

#[derive(Template)]
#[template(path = "audio_engine/hwout_send.txt")]
struct HwOutSendTemplate<'a> {
    project:  &'a EngineProjectTemplateSnapshot,
    instance: &'a EngineFixedInstance,
}

impl<'a> HwOutSendTemplate<'a> {
    fn reaper_channel_pairs(&self) -> Vec<(usize, usize)> {
        if let Some(routing) = self.instance.routing {
            let start = routing.send_channel;
            (0..routing.send_count).chunks(2)
                                   .into_iter()
                                   .map(move |mut chunk| match (chunk.next(), chunk.next()) {
                                       (Some(channel), Some(_)) => (start + channel, channel),
                                       (Some(channel), None) => ((start + channel) | 1024, channel | 1024),
                                       _ => unreachable!(),
                                   })
                                   .collect()
        } else {
            vec![]
        }
    }
}

#[derive(Template)]
#[template(path = "audio_engine/hwout_return.txt")]
struct HwOutReturnTemplate<'a> {
    project:  &'a EngineProjectTemplateSnapshot,
    instance: &'a EngineFixedInstance,
}

impl<'a> HwOutReturnTemplate<'a> {
    fn reaper_rec_input(&self) -> i32 {
        if let Some(routing) = self.instance.routing {
            (match routing.return_count {
                1 => routing.return_channel,
                2 => routing.return_channel | 1024,
                x => routing.return_channel | (x << 4),
            }) as i32
        } else {
            -1
        }
    }
}

#[derive(Template)]
#[template(path = "audio_engine/reainsert_return.txt")]
struct ReaInsertReturnTemplate<'a> {
    project:  &'a EngineProjectTemplateSnapshot,
    instance: &'a EngineFixedInstance,
}
