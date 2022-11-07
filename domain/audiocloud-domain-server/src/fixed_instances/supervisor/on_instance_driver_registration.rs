/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;
use std::time::Instant;

use actix::Handler;
use actix_broker::BrokerIssue;
use futures::executor::block_on;
use tokio::task::block_in_place;
use tracing::*;

use audiocloud_api::cloud::domains::{FixedInstanceRouting, FixedInstanceRoutingMap, TimestampedInstanceDriverConfig};
use audiocloud_api::{cloud::domains::InstanceDriverConfig, Timestamped};

use crate::config::NotifyFixedInstanceRouting;
use crate::fixed_instances::NotifyInstanceDriverUrl;
use crate::{fixed_instances::RegisterInstanceDriver, DomainResult};

use super::{FixedInstancesSupervisor, SupervisedInstanceDriver};

impl Handler<RegisterInstanceDriver> for FixedInstancesSupervisor {
    type Result = DomainResult<TimestampedInstanceDriverConfig>;

    #[instrument(skip_all, err, fields(driver_id = %msg.driver_id), name = "register_instance_driver")]
    fn handle(&mut self, mut msg: RegisterInstanceDriver, _ctx: &mut Self::Context) -> Self::Result {
        let result = self.drivers
                         .entry(msg.driver_id.clone())
                         .or_insert_with(|| SupervisedInstanceDriver { config:    { msg.provided.clone() },
                                                                       instances: { HashMap::new() },
                                                                       last_seen: { Instant::now() },
                                                                       online:    { Timestamped::new(true) },
                                                                       url:       { msg.base_url.clone() }, });

        result.last_seen = Instant::now();
        let mut config_updated = false;
        if result.config.timestamp() < msg.provided.timestamp() {
            result.config = msg.provided.clone();
            config_updated = true;
        }

        let rv = result.config.clone();

        for instance in self.drivers
                            .get(&msg.driver_id)
                            .iter()
                            .flat_map(|driver| driver.config.instances.keys())
        {
            self.issue_system_async(NotifyInstanceDriverUrl { instance_id: { instance.clone() },
                                                              base_url:    { Some(msg.base_url.clone()) }, });
        }

        if config_updated {
            self.update_routing();
        }

        Ok(rv)
    }
}

impl FixedInstancesSupervisor {
    fn update_routing(&mut self) {
        let mut routing = FixedInstanceRoutingMap::new();

        for (driver_id, driver) in self.drivers.iter() {
            for (instance_id, instance) in driver.config.instances.iter() {
                if let Some(engine) = instance.engine.as_ref() {
                    let db = self.db.clone();
                    let model_id = instance_id.model_id();
                    if let Ok(Some(model)) = block_in_place(move || block_on(db.get_model(&model_id))) {
                        routing.insert(instance_id.clone(),
                                       FixedInstanceRouting { engine:         { engine.name.clone() },
                                                              send_count:     { model.get_audio_input_channel_count() },
                                                              send_channel:   { engine.input_start as usize },
                                                              return_count:   { model.get_audio_output_channel_count() },
                                                              return_channel: { engine.output_start as usize }, });
                    }
                }
            }
        }

        self.routing = routing.clone();
        self.issue_system_async(NotifyFixedInstanceRouting { routing })
    }
}
