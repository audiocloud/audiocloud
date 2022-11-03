/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */


use std::fs::File;
use std::io;
use std::os::unix::prelude::*;
use std::time::Duration;

use nix::{ioctl_none, ioctl_write_ptr};
use serde::{Deserialize, Serialize};
use tracing::*;

use audiocloud_api::common::time::{now, Timestamp};
use audiocloud_api::instance_driver::{InstanceDriverError};
use audiocloud_api::newtypes::FixedInstanceId;
use audiocloud_api::{toggle_off, toggle_value, Stereo, ToggleOr};
use audiocloud_models::distopik::dual1084::*;
use audiocloud_models::distopik::{Dual1084Parameters, Dual1084Preset, Dual1084Reports};

use crate::driver::Driver;
use crate::driver::Result;
use crate::utils::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    #[serde(default = "Config::default_device")]
    device: String,
}

impl Config {
    fn default_device() -> String {
        "/dev/PIVO".to_string()
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self> {
        if json.is_object() {
            serde_json::from_value(json).map_err(|error| InstanceDriverError::ConfigMalformed { error: error.to_string() })
        } else {
            Ok(Self { device: Self::default_device(), })
        }
    }

    pub fn driver(self, id: &FixedInstanceId) -> Result<impl Driver> {
        Dual1084::new(id.clone(), self)
    }
}

const RECV_TIMEOUT: Duration = Duration::from_millis(10);

pub struct Dual1084 {
    id:               FixedInstanceId,
    config:           Config,
    last_update:      Timestamp,
    raw_fd:           RawFd,
    io_exp_data:      [[u16; 6]; 8],
    values:           Dual1084Preset,
    input_gain:       [UnirelRegion; 2], //[L,D]
    high_pass_filter: [UnirelRegion; 4],
    low_freq:         [UnirelRegion; 4], //[L,D,L,D]
    low_gain:         [UnipotRegion; 2],
    low_mid_freq:     [UnirelRegion; 2],
    low_mid_gain:     [UnipotRegion; 2],
    low_mid_width:    [UnirelRegion; 2],
    high_mid_freq:    [UnirelRegion; 2],
    high_mid_gain:    [UnipotRegion; 2],
    high_mid_width:   [UnirelRegion; 2],
    high_freq:        [UnirelRegion; 4],
    high_gain:        [UnipotRegion; 2],
    output_pad:       [UnirelRegion; 2],
    eql_toggle:       [UnirelRegion; 2],
}

// TODO: move to separate utils file, as we think there will be many more "unipot" etc drivers
struct UnipotRegion {
    bits:   Vec<usize>,
    pot_id: usize,
}

impl UnipotRegion {
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

impl Dual1084 {
    pub fn new(id: FixedInstanceId, config: Config) -> Result<Self> {
        info!(device = &config.device, "👋 Summatra Nuclear instance_driver");
        let raw_fd = File::options().read(true)
                                    .write(true)
                                    .open(&config.device)
                                    .map_err(|error| InstanceDriverError::IOError { error: error.to_string() })?
                                    .into_raw_fd();

        let values = Dual1084Preset { low_gain:         Stereo::both(0.0),
                                      eql_toggle:       Stereo::both(false),
                                      high_freq:        Stereo::both(toggle_off()),
                                      high_mid_freq:    Stereo::both(toggle_off()),
                                      high_gain:        Stereo::both(0.0),
                                      low_mid_freq:     Stereo::both(toggle_off()),
                                      high_mid_width:   Stereo::both(false),
                                      input_gain:       Stereo::both(toggle_value(0)),
                                      high_pass_filter: Stereo::both(toggle_off()),
                                      high_mid_gain:    Stereo::both(0.0),
                                      low_freq:         Stereo::both(toggle_off()),
                                      output_pad:       Stereo::both(toggle_off()),
                                      low_mid_gain:     Stereo::both(0.0),
                                      low_mid_width:    Stereo::both(false), };

        Ok(Dual1084 { id,
                      config,
                      raw_fd,
                      values,
                      last_update: now(),
                      io_exp_data: [[0; 6]; 8],
                      input_gain: [UnirelRegion::new(3, 72..=79), UnirelRegion::new(1, 72..=79)],
                      high_pass_filter: [UnirelRegion::new(3, 56..=61),
                                         UnirelRegion::new(1, 56..=61),
                                         UnirelRegion::new(3, [62, 63, 48, 49, 50, 51].into_iter()),
                                         UnirelRegion::new(1, [62, 63, 48, 49, 50, 51].into_iter())],
                      low_freq: [UnirelRegion::new(3, 16..=21),
                                 UnirelRegion::new(1, (16..=21).rev()),
                                 UnirelRegion::new(3, [22, 23, 40, 41, 42, 43].into_iter()),
                                 UnirelRegion::new(1, [22, 23, 40, 41, 42, 43].into_iter())],
                      low_gain: [UnipotRegion::new(5, 57..=63),
                                 UnipotRegion::new(5, [28, 29, 30, 31, 23, 22, 21].into_iter())],
                      low_mid_freq: [UnirelRegion::new(3, [4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31].into_iter()),
                                     UnirelRegion::new(1, [4, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 31].into_iter())],
                      low_mid_gain: [UnipotRegion::new(5, (49..=55).rev()),
                                     UnipotRegion::new(5, [20, 19, 18, 17, 16, 8, 9].into_iter())],
                      low_mid_width: [UnirelRegion::new(3, 54..=54), UnirelRegion::new(1, 54..=54)],
                      high_mid_freq: [UnirelRegion::new(3, [8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3].into_iter()),
                                      UnirelRegion::new(1, [8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3].into_iter())],
                      high_mid_gain: [UnipotRegion::new(5, [48, 40, 41, 42, 43, 44, 45].into_iter()),
                                      UnipotRegion::new(5, [10, 11, 12, 13, 14, 15, 7].into_iter())],
                      high_mid_width: [UnirelRegion::new(3, 53..=53), UnirelRegion::new(1, 53..=53)],
                      high_freq: [UnirelRegion::new(3, [44, 45, 46, 47, 32, 33].into_iter()),
                                  UnirelRegion::new(1, [44, 45, 46, 47, 32, 33].into_iter()),
                                  UnirelRegion::new(3, 34..=39),
                                  UnirelRegion::new(1, 34..=39)],
                      high_gain: [UnipotRegion::new(5, [46, 47, 39, 38, 37, 36, 35].into_iter()),
                                  UnipotRegion::new(5, (0..=6).rev())],
                      output_pad: [UnirelRegion::new(7, 0..=2), UnirelRegion::new(7, 3..=5)],
                      eql_toggle: [UnirelRegion::new(3, 52..=52), UnirelRegion::new(1, 52..=52)] })
    }

    fn set_input_gain(&mut self, left: ToggleOr<i64>, right: ToggleOr<i64>) {
        self.input_gain[0].write_nrot_switch(&mut self.io_exp_data, repoint(left.to_f64(), &INPUT_GAIN_VALUES) as u16);
        self.input_gain[1].write_nrot_switch(&mut self.io_exp_data, repoint(right.to_f64(), &INPUT_GAIN_VALUES) as u16);

        self.values.input_gain = Stereo { left, right };
    }

    fn set_high_pass_filter(&mut self, left: ToggleOr<u64>, right: ToggleOr<u64>) {
        let rescaled = repoint(left.to_f64(), &HIGH_PASS_FILTER_VALUES);
        self.high_pass_filter[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        self.high_pass_filter[2].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        let rescaled = repoint(right.to_f64(), &HIGH_PASS_FILTER_VALUES);
        self.high_pass_filter[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        self.high_pass_filter[3].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        self.values.high_pass_filter = Stereo { left, right };
    }

    fn set_low_gain(&mut self, left: f64, right: f64) {
        let rescaled = rescale(left, &LOW_GAIN_VALUES, 128_f64);
        self.low_gain[0].write(&mut self.io_exp_data, rescaled as u16);

        let rescaled = rescale(right, &LOW_GAIN_VALUES, 128_f64);
        self.low_gain[1].write(&mut self.io_exp_data, rescaled as u16);

        self.values.low_gain = Stereo { left, right };
    }

    fn set_low_freq(&mut self, left: ToggleOr<u64>, right: ToggleOr<u64>) {
        let rescaled = repoint(left.to_f64(), &LOW_FREQ_VALUES);
        self.low_freq[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        self.low_freq[2].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        let rescaled = repoint(right.to_f64(), &LOW_FREQ_VALUES);
        self.low_freq[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        self.low_freq[3].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        self.values.low_freq = Stereo { left, right };
    }

    fn set_low_mid_gain(&mut self, left: f64, right: f64) {
        let rescaled = rescale(left, &LOW_MID_GAIN_VALUES, 128.0);
        self.low_mid_gain[0].write(&mut self.io_exp_data, rescaled as u16);

        let rescaled = rescale(right, &LOW_MID_GAIN_VALUES, 128.0);
        self.low_mid_gain[1].write(&mut self.io_exp_data, rescaled as u16);

        self.values.low_mid_gain = Stereo { left, right };
    }

    fn set_low_mid_freq(&mut self, left: ToggleOr<u64>, right: ToggleOr<u64>) {
        let rescaled = repoint(left.to_f64(), &LOW_MID_FREQ_VALUES);
        self.low_mid_freq[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        let rescaled = repoint(right.to_f64(), &LOW_MID_FREQ_VALUES);
        self.low_mid_freq[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        self.values.low_mid_freq = Stereo { left, right };
    }

    fn set_low_mid_width(&mut self, left: bool, right: bool) {
        self.low_mid_width[0].write_switch(&mut self.io_exp_data, left as u16);
        self.low_mid_width[1].write_switch(&mut self.io_exp_data, right as u16);

        self.values.low_mid_width = Stereo { left, right };
    }

    fn set_high_mid_gain(&mut self, left: f64, right: f64) {
        let rescaled = rescale(left, &HIGH_MID_GAIN_VALUES, 128.0);
        self.high_mid_gain[0].write(&mut self.io_exp_data, rescaled as u16);

        let rescaled = rescale(right, &HIGH_MID_GAIN_VALUES, 128.0);
        self.high_mid_gain[1].write(&mut self.io_exp_data, rescaled as u16);

        self.values.high_mid_gain = Stereo { left, right };
    }

    fn set_high_mid_freq(&mut self, left: ToggleOr<u64>, right: ToggleOr<u64>) {
        let rescaled = repoint(left.to_f64(), &HIGH_MID_FREQ_VALUES);
        self.high_mid_freq[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        let rescaled = repoint(right.to_f64(), &HIGH_MID_FREQ_VALUES);
        self.high_mid_freq[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        self.values.high_mid_freq = Stereo { left, right };
    }

    fn set_high_mid_width(&mut self, left: bool, right: bool) {
        self.high_mid_width[0].write_switch(&mut self.io_exp_data, left as u16);
        self.high_mid_width[1].write_switch(&mut self.io_exp_data, right as u16);

        self.values.high_mid_width = Stereo { left, right };
    }

    fn set_high_gain(&mut self, left: f64, right: f64) {
        let rescaled = rescale(left, &HIGH_GAIN_VALUES, 128.0);
        self.high_gain[0].write(&mut self.io_exp_data, rescaled as u16);

        let rescaled = rescale(right, &HIGH_GAIN_VALUES, 128.0);
        self.high_gain[1].write(&mut self.io_exp_data, rescaled as u16);

        self.values.high_gain = Stereo { left, right };
    }

    fn set_high_freq(&mut self, left: ToggleOr<u64>, right: ToggleOr<u64>) {
        let rescaled = repoint(left.to_f64(), &HIGH_FREQ_VALUES);
        self.high_freq[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        self.high_freq[2].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        let rescaled = repoint(right.to_f64(), &HIGH_FREQ_VALUES);
        self.high_freq[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);
        self.high_freq[3].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        self.values.high_freq = Stereo { left, right };
    }

    fn set_output_pad(&mut self, left: ToggleOr<i64>, right: ToggleOr<i64>) {
        let rescaled = repoint(left.to_f64(), &OUTPUT_PAD_VALUES);
        self.output_pad[0].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        let rescaled = repoint(right.to_f64(), &OUTPUT_PAD_VALUES);
        self.output_pad[1].write_nrot_switch(&mut self.io_exp_data, rescaled as u16);

        self.values.output_pad = Stereo { left, right };
    }

    fn set_eql_toggle(&mut self, left: bool, right: bool) {
        self.eql_toggle[0].write_switch(&mut self.io_exp_data, left as u16);
        self.eql_toggle[1].write_switch(&mut self.io_exp_data, right as u16);

        self.values.eql_toggle = Stereo { left, right };
    }
}

impl Driver for Dual1084 {
    type Params = Dual1084Parameters;
    type Reports = Dual1084Reports;

    fn on_parameters_changed(&mut self, mut params: Self::Params) -> Result {
        if let Some(Stereo { left, right }) = params.input_gain.take() {
            self.set_input_gain(left, right);
        }
        if let Some(Stereo { left, right }) = params.high_pass_filter.take() {
            self.set_high_pass_filter(left, right);
        }
        if let Some(Stereo { left, right }) = params.low_gain.take() {
            self.set_low_gain(left, right);
        }
        if let Some(Stereo { left, right }) = params.low_freq.take() {
            self.set_low_freq(left, right);
        }
        if let Some(Stereo { left, right }) = params.low_mid_gain.take() {
            self.set_low_mid_gain(left, right);
        }
        if let Some(Stereo { left, right }) = params.low_mid_freq.take() {
            self.set_low_mid_freq(left, right);
        }
        if let Some(Stereo { left, right }) = params.low_mid_width.take() {
            self.set_low_mid_width(left, right);
        }
        if let Some(Stereo { left, right }) = params.high_mid_gain.take() {
            self.set_high_mid_gain(left, right);
        }
        if let Some(Stereo { left, right }) = params.high_mid_freq.take() {
            self.set_high_mid_freq(left, right);
        }
        if let Some(Stereo { left, right }) = params.high_mid_width.take() {
            self.set_high_mid_width(left, right);
        }
        if let Some(Stereo { left, right }) = params.high_gain.take() {
            self.set_high_gain(left, right);
        }
        if let Some(Stereo { left, right }) = params.high_freq.take() {
            self.set_high_freq(left, right);
        }
        if let Some(Stereo { left, right }) = params.output_pad.take() {
            self.set_output_pad(left, right);
        }
        if let Some(Stereo { left, right }) = params.eql_toggle {
            self.set_eql_toggle(left, right);
        }

        // TODO: implement
        // self.write_io_expanders();
        Dual1084::set_io_expanders(&self);

        // self.issue_system_async(self.values.clone());

        Ok(())
    }
}

impl Dual1084 {
    pub fn set_io_expanders(&self) {
        let mut spi_data: [u32; 9] = [0; 9];
        const IO_BOARDS: [u16; 4] = [3, 1, 5, 7];
        const IO_OUTPUT_ADDRESS: [u16; 5] = [0x4000, 0x4200, 0x4400, 0x4600, 0x4800];

        for j in 0..5 {
            //spi_data = [0; 9];
            for i in 0..4 {
                if self.io_exp_data[IO_BOARDS[i] as usize][0] == 1 {
                    if j < 5 && (IO_BOARDS[i] != 7) {
                        spi_data[IO_BOARDS[i] as usize] =
                            ((IO_OUTPUT_ADDRESS[j] as u32 | 0x12) << 16) | swap_u16(self.io_exp_data[IO_BOARDS[i] as usize][j + 1]) as u32;
                        spi_data[8] |= 1 << IO_BOARDS[i];
                    }
                    if j == 0 && (IO_BOARDS[i] == 7) {
                        spi_data[IO_BOARDS[i] as usize] =
                            ((IO_OUTPUT_ADDRESS[j] as u32 | 0x9) << 16) | self.io_exp_data[IO_BOARDS[i] as usize][j + 1] as u32;
                        spi_data[8] |= 1 << IO_BOARDS[i];
                        //info!("uint8_t: {:#?}", );
                    }
                }
            }
            println!("data: {:#?}", spi_data);
            println!("{:?}", write_data(self.raw_fd, &mut spi_into_driver::write(&mut spi_data)));
            println!("{:?}", transfer_data(self.raw_fd));
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct spi_into_driver {
    tx_buf: u64,
    len:    u32,
}

impl spi_into_driver {
    pub fn write(buff: &mut [u32]) -> Self {
        spi_into_driver {
            tx_buf: buff.as_ptr() as *const () as usize as u64,
            len: (buff.len() * 4) as u32,
            //data: [0;9],
        }
    }
}

pub type SpiTransfer = spi_into_driver;

mod ioctl {
    use super::*;

    const PIVO_SPI_MAGIC: u8 = b'q';
    const PIVO_SPI_WRITE: u8 = 2;
    const PIVO_SET_DATA: u8 = 3;

    ioctl_write_ptr!(set_data_32, PIVO_SPI_MAGIC, PIVO_SET_DATA, spi_into_driver);

    ioctl_none!(write_data_32, PIVO_SPI_MAGIC, PIVO_SPI_WRITE);
}

pub fn write_data(fd: RawFd, transfers: &mut SpiTransfer) -> io::Result<()> {
    unsafe { ioctl::set_data_32(fd, transfers) }?;
    Ok(())
}

pub fn transfer_data(fd: RawFd) -> io::Result<()> {
    unsafe { ioctl::write_data_32(fd) }?;
    Ok(())
}
