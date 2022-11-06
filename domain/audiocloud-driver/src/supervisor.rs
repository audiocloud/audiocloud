/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use actix::fut::LocalBoxActorFuture;
use actix::{fut, Actor, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner, Handler, WrapFuture};
use actix_broker::BrokerSubscribe;
use serde_json::json;
use tracing::*;

use audiocloud_api::cloud::domains::{FixedInstanceConfig, InstanceDriverConfig};
use audiocloud_api::domain::DomainError;
use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverError, InstanceDriverEvent, InstanceDriverResult, InstanceParametersUpdated,
    InstanceWithStatus, InstanceWithStatusList,
};
use audiocloud_api::{
    now, DesiredInstancePlayState, FixedInstanceId, InstanceDriverId, InstanceParameters, InstancePlayState, InstanceReports, Timestamp,
    Timestamped,
};
use audiocloud_rust_clients::DomainServerClient;

use crate::driver::{DriverHandle, DriverRunner};
use crate::messages::{
    GetInstanceMsg, GetInstancesMsg, NotifyInstanceReportsMsg, SetDesiredStateMsg, SetInstanceDriverConfigMsg, SetParametersMsg,
};
use crate::nats;

pub struct DriverSupervisor {
    driver_id: InstanceDriverId,
    client:    Option<DomainServerClient>,
    config:    InstanceDriverConfig,
    instances: HashMap<FixedInstanceId, SupervisedDriver>,
}

pub struct SupervisedDriver {
    handle:             DriverHandle,
    config:             FixedInstanceConfig,
    parameters:         InstanceParameters,
    reports:            HashMap<Timestamp, InstanceReports>,
    desired_play_state: Timestamped<Option<DesiredInstancePlayState>>,
    actual_play_state:  Timestamped<Option<InstancePlayState>>,
}

impl Actor for DriverSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1), Self::register_and_update_config);
        ctx.run_interval(Duration::from_secs(1), Self::enact_config);
        ctx.run_interval(Duration::from_millis(100), Self::prune_old_reports);

        self.subscribe_system_async::<NotifyInstanceReportsMsg>(ctx);
    }
}

impl Handler<GetInstancesMsg> for DriverSupervisor {
    type Result = InstanceDriverResult<InstanceWithStatusList>;

    fn handle(&mut self, _msg: GetInstancesMsg, _ctx: &mut Self::Context) -> Self::Result {
        let mut rv = InstanceWithStatusList::new();

        for (instance_id, instance) in self.instances.iter() {
            rv.push(Self::instance_to_json(instance_id, instance));
        }

        Ok(rv)
    }
}

impl Handler<NotifyInstanceReportsMsg> for DriverSupervisor {
    type Result = ();

    fn handle(&mut self, msg: NotifyInstanceReportsMsg, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(instance) = self.instances.get_mut(&msg.instance_id) {
            instance.reports.insert(now(), msg.reports.clone());
        }
        nats::publish(&msg.instance_id, InstanceDriverEvent::Reports { reports: msg.reports });
    }
}

impl Handler<GetInstanceMsg> for DriverSupervisor {
    type Result = InstanceDriverResult<InstanceWithStatus>;

    fn handle(&mut self, msg: GetInstanceMsg, _ctx: &mut Self::Context) -> Self::Result {
        match self.instances.get(&msg.instance_id) {
            None => Err(InstanceDriverError::InstanceNotFound { instance: msg.instance_id }),
            Some(instance) => Ok(Self::instance_to_json(&msg.instance_id, instance)),
        }
    }
}

impl Handler<SetParametersMsg> for DriverSupervisor {
    type Result = LocalBoxActorFuture<Self, InstanceDriverResult<InstanceParametersUpdated>>;

    fn handle(&mut self, msg: SetParametersMsg, _ctx: &mut Self::Context) -> Self::Result {
        match self.instances.get(&msg.instance_id) {
            None => fut::err(InstanceDriverError::InstanceNotFound { instance: msg.instance_id }).into_actor(self)
                                                                                                 .boxed_local(),
            Some(instance) => instance.handle
                                      .clone()
                                      .set_parameters(msg.parameters)
                                      .into_actor(self)
                                      .map(move |res, actor, ctx| actor.handle_returned_set_parameters(res))
                                      .boxed_local(),
        }
    }
}

impl Handler<SetDesiredStateMsg> for DriverSupervisor {
    type Result = LocalBoxActorFuture<Self, InstanceDriverResult<DesiredInstancePlayStateUpdated>>;

    fn handle(&mut self, msg: SetDesiredStateMsg, _ctx: &mut Self::Context) -> Self::Result {
        match self.instances.get(&msg.instance_id) {
            None => fut::err(InstanceDriverError::InstanceNotFound { instance: msg.instance_id }).into_actor(self)
                                                                                                 .boxed_local(),
            Some(instance) => instance.handle
                                      .clone()
                                      .set_desired_play_state(msg.play_state)
                                      .into_actor(self)
                                      .map(move |res, actor, ctx| actor.handle_returned_set_desired_play_state(res))
                                      .boxed_local(),
        }
    }
}

impl Handler<SetInstanceDriverConfigMsg> for DriverSupervisor {
    type Result = InstanceDriverResult;

    fn handle(&mut self, msg: SetInstanceDriverConfigMsg, ctx: &mut Self::Context) -> Self::Result {
        self.config = msg.config;
        self.enact_config(ctx);

        Ok(())
    }
}

impl DriverSupervisor {
    fn handle_returned_set_parameters(&mut self,
                                      result: InstanceDriverResult<InstanceParametersUpdated>)
                                      -> InstanceDriverResult<InstanceParametersUpdated> {
        match result {
            Ok(InstanceParametersUpdated::Updated { id, parameters }) => {
                if let Some(instance) = self.instances.get_mut(&id) {
                    instance.parameters = parameters.clone();
                }
                Ok(InstanceParametersUpdated::Updated { id, parameters })
            }
            Err(err) => Err(err),
        }
    }

    fn handle_returned_set_desired_play_state(&mut self,
                                              result: InstanceDriverResult<DesiredInstancePlayStateUpdated>)
                                              -> InstanceDriverResult<DesiredInstancePlayStateUpdated> {
        match result {
            Ok(DesiredInstancePlayStateUpdated::Updated { id, desired, actual }) => {
                if let Some(instance) = self.instances.get_mut(&id) {
                    instance.desired_play_state = Some(desired.clone()).into();
                    instance.actual_play_state = Some(actual.clone()).into();
                }

                Ok(DesiredInstancePlayStateUpdated::Updated { id, desired, actual })
            }
            Err(err) => Err(err),
        }
    }

    fn enact_config(&mut self, ctx: &mut Context<Self>) {
        self.shutdown_unneeded_drivers();

        self.create_or_update_drivers(ctx)
    }

    fn prune_old_reports(&mut self, _ctx: &mut Context<Self>) {
        let threshold = now() - chrono::Duration::seconds(10);
        for instance in self.instances.values_mut() {
            instance.reports.retain(|id, _| *id >= threshold);
        }
    }

    fn create_or_update_drivers(&mut self, ctx: &mut Context<DriverSupervisor>) {
        for (instance_id, config) in &mut self.config.instances {
            if config.maintenance.iter().any(|m| m.time.contains_now()) {
                continue;
            };

            let maybe_existing = self.instances.get(&instance_id);
            let should_replace = maybe_existing.map(|instance| &instance.config != config).unwrap_or(true);

            if !should_replace {
                continue;
            }

            let parameters = maybe_existing.map(|instance| instance.parameters.clone())
                                           .unwrap_or_else(|| json!({}));
            let reports = maybe_existing.map(|instance| instance.reports.clone()).unwrap_or_default();
            let desired_play_state = maybe_existing.map(|instance| instance.desired_play_state.clone())
                                                   .unwrap_or_default();
            let actual_play_state = maybe_existing.map(|instance| instance.actual_play_state.clone())
                                                  .unwrap_or_default();

            let handle = match new_driver_handle(instance_id, config) {
                Ok(driver) => driver,
                Err(error) => {
                    warn!(%error, %instance_id, ?config, "Could not create driver");
                    return;
                }
            };

            ctx.notify(SetParametersMsg { instance_id: { instance_id.clone() },
                                          parameters:  { parameters.clone() }, });

            if let Some(desired_play_state) = desired_play_state.value_copy().as_ref() {
                ctx.notify(SetDesiredStateMsg { instance_id: { instance_id.clone() },
                                                play_state:  { desired_play_state.clone() }, });
            }

            self.instances.insert(instance_id.clone(),
                                  SupervisedDriver { handle:             { handle },
                                                     config:             { config.clone() },
                                                     parameters:         { parameters },
                                                     reports:            { reports },
                                                     desired_play_state: { desired_play_state },
                                                     actual_play_state:  { actual_play_state }, });
        }
    }

    fn shutdown_unneeded_drivers(&mut self) {
        let mut kill_list = HashSet::new();

        kill_list.extend(self.config
                             .instances
                             .iter()
                             .filter_map(|(id, config)| {
                                 if config.maintenance.iter().any(|maintenance| maintenance.time.contains_now()) {
                                     Some(id)
                                 } else {
                                     None
                                 }
                             })
                             .cloned());

        kill_list.extend(self.instances.keys().filter(|id| !self.config.instances.contains_key(id)).cloned());

        for id in kill_list {
            self.instances.remove(&id);
        }
    }

    fn register_and_update_config(&mut self, ctx: &mut Context<Self>) {
        if let Some(client) = self.client.clone() {
            let driver_id = self.driver_id.clone();
            let config = self.config.clone();
            let fut = async move { client.register_instance_driver(&driver_id, &config).await }.into_actor(self);
            fut.map(Self::handle_returned_registration_config).spawn(ctx);
        }
    }

    fn handle_returned_registration_config(result: Result<InstanceDriverConfig, DomainError>, actor: &mut Self, ctx: &mut Context<Self>) {
        match result {
            Ok(config) => {
                ctx.notify(SetInstanceDriverConfigMsg { config });
            }
            Err(error) => {
                warn!(%error, driver_id = %actor.driver_id, "Could not register instance driver");
            }
        }
    }

    fn instance_to_json(instance_id: &FixedInstanceId, instance: &SupervisedDriver) -> InstanceWithStatus {
        InstanceWithStatus { id:                 { instance_id.clone() },
                             parameters:         { instance.parameters.clone() },
                             reports:            { instance.reports.clone() },
                             desired_play_state: { instance.desired_play_state.clone() },
                             actual_play_state:  { instance.actual_play_state.clone() }, }
    }
}

fn new_driver_handle(id: &FixedInstanceId, config: &FixedInstanceConfig) -> InstanceDriverResult<DriverHandle> {
    use audiocloud_models as models;
    match (id.manufacturer.as_str(), id.name.as_str()) {
        #[cfg(unix)]
        (models::distopik::NAME, models::distopik::dual1084::NAME) => {
            let driver = crate::distopik::dual_1084::Dual1084::new(id.clone(),
                                                                   crate::distopik::dual_1084::Config::from_json(config.additional
                                                                                                                       .clone())?)?;
            Ok(DriverRunner::run(id.clone(), driver))
        }
        (models::netio::NAME, models::netio::power_pdu_4c::NAME) => {
            let driver = crate::netio::power_pdu_4c::PowerPdu4c::new(id.clone(),
                                                                     crate::netio::power_pdu_4c::Config::from_json(config.additional
                                                                                                                         .clone())?)?;
            Ok(DriverRunner::run(id.clone(), driver))
        }
        (manufacturer, name) => Err(InstanceDriverError::DriverNotSupported { manufacturer: manufacturer.to_string(),
                                                                              name:         name.to_string(), }),
    }
}

pub fn init(driver_id: InstanceDriverId, client: Option<DomainServerClient>, config: InstanceDriverConfig) -> Addr<DriverSupervisor> {
    DriverSupervisor { driver_id: { driver_id },
                       client:    { client },
                       config:    { config },
                       instances: { Default::default() }, }.start()
}
