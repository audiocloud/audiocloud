use actix::fut::LocalBoxActorFuture;
use actix::{fut, ActorFutureExt, Handler, WrapFuture};
use tracing::*;

use audiocloud_api::domain::tasks::TaskUpdated;
use audiocloud_api::domain::DomainError;

use crate::tasks::supervisor::SupervisedTask;
use crate::tasks::ModifyTask;
use crate::DomainResult;

use super::TasksSupervisor;

impl Handler<ModifyTask> for TasksSupervisor {
    type Result = LocalBoxActorFuture<Self, DomainResult<TaskUpdated>>;

    fn handle(&mut self, msg: ModifyTask, ctx: &mut Self::Context) -> Self::Result {
        use DomainError::*;

        match self.tasks.get_mut(&msg.task_id) {
            Some(task) => match task.actor.as_ref() {
                Some(actor) => actor
                    .send(msg)
                    .into_actor(self)
                    .map(|result, _, _| match result {
                        Ok(result) => result,
                        Err(err) => Err(BadGateway {
                            error: err.to_string(),
                        }),
                    })
                    .boxed_local(),
                None => fut::ready(Self::modify_task_spec(task, msg))
                    .into_actor(self)
                    .boxed_local(),
            },
            None => {
                warn!(task_id = %msg.task_id, "Refusing to modify unknown task");
                fut::err(TaskNotFound {
                    task_id: msg.task_id.clone(),
                })
                .into_actor(self)
                .boxed_local()
            }
        }
    }
}

impl TasksSupervisor {
    fn modify_task_spec(task: &mut SupervisedTask, msg: ModifyTask) -> DomainResult<TaskUpdated> {
        use DomainError::*;

        let mut spec = task.spec.clone();

        for modification in msg.modify_spec {
            spec.modify(modification)
                .map_err(|error| TaskModification {
                    task_id: { msg.task_id.clone() },
                    error: { error },
                })?;
        }

        task.spec = spec;

        Ok(TaskUpdated::Updated {
            task_id: { msg.task_id.clone() },
            revision: { task.spec.revision },
        })
    }
}
