/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::HashMap;
use std::time::{Duration, Instant};

use actix::{Actor, Addr, AsyncContext, Context, Handler, MessageResult};
use actix_broker::BrokerSubscribe;
use reqwest::Url;
use tracing::*;

use audiocloud_api::cloud::domains::{DomainConfig, FixedInstanceConfig, InstanceDriverConfig, TimestampedInstanceDriverConfig};
use audiocloud_api::{FixedInstanceId, InstanceDriverId, Timestamped};

use crate::config::NotifyDomainConfiguration;
use crate::db::Db;
use crate::fixed_instances::instance::FixedInstanceActor;
use crate::fixed_instances::{GetMultipleFixedInstanceState, NotifyFixedInstanceReports, NotifyInstanceState};

mod forward_instance_reports;
mod forward_merge_parameters;
mod forward_set_parameters;
mod forward_set_play_state;
mod on_domain_config_change;
mod on_instance_driver_registration;
mod update_instance_actors;

pub struct FixedInstancesSupervisor {
    config:  DomainConfig,
    drivers: HashMap<InstanceDriverId, SupervisedInstanceDriver>,
    db:      Db,
}

#[derive(Clone, Debug)]
struct SupervisedInstanceDriver {
    config:    TimestampedInstanceDriverConfig,
    instances: HashMap<FixedInstanceId, SupervisedInstance>,
    last_seen: Instant,
    online:    Timestamped<bool>,
    url:       Url,
}

#[derive(Clone, Debug)]
struct SupervisedInstance {
    address: Addr<FixedInstanceActor>,
    config:  FixedInstanceConfig,
    state:   Option<NotifyInstanceState>,
}

impl FixedInstancesSupervisor {
    pub async fn new(boot: &DomainConfig, db: Db) -> anyhow::Result<Self> {
        Ok(Self { db:      { db },
                  drivers: { HashMap::new() },
                  config:  { boot.clone() }, })
    }
}

impl Actor for FixedInstancesSupervisor {
    type Context = Context<Self>;

    #[instrument(skip_all)]
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<NotifyDomainConfiguration>(ctx);
        self.subscribe_system_async::<NotifyFixedInstanceReports>(ctx);
        ctx.run_interval(Duration::from_secs(1), Self::update_instance_actors);
    }
}

impl Handler<GetMultipleFixedInstanceState> for FixedInstancesSupervisor {
    type Result = MessageResult<GetMultipleFixedInstanceState>;

    fn handle(&mut self, msg: GetMultipleFixedInstanceState, _ctx: &mut Self::Context) -> Self::Result {
        let mut rv = HashMap::new();

        for id in msg.instance_ids {
            for driver in self.drivers.values() {
                if let Some(instance) = driver.instances.get(&id) {
                    if let Some(state) = instance.state.clone() {
                        rv.insert(id.clone(), state);
                    }
                }
            }
        }

        MessageResult(rv)
    }
}
