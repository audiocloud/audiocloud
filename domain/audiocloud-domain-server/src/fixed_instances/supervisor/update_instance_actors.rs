/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Actor, Context};
use futures::executor::block_on;
use tokio::task::block_in_place;
use tracing::*;

use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::supervisor::SupervisedInstance;
use crate::fixed_instances::FixedInstancesSupervisor;

impl FixedInstancesSupervisor {
    pub(crate) fn update_instance_actors(&mut self, _ctx: &mut Context<Self>) {
        for driver in self.drivers.values_mut() {
            for (instance_id, instance_config) in &driver.config.instances {
                let model_id = instance_id.model_id();
                let db = self.db.clone();
                let model = match block_in_place(move || block_on(db.get_model(&model_id))) {
                    Ok(Some(model)) => model,
                    _ => continue,
                };

                if driver.instances.get_mut(instance_id).is_none() {
                    let instance_addr =
                        match FixedInstanceActor::new(instance_id.clone(), Some(driver.url.clone()), instance_config.clone(), model) {
                            Ok(actor) => actor.start(),
                            Err(error) => {
                                error!(%error, "Failed to start fixed instance actor");
                                continue;
                            }
                        };

                    driver.instances.insert(instance_id.clone(),
                                            SupervisedInstance { address: { instance_addr },
                                                                 config:  { instance_config.clone() },
                                                                 state:   { None }, });
                }

                driver.instances
                      .retain(|instance_id, _| driver.config.instances.contains_key(instance_id));
            }
        }
    }
}
