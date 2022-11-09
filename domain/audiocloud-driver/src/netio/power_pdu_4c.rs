/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

#![allow(unused_variables)]

use std::time::Duration;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tracing::*;

use audiocloud_api::common::time::Timestamp;
use audiocloud_api::instance_driver::{InstanceDriverError};
use audiocloud_api::newtypes::FixedInstanceId;
use audiocloud_models::netio::PowerPdu4CReports;

use crate::driver::Driver;
use crate::driver::Result;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub address: String,
    #[serde(default)]
    pub auth:    Option<(String, String)>,
}

impl Config {
    pub fn from_json(config: serde_json::Value) -> Result<Self> {
        serde_json::from_value(config).map_err(|error| InstanceDriverError::ConfigMalformed { error: error.to_string() })
    }

    pub fn driver(self, id: &FixedInstanceId) -> Result<impl Driver> {
        PowerPdu4c::new(id.clone(), self)
    }
}

pub struct PowerPdu4c {
    id:       FixedInstanceId,
    config:   Config,
    base_url: Url,
    client:   reqwest::blocking::Client,
}

impl Driver for PowerPdu4c {
    #[instrument(skip(self))]
    fn set_power_channel(&mut self, channel: usize, value: bool) -> Result {
        let action = if value { PowerAction::On } else { PowerAction::Off };
        let update = NetioPowerRequest { outputs: vec![NetioPowerOutputAction { id:     { (channel + 1) as u32 },
                                                                                action: { action }, }], };

        let url = self.base_url.join("/netio.json").map_err(io_error)?;

        let response = self.client
                           .post(url)
                           .json(&update)
                           .send()
                           .map_err(io_error)?
                           .json::<NetioPowerResponse>()
                           .map_err(io_error)?;

        self.handle_response(response);

        Ok(())
    }

    fn poll(&mut self) -> Option<Duration> {
        let attempt = || {
            let url = self.base_url.join("/netio.json").map_err(io_error)?;
            let client = reqwest::blocking::Client::new();
            self.client
                .get(url)
                .send()
                .map_err(io_error)?
                .json::<NetioPowerResponse>()
                .map_err(io_error)
        };

        match attempt() {
            Ok(response) => {
                self.handle_response(response);
            }
            Err(error) => {
                warn!(%error, "While polling power PDU");
            }
        }

        Some(Duration::from_secs(15))
    }
}

impl PowerPdu4c {
    pub fn new(id: FixedInstanceId, config: Config) -> Result<Self> {
        let base_url = Url::parse(&config.address).map_err(|error| InstanceDriverError::ConfigMalformed { error: error.to_string() })?;
        Ok(Self { id:       { id },
                  config:   { config },
                  base_url: { base_url },
                  client:   { reqwest::blocking::Client::new() }, })
    }

    #[instrument(skip_all)]
    fn handle_response(&self, response: NetioPowerResponse) {
        println!("response: {response:#?}");

        let mut power_values = vec![false; 4];
        let mut current_values = vec![0.0; 4];
        let mut reports = PowerPdu4CReports::default();

        for channel in response.outputs {
            let power_value = channel.state == PowerState::On;
            let current_value = channel.current as f64 / 1000.0;
            let channel_id = (channel.id as usize) - 1;

            power_values[channel_id] = power_value;
            current_values[channel_id] = current_value;
        }

        reports.power = Some(power_values);
        reports.current = Some(current_values);

        match serde_json::to_value(&reports) {
            Ok(reports) => {
                let _ = self.emit_reports(self.id.clone(), reports);
            }
            Err(error) => {
                error!(%error, "Failed to encode NETIO reports");
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct NetioPowerRequest {
    pub outputs: Vec<NetioPowerOutputAction>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct NetioPowerOutputAction {
    #[serde(rename = "ID")]
    pub id:     u32,
    pub action: PowerAction,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NetioPowerResponse {
    global_measure: NetioGlobalMeasure,
    outputs:        Vec<NetioPowerOutput>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NetioGlobalMeasure {
    voltage:              f64,
    frequency:            f64,
    total_current:        u32,
    overall_power_factor: f64,
    total_load:           u32,
    total_energy:         u32,
    energy_start:         Timestamp,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NetioPowerOutput {
    #[serde(rename = "ID")]
    id:           u32,
    name:         String,
    current:      u32,
    power_factor: f64,
    load:         u32,
    energy:       u32,
    state:        PowerState,
    action:       PowerAction,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PowerAction {
    Off = 0,
    On = 1,
    ShortOff = 2,
    ShortOn = 3,
    Toggle = 4,
    NoChange = 5,
    Ignore = 6,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PowerState {
    Off = 0,
    On = 1,
}

fn io_error(err: impl ToString) -> InstanceDriverError {
    InstanceDriverError::IOError { error: err.to_string() }
}
