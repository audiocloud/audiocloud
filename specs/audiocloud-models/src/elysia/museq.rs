/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::model::AmplifierId::{Global};
use audiocloud_api::model::FilterParameterRole::{Bandwidth, Frequency, Gain as FilterGain};
use audiocloud_api::model::AmplifierParameterRole::{Gain};
use audiocloud_api::model::GlobalParameterRole::{Enable};
use audiocloud_api::model::ModelParameterRole::{Filter, Global as GlobalInstance, Amplifier};
use audiocloud_api::model::ModelValueUnit::{Decibels, Hertz, Toggle};
use audiocloud_api::model::{ModelValueOption, Model, ModelElementScope, ModelParameter};
use ModelElementScope::{AllInputs, Size};
use audiocloud_api::newtypes::FilterId::{High, HighMid, Low, LowMid, Mid};
use audiocloud_api::newtypes::{ModelId, ParameterId};

use crate::Manufacturers::Elysia;
use crate::{left_and_right_inputs, left_and_right_outputs, values};

pub fn distopik_dual_1084_id() -> ModelId {
  ModelId::new(Elysia.to_string(), "museq".to_owned())
}

lazy_static! {
  // --- power section
  pub static ref CH_ON: ParameterId = ParameterId::from("ch_on");
  // --- freq section
  pub static ref LOW_GAIN: ParameterId = ParameterId::from("low_gain");
  pub static ref LOW_FREQ: ParameterId = ParameterId::from("low_freq");
  pub static ref BOTTOM_GAIN: ParameterId = ParameterId::from("bottom_gain");
  pub static ref BOTTOM_FREQ: ParameterId = ParameterId::from("bottom_freq");
  pub static ref MIDDLE_GAIN: ParameterId = ParameterId::from("middle_gain");
  pub static ref MIDDLE_FREQ: ParameterId = ParameterId::from("middle_freq");
  pub static ref TOP_GAIN: ParameterId = ParameterId::from("top_gain");
  pub static ref TOP_FREQ: ParameterId = ParameterId::from("top_freq");
  pub static ref HIGH_GAIN: ParameterId = ParameterId::from("high_gain");
  pub static ref HIGH_FREQ: ParameterId = ParameterId::from("high_freq");
  // --- shelf/narrow settings
  pub static ref Q_BOTTOM: ParameterId = ParameterId::from("q_bottom");
  pub static ref Q_LOW: ParameterId = ParameterId::from("q_low");
  pub static ref Q_MIDDLE: ParameterId = ParameterId::from("q_middle");
  pub static ref Q_TOP: ParameterId = ParameterId::from("q_top");
  pub static ref Q_HIGH: ParameterId = ParameterId::from("q_high");
  // --- boost/cut settings
  pub static ref B_BOTTOM: ParameterId = ParameterId::from("b_bottom");
  pub static ref B_LOW: ParameterId = ParameterId::from("b_low");
  pub static ref B_MIDDLE: ParameterId = ParameterId::from("b_middle");
  pub static ref B_TOP: ParameterId = ParameterId::from("b_top");
  pub static ref B_HIGH: ParameterId = ParameterId::from("b_high");
  // --- boost/cut settings
  pub static ref WARM: ParameterId = ParameterId::from("warm");
  pub static ref STEREO_LINK: ParameterId = ParameterId::from("stereo_link");
}

pub fn distopik_xfilter_model() -> Model {
  let params = hashmap! {
    CH_ON.clone() => ch_on(),
    LOW_GAIN.clone() => low_gain(),
    LOW_FREQ.clone() => low_freq(),
    BOTTOM_GAIN.clone() => bottom_gain(),
    BOTTOM_FREQ.clone() => bottom_freq(),
    MIDDLE_GAIN.clone() => middle_gain(),
    MIDDLE_FREQ.clone() => middle_freq(),
    TOP_GAIN.clone() => top_gain(),
    TOP_FREQ.clone() => top_freq(),
    HIGH_GAIN.clone() => high_gain(),
    HIGH_FREQ.clone() => high_freq(),
    //shelf/narrow settings
    Q_BOTTOM.clone() => q_bottom(),
    Q_LOW.clone() => q_low(),
    Q_MIDDLE.clone() => q_middle(),
    Q_TOP.clone() => q_top(),
    Q_HIGH.clone() => q_high(),
    //boost/cut settings
    B_BOTTOM.clone() => b_bottom(),
    B_LOW.clone() => b_low(),
    B_MIDDLE.clone() => b_middle(),
    B_TOP.clone() => b_top(),
    B_HIGH.clone() => b_high(),
    // main actions
    WARM.clone() => warm(),
    STEREO_LINK.clone() => stereo_link(),
  };

  Model { inputs:     left_and_right_inputs(),
          outputs:    left_and_right_outputs(),
          parameters: params,
          resources:  Default::default(),
          reports:    Default::default(),
          media:      false,
          capabilities: Default::default(), }
}

fn ch_on() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   GlobalInstance(Enable),
                   values: values::toggle(), }
}

fn low_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(Low, FilterGain),
                   values: filter_gain_values_15(), }
}

fn low_freq() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(Low, Frequency),
                   values: vec![values::integer(9),
                                values::integer(10),
                                values::integer(11),
                                values::integer(12),
                                values::integer(15),
                                values::integer(18),
                                values::integer(30),
                                values::integer(45),
                                values::integer(55),
                                values::integer(60),
                                values::integer(65),
                                values::integer(60),
                                values::integer(75),
                                values::integer(80),
                                values::integer(95),
                                values::integer(110),
                                values::integer(120),
                                values::integer(140),
                                values::integer(160),
                                values::integer(200),
                                values::integer(170),], }
}

fn bottom_gain() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Decibels,
                     role:   Filter(LowMid, FilterGain),
                     values: filter_gain_values_15(), }
}

fn bottom_freq() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(LowMid, Frequency),
                   values: vec![values::integer(18),
                                values::integer(20),
                                values::integer(23),
                                values::integer(25),
                                values::integer(30),
                                values::integer(35),
                                values::integer(50),
                                values::integer(60),
                                values::integer(80),
                                values::integer(110),
                                values::integer(120),
                                values::integer(130),
                                values::integer(140),
                                values::integer(150),
                                values::integer(165),
                                values::integer(185),
                                values::integer(210),
                                values::integer(230),
                                values::integer(280),
                                values::integer(320),
                                values::integer(400),], }
}

fn middle_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(Mid, FilterGain),
                   values: filter_gain_values_15(), }
}

fn middle_freq() -> ModelParameter {
ModelParameter { scope:  AllInputs,
                 unit:   Hertz,
                 role:   Filter(Mid, Frequency),
                 values: vec![values::integer(150),
                              values::integer(170),
                              values::integer(190),
                              values::integer(210),
                              values::integer(255),
                              values::integer(300),
                              values::integer(410),
                              values::integer(520),
                              values::integer(630),
                              values::integer(770),
                              values::integer(900),
                              values::integer(1100),
                              values::integer(1200),
                              values::integer(1300),
                              values::integer(1400),
                              values::integer(1500),
                              values::integer(1750),
                              values::integer(2000),
                              values::integer(1400),
                              values::integer(2700),
                              values::integer(3500),], }
}

fn top_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(HighMid, FilterGain),
                   values: filter_gain_values_15(), }
}

fn top_freq() -> ModelParameter {
ModelParameter { scope:  AllInputs,
                 unit:   Hertz,
                 role:   Filter(HighMid, Frequency),
                 values: vec![values::integer(700),
                              values::integer(780),
                              values::integer(890),
                              values::integer(1000),
                              values::integer(1350),
                              values::integer(1700),
                              values::integer(2100),
                              values::integer(2500),
                              values::integer(3100),
                              values::integer(3700),
                              values::integer(4300),
                              values::integer(4900),
                              values::integer(5400),
                              values::integer(5800),
                              values::integer(4500),
                              values::integer(7100),
                              values::integer(8200),
                              values::integer(9300),
                              values::integer(11200),
                              values::integer(13000),
                              values::integer(16000),], }
}

fn high_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(High, FilterGain),
                   values: filter_gain_values_15(), }
}

fn high_freq() -> ModelParameter {
ModelParameter { scope:  AllInputs,
                 unit:   Hertz,
                 role:   Filter(High, Frequency),
                 values: vec![values::integer(1800),
                              values::integer(2000),
                              values::integer(2300),
                              values::integer(2500),
                              values::integer(3000),
                              values::integer(3500),
                              values::integer(4100),
                              values::integer(5700),
                              values::integer(7800),
                              values::integer(10000),
                              values::integer(11000),
                              values::integer(12000),
                              values::integer(13000),
                              values::integer(14000),
                              values::integer(15500),
                              values::integer(17000),
                              values::integer(19500),
                              values::integer(22000),
                              values::integer(26000),
                              values::integer(30000),
                              values::integer(35000),], }
}

//shelf/narrow settings
fn q_low() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(Low, Bandwidth),
                   values: values::toggle(), }
}
fn q_bottom() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(LowMid, Bandwidth),
                   values: values::toggle(), }
}
fn q_middle() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(Mid, Bandwidth),
                   values: values::toggle(), }
}
fn q_top() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(HighMid, Bandwidth),
                   values: values::toggle(), }
}
fn q_high() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}

//boost/cut settings
fn b_low() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(Low, Bandwidth),
                   values: values::toggle(), }
}
fn b_bottom() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(LowMid, Bandwidth),
                   values: values::toggle(), }
}
fn b_middle() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(Mid, Bandwidth),
                   values: values::toggle(), }
}
fn b_top() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(HighMid, Bandwidth),
                   values: values::toggle(), }
}
fn b_high() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}

// main actions
fn warm() -> ModelParameter {
  ModelParameter { scope:  Size(1),
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}
fn stereo_link() -> ModelParameter {
  ModelParameter { scope:  Size(1),
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}


fn filter_gain_values_15() -> Vec<ModelValueOption> {
  vec![values::numbers(0_f64, 15_f64)]
}
