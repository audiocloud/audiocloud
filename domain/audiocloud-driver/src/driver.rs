use std::time::Duration;

use actix::{Actor, AsyncContext, Context, Handler, Recipient, Supervised};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::*;

use audiocloud_api::instance_driver::{InstanceDriverCommand, InstanceDriverError, InstanceDriverEvent};
use audiocloud_api::{FixedInstanceId, PlayId, RenderId};

use crate::{emit_event, Command};

pub type Result = std::result::Result<(), InstanceDriverError>;

pub trait Driver: Unpin + Sized + 'static {
    type Params: DeserializeOwned;
    type Reports: Serialize;

    fn on_parameters_changed(&mut self, params: Self::Params) -> Result {
        drop(params);
        Ok(())
    }

    fn play(&mut self, play_id: PlayId) -> Result {
        drop(play_id);
        Err(InstanceDriverError::MediaNotPresent)
    }

    fn render(&mut self, render_id: RenderId, length: f64) -> Result {
        drop((render_id, length));
        Err(InstanceDriverError::MediaNotPresent)
    }

    fn rewind(&mut self, to: f64) -> Result {
        drop(to);
        Err(InstanceDriverError::MediaNotPresent)
    }

    fn stop(&mut self) -> Result {
        Err(InstanceDriverError::MediaNotPresent)
    }

    fn check_connection(&mut self) -> Result {
        Ok(())
    }

    fn set_power_channel(&mut self, channel: usize, value: bool) -> Result {
        drop((channel, value));
        Err(InstanceDriverError::NotPowerController)
    }

    fn restarted(&mut self) {}

    fn poll(&mut self) -> Option<Duration> {
        None
    }

    #[instrument(skip(reports), err)]
    fn emit_reports(instance_id: FixedInstanceId, reports: Self::Reports) -> Result {
        let reports = serde_json::to_value(&reports).map_err(|e| InstanceDriverError::ReportsMalformed { error: e.to_string() })?;

        emit_event(instance_id, InstanceDriverEvent::Reports { reports });

        Ok(())
    }
}

pub struct DriverActor<D>
    where D: Driver
{
    driver: D,
}

impl<D> DriverActor<D> where D: Driver
{
    pub fn start_recipient(driver: D) -> Recipient<Command> {
        Self { driver }.start().recipient()
    }

    fn update(&mut self, ctx: &mut Context<Self>) {
        if let Some(when) = self.driver.poll() {
            ctx.run_later(when, Self::update);
        }
    }
}

impl<D> Actor for DriverActor<D> where D: Driver
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.restarting(ctx);
    }
}

impl<D> Supervised for DriverActor<D> where D: Driver
{
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.driver.restarted();
        ctx.run_later(Duration::default(), Self::update);
    }
}

impl<D> Handler<Command> for DriverActor<D> where D: Driver
{
    type Result = Result;

    fn handle(&mut self, msg: Command, _ctx: &mut Self::Context) -> Self::Result {
        match msg.command {
            InstanceDriverCommand::CheckConnection => {
                self.driver.check_connection()?;
            }
            InstanceDriverCommand::Stop => {
                self.driver.stop()?;
            }
            InstanceDriverCommand::Play { play_id } => {
                self.driver.play(play_id)?;
            }
            InstanceDriverCommand::Render { length, render_id } => {
                self.driver.render(render_id, length)?;
            }
            InstanceDriverCommand::Rewind { to } => {
                self.driver.rewind(to)?;
            }
            InstanceDriverCommand::SetParameters(parameters) => {
                let parameters =
                    serde_json::from_value(parameters).map_err(|e| InstanceDriverError::ParametersMalformed { error: e.to_string() })?;
                self.driver.on_parameters_changed(parameters)?;
            }
            InstanceDriverCommand::SetPowerChannel { channel, power } => {
                self.driver.set_power_channel(channel, power)?;
            }
        }

        Ok(())
    }
}
