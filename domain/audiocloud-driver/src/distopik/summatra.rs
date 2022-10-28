use std::time::Duration;

use actix::{Actor, Context, Handler, Recipient};

use actix_web::web::to;
use hidapi::{HidApi, HidDevice};
use byteorder::{LittleEndian, ByteOrder};
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
        "00000050011A".to_string()
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
    values:            SummatraPreset,
    bus_assign:        [BusRegion; 24], //[CH_x, CH_x+1 ...]
    input:             [DigipotRegion; 24],
    pan:               [DigipotRegion; 24],
    masters:           [DigipotRegion; 4],
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

    pub fn write_lr(&self, memory: &mut [[u16; 32]; 2], left: u16, right: u16) {
        //writes a value to a correct location in memory
        memory[self.page][self.index_left] = (memory[self.page][self.index_left] & 0xE000) | (left & 0x3FF);
        memory[self.page][self.index_right] = (memory[self.page][self.index_right] & 0xE000) | (right & 0x3FF);
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
        memory[self.page][self.index] &= 0x1FFF;
        memory[self.page][self.index] |= 1 << (value + 13);
    }
}

impl Summatra {
    pub fn new(id: FixedInstanceId, config: Config) -> anyhow::Result<Self> {
        info!("👋 Summatra Nuclear instance_driver");
        let api = HidApi::new()?;
        let hid_api = api.open_serial(config.vendor_id, config.product_id, &config.serial)?;


        let values = SummatraPreset { bus_assign:   vec![0; 24],
                                      input:        vec![-48.0; 24],
                                      pan:          vec![0.0; 24], };

        Ok(Summatra { id,
                      config,
                      hid_api,
                      values,
                      last_update: now(),
                      pages: [[0; 32]; 2],
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
                            DigipotRegion::new(1, 8, 20), DigipotRegion::new(1, 9, 21), DigipotRegion::new(1, 11, 22), DigipotRegion::new(1, 11, 23), DigipotRegion::new(1, 12, 24), DigipotRegion::new(1, 13, 25)],
                    
                      masters: [DigipotRegion::new(0, 24, 25), DigipotRegion::new(0, 26, 27), DigipotRegion::new(1, 24, 25), DigipotRegion::new(1, 26, 27)]})
                      
    }

    fn set_input(&mut self, channels: Vec<f64>) {

        for i in 0..channels.len(){
            let rescaled = rescale(db_to_gain_factor(channels[i]), &INPUT_VALUES, 1023_f64);
            self.input[i].write(&mut self.pages, rescaled as u16);
        }

        self.values.input = channels;
    }

    fn set_pan(&mut self, channels: Vec<f64>) {

        for i in 0..channels.len(){
            let rescaled = rescale(db_to_gain_factor(self.values.input[i]), &INPUT_VALUES, 1023_f64);
            let left = rescaled * ((1.0 - rescale(channels[i], &PAN_VALUES, 1_f64)).sqrt());
            let right = rescaled * (rescale(channels[i], &PAN_VALUES, 1_f64).sqrt());
            self.pan[i].write_lr(&mut self.pages, left as u16, right as u16);
        }

        self.values.pan = channels;
    }

    fn set_bus_assign(&mut self, channels: Vec<u64>) {

        for i in 0..channels.len(){
            self.bus_assign[i].write(&mut self.pages, channels[i] as u16);
        }

        self.values.bus_assign = channels;
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
                Summatra::send_pages(self);

                // self.issue_system_async(self.values.clone());

                Ok(())
            }
            InstanceDriverCommand::SetPowerChannel { .. } => Ok(()),
        }
    }
}

impl Summatra {

    fn send_pages(&mut self) {

        self.set_master_faders();

        self.pages[0][0] = 0x80;
        self.pages[0][1] = 0x00;
        self.pages[1][0] = 0x80;
        self.pages[1][1] = 0x01;
        
        let mut temp_page: [u8; 64] = [0; 64];
        LittleEndian::write_u16_into(&self.pages[0], &mut temp_page);
        println!("temp_page: {:#?}",temp_page);
        info!("{:#?}",self.hid_api.write(&temp_page));
    
        LittleEndian::write_u16_into(&self.pages[0], &mut temp_page);
        info!("{:#?}",self.hid_api.write(&temp_page));
    }

    fn set_master_faders(&mut self){

        let rescaled = rescale(db_to_gain_factor(0_f64), &INPUT_VALUES, 1023_f64) as u16;
        self.masters[0].write(&mut self.pages, rescaled);
        self.masters[1].write(&mut self.pages, rescaled);
        self.masters[2].write(&mut self.pages, rescaled);
        self.masters[3].write(&mut self.pages, rescaled);

    }
}