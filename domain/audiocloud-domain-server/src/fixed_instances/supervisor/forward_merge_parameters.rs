/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::fut::LocalBoxActorFuture;
use actix::{fut, ActorFutureExt, Context, Handler, WrapFuture};

use audiocloud_api::domain::DomainError;

use crate::fixed_instances::{FixedInstancesSupervisor, MergeInstanceParameters};
use crate::DomainResult;

impl Handler<MergeInstanceParameters> for FixedInstancesSupervisor {
    type Result = LocalBoxActorFuture<Self, DomainResult>;

    fn handle(&mut self, msg: MergeInstanceParameters, _ctx: &mut Context<FixedInstancesSupervisor>) -> Self::Result {
        if let Some(instance) = self.instances.get(&msg.instance_id) {
            instance.address
                    .send(msg)
                    .into_actor(self)
                    .map(|res, _actor, _ctx| match res {
                        Ok(res) => res,
                        Err(err) => Err(DomainError::BadGateway { error: format!("Failed to merge instance parameters: {err}"), }),
                    })
                    .boxed_local()
        } else {
            fut::err(DomainError::InstanceNotFound { instance_id: msg.instance_id, }).boxed_local()
        }
    }
}
