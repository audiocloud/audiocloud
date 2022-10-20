use actix::Handler;

use audiocloud_api::audio_engine::{EngineCommand, TaskPlayStopped};
use audiocloud_api::domain::DomainError;

use crate::tasks::task::TaskActor;
use crate::tasks::StopPlayTask;
use crate::DomainResult;

impl Handler<StopPlayTask> for TaskActor {
    type Result = DomainResult<TaskPlayStopped>;

    fn handle(&mut self, msg: StopPlayTask, ctx: &mut Self::Context) -> Self::Result {
        let play_state = self.engine.get_actual_play_state();
        let play_id = msg.stop.play_id;

        if play_state.is_playing(&play_id) {
            self.engine.enqueue(EngineCommand::StopPlay { task_id: { self.id.clone() },
                                                          play_id: { play_id.clone() }, });

            Ok(TaskPlayStopped::Stopped { task_id: { self.id.clone() },
                                          play_id: { play_id }, })
        } else {
            Err(DomainError::TaskIllegalPlayState { task_id: { self.id.clone() },
                                                    state:   { play_state.into() }, })
        }
    }
}
