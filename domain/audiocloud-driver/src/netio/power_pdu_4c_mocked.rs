/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

#![allow(unused_variables)]

use std::time::Duration;

use serde::{Deserialize, Serialize};

use audiocloud_api::newtypes::FixedInstanceId;
use audiocloud_models::netio::PowerPdu4CReports;

use crate::driver::{Driver, Result};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config;

struct Netio4cMocked {
    id:    FixedInstanceId,
    state: Vec<bool>,
}

impl Driver for Netio4cMocked {
    fn set_power_channel(&mut self, channel: usize, value: bool) -> Result {
        self.state[channel] = value;
        Ok(())
    }

    fn poll(&mut self) -> Option<Duration> {
        if let Ok(reports) = serde_json::to_value(PowerPdu4CReports { power: Some(self.state.clone()),
                                                                      ..Default::default() })
        {
            let _ = self.emit_reports(self.id.clone(), reports);
        }

        Some(Duration::from_secs(5))
    }
}
