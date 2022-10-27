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
    id:               FixedInstanceId,
    config:           Config,
    last_update:      Timestamp,
    hid_api:          HidDevice,
    pages:            [[u16; 32]; 2],
    values:           SummatraPreset,
    bus_assign:       [UnirelRegion; 1], //[CH_x, CH_x+1 ...]
    input:            [DigipotRegion; 1],
    pan:              [DigipotRegion; 1],
}

// TODO: move to separate utils file, as we think there will be many more "unipot" etc drivers
struct DigipotRegion {
    bits:   Vec<usize>,
    pot_id: usize,
}

impl DigipotRegion {
    pub fn new(pot_id: usize, bits: impl Iterator<Item = usize>) -> Self {
        Self { bits: bits.collect(),
               pot_id }
    }

    pub fn write(&self, memory: &mut [[u16; 6]; 8], value: u16) {
        // for every i-th bit set in the value, we set self.bits[i]-th bit in `memory`
        // bits can be arbitrarily far from the beginning of the buffer, as long as there
        // is space
        for (i, bit) in self.bits.iter().copied().enumerate() {
            let bit_value = value & (1 << i);
            write_bit_16(&mut memory[self.pot_id][(bit / 16) + 1], (bit % 16) as u16, bit_value);
        }
        memory[self.pot_id][0] = 1;
    }
}

struct UnirelRegion {
    bits:   Vec<usize>,
    pot_id: usize,
}

impl UnirelRegion {
    pub fn new(pot_id: usize, bits: impl Iterator<Item = usize>) -> Self {
        Self { bits: bits.collect(),
               pot_id }
    }

    pub fn write_switch(&self, memory: &mut [[u16; 6]; 8], value: u16) {
        //writes a bit to a correct location in memory
        for (_i, bit) in self.bits.iter().copied().enumerate() {
            write_bit_16(&mut memory[self.pot_id][(bit / 16) + 1], (bit % 16) as u16, value);
        }
        memory[self.pot_id][0] = 1;
    }
    pub fn write_rot_switch(&self, memory: &mut [[u16; 6]; 8], value: u16) {
        // rotation switches use moving bits 0000 -> 0001 -> 0010 -> 0100...
        for (i, bit) in self.bits.iter().copied().enumerate() {
            write_bit_16(&mut memory[self.pot_id][(bit / 16) + 1],
                         (bit % 16) as u16,
                         (value == i as u16) as u16);
        }
        memory[self.pot_id][0] = 1;
    }
    pub fn write_nrot_switch(&self, memory: &mut [[u16; 6]; 8], value: u16) {
        // negated rot switch has negated first bit/switch
        for (i, bit) in self.bits.iter().copied().enumerate() {
            let mut temp: bool = false;
            if value as usize == i {
                temp = true;
            }
            if i == 0 {
                if temp == false {
                    temp = true;
                } else {
                    temp = false;
                }
            }

            write_bit_16(&mut memory[self.pot_id][(bit / 16) + 1], (bit % 16) as u16, temp as u16);
        }
        memory[self.pot_id][0] = 1;
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
                      bus_assign: [UnirelRegion::new(0, [0,1,2].into_iter())],
                      input: [DigipotRegion::new(0, [0,1,2].into_iter())],
                      pan: [DigipotRegion::new(0, [0,1,2].into_iter().clone())],})
                      
    }

    fn set_input_gain(&mut self, left: ToggleOr<i64>, right: ToggleOr<i64>) {
        // self.input_gain[0].write_nrot_switch(&mut self.io_exp_data, repoint(left.to_f64(), &INPUT_GAIN_VALUES) as u16);
        // self.input_gain[1].write_nrot_switch(&mut self.io_exp_data, repoint(right.to_f64(), &INPUT_GAIN_VALUES) as u16);

        // self.values.input_gain = Stereo { left, right };
    }

    fn set_high_pass_filter(&mut self, left: ToggleOr<u64>, right: ToggleOr<u64>) {
        // let rescaled = repoint(left.to_f64(), &HIGH_PASS_FILTER_VALUES);
        // self.high_pass_filter[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        // self.high_pass_filter[2].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        // let rescaled = repoint(right.to_f64(), &HIGH_PASS_FILTER_VALUES);
        // self.high_pass_filter[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        // self.high_pass_filter[3].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        // self.values.high_pass_filter = Stereo { left, right };
    }

    fn set_low_gain(&mut self, left: f64, right: f64) {
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

                // if let Some(Stereo { left, right }) = params.input_gain.take() {
                //     self.set_input_gain(left, right);
                // }
                // if let Some(Stereo { left, right }) = params.high_pass_filter.take() {
                //     self.set_high_pass_filter(left, right);
                // }
                // if let Some(Stereo { left, right }) = params.low_gain.take() {
                //     self.set_low_gain(left, right);
                // }

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