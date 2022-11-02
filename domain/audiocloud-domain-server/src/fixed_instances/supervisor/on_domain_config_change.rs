/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use actix::{Actor, Context, Handler};
use futures::executor::block_on;
use tracing::*;

use audiocloud_api::{hashmap_changes, HashMapChanges};

use crate::config::NotifyDomainConfiguration;
use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::supervisor::SupervisedInstance;
use crate::fixed_instances::FixedInstancesSupervisor;

impl Handler<NotifyDomainConfiguration> for FixedInstancesSupervisor {
    type Result = ();

    #[instrument(skip_all, name = "handle_notify_domain_configuration")]
    fn handle(&mut self, msg: NotifyDomainConfiguration, _ctx: &mut Self::Context) -> Self::Result {
        self.config = msg.config;
    }
}

impl FixedInstancesSupervisor {
    fn apply_config(&mut self, ctx: &mut Context<Self>) {
        // let existing = self.instances
        //                    .iter()
        //                    .map(|(id, instance)| (id.clone(), instance.config.clone()))
        //                    .collect();
        //
        // let HashMapChanges { changed, added, removed } = hashmap_changes(&existing, &msg.config.fixed_instances);
        //
        // for id in removed {
        //     self.instances.remove(&id);
        // }
        //
        // for (id, config) in added {
        //     if let Ok(Some(model)) = block_on(self.db.get_model(&id.model_id())) {
        //         match FixedInstanceActor::new(id.clone(), config.clone(), model) {
        //             Ok(actor) => {
        //                 let address = actor.start();
        //
        //                 self.instances.insert(id.clone(),
        //                                       SupervisedInstance { address: { address },
        //                                                            config:  { config },
        //                                                            state:   { None }, });
        //             }
        //             Err(error) => {
        //                 warn!(%id, %error, "Could not create instance actor");
        //             }
        //         }
        //     }
        // }
        //
        // for (id, config) in changed {
        //     if let Some(instance) = self.instances.get_mut(&id) {
        //         instance.config = config;
        //         // TODO: set configuration of instance actor
        //     }
        // }
    }
}
