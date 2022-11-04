/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use actix::{spawn, Actor, AsyncContext, Context, Handler};
use actix_broker::{BrokerIssue, BrokerSubscribe};
use itertools::Itertools;
use tracing::*;

use audiocloud_api::cloud::domains::{DomainConfig, InstanceDriverConfig};
use audiocloud_api::{InstanceDriverId, Json};

use crate::config::NotifyDomainConfiguration;
use crate::fixed_instances::{NotifyInstanceDriverUrl, RegisterInstanceDriver};
use crate::DomainResult;

pub struct DriversSupervisor {
    config:     DomainConfig,
    registered: HashMap<InstanceDriverId, SupervisedInstanceDriver>,
}

#[derive(Clone, Debug)]
struct SupervisedInstanceDriver {
    provided:  InstanceDriverConfig,
    total:     InstanceDriverConfig,
    last_seen: Instant,
}

impl DriversSupervisor {
    pub fn new(config: DomainConfig) -> Self {
        Self { config:     { config },
               registered: { HashMap::new() }, }
    }
}

impl Actor for DriversSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1), Self::update);
        self.subscribe_system_async::<NotifyDomainConfiguration>(ctx);
    }
}

impl Handler<RegisterInstanceDriver> for DriversSupervisor {
    type Result = DomainResult<InstanceDriverConfig>;

    #[instrument(skip_all, err, fields(driver_id = %msg.driver_id), name = "register_instance_driver")]
    fn handle(&mut self, msg: RegisterInstanceDriver, _ctx: &mut Self::Context) -> Self::Result {
        let result = self.registered
                         .entry(msg.driver_id.clone())
                         .or_insert_with(|| SupervisedInstanceDriver { provided:  { msg.provided.clone() },
                                                                       total:     { msg.provided.clone() },
                                                                       last_seen: { Instant::now() }, });

        result.last_seen = Instant::now();
        let rv = result.total.clone();

        for instance in self.registered
                            .get(&msg.driver_id)
                            .iter()
                            .flat_map(|driver| driver.total.instances.keys())
        {
            self.issue_system_async(NotifyInstanceDriverUrl { instance_id: { instance.clone() },
                                                              base_url:    { Some(msg.base_url.clone()) }, });
        }

        Ok(rv)
    }
}

impl Handler<NotifyDomainConfiguration> for DriversSupervisor {
    type Result = ();

    #[instrument(skip_all, name = "notify_domain_configuration")]
    fn handle(&mut self, msg: NotifyDomainConfiguration, _ctx: &mut Self::Context) -> Self::Result {
        self.config = msg.config;
        for driver in self.registered.values_mut() {
            driver.total = driver.provided.clone();
        }

        for (id, instance) in &self.config.fixed_instances {
            if let Some(driver) = instance.driver.as_ref() {
                if let Some(driver) = self.registered.get_mut(driver) {
                    driver.total.instances.insert(id.clone(), instance.clone());
                }
            }
        }
    }
}

impl DriversSupervisor {
    #[instrument(skip_all, name = "drivers_supervisor_update")]
    fn update(&mut self, _ctx: &mut Context<Self>) {
        let mut events = vec![];
        let check_driver = |driver_id: &InstanceDriverId, driver: &mut SupervisedInstanceDriver| {
            if driver.last_seen.elapsed() < Duration::from_secs(15) {
                warn!(%driver_id, "Driver has not checked in for some time, removing");
                for (id, instance) in &self.config.fixed_instances {
                    if instance.driver.as_ref() == Some(driver_id) {
                        events.push(NotifyInstanceDriverUrl { instance_id: { id.clone() },
                                                              base_url:    { None }, });
                    }
                }

                false
            } else {
                true
            }
        };

        self.registered.retain(check_driver);

        for event in events {
            self.issue_system_async(event);
        }
    }
}
