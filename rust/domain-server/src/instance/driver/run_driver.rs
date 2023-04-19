use anyhow::bail;
use tracing::instrument;

use api::instance::driver::config::InstanceDriverConfig;
use api::instance::driver::events::InstanceDriverEvent;
use api::instance::driver::requests::{SetInstanceParameterResponse, SetInstanceParametersRequest};

use crate::instance::driver::http::run_http_driver;
use crate::instance::driver::mock::run_mock_driver;
use crate::instance::driver::scripting::ScriptingEngine;
use crate::instance::driver::serial::run_serial_driver;
use crate::instance::driver::usb_hid::run_usb_driver;

use super::Result;

#[derive(Debug)]
pub enum InstanceDriverCommand {
  SetParameters(SetInstanceParametersRequest, flume::Sender<SetInstanceParameterResponse>),
  Terminate,
}

#[instrument(err, skip(config, rx_cmd, tx_evt, scripting_engine))]
pub async fn run_driver_server(instance_id: String,
                               config: InstanceDriverConfig,
                               scripting_engine: ScriptingEngine,
                               rx_cmd: flume::Receiver<InstanceDriverCommand>,
                               tx_evt: flume::Sender<InstanceDriverEvent>)
                               -> Result {
  match config {
    | InstanceDriverConfig::USBHID(usb_hid) => {
      run_usb_driver(instance_id, usb_hid, rx_cmd, tx_evt, scripting_engine).await?;
    }
    | InstanceDriverConfig::Serial(serial) => {
      run_serial_driver(instance_id, serial, rx_cmd, tx_evt, scripting_engine).await?;
    }
    | InstanceDriverConfig::OSC(_) => {}
    | InstanceDriverConfig::HTTP(http) => {
      run_http_driver(instance_id, http, rx_cmd, tx_evt, scripting_engine).await?;
    }
    | InstanceDriverConfig::SPI(_) => {}
    | InstanceDriverConfig::Mock => {
      run_mock_driver(instance_id, rx_cmd, tx_evt, scripting_engine).await?;
    }
  }

  bail!("Driver server exited unexpectedly")
}
