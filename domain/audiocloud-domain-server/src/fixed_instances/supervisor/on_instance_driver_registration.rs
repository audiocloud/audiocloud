/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;
use std::time::Instant;

use actix::fut::LocalBoxActorFuture;
use actix::{fut, ActorFutureExt, Handler, WrapFuture};
use actix_broker::BrokerIssue;
use futures::executor::block_on;
use tokio::task::block_in_place;
use tracing::*;

use audiocloud_api::cloud::domains::{FixedInstanceConfig, FixedInstanceRouting, FixedInstanceRoutingMap, TimestampedInstanceDriverConfig};
use audiocloud_api::{cloud::domains::InstanceDriverConfig, FixedInstanceId, InstanceDriverId, Timestamped};

use crate::config::NotifyFixedInstanceRouting;
use crate::db::Db;
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
            Self::generate_routing(self.db.clone(), self.drivers.clone()).into_actor(self).map(move |routing, actor, ctx| {
                actor.routing = routing;
                ctx.issue_system_async(NotifyFixedInstanceRouting { routing: { actor.routing.clone() }, });
                Ok(rv)
            }).boxed_local()
        } else {
            fut::ready(Ok(rv)).into_actor(self).boxed_local()
        }
    }
}

impl FixedInstancesSupervisor {
    async fn generate_routing(db: Db,
                              drivers: HashMap<InstanceDriverId, SupervisedInstanceDriver>)
                              -> HashMap<FixedInstanceId, FixedInstanceRouting> {
        let mut routing = FixedInstanceRoutingMap::new();

        for (driver_id, driver) in drivers.iter() {
            for (instance_id, instance) in driver.config.instances.iter() {
                if let Some(engine) = instance.engine.as_ref() {
                    let model_id = instance_id.model_id();
                    if let Ok(Some(model)) = db.get_model(&model_id).await {
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

        routing
    }
}
