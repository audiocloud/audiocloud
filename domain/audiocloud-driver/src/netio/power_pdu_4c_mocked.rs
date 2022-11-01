#![allow(unused_variables)]

use std::time::Duration;

use actix::Recipient;
use serde::{Deserialize, Serialize};
use tracing::*;

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
    type Params = ();
    type Reports = PowerPdu4CReports;

    fn set_power_channel(&mut self, channel: usize, value: bool) -> Result {
        self.state[channel] = value;
        Ok(())
    }

    fn poll(&mut self) -> Option<Duration> {
        Self::emit_reports(self.id.clone(),
                           PowerPdu4CReports { power: Some(self.state.clone()),
                                               ..Default::default() });

        Some(Duration::from_secs(5))
    }
}
