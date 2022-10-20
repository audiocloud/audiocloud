use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::model::FilterParameterRole::{Bandwidth, Frequency, Gain as FilterGain};
use audiocloud_api::model::GlobalParameterRole::{Bypass, Enable};
use audiocloud_api::model::ModelParameterRole::{Filter, Global};
use audiocloud_api::model::ModelValueUnit::{Decibels, Hertz, Toggle};
use audiocloud_api::model::{ModelValueOption, Model, ModelElementScope, ModelParameter};
use ModelElementScope::AllInputs;
use audiocloud_api::newtypes::FilterId::{High, HighMid, HighPass, Low, LowMid};
use audiocloud_api::newtypes::{ModelId, ParameterId};

use crate::Manufacturers::Elysia;
use crate::{mono_input, mono_output, values};

pub fn distopik_dual_1084_id() -> ModelId {
  ModelId::new(Elysia.to_string(), "xfilter".to_owned())
}

lazy_static! {
  // --- power section
  pub static ref HIT_IT: ParameterId = ParameterId::from("hit_it");
  // --- low eq section
  pub static ref LOW_FREQ: ParameterId = ParameterId::from("low_freq");
  pub static ref LOW_GAIN: ParameterId = ParameterId::from("low_gain");
  pub static ref LOW_WIDTH: ParameterId = ParameterId::from("low_width");
  // --- low mid eq section
  pub static ref LOW_MID_FREQ: ParameterId = ParameterId::from("low_mid_freq");
  pub static ref LOW_MID_GAIN: ParameterId = ParameterId::from("low_mid_gain");
  pub static ref LOW_MID_WIDTH: ParameterId = ParameterId::from("low_mid_width");
  // --- high mid eq section
  pub static ref HIGH_MID_FREQ: ParameterId = ParameterId::from("high_mid_freq");
  pub static ref HIGH_MID_GAIN: ParameterId = ParameterId::from("high_mid_gain");
  pub static ref HIGH_MID_WIDTH: ParameterId = ParameterId::from("high_mid_width");
  // --- high eq section
  pub static ref HIGH_FREQ: ParameterId = ParameterId::from("high_freq");
  pub static ref HIGH_GAIN: ParameterId = ParameterId::from("high_gain");
  pub static ref HIGH_WIDTH: ParameterId = ParameterId::from("high_width");
  // --- output section
  pub static ref PASSIVE: ParameterId = ParameterId::from("passive");
}

pub fn distopik_xfilter_model() -> Model {
  let params = hashmap! {
    HIT_IT.clone() => hit_it(),
    LOW_FREQ.clone() => low_freq(),
    LOW_GAIN.clone() => low_gain(),
    LOW_WIDTH.clone() => low_width(),
    LOW_MID_FREQ.clone() => low_mid_freq(),
    LOW_MID_GAIN.clone() => low_mid_gain(),
    LOW_MID_WIDTH.clone() => low_mid_width(),
    HIGH_MID_FREQ.clone() => high_mid_freq(),
    HIGH_MID_GAIN.clone() => high_mid_gain(),
    HIGH_MID_WIDTH.clone() => high_mid_width(),
    HIGH_FREQ.clone() => high_freq(),
    HIGH_GAIN.clone() => high_gain(),
    HIGH_WIDTH.clone() => high_width(),
    PASSIVE.clone() => passive()
  };

  Model { inputs:     mono_input(),
          outputs:    mono_output(),
          parameters: params,
          resources:  Default::default(),
          reports:    Default::default(),
          media:      false,
          capabilities: Default::default(), }
}

fn hit_it() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Global(Enable),
                   values: values::toggle(), }
}
fn low_freq() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(Low, Frequency),
                   values: vec![values::integer(20),
                                values::integer(23),
                                values::integer(25),
                                values::integer(28),
                                values::integer(30),
                                values::integer(38),
                                values::integer(50),
                                values::integer(65),
                                values::integer(73),
                                values::integer(80),
                                values::integer(90),
                                values::integer(100),
                                values::integer(125),
                                values::integer(150),
                                values::integer(170),
                                values::integer(200),
                                values::integer(300),
                                values::integer(450),
                                values::integer(600),
                                values::integer(750),
                                values::integer(900)], }
}
fn low_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(Low, FilterGain),
                   values: filter_gain_values_16(), }
}
fn low_width() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(Low, Bandwidth),
                   values: values::toggle(), }
}
fn low_mid_freq() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(LowMid, Frequency),
                   values: vec![values::integer(45),
                                values::integer(47),
                                values::integer(50),
                                values::integer(60),
                                values::integer(75),
                                values::integer(100),
                                values::integer(120),
                                values::integer(150),
                                values::integer(180),
                                values::integer(200),
                                values::integer(250),
                                values::integer(300),
                                values::integer(350),
                                values::integer(400),
                                values::integer(500),
                                values::integer(600),
                                values::integer(900),
                                values::integer(1200),
                                values::integer(1500),
                                values::integer(1900),
                                values::integer(2200)], }
}
fn low_mid_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(LowMid, Frequency),
                   values: filter_gain_values_13(), }
}
fn low_mid_width() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(LowMid, Bandwidth),
                   values: values::toggle(), }
}
fn high_mid_freq() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(HighMid, Frequency),
                   values: vec![values::integer(300),
                                values::integer(325),
                                values::integer(350),
                                values::integer(480),
                                values::integer(800),
                                values::integer(1000),
                                values::integer(1300),
                                values::integer(1500),
                                values::integer(1800),
                                values::integer(2000),
                                values::integer(2300),
                                values::integer(2500),
                                values::integer(2900),
                                values::integer(3500),
                                values::integer(4000),
                                values::integer(4500),
                                values::integer(8000),
                                values::integer(10000),
                                values::integer(12000),
                                values::integer(14000),
                                values::integer(16000)], }
}
fn high_mid_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(HighMid, FilterGain),
                   values: filter_gain_values_13(), }
}
fn high_mid_width() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(HighMid, Bandwidth),
                   values: values::toggle(), }
}
fn high_freq() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(High, Frequency),
                   values: vec![values::integer(700),
                                values::integer(780),
                                values::integer(850),
                                values::integer(1000),
                                values::integer(1500),
                                values::integer(2000),
                                values::integer(2800),
                                values::integer(3300),
                                values::integer(3800),
                                values::integer(4200),
                                values::integer(4600),
                                values::integer(5000),
                                values::integer(6000),
                                values::integer(7500),
                                values::integer(9000),
                                values::integer(12000),
                                values::integer(16000),
                                values::integer(20000),
                                values::integer(22000),
                                values::integer(24000),
                                values::integer(28000)], }
}
fn high_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(High, FilterGain),
                   values: filter_gain_values_16(), }
}
fn high_width() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}
fn passive() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}

fn filter_gain_values_16() -> Vec<ModelValueOption> {
  vec![values::numbers(-16_f64, 16_f64)]
}

fn filter_gain_values_13() -> Vec<ModelValueOption> {
  vec![values::numbers(-13_f64, 13_f64)]
}