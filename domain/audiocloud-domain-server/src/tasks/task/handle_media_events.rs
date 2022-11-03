/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use crate::tasks::task::TaskActor;
use crate::tasks::NotifyMediaTaskState;

impl Handler<NotifyMediaTaskState> for TaskActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyMediaTaskState, ctx: &mut Self::Context) -> Self::Result {
        self.media_objects.update_media(msg.media);
        self.update(ctx);
    }
}
