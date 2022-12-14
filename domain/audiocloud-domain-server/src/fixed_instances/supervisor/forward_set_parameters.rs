/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::fut::LocalBoxActorFuture;
use actix::{fut, ActorFutureExt, Context, Handler, WrapFuture};

use audiocloud_api::domain::DomainError;

use crate::fixed_instances::{FixedInstancesSupervisor, SetInstanceParameters};
use crate::DomainResult;

impl Handler<SetInstanceParameters> for FixedInstancesSupervisor {
    type Result = LocalBoxActorFuture<Self, DomainResult>;

    fn handle(&mut self, msg: SetInstanceParameters, _ctx: &mut Context<FixedInstancesSupervisor>) -> Self::Result {
        for driver in self.drivers.values() {
            if let Some(instance) = driver.instances.get(&msg.instance_id) {
                return instance.address
                               .send(msg)
                               .into_actor(self)
                               .map(|res, _actor, _ctx| match res {
                                   Ok(res) => res,
                                   Err(err) => Err(DomainError::BadGateway { error: format!("Failed to set instance parameters: {err}"), }),
                               })
                               .boxed_local();
            }
        }

        fut::err(DomainError::InstanceNotFound { instance_id: msg.instance_id, }).boxed_local()
    }
}
