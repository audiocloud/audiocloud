/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;
use std::time::Instant;

use actix::Handler;
use actix_broker::BrokerIssue;
use tracing::*;

use audiocloud_api::{cloud::domains::InstanceDriverConfig, Timestamped};

use crate::fixed_instances::NotifyInstanceDriverUrl;
use crate::{fixed_instances::RegisterInstanceDriver, DomainResult};

use super::{FixedInstancesSupervisor, SupervisedInstanceDriver};

impl Handler<RegisterInstanceDriver> for FixedInstancesSupervisor {
    type Result = DomainResult<InstanceDriverConfig>;

    #[instrument(skip_all, err, fields(driver_id = %msg.driver_id), name = "register_instance_driver")]
    fn handle(&mut self, mut msg: RegisterInstanceDriver, _ctx: &mut Self::Context) -> Self::Result {
        for (instance_id, instance_config) in &mut msg.provided.instances {
            instance_config.driver = Some(msg.driver_id.clone());

            self.config
                .fixed_instances
                .entry(instance_id.clone())
                .or_default()
                .merge_from_driver(instance_config);
        }

        let result = self.drivers
                         .entry(msg.driver_id.clone())
                         .or_insert_with(|| SupervisedInstanceDriver { config:    { msg.provided.clone() },
                                                                       instances: { HashMap::new() },
                                                                       last_seen: { Instant::now() },
                                                                       online:    { Timestamped::new(true) },
                                                                       url:       { msg.base_url.clone() }, });

        result.last_seen = Instant::now();
        let rv = result.config.clone();

        for instance in self.drivers
                            .get(&msg.driver_id)
                            .iter()
                            .flat_map(|driver| driver.config.instances.keys())
        {
            self.issue_system_async(NotifyInstanceDriverUrl { instance_id: { instance.clone() },
                                                              base_url:    { Some(msg.base_url.clone()) }, });
        }

        Ok(rv)
    }
}
