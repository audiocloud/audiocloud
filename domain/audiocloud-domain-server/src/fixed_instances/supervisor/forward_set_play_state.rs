/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::fut::LocalBoxActorFuture;
use actix::{fut, ActorFutureExt, Handler, WrapFuture};

use audiocloud_api::domain::DomainError;

use crate::fixed_instances::{FixedInstancesSupervisor, SetInstanceDesiredPlayState};
use crate::DomainResult;

impl Handler<SetInstanceDesiredPlayState> for FixedInstancesSupervisor {
    type Result = LocalBoxActorFuture<Self, DomainResult>;

    fn handle(&mut self, msg: SetInstanceDesiredPlayState, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(instance) = self.instances.get(&msg.instance_id) {
            instance.address
                    .send(msg)
                    .into_actor(self)
                    .map(|res, _actor, _ctx| match res {
                        Ok(res) => res,
                        Err(err) => Err(DomainError::BadGateway { error: format!("Failed to set instance desired play state: {err}"), }),
                    })
                    .boxed_local()
        } else {
            fut::err(DomainError::InstanceNotFound { instance_id: msg.instance_id, }).boxed_local()
        }
    }
}
