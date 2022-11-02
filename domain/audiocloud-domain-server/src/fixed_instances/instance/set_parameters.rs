/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::Handler;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::MergeInstanceParameters;

impl Handler<MergeInstanceParameters> for FixedInstanceActor {
    type Result = ();

    fn handle(&mut self, msg: MergeInstanceParameters, ctx: &mut Self::Context) -> Self::Result {
        let params = self.parameters.get_mut();
        let mut modified = false;
        if let Some(obj) = params.as_object_mut() {
            if let Some(keyed_value) = obj.get_mut(msg.parameter.as_str()) {
                if let Some(array) = keyed_value.as_array_mut() {
                    if array.len() > msg.channel {
                        array[msg.channel] = msg.value.to_json();
                        modified = true;
                    }
                }
            }
        }

        if modified {
            self.parameters.mark_modified();
        }
    }
}
