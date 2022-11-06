/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

#![allow(unused_variables)]

use std::time::Duration;

use actix::{Actor, ActorFutureExt, AsyncContext, Context, ContextFutureSpawner, StreamHandler, WrapFuture, WrapStream};
use actix_broker::BrokerIssue;
use futures::FutureExt;
use reqwest::Url;
use tracing::*;

use audiocloud_api::cloud::domains::FixedInstanceConfig;
use audiocloud_api::domain::DomainError;
use audiocloud_api::instance_driver::InstanceDriverEvent;
use audiocloud_api::{FixedInstanceId, InstancePlayState, InstanceReports, Model, Timestamped};
use audiocloud_rust_clients::InstanceDriverClient;
use media::Media;
use power::Power;

use crate::fixed_instances::{get_instance_supervisor, NotifyFixedInstanceReports, NotifyInstanceState};
use crate::remote_value::RemoteValue;
use crate::tasks::NotifyTaskSpec;
use crate::{nats, DomainResult};

mod media;
mod merge_parameters;
mod on_instance_driver_event;
mod on_instance_driver_url;
mod on_instance_reports;
mod on_task_deleted;
mod on_task_spec_changed;
mod power;
mod set_desired_play_state;
mod set_parameters;

pub struct FixedInstanceActor {
    id:              FixedInstanceId,
    connected:       Timestamped<bool>,
    config:          FixedInstanceConfig,
    power:           Option<Power>,
    media:           Option<Media>,
    spec:            Timestamped<Option<NotifyTaskSpec>>,
    parameters:      RemoteValue<serde_json::Value>,
    instance_client: InstanceDriverClient,
    model:           Model,
}

pub struct ActorHandle {
    id: FixedInstanceId,
}

impl FixedInstanceActor {
    pub fn new(id: FixedInstanceId, client_url: Option<Url>, config: FixedInstanceConfig, model: Model) -> DomainResult<Self> {
        let power = config.power.clone().map(Power::new);
        let media = config.media.clone().map(Media::new);
        let parameters = model.default_parameter_values();

        let instance_driver_err = |error| DomainError::InstanceDriver { error,
                                                                        instance_id: id.clone() };

        Ok(Self { id:              { id.clone() },
                  connected:       { Timestamped::new(false) },
                  config:          { config },
                  power:           { power },
                  media:           { media },
                  model:           { model },
                  spec:            { Default::default() },
                  instance_client: { InstanceDriverClient::new(client_url).map_err(instance_driver_err)? },
                  parameters:      { RemoteValue::new(parameters) }, })
    }

    fn force_update_parameters(&mut self) {
        self.parameters.flush();
    }

    fn on_instance_driver_reports(&mut self, reports: InstanceReports) {
        self.issue_system_async(NotifyFixedInstanceReports { instance_id: self.id.clone(),
                                                             reports });
    }

    fn on_instance_driver_connected(&mut self, ctx: &mut Context<FixedInstanceActor>) {
        self.force_update_parameters();
    }

    fn on_instance_driver_play_state_changed(&mut self, current: InstancePlayState, media_pos: Option<f64>) {
        if let Some(media) = self.media.as_mut() {
            media.on_instance_play_state_changed(current, media_pos);
        }
    }
}

impl Actor for FixedInstanceActor {
    type Context = Context<Self>;

    #[instrument(skip_all)]
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_millis(30), Self::update);
        self.subscribe_instance_driver_events(ctx);
    }
}

impl FixedInstanceActor {
    #[instrument(skip_all)]
    fn subscribe_instance_driver_events(&mut self, ctx: &mut Context<Self>) {
        ctx.add_stream(nats::subscribe_json::<InstanceDriverEvent>(self.id.driver_event_subject()));
    }
}

impl FixedInstanceActor {
    fn update(&mut self, ctx: &mut Context<Self>) {
        if let Some(power) = self.power.as_mut() {
            if let Some(set_power) = power.update(&self.spec) {
                get_instance_supervisor().send(set_power).map(drop).into_actor(self).spawn(ctx);
            }
        }

        if let Some(media) = self.media.as_mut() {
            if let Some((version, set_media)) = media.update() {
                let client = self.instance_client.clone();
                let instance_id = self.id.clone();
                let fut = async move { client.set_desired_play_state(&instance_id, &set_media).await };

                fut.into_actor(self)
                   .map(move |result, actor, ctx| {
                       if let Some(media) = &mut actor.media {
                           media.finish_update(version, result.is_ok());
                       }
                   })
                   .spawn(ctx);
            }
        }
    }

    fn emit_instance_state(&self, ctx: &mut <Self as Actor>::Context) {
        self.issue_system_async(NotifyInstanceState { instance_id: self.id.clone(),
                                                      power:       self.power.as_ref().map(|power| power.get_power_state()),
                                                      play:        self.media.as_ref().map(|media| media.get_play_state()),
                                                      connected:   self.connected, });
    }
}
