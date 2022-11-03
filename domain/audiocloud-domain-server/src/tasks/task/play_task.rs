/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use audiocloud_api::audio_engine::TaskPlaying;

use audiocloud_api::{DesiredInstancePlayState, DesiredTaskPlayState};

use crate::tasks::task::TaskActor;
use crate::tasks::PlayTask;
use crate::DomainResult;

impl Handler<PlayTask> for TaskActor {
    type Result = DomainResult<TaskPlaying>;

    fn handle(&mut self, msg: PlayTask, ctx: &mut Self::Context) -> Self::Result {
        // TODO: check play_id history

        let rv = TaskPlaying::Playing { task_id: { self.id.clone() },
                                        play_id: { msg.play.play_id.clone() }, };

        let desired_instance_state = DesiredInstancePlayState::Playing { play_id: { msg.play.play_id.clone() }, };
        let desired_task_state = DesiredTaskPlayState::Play(msg.play);

        self.fixed_instances.set_desired_state(desired_instance_state);
        self.engine.set_desired_state(desired_task_state);

        Ok(rv)
    }
}
