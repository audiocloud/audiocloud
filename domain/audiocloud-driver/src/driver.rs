use std::future::Future;
use std::result::Result as R;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::Shared;
use futures::FutureExt;

use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::spawn;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::oneshot::{Receiver as OneShotReceiver, Sender as OneShotSender};
use tokio::time::error::Elapsed;
use tokio::time::{timeout};
use tracing::*;

use audiocloud_api::instance_driver::{
    DesiredInstancePlayStateUpdated, InstanceDriverError, InstanceDriverEvent, InstanceParametersUpdated,
};
use audiocloud_api::{DesiredInstancePlayState, FixedInstanceId, InstancePlayState, PlayId, RenderId};

use crate::nats;


pub type Result<T = ()> = std::result::Result<T, InstanceDriverError>;

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

    fn get_actual_play_state(&self) -> InstancePlayState {
        InstancePlayState::Stopped { position: None }
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

        nats::publish(&instance_id, InstanceDriverEvent::Reports { reports });

        Ok(())
    }
}

pub struct DriverRunner<T: Driver> {
    driver:                    T,
    instance_id:               FixedInstanceId,
    set_desired_play_state_rx: HandleReceiver<DesiredInstancePlayState, DesiredInstancePlayStateUpdated>,
    set_parameters_rx:         HandleReceiver<serde_json::Value, InstanceParametersUpdated>,
    shut_down:                 Shared<OneShotReceiver<()>>,
    next_poll_at:              Instant,
}

impl<T> DriverRunner<T> where T: Driver + Send
{
    pub fn run(instance_id: FixedInstanceId, driver: T) -> DriverHandle {
        let (set_parameters_tx, set_parameters_rx) = unbounded_channel();
        let (set_desired_play_state_tx, set_desired_play_state_rx) = unbounded_channel();
        let (shut_down_tx, shut_down_rx) = oneshot::channel();
        let next_poll_at = Instant::now();

        let mut host = DriverRunner { driver:                    { driver },
                                      instance_id:               { instance_id },
                                      set_desired_play_state_rx: { set_desired_play_state_rx },
                                      set_parameters_rx:         { set_parameters_rx },
                                      shut_down:                 { shut_down_rx.shared() },
                                      next_poll_at:              { next_poll_at }, };

        spawn(async move {
            loop {
                if !host.run_one().await {
                    break;
                }
            }
        });

        DriverHandle { set_desired_play_state_tx: { set_desired_play_state_tx },
                       set_parameters_tx:         { set_parameters_tx },
                       shut_down_tx:              { Arc::new(shut_down_tx) }, }
    }

    async fn run_one(&mut self) -> bool {
        tokio::select! {
            Some((params, sender)) = self.set_parameters_rx.recv() => {
                let result = match serde_json::from_value(params) {
                    Ok(params) => {
                        self.driver.on_parameters_changed(params)
                    },
                    Err(err) => {
                        Err(InstanceDriverError::ParametersMalformed { error: err.to_string() })
                    }
                };

                let _ = sender.send(result.map(|_| InstanceParametersUpdated::Updated {id: self.instance_id.clone()}));
            },
            Some((play_state, sender)) = self.set_desired_play_state_rx.recv() => {
                let result = match play_state {
                    DesiredInstancePlayState::Playing {play_id} => {
                        self.driver.play(play_id)
                    },
                    DesiredInstancePlayState::Rendering{render_id, length} => {
                        self.driver.render(render_id, length)
                    },
                    DesiredInstancePlayState::Stopped {position: _} => {
                        self.driver.stop()
                    },
                };

                let _ = sender.send(result.map(|_| DesiredInstancePlayStateUpdated::Updated {
                    id: self.instance_id.clone(),
                    desired: play_state.clone(),
                    actual: self.driver.get_actual_play_state()
                }));
            },
            _ = self.shut_down.clone() => {
                return false;
            },
            _ = tokio::time::sleep(self.next_poll_at.duration_since(Instant::now())) => {
                self.next_poll_at += self.driver.poll().unwrap_or(Duration::from_secs(1));
            }
        }

        true
    }
}

type HandleSender<T, R> = Sender<(T, OneShotSender<Result<R>>)>;
type HandleReceiver<T, R> = Receiver<(T, OneShotSender<Result<R>>)>;

#[derive(Clone)]
pub struct DriverHandle {
    set_parameters_tx:         HandleSender<serde_json::Value, InstanceParametersUpdated>,
    set_desired_play_state_tx: HandleSender<DesiredInstancePlayState, DesiredInstancePlayStateUpdated>,
    shut_down_tx:              Arc<OneShotSender<()>>,
}

impl DriverHandle {
    pub async fn set_parameters(&self, parameters: serde_json::Value) -> Result<InstanceParametersUpdated> {
        let (tx, rx) = oneshot::channel();
        let _ = self.set_parameters_tx.send((parameters, tx));

        Self::respond(timeout(Duration::from_secs(1), rx).await)
    }

    pub async fn set_desired_play_state(&self, state: DesiredInstancePlayState) -> Result<DesiredInstancePlayStateUpdated> {
        let (tx, rx) = oneshot::channel();
        let _ = self.set_desired_play_state_tx.send((state, tx));

        Self::respond(timeout(Duration::from_secs(1), rx).await)
    }

    fn respond<T>(result: R<R<R<T, InstanceDriverError>, RecvError>, Elapsed>) -> Result<T> {
        result.map_err(|_| InstanceDriverError::RPC { error: format!("Timed out"), })?
              .map_err(|_| InstanceDriverError::RPC { error: format!("Channel closed"), })?
    }
}
