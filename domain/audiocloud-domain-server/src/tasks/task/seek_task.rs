use actix::Handler;

use audiocloud_api::audio_engine::{EngineCommand, TaskSought};

use audiocloud_api::domain::DomainError;
use audiocloud_api::{RequestSeek, TaskPlayState, UpdateTaskPlay};

use crate::tasks::task::TaskActor;
use crate::tasks::SeekTask;
use crate::DomainResult;

impl Handler<SeekTask> for TaskActor {
    type Result = DomainResult<TaskSought>;

    fn handle(&mut self, msg: SeekTask, ctx: &mut Self::Context) -> Self::Result {
        match self.engine.get_actual_play_state() {
            TaskPlayState::Playing(playing) if &playing.play_id == &msg.seek.play_id => {
                let RequestSeek {
                    play_id,
                    segment,
                    start_at,
                    looping,
                } = msg.seek;

                let sought = TaskSought::Sought {
                    task_id: { msg.task_id.clone() },
                    play_id: { play_id.clone() },
                };

                let update = UpdateTaskPlay {
                    play_id: { msg.seek.play_id },
                    mixer_id: { None },
                    segment: { Some(segment) },
                    start_at: { Some(start_at) },
                    looping: { Some(looping) },
                };

                self.engine.enqueue(EngineCommand::UpdatePlay {
                    task_id: { msg.task_id },
                    update: { update },
                });

                Ok(sought)
            }
            _ => Err(DomainError::TaskIllegalPlayState {
                task_id: { msg.task_id.clone() },
                state: { self.engine.get_actual_play_state().into() },
            }),
        }
    }
}
