#![allow(unused_variables)]

use std::time::Duration;

use actix::{Actor, ActorFutureExt, AsyncContext, Context, ContextFutureSpawner, Handler, StreamHandler, WrapFuture};
use actix_broker::BrokerIssue;
use futures::FutureExt;
use tracing::*;

use audiocloud_api::cloud::domains::FixedInstanceConfig;
use audiocloud_api::domain::DomainError;
use audiocloud_api::instance_driver::{InstanceDriverCommand, InstanceDriverEvent};
use audiocloud_api::{
    FixedInstanceId, InstancePlayState, InstanceReports, Model, ModelCapability, PowerDistributorReports, Request, SerializableResult,
    Timestamped,
};

use crate::fixed_instances::values::merge_values;
use crate::fixed_instances::{
    get_instance_supervisor, NotifyFixedInstanceReports, NotifyInstancePowerChannelsChanged, NotifyInstanceState, SetDesiredPowerChannel,
    SetInstanceDesiredPlayState, SetInstanceParameters,
};
use crate::tasks::{NotifyTaskDeleted, NotifyTaskSpec};
use crate::{nats, DomainResult};

use super::media::Media;
use super::power::Power;

pub struct InstanceActor {
    id:                  FixedInstanceId,
    connected:           Timestamped<bool>,
    config: FixedInstanceConfig,
    power:               Option<Power>,
    media:               Option<Media>,
    spec:                Timestamped<Option<NotifyTaskSpec>>,
    parameters:          serde_json::Value,
    instance_driver_cmd: String,
    model:               Model,
}

impl InstanceActor {
    pub fn new(id: FixedInstanceId, config: FixedInstanceConfig, model: Model) -> anyhow::Result<Self> {
        let power = config.power.clone().map(Power::new);
        let media = config.media.clone().map(Media::new);
        let instance_driver_cmd = id.driver_command_subject();

        Ok(Self { id:                  { id },
                  connected:           { Timestamped::new(false) },
                  config:              { config },
                  power:               { power },
                  media:               { media },
                  model:               { model },
                  spec:                { Default::default() },
                  parameters:          { Default::default() },
                  instance_driver_cmd: { instance_driver_cmd }, })
    }

    fn on_instance_driver_reports(&mut self, reports: InstanceReports) {
        if self.model.capabilities.contains(&ModelCapability::PowerDistributor) {
            match serde_json::from_value::<PowerDistributorReports>(reports.clone()) {
                Ok(reports) => {
                    if let Some(power_channels) = reports.power {
                        let change_msg = NotifyInstancePowerChannelsChanged { instance_id: self.id.clone(),
                                                                              power:       power_channels, };
                        self.issue_system_async(change_msg);
                    }
                }
                Err(_) => {}
            }
        }

        self.issue_system_async(NotifyFixedInstanceReports { instance_id: self.id.clone(),
                                                             reports });
    }

    fn on_instance_driver_connected(&mut self, ctx: &mut Context<InstanceActor>) {
        // set current parameters
        self.request_instance_driver(InstanceDriverCommand::SetParameters(self.parameters.clone()), ctx);
    }

    fn on_instance_driver_play_state_changed(&mut self, current: InstancePlayState, media_pos: Option<f64>) {
        if let Some(media) = self.media.as_mut() {
            media.on_instance_play_state_changed(current, media_pos);
        }
    }
}

impl Actor for InstanceActor {
    type Context = Context<Self>;

    #[instrument(skip_all)]
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_millis(30), Self::update);
        self.subscribe_instance_driver_events(ctx);
    }
}

impl InstanceActor {
    #[instrument(skip_all)]
    fn subscribe_instance_driver_events(&mut self, ctx: &mut Context<Self>) {
        ctx.add_stream(nats::subscribe_json::<InstanceDriverEvent>(self.id.driver_event_subject()));
    }
}

impl Handler<NotifyTaskSpec> for InstanceActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskSpec, ctx: &mut Self::Context) -> Self::Result {
        if msg.spec.get_fixed_instance_ids().contains(&self.id) {
            self.spec = Some(msg).into();
            self.update(ctx);
        }
    }
}

impl Handler<NotifyTaskDeleted> for InstanceActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyTaskDeleted, ctx: &mut Self::Context) -> Self::Result {
        if self.spec.value().as_ref().map(|prev_notify| &prev_notify.task_id == &msg.task_id) == Some(true) {
            self.spec = None.into();
        }
    }
}

impl Handler<SetInstanceParameters> for InstanceActor {
    type Result = DomainResult;

    fn handle(&mut self, msg: SetInstanceParameters, ctx: &mut Self::Context) -> Self::Result {
        merge_values(&mut self.parameters, msg.parameters);

        Ok(())
    }
}

impl Handler<SetDesiredPowerChannel> for InstanceActor {
    type Result = DomainResult;

    fn handle(&mut self, msg: SetDesiredPowerChannel, ctx: &mut Self::Context) -> Self::Result {
        // TODO: if we support it, we support it
        Ok(())
    }
}

impl Handler<SetInstanceDesiredPlayState> for InstanceActor {
    type Result = DomainResult<()>;

    fn handle(&mut self, msg: SetInstanceDesiredPlayState, ctx: &mut Self::Context) -> Self::Result {
        if let Some(media) = self.media.as_mut() {
            media.set_desired_state(msg.desired);
            Ok(())
        } else {
            Err(DomainError::InstanceNotCapable { instance_id: self.id.clone(),
                                                  operation:   format!("SetInstanceDesiredPlayState"), })
        }
    }
}

impl Handler<NotifyInstancePowerChannelsChanged> for InstanceActor {
    type Result = ();

    fn handle(&mut self, msg: NotifyInstancePowerChannelsChanged, ctx: &mut Self::Context) -> Self::Result {
        if let Some(power) = self.power.as_mut() {
            power.on_instance_power_channels_changed(msg);
        }
    }
}

impl StreamHandler<InstanceDriverEvent> for InstanceActor {
    fn handle(&mut self, item: InstanceDriverEvent, ctx: &mut Self::Context) {
        match item {
            InstanceDriverEvent::Started => {}
            InstanceDriverEvent::IOError { .. } => {}
            InstanceDriverEvent::ConnectionLost => {
                self.connected = false.into();
            }
            InstanceDriverEvent::Connected => {
                self.connected = true.into();
                self.on_instance_driver_connected(ctx);
            }
            InstanceDriverEvent::Reports { reports } => {
                self.on_instance_driver_reports(reports);
            }
            InstanceDriverEvent::PlayState { desired,
                                             current,
                                             media: media_pos, } => self.on_instance_driver_play_state_changed(current, media_pos),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        self.subscribe_instance_driver_events(ctx);
    }
}

impl InstanceActor {
    fn update(&mut self, ctx: &mut Context<Self>) {
        if let Some(power) = self.power.as_mut() {
            if let Some(set_power) = power.update(&self.spec) {
                get_instance_supervisor().send(set_power).map(drop).into_actor(self).spawn(ctx)
            }
        }
        if let Some(media) = self.media.as_mut() {
            if let Some(cmd) = media.update() {
                self.request_instance_driver(cmd, ctx);
            }
        }
    }

    fn request_instance_driver(&self, driver: InstanceDriverCommand, ctx: &mut <Self as Actor>::Context) {
        nats::request_json(self.instance_driver_cmd.to_string(), driver).into_actor(self)
                                                                        .map(Self::on_instance_driver_response)
                                                                        .spawn(ctx);
    }

    fn on_instance_driver_response(response: anyhow::Result<<InstanceDriverCommand as Request>::Response>,
                                   actor: &mut Self,
                                   ctx: &mut <Self as Actor>::Context) {
        let instance = &actor.id;
        match response {
            Ok(result) => match result {
                SerializableResult::Ok(_) => {}
                SerializableResult::Error(error) => {
                    warn!(%instance, %error, "Instance driver responded with error");
                }
            },
            Err(error) => {
                warn!(%instance, %error, "Failed to send command to instance driver");
            }
        }
    }

    fn emit_instance_state(&self, ctx: &mut <Self as Actor>::Context) {
        let x = NotifyInstanceState { instance_id: self.id.clone(),
                                      power:       self.power.as_ref().map(|power| power.get_power_state()),
                                      play:        self.media.as_ref().map(|media| media.get_play_state()),
                                      connected:   self.connected, };
    }
}
