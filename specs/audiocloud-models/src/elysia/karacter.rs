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
use audiocloud_api::model::ModelValueUnit::{Decibels, Hertz, Toggle, Unitless, Percent};
use audiocloud_api::model::{ModelValueOption, Model, ModelElementScope, ModelParameter};
use ModelElementScope::{AllInputs, Size};
use audiocloud_api::newtypes::{ModelId, ParameterId};
use audiocloud_api::newtypes::FilterId::{High, HighMid, HighPass, Low, LowMid};



use crate::Manufacturers::Elysia;
use crate::{left_and_right_inputs, left_and_right_outputs, values};

pub fn distopik_dual_1084_id() -> ModelId {
  ModelId::new(Elysia.to_string(), "karacter".to_owned())
}

lazy_static! {
  // --- power section
  pub static ref CH_ON: ParameterId = ParameterId::from("ch_on");
  // --- knob section
  pub static ref DRIVE: ParameterId = ParameterId::from("drive");
  pub static ref COLOR: ParameterId = ParameterId::from("color");

  pub static ref GAIN: ParameterId = ParameterId::from("gain");
  pub static ref MIX: ParameterId = ParameterId::from("mix");
  // --- buttons section
  pub static ref FET_SHRED: ParameterId = ParameterId::from("fet_shred");
  pub static ref TURBO_BOOST: ParameterId = ParameterId::from("turbo_boost");
  pub static ref STEREO_LINK: ParameterId = ParameterId::from("stereo_link");
  pub static ref MS_MODE: ParameterId = ParameterId::from("ms_mode");
}

pub fn distopik_xfilter_model() -> Model {
  let params = hashmap! {
    CH_ON.clone() => ch_on(),
    DRIVE.clone() => drive(),
    COLOR.clone() => color(),
    GAIN.clone() => gain(),
    MIX.clone() => mix(),
    FET_SHRED.clone() => fet_shred(),
    TURBO_BOOST.clone() => turbo_boost(),
    STEREO_LINK.clone() => stereo_link(),
    MS_MODE.clone() => ms_mode(),
  };

  Model { inputs:     left_and_right_inputs(),
          outputs:    left_and_right_outputs(),
          parameters: params,
          resources:  Default::default(),
          reports:    Default::default(),
          media:      false,
          capabilities: Default::default(),  }
}

fn ch_on() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   GlobalInstance(Enable),
                   values: values::toggle(), }
}

fn drive() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Unitless,
                   role:   Amplifier(Global, Gain),
                   values: drive_values(), }
}
fn color() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Unitless,
                   role:   Amplifier(Global, Gain),
                   values: color_values(), }
}
fn gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Amplifier(Global, Gain),
                   values: filter_gain_values_11(), }
}
fn mix() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Percent,
                   role:   Amplifier(Global, Gain),
                   values: mix_values(), }
}

fn fet_shred() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}
fn turbo_boost() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
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
fn ms_mode() -> ModelParameter {
  ModelParameter { scope:  Size(1),
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}

fn filter_gain_values_11() -> Vec<ModelValueOption> {
  vec![values::numbers(-11_f64, 11_f64)]
}
fn drive_values() -> Vec<ModelValueOption> {
  vec![values::numbers(0_f64, 11_f64)]
}
fn color_values() -> Vec<ModelValueOption> {
  vec![values::numbers(-1_f64, 1_f64)]
}
fn mix_values() -> Vec<ModelValueOption> {
  vec![values::numbers(0_f64, 100_f64)]
}