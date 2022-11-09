/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use axum::async_trait;
use coerce::actor::context::ActorContext;
use coerce::actor::message::{Handler, Message};
use coerce::actor::{new_actor, Actor, ActorRef, LocalActorRef};
use once_cell::sync::OnceCell;
use serde_json::json;
use tokio::spawn;
use tokio::task::JoinHandle;
use tracing::*;

use audiocloud_api::cloud::domains::{FixedInstanceConfig, InstanceDriverConfig, TimestampedInstanceDriverConfig};
use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverError, InstanceDriverEvent, InstanceDriverResult, InstanceParametersUpdated,
    InstanceWithStatus, InstanceWithStatusList,
};
use audiocloud_api::{
    now, DesiredInstancePlayState, FixedInstanceId, InstanceDriverId, InstanceParameters, InstancePlayState, InstanceReports, Timestamp,
    Timestamped,
};
use audiocloud_rust_clients::DomainServerClient;

use crate::driver::DriverHandle;
use crate::messages::{
    GetInstanceMsg, GetInstancesMsg, NotifyInstanceReportsMsg, SetDesiredStateMsg, SetInstanceDriverConfigMsg, SetParametersMsg,
};
use crate::{nats, BroadcastSender};

static DRIVER_SUPERVISOR: OnceCell<LocalActorRef<DriverSupervisor>> = OnceCell::new();

pub fn get_driver_supervisor() -> &'static LocalActorRef<DriverSupervisor> {
    DRIVER_SUPERVISOR.get().expect("Driver supervisor not initialized")
}

pub struct DriverSupervisor {
    driver_id: InstanceDriverId,
    client:    Option<DomainServerClient>,
    config:    Timestamped<InstanceDriverConfig>,
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

#[derive(Clone, Copy)]
struct NotifyUpdateConfigMsg;

impl Message for NotifyUpdateConfigMsg {
    type Result = ();
}

#[derive(Clone, Copy)]
struct NotifyEnactConfigMsg;

impl Message for NotifyEnactConfigMsg {
    type Result = ();
}

#[derive(Clone, Copy)]
struct PruneOldReportsMsg;

impl Message for PruneOldReportsMsg {
    type Result = ();
}

#[async_trait]
impl Actor for DriverSupervisor {
    #[instrument(skip_all)]
    async fn started(&mut self, ctx: &mut ActorContext) {
        schedule_interval(ctx.actor_ref::<Self>(), Duration::from_secs(1), NotifyUpdateConfigMsg);
        schedule_interval(ctx.actor_ref::<Self>(), Duration::from_secs(1), NotifyEnactConfigMsg);
        schedule_interval(ctx.actor_ref::<Self>(), Duration::from_millis(100), PruneOldReportsMsg);
    }
}

#[async_trait]
impl Handler<GetInstancesMsg> for DriverSupervisor {
    async fn handle(&mut self, _: GetInstancesMsg, _: &mut ActorContext) -> InstanceDriverResult<InstanceWithStatusList> {
        let mut rv = InstanceWithStatusList::new();

        for (instance_id, instance) in self.instances.iter() {
            rv.push(Self::instance_to_json(instance_id, instance));
        }

        Ok(rv)
    }
}

#[async_trait]
impl Handler<NotifyInstanceReportsMsg> for DriverSupervisor {
    async fn handle(&mut self, msg: NotifyInstanceReportsMsg, _ctx: &mut ActorContext) {
        if let Some(instance) = self.instances.get_mut(&msg.instance_id) {
            instance.reports.insert(now(), msg.reports.clone());
        }
        nats::publish(&msg.instance_id, InstanceDriverEvent::Reports { reports: msg.reports });
    }
}

#[async_trait]
impl Handler<GetInstanceMsg> for DriverSupervisor {
    async fn handle(&mut self, msg: GetInstanceMsg, _ctx: &mut ActorContext) -> InstanceDriverResult<InstanceWithStatus> {
        match self.instances.get(&msg.instance_id) {
            None => Err(InstanceDriverError::InstanceNotFound { instance: msg.instance_id }),
            Some(instance) => Ok(Self::instance_to_json(&msg.instance_id, instance)),
        }
    }
}

#[async_trait]
impl Handler<SetParametersMsg> for DriverSupervisor {
    async fn handle(&mut self, msg: SetParametersMsg, _ctx: &mut ActorContext) -> InstanceDriverResult<InstanceParametersUpdated> {
        match self.instances.get(&msg.instance_id) {
            None => Err(InstanceDriverError::InstanceNotFound { instance: msg.instance_id }),
            Some(instance) => self.handle_returned_set_parameters(instance.handle.set_parameters(msg.parameters).await),
        }
    }
}

#[async_trait]
impl Handler<SetDesiredStateMsg> for DriverSupervisor {
    async fn handle(&mut self, msg: SetDesiredStateMsg, _ctx: &mut ActorContext) -> InstanceDriverResult<DesiredInstancePlayStateUpdated> {
        match self.instances.get(&msg.instance_id) {
            None => Err(InstanceDriverError::InstanceNotFound { instance: msg.instance_id }),
            Some(instance) => self.handle_returned_set_desired_play_state(instance.handle.set_desired_play_state(msg.play_state).await),
        }
    }
}

#[async_trait]
impl Handler<SetInstanceDriverConfigMsg> for DriverSupervisor {
    async fn handle(&mut self, msg: SetInstanceDriverConfigMsg, ctx: &mut ActorContext) -> InstanceDriverResult {
        // compare and update
        if msg.config.timestamp() > self.config.timestamp() {
            self.config = msg.config;
            self.enact_config(ctx).await;
        }

        Ok(())
    }
}

#[async_trait]
impl Handler<NotifyUpdateConfigMsg> for DriverSupervisor {
    async fn handle(&mut self, _: NotifyUpdateConfigMsg, ctx: &mut ActorContext) {
        if let Some(client) = self.client.clone() {
            let driver_id = self.driver_id.clone();
            let config = self.config.clone();

            match client.register_instance_driver(&driver_id, &config).await {
                Ok(config) => {
                    let _ = ctx.actor_ref::<Self>().notify(SetInstanceDriverConfigMsg { config });
                }
                Err(error) => {
                    warn!(%error, driver_id = %self.driver_id, "Could not register instance driver");
                }
            }
        }
    }
}

#[async_trait]
impl Handler<NotifyEnactConfigMsg> for DriverSupervisor {
    async fn handle(&mut self, _: NotifyEnactConfigMsg, ctx: &mut ActorContext) {
        self.enact_config(ctx).await;
    }
}

#[async_trait]
impl Handler<PruneOldReportsMsg> for DriverSupervisor {
    async fn handle(&mut self, _: PruneOldReportsMsg, _: &mut ActorContext) {
        let threshold = now() - chrono::Duration::seconds(10);
        for instance in self.instances.values_mut() {
            instance.reports.retain(|id, _| *id >= threshold);
        }
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

    async fn enact_config(&mut self, ctx: &mut ActorContext) {
        self.shutdown_unneeded_drivers();

        self.create_or_update_drivers(ctx).await;
    }

    async fn create_or_update_drivers(&mut self, ctx: &mut ActorContext) {
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

            let handle = match DriverHandle::new(instance_id.clone(), config.clone()).await {
                Ok(driver) => driver,
                Err(error) => {
                    warn!(%error, %instance_id, ?config, "Could not create driver");
                    return;
                }
            };

            let _ = ctx.actor_ref::<Self>()
                       .notify(SetParametersMsg { instance_id: { instance_id.clone() },
                                                  parameters:  { parameters.clone() }, });

            if let Some(desired_play_state) = desired_play_state.value_copied().as_ref() {
                let _ = ctx.actor_ref::<Self>()
                           .notify(SetDesiredStateMsg { instance_id: { instance_id.clone() },
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

    fn instance_to_json(instance_id: &FixedInstanceId, instance: &SupervisedDriver) -> InstanceWithStatus {
        InstanceWithStatus { id:                 { instance_id.clone() },
                             parameters:         { instance.parameters.clone() },
                             reports:            { instance.reports.clone() },
                             desired_play_state: { instance.desired_play_state.clone() },
                             actual_play_state:  { instance.actual_play_state.clone() }, }
    }
}

pub async fn init(driver_id: InstanceDriverId,
                  client: Option<DomainServerClient>,
                  config: TimestampedInstanceDriverConfig)
                  -> LocalActorRef<DriverSupervisor> {
    let actor_ref: LocalActorRef<DriverSupervisor> =
        new_actor(DriverSupervisor { driver_id: { driver_id },
                                     client:    { client },
                                     config:    { config },
                                     instances: { Default::default() }, }).await
                                                                          .expect("Could not create driver supervisor")
                                                                          .into();
    DRIVER_SUPERVISOR.set(actor_ref.clone()).expect("Could not set driver supervisor");
    actor_ref
}

fn schedule_interval<A: Actor, M: Message + Copy>(actor: impl Into<ActorRef<A>>, interval: Duration, msg: M) -> JoinHandle<()>
    where A: Handler<M>
{
    let actor = actor.into();
    spawn(async move {
        let mut interval = tokio::time::interval(interval);
        loop {
            interval.tick().await;
            if let Err(_) = actor.send(msg).await {
                break;
            }
        }
    })
}

fn subscribe<A: Actor, M: Message + Clone>(actor: impl Into<ActorRef<A>>, sender: &BroadcastSender<M>) -> JoinHandle<()>
    where A: Handler<M>
{
    let actor = actor.into();
    let mut receiver = sender.subscribe();
    spawn(async move {
        loop {
            match receiver.recv().await {
                Ok(message) => match actor.send(message).await {
                    Ok(_) => {}
                    Err(_) => break,
                },
                Err(error) => {
                    trace!(%error, "Subscription to broadcast channel closed");
                    break;
                }
            }
        }
    })
}
