/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

impl Handler<SetInstanceParameters> for InstanceActor {
    type Result = DomainResult;

    fn handle(&mut self, msg: SetInstanceParameters, ctx: &mut Self::Context) -> Self::Result {
        merge_values(self.parameters.get_mut(), msg.parameters);
        self.parameters.mark_modified();

        Ok(())
    }
}
