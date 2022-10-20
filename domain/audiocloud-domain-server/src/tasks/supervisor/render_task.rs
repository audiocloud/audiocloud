use actix::fut::LocalBoxActorFuture;
use actix::{fut, ActorFutureExt, Handler, WrapFuture};

use audiocloud_api::audio_engine::TaskRendering;

use audiocloud_api::domain::DomainError;

use crate::tasks::RenderTask;
use crate::DomainResult;

use super::TasksSupervisor;

impl Handler<RenderTask> for TasksSupervisor {
    type Result = LocalBoxActorFuture<Self, DomainResult<TaskRendering>>;

    fn handle(&mut self, msg: RenderTask, ctx: &mut Self::Context) -> Self::Result {
        use DomainError::*;

        if let Some(task) = self.tasks.get(&msg.task_id).and_then(|task| task.actor.as_ref()) {
            let task_id = msg.task_id.clone();
            task.send(msg)
                .into_actor(self)
                .map(move |res, actor, ctx| match res {
                    Ok(result) => result,
                    Err(err) => Err(BadGateway { error: format!("Task actor {task_id} failed to render: {err}"), }),
                })
                .boxed_local()
        } else {
            fut::err(TaskNotFound { task_id: msg.task_id.clone(), }).into_actor(self)
                                                                    .boxed_local()
        }
    }
}
