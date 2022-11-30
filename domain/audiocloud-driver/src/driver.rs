/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::result::Result as R;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use flume::RecvTimeoutError;
use serde_json::json;
use tokio::spawn;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tokio::task::spawn_blocking;
use tokio::time::error::Elapsed;
use tokio::time::timeout;
use tracing::*;

use audiocloud_api::cloud::domains::FixedInstanceConfig;
use audiocloud_api::instance_driver::{DesiredInstancePlayStateUpdated, InstanceDriverError, InstanceParametersUpdated};
use audiocloud_api::{DesiredInstancePlayState, FixedInstanceId, InstanceParameters, InstancePlayState, InstanceReports, PlayId, RenderId};

use crate::messages::NotifyInstanceReportsMsg;
use crate::supervisor::get_driver_supervisor;

pub type Result<T = ()> = std::result::Result<T, InstanceDriverError>;

pub trait Driver {
    fn on_parameters_changed(&mut self, params: InstanceParameters) -> Result<InstanceParameters> {
        drop(params);
        Ok(json!({}))
    }

    fn play(&mut self, play_id: PlayId) -> Result {
        drop(play_id);
        Err(InstanceDriverError::MediaNotPresent)
    }

    fn render(&mut self, render_id: RenderId, length: f64) -> Result {
        drop((render_id, length));
        Err(InstanceDriverError::MediaNotPresent)
    }

    fn stop(&mut self, position: Option<f64>) -> Result {
        drop(position);
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

    fn emit_reports(&self, instance_id: FixedInstanceId, reports: InstanceReports) -> Result {
        spawn(get_driver_supervisor().send(NotifyInstanceReportsMsg { instance_id, reports }));

        Ok(())
    }
}

struct DriverRunner {
    id:       FixedInstanceId,
    driver:   Box<dyn Driver>,
    receiver: flume::Receiver<DriverRunnerCmd>,
}

enum DriverRunnerCmd {
    SetParameters(InstanceParameters, oneshot::Sender<Result<InstanceParametersUpdated>>),
    SetDesiredState(DesiredInstancePlayState, oneshot::Sender<Result<DesiredInstancePlayStateUpdated>>),
    Shutdown,
}

impl DriverRunner {
    async fn run(&mut self) {
        self.driver.restarted();

        loop {
            let deadline = Instant::now() + self.driver.poll().unwrap_or(Duration::from_secs(5));
            if !self.process_until(deadline) {
                break;
            }
        }
    }

    fn process_until(&mut self, deadline: Instant) -> bool {
        while Instant::now() < deadline {
            let maybe_cmd = self.receiver.recv_deadline(deadline);
            if !self.execute_cmd(maybe_cmd) {
                return false;
            }
        }
        true
    }

    fn execute_cmd(&mut self, maybe_cmd: std::result::Result<DriverRunnerCmd, RecvTimeoutError>) -> bool {
        match maybe_cmd {
            Ok(cmd) => match cmd {
                DriverRunnerCmd::SetParameters(params, sender) => {
                    self.set_parameters(params, sender);
                    true
                }
                DriverRunnerCmd::SetDesiredState(state, sender) => {
                    self.set_desired_state(state, sender);
                    true
                }
                DriverRunnerCmd::Shutdown => false,
            },
            Err(RecvTimeoutError::Disconnected) => false,
            _ => true,
        }
    }

    fn set_desired_state(&mut self, state: DesiredInstancePlayState, sender: oneshot::Sender<Result<DesiredInstancePlayStateUpdated>>) {
        let result = match state.clone() {
                         DesiredInstancePlayState::Playing { play_id } => self.driver.play(play_id),
                         DesiredInstancePlayState::Rendering { length, render_id } => self.driver.render(render_id, length),
                         DesiredInstancePlayState::Stopped { position } => self.driver.stop(position),
                     }.map(|_| DesiredInstancePlayStateUpdated::Updated { actual:  { self.driver.get_actual_play_state() },
                                                                          desired: { state },
                                                                          id:      { self.id.clone() }, });
        let _ = sender.send(result);
    }

    fn set_parameters(&mut self, params: InstanceParameters, sender: oneshot::Sender<Result<InstanceParametersUpdated>>) {
        let _ = sender.send(self.driver
                                .on_parameters_changed(params)
                                .map(|parameters| InstanceParametersUpdated::Updated { id: self.id.clone(),
                                                                                       parameters }));
    }
}

pub struct DriverHandle {
    thread: JoinHandle<()>,
    sender: flume::Sender<DriverRunnerCmd>,
}

impl DriverHandle {
    pub async fn set_parameters(&self, parameters: serde_json::Value) -> Result<InstanceParametersUpdated> {
        let (tx, rx) = oneshot::channel();
        let _ = timeout(SEND_COMMAND_TIMEOUT,
                        self.sender.send_async(DriverRunnerCmd::SetParameters(parameters, tx))).await;
        Self::respond(timeout(SET_PARAMETERS_TIMEOUT, rx).await)
    }

    pub async fn set_desired_play_state(&self, state: DesiredInstancePlayState) -> Result<DesiredInstancePlayStateUpdated> {
        let (tx, rx) = oneshot::channel();
        let _ = timeout(SEND_COMMAND_TIMEOUT,
                        self.sender.send_async(DriverRunnerCmd::SetDesiredState(state, tx))).await;
        Self::respond(timeout(SET_STATE_TIMEOUT, rx).await)
    }

    pub async fn drop(self) {
        let Self { thread, sender } = self;
        let _ = timeout(SEND_COMMAND_TIMEOUT, sender.send_async(DriverRunnerCmd::Shutdown)).await;
        let _ = timeout(THREAD_SHUTDOWN_TIMEOUT, spawn_blocking(move || thread.join())).await;
    }

    fn respond<T>(result: R<R<R<T, InstanceDriverError>, RecvError>, Elapsed>) -> Result<T> {
        result.map_err(|_| InstanceDriverError::RPC { error: format!("Timed out"), })?
              .map_err(|_| InstanceDriverError::RPC { error: format!("Channel closed"), })?
    }

    pub async fn new(id: FixedInstanceId, config: FixedInstanceConfig) -> Result<Self> {
        let (init_tx, init_rx) = oneshot::channel();
        let (sender, receiver) = flume::unbounded();

        let thread = thread::spawn(move || {
            let driver = match create_driver(id.clone(), config) {
                Ok(driver) => {
                    init_tx.send(Ok(())).expect("Failed to send init result");
                    driver
                }
                Err(error) => {
                    init_tx.send(Err(error)).expect("Failed to send init result");
                    return;
                }
            };

            DriverRunner { id, driver, receiver }.run();
        });

        match init_rx.await {
            Err(_) | Ok(Err(_)) => {
                return Err(InstanceDriverError::IOError { error: format!("Failed to initialize driver"), });
            }
            _ => {}
        }

        Ok(Self { thread, sender })
    }
}

fn create_driver(id: FixedInstanceId, config: FixedInstanceConfig) -> anyhow::Result<Box<dyn Driver>> {
    use ::audiocloud_models as models;
    match (id.manufacturer.as_str(), id.name.as_str()) {
        #[cfg(unix)]
        (models::distopik::NAME, models::distopik::dual1084::NAME) => {
            let config = crate::distopik::dual_1084::Config::from_json(config.additional)?;
            Ok(Box::new(crate::distopik::dual_1084::Dual1084::new(id, config)?))
        }
        (models::netio::NAME, models::netio::power_pdu_4c::NAME) => {
            let config = crate::netio::power_pdu_4c::Config::from_json(config.additional)?;
            Ok(Box::new(crate::netio::power_pdu_4c::PowerPdu4c::new(id, config)?))
        }
        (manufacturer, name) => Err(InstanceDriverError::DriverNotSupported { manufacturer: manufacturer.to_string(),
                                                                              name:         name.to_string(), }.into()),
    }
}

const SEND_COMMAND_TIMEOUT: Duration = Duration::from_millis(100);
const SET_PARAMETERS_TIMEOUT: Duration = Duration::from_secs(5);
const SET_STATE_TIMEOUT: Duration = Duration::from_secs(5);
const THREAD_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);
