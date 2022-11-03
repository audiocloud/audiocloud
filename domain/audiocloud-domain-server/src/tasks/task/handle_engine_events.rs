/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use audiocloud_api::audio_engine::EngineEvent;

use audiocloud_api::DesiredTaskPlayState;

use crate::tasks::task::TaskActor;
use crate::tasks::NotifyEngineEvent;

impl Handler<NotifyEngineEvent> for TaskActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyEngineEvent, ctx: &mut Self::Context) -> Self::Result {
        use EngineEvent::*;

        if &self.engine_id != &msg.engine_id {
            return;
        }

        match msg.event {
            Stopped { task_id } => {
                if &self.id == &task_id {
                    self.engine.set_actual_stopped();
                }
            }
            Playing { task_id,
                      play_id,
                      audio,
                      peak_metering,
                      dynamic_reports, } => {
                if &self.id == &task_id && self.engine.should_be_playing(&play_id) {
                    self.engine.set_actual_playing(play_id);
                    self.merge_peak_meters(peak_metering);
                    self.push_compressed_audio(audio);
                    self.maybe_send_packet();
                }
            }
            PlayingFailed { task_id, play_id, error } => {
                if &self.id == &task_id {
                    self.engine.set_desired_state(DesiredTaskPlayState::Stopped);
                    self.engine.set_actual_stopped();
                }
            }
            Rendering { task_id,
                        render_id,
                        completion, } => {
                if &self.id == &task_id {
                    self.engine.set_actual_rendering(render_id);
                }
            }
            RenderingFinished { task_id, render_id, path } => {
                if &self.id == &task_id {
                    self.engine.set_desired_state(DesiredTaskPlayState::Stopped);
                    self.engine.set_actual_stopped();
                }
            }
            RenderingFailed { task_id, render_id, error } => {
                if &self.id == &task_id {
                    self.engine.set_desired_state(DesiredTaskPlayState::Stopped);
                    self.engine.set_actual_stopped();
                }
            }
            Error { task_id, error } => {
                if &self.id == &task_id {
                    // do not modify desired states..
                }
            }
        }
    }
}
