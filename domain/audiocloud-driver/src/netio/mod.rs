use actix::{Actor, Recipient};
use serde::{Deserialize, Serialize};

use audiocloud_api::newtypes::FixedInstanceId;

pub mod power_pdu_4c;
pub mod power_pdu_4c_mocked;

#[cfg(test)]
mod tests;
