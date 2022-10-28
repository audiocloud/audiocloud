use std::time::Duration;

use actix::{Actor, Context, Handler, Recipient};

use actix_web::web::to;
use hidapi::{HidApi, HidDevice};
use serde::{Deserialize, Serialize};
use tracing::*;

use audiocloud_api::common::time::{now, Timestamp};
use audiocloud_api::instance_driver::{InstanceDriverCommand, InstanceDriverError};
use audiocloud_api::newtypes::FixedInstanceId;
use audiocloud_api::{toggle_off, toggle_value, Stereo, ToggleOr};
use audiocloud_models::distopik::{
    SummatraParameters, SummatraPreset, BUS_ASSIGN_VALUES, INPUT_VALUES, PAN_VALUES,
};

use crate::utils::*;
use crate::{Command, InstanceConfig};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    #[serde(default = "Config::default_serial")]
    serial: String,
    #[serde(default = "Config::default_vendor_id")]
    vendor_id: u16,
    #[serde(default = "Config::default_product_id")]
    product_id: u16,
}

impl Config {
    fn default_serial() -> String {
        todo!("add Serial number").to_string()
    }
    fn default_vendor_id() -> u16 {
        1155 as u16
    }
    fn default_product_id() -> u16 {
        22353 as u16
    }
}

const RECV_TIMEOUT: Duration = Duration::from_millis(10);

impl InstanceConfig for Config {
    fn create(self, id: FixedInstanceId) -> anyhow::Result<Recipient<Command>> {
        Ok(Summatra::new(id, self)?.start().recipient())
    }
}

struct Summatra {
    id:                FixedInstanceId,
    config:            Config,
    last_update:       Timestamp,
    hid_api:           HidDevice,
    pages:             [[u16; 32]; 2],
    input_gain_memory: [[u16; 32]; 2],
    values:            SummatraPreset,
    bus_assign:        [BusRegion; 24], //[CH_x, CH_x+1 ...]
    input:             [DigipotRegion; 24],
    pan:               [DigipotRegion; 24],
}

struct DigipotRegion {
    index_left:    usize,
    index_right:   usize,
    page:          usize,
}

impl DigipotRegion {
    pub fn new(page: usize, index_left: usize, index_right: usize) -> Self {
        Self { index_left,
               index_right,
               page }
    }

    pub fn write(&self, memory: &mut [[u16; 32]; 2], value: u16) {
        //writes a value to a correct location in memory
        memory[self.page][self.index_left] = (memory[self.page][self.index_left] & 0xE000) | (value & 0x3FF);
        memory[self.page][self.index_right] = (memory[self.page][self.index_right] & 0xE000) | (value & 0x3FF);
    }
}

struct BusRegion {
    index:   usize,
    page:    usize,
}

impl BusRegion {
    pub fn new(page: usize, index: usize) -> Self {
        Self { index,
               page }
    }

    pub fn write(&self, memory: &mut [[u16; 32]; 2], value: u16) {
        //writes a bit to a correct location in memory
        memory[self.page][self.index] = 1 << (value + 13);
    }
}

impl Summatra {
    pub fn new(id: FixedInstanceId, config: Config) -> anyhow::Result<Self> {
        info!("👋 Summatra Nuclear instance_driver");
        let api = HidApi::new()?;
        let hid_api = api.open_serial(config.vendor_id, config.product_id, &config.serial)?;

        todo!("assign vectors");
        let values = SummatraPreset { bus_assign:   Vec::new(),
                                      input:        Vec::new(),
                                      pan:          Vec::new(), };

        Ok(Summatra { id,
                      config,
                      hid_api,
                      values,
                      last_update: now(),
                      pages: [[0; 32]; 2],
                      input_gain_memory: [[0; 32]; 2],
                      bus_assign: [BusRegion::new(0, 14), BusRegion::new(0, 15), BusRegion::new(0, 16), BusRegion::new(0, 17), BusRegion::new(0, 18), BusRegion::new(0, 19),
                                   BusRegion::new(0, 20), BusRegion::new(0, 21), BusRegion::new(0, 22), BusRegion::new(0, 23), BusRegion::new(0, 24), BusRegion::new(0, 25),
                                   BusRegion::new(1, 14), BusRegion::new(1, 15), BusRegion::new(1, 16), BusRegion::new(1, 17), BusRegion::new(1, 18), BusRegion::new(1, 19),
                                   BusRegion::new(1, 20), BusRegion::new(1, 21), BusRegion::new(1, 22), BusRegion::new(1, 23), BusRegion::new(1, 24), BusRegion::new(1, 25)],
                                   
                      input: [DigipotRegion::new(0, 2, 14), DigipotRegion::new(0, 3, 15), DigipotRegion::new(0, 4, 16), DigipotRegion::new(0, 5, 17), DigipotRegion::new(0, 6, 18), DigipotRegion::new(0, 7, 19), 
                              DigipotRegion::new(0, 8, 20), DigipotRegion::new(0, 9, 21), DigipotRegion::new(0, 10, 22), DigipotRegion::new(0, 11, 23), DigipotRegion::new(0, 12, 24), DigipotRegion::new(0, 13, 25),
                              DigipotRegion::new(1, 2, 14), DigipotRegion::new(1, 3, 15), DigipotRegion::new(1, 4, 16), DigipotRegion::new(1, 5, 17), DigipotRegion::new(1, 6, 18), DigipotRegion::new(1, 7, 19), 
                              DigipotRegion::new(1, 8, 20), DigipotRegion::new(1, 9, 21), DigipotRegion::new(1, 11, 22), DigipotRegion::new(1, 11, 23), DigipotRegion::new(1, 12, 24), DigipotRegion::new(1, 13, 25)],

                      pan: [DigipotRegion::new(0, 2, 14), DigipotRegion::new(0, 3, 15), DigipotRegion::new(0, 4, 16), DigipotRegion::new(0, 5, 17), DigipotRegion::new(0, 6, 18), DigipotRegion::new(0, 7, 19), 
                            DigipotRegion::new(0, 8, 20), DigipotRegion::new(0, 9, 21), DigipotRegion::new(0, 10, 22), DigipotRegion::new(0, 11, 23), DigipotRegion::new(0, 12, 24), DigipotRegion::new(0, 13, 25),
                            DigipotRegion::new(1, 2, 14), DigipotRegion::new(1, 3, 15), DigipotRegion::new(1, 4, 16), DigipotRegion::new(1, 5, 17), DigipotRegion::new(1, 6, 18), DigipotRegion::new(1, 7, 19), 
                            DigipotRegion::new(1, 8, 20), DigipotRegion::new(1, 9, 21), DigipotRegion::new(1, 11, 22), DigipotRegion::new(1, 11, 23), DigipotRegion::new(1, 12, 24), DigipotRegion::new(1, 13, 25)]})
                      
    }

    fn set_input(&mut self, channels: Vec<f64>) {
        // self.input_gain[0].write_nrot_switch(&mut self.io_exp_data, repoint(left.to_f64(), &INPUT_GAIN_VALUES) as u16);
        // self.input_gain[1].write_nrot_switch(&mut self.io_exp_data, repoint(right.to_f64(), &INPUT_GAIN_VALUES) as u16);

        // self.values.input_gain = Stereo { left, right };
    }

    fn set_pan(&mut self, channels: Vec<f64>) {
        // let rescaled = repoint(left.to_f64(), &HIGH_PASS_FILTER_VALUES);
        // self.high_pass_filter[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        // self.high_pass_filter[2].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        // let rescaled = repoint(right.to_f64(), &HIGH_PASS_FILTER_VALUES);
        // self.high_pass_filter[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        // self.high_pass_filter[3].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        // self.values.high_pass_filter = Stereo { left, right };
    }

    fn set_bus_assign(&mut self, channels: Vec<u64>) {
        // let rescaled = rescale(left, &LOW_GAIN_VALUES, 128_f64);
        // self.low_gain[0].write(&mut self.io_exp_data, rescaled as u16);

        // let rescaled = rescale(right, &LOW_GAIN_VALUES, 128_f64);
        // self.low_gain[1].write(&mut self.io_exp_data, rescaled as u16);

        // self.values.low_gain = Stereo { left, right };
    }
}

impl Actor for Summatra {
    type Context = Context<Self>;
}

impl Handler<Command> for Summatra {
    type Result = Result<(), InstanceDriverError>;

    fn handle(&mut self, msg: Command, _ctx: &mut Self::Context) -> Self::Result {
        info!("in da loop");
        match msg.command {
            InstanceDriverCommand::CheckConnection => Ok(()),
            InstanceDriverCommand::Stop
            | InstanceDriverCommand::Play { .. }
            | InstanceDriverCommand::Render { .. }
            | InstanceDriverCommand::Rewind { .. } => Err(InstanceDriverError::MediaNotPresent),
            InstanceDriverCommand::SetParameters(params) => {
                let mut params = serde_json::from_value::<SummatraParameters>(params).map_err(|err| {
                                                                                         InstanceDriverError::ParameterDoesNotExist {
                            error: err.to_string(),
                        }
                                                                                     })?;

                if let Some(channels) = params.input.take() {
                    self.set_input(channels);
                }
                if let Some(channels) = params.pan.take() {
                    self.set_pan(channels);
                }
                if let Some(channels) = params.bus_assign.take() {
                    self.set_bus_assign(channels);
                }

                todo!("set func and pages");
                // self.write_io_expanders();
                //Summatra::send_pages(&self);

                // self.issue_system_async(self.values.clone());

                Ok(())
            }
            InstanceDriverCommand::SetPowerChannel { .. } => Ok(()),
        }
    }
}