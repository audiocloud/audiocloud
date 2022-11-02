/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use audiocloud_api::domain::DomainError;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::SetInstanceDesiredPlayState;
use crate::DomainResult;

impl Handler<SetInstanceDesiredPlayState> for FixedInstanceActor {
    type Result = DomainResult<()>;

    fn handle(&mut self, msg: SetInstanceDesiredPlayState, ctx: &mut Self::Context) -> Self::Result {
        if let Some(media) = self.media.as_mut() {
            media.set_desired_state(msg.desired);
            Ok(())
        } else {
            Err(DomainError::InstanceNotCapable { instance_id: self.id.clone(),
                                                  operation:   format!("SetInstanceDesiredPlayState"), })
        }
    }
}
