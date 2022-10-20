use actix::Handler;

use audiocloud_api::audio_engine::{EngineCommand, TaskRenderCancelled};
use audiocloud_api::domain::DomainError;

use crate::tasks::task::TaskActor;
use crate::tasks::CancelRenderTask;
use crate::DomainResult;

impl Handler<CancelRenderTask> for TaskActor {
    type Result = DomainResult<TaskRenderCancelled>;

    fn handle(&mut self, msg: CancelRenderTask, ctx: &mut Self::Context) -> Self::Result {
        let play_state = self.engine.get_actual_play_state();
        let render_id = msg.cancel.render_id;

        if play_state.is_rendering(&render_id) {
            self.engine.enqueue(EngineCommand::CancelRender {
                task_id: { self.id.clone() },
                render_id: { render_id.clone() },
            });

            Ok(TaskRenderCancelled::Cancelled {
                task_id: { self.id.clone() },
                render_id: { render_id },
            })
        } else {
            Err(DomainError::TaskIllegalPlayState {
                task_id: { self.id.clone() },
                state: { play_state.into() },
            })
        }
    }
}
