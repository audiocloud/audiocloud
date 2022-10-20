use actix::Handler;

use audiocloud_api::audio_engine::TaskRendering;

use audiocloud_api::{DesiredInstancePlayState, DesiredTaskPlayState};

use crate::tasks::task::TaskActor;
use crate::tasks::RenderTask;
use crate::DomainResult;

impl Handler<RenderTask> for TaskActor {
    type Result = DomainResult<TaskRendering>;

    fn handle(&mut self, msg: RenderTask, ctx: &mut Self::Context) -> Self::Result {
        // TODO: check render_id history

        let rv = TaskRendering::Rendering {
            task_id: { self.id.clone() },
            render_id: { msg.render.render_id.clone() },
        };

        let desired_instance_state = DesiredInstancePlayState::Rendering {
            length: { msg.render.segment.length },
            render_id: { msg.render.render_id.clone() },
        };
        let desired_task_state = DesiredTaskPlayState::Render(msg.render);

        self.fixed_instances
            .set_desired_state(desired_instance_state);
        self.engine.set_desired_state(desired_task_state);

        Ok(rv)
    }
}
