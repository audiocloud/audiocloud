use std::collections::VecDeque;

use audiocloud_api::audio_engine::EngineCommand;
use audiocloud_api::{AppTaskId, DesiredTaskPlayState, PlayId, RenderId, TaskPlayState, Timestamped};

use crate::tracker::RequestTracker;

pub struct TaskEngine {
    id:                  AppTaskId,
    desired_play_state:  Timestamped<DesiredTaskPlayState>,
    actual_play_state:   Timestamped<TaskPlayState>,
    tracker:             RequestTracker,
    instances_are_ready: Timestamped<bool>,
    media_is_ready:      Timestamped<bool>,
    commands:            VecDeque<Timestamped<EngineCommand>>,
    version:             u64,
}

impl TaskEngine {
    pub fn new(id: AppTaskId) -> Self {
        Self { id:                  { id },
               desired_play_state:  { Timestamped::new(DesiredTaskPlayState::Stopped) },
               actual_play_state:   { Timestamped::new(TaskPlayState::Stopped) },
               tracker:             { Default::default() },
               instances_are_ready: { Default::default() },
               media_is_ready:      { Default::default() },
               commands:            { Default::default() },
               version:             { 0 }, }
    }

    pub fn enqueue(&mut self, cmd: EngineCommand) {
        self.commands.push_back(Timestamped::new(cmd));
    }

    pub fn get_actual_play_state(&self) -> &TaskPlayState {
        self.actual_play_state.value()
    }

    pub fn set_desired_state(&mut self, desired: DesiredTaskPlayState) -> u64 {
        if self.desired_play_state.value() != &desired {
            self.desired_play_state = Timestamped::new(desired);
            self.tracker.reset();
            self.version + 1
        } else {
            self.version
        }
    }

    pub fn set_instances_are_ready(&mut self, ready: bool) {
        self.instances_are_ready = Timestamped::new(ready);
    }

    pub fn set_media_is_ready(&mut self, ready: bool) {
        self.media_is_ready = Timestamped::new(ready);
    }

    pub fn update(&mut self) -> Option<EngineCommand> {
        if self.actual_play_state.value().satisfies(self.desired_play_state.value()) {
            if self.tracker.should_retry() {
                let engine_cmd = match (self.desired_play_state.value(), self.actual_play_state.value()) {
                    (_, TaskPlayState::Playing(play)) => Some(EngineCommand::StopPlay { task_id: self.id.clone(),
                                                                                        play_id: play.play_id, }),
                    (_, TaskPlayState::Rendering(render)) => Some(EngineCommand::CancelRender { task_id:   self.id.clone(),
                                                                                                render_id: render.render_id, }),
                    (DesiredTaskPlayState::Play(play), TaskPlayState::Stopped) => Some(EngineCommand::Play { task_id: self.id.clone(),
                                                                                                             play:    play.clone(), }),
                    (DesiredTaskPlayState::Render(render), TaskPlayState::Stopped) => {
                        Some(EngineCommand::Render { task_id: self.id.clone(),
                                                     render:  render.clone(), })
                    }
                    _ => None,
                };

                if engine_cmd.is_some() {
                    self.tracker.retried();
                }

                engine_cmd
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_actual_state(&mut self, actual: TaskPlayState) {
        self.actual_play_state = Timestamped::new(actual);
        self.tracker.reset();
    }

    pub fn set_actual_stopped(&mut self) {
        self.set_actual_state(TaskPlayState::Stopped);
    }

    pub fn set_actual_playing(&mut self, play_id: PlayId) {
        if let DesiredTaskPlayState::Play(play) = self.desired_play_state.value() {
            if &play.play_id == &play_id {
                self.set_actual_state(TaskPlayState::Playing(play.clone()));
            }
        }
    }

    pub fn set_actual_rendering(&mut self, render_id: RenderId) {
        if let DesiredTaskPlayState::Render(render) = self.desired_play_state.value() {
            if &render.render_id == &render_id {
                self.set_actual_state(TaskPlayState::Rendering(render.clone()));
            }
        }
    }

    pub fn should_be_playing(&self, play_id: &PlayId) -> bool {
        matches!(self.desired_play_state.value(), DesiredTaskPlayState::Play(play) if &play.play_id == play_id)
    }
}
