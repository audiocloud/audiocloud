use actix::Handler;

use audiocloud_api::audio_engine::EngineCommand;
use audiocloud_api::domain::tasks::TaskUpdated;
use audiocloud_api::domain::DomainError;

use crate::tasks::task::TaskActor;
use crate::tasks::ModifyTask;
use crate::DomainResult;

impl Handler<ModifyTask> for TaskActor {
    type Result = DomainResult<TaskUpdated>;

    fn handle(&mut self, msg: ModifyTask, ctx: &mut Self::Context) -> Self::Result {
        let play_state = self.engine.get_actual_play_state();

        if msg.revision < self.spec.revision {
            if msg.optional {
                Ok(TaskUpdated::Ignored {
                    task_id: self.id.clone(),
                    revision: self.spec.revision,
                })
            } else {
                Err(DomainError::TaskModificationRevisionOutOfDate {
                    task_id: self.id.clone(),
                    revision: self.spec.revision,
                })
            }
        } else if play_state.is_rendering_any() {
            Err(DomainError::TaskIllegalPlayState {
                task_id: self.id.clone(),
                state: play_state.into(),
            })
        } else {
            let mut clone = self.spec.clone();
            for update in msg.modify_spec {
                clone
                    .modify(update)
                    .map_err(|error| DomainError::TaskModification {
                        task_id: self.id.clone(),
                        error,
                    })?;
            }

            clone.revision += 1;
            self.spec = clone;
            self.engine.enqueue(EngineCommand::SetSpec {
                task_id: self.id.clone(),
                spec: self.spec.clone(),
                instances: self.fixed_instance_routing.clone(),
                media_ready: self.media_objects.ready_for_engine(),
            });

            Ok(TaskUpdated::Updated {
                task_id: self.id.clone(),
                revision: self.spec.revision,
            })
        }
    }
}
