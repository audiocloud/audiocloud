/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::values::merge_values;
use crate::fixed_instances::SetInstanceParameters;
use crate::DomainResult;

impl Handler<SetInstanceParameters> for FixedInstanceActor {
    type Result = DomainResult;

    fn handle(&mut self, msg: SetInstanceParameters, ctx: &mut Self::Context) -> Self::Result {
        merge_values(self.parameters.get_mut(), msg.parameters);
        self.parameters.mark_modified();

        Ok(())
    }
}
