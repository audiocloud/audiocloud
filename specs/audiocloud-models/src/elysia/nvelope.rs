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
use audiocloud_api::newtypes::FilterId::{High, HighMid, HighPass, Low, LowMid};
use audiocloud_api::newtypes::{ModelId, ParameterId};

use crate::Manufacturers::Elysia;
use crate::{left_and_right_inputs, left_and_right_outputs, values};

pub fn distopik_dual_1084_id() -> ModelId {
  ModelId::new(Elysia.to_string(), "nvelope".to_owned())
}

lazy_static! {
  // --- power section
  pub static ref CH_ON: ParameterId = ParameterId::from("ch_on");
  // --- knob section
  pub static ref ATTACK: ParameterId = ParameterId::from("attack");
  pub static ref FREQ_A: ParameterId = ParameterId::from("freq_a");

  pub static ref SUSTAIN: ParameterId = ParameterId::from("sustain");
  pub static ref FREQ_S: ParameterId = ParameterId::from("freq_s");
  // --- buttons section
  pub static ref EQ_MODE: ParameterId = ParameterId::from("eq_mode");
  pub static ref FULL_RANGE: ParameterId = ParameterId::from("full_range");
  pub static ref STEREO_LINK: ParameterId = ParameterId::from("stereo_link");
  pub static ref AUTO_GAIN: ParameterId = ParameterId::from("auto_gain");
}

pub fn distopik_xfilter_model() -> Model {
  let params = hashmap! {
    CH_ON.clone() => ch_on(),
    ATTACK.clone() => attack(),
    FREQ_A.clone() => freq_a(),
    SUSTAIN.clone() => sustain(),
    FREQ_S.clone() => freq_s(),
    EQ_MODE.clone() => eq_mode(),
    FULL_RANGE.clone() => full_range(),
    STEREO_LINK.clone() => stereo_link(),
    AUTO_GAIN.clone() => auto_gain(),
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

fn attack() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Amplifier(Global, Gain),
                   values: filter_gain_values_15(), }
}

fn freq_a() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(Low, Frequency),
                   values: vec![values::integer(20),
                                values::integer(21),
                                values::integer(22),
                                values::integer(24),
                                values::integer(25),
                                values::integer(26),
                                values::integer(27),
                                values::integer(28),
                                values::integer(30),
                                values::integer(36),
                                values::integer(42),
                                values::integer(50),
                                values::integer(60),
                                values::integer(75),
                                values::integer(90),
                                values::integer(110),
                                values::integer(125),
                                values::integer(140),
                                values::integer(150),
                                values::integer(160),
                                values::integer(170),
                                values::integer(185),
                                values::integer(200),
                                values::integer(225),
                                values::integer(250),
                                values::integer(275),
                                values::integer(300),
                                values::integer(360),
                                values::integer(420),
                                values::integer(500),
                                values::integer(590),
                                values::integer(750),
                                values::integer(950),
                                values::integer(1200),
                                values::integer(1900),
                                values::integer(2600),
                                values::integer(3300),
                                values::integer(4000),
                                values::integer(5500),
                                values::integer(6800),
                                values::integer(8000)], }
}

fn sustain() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Decibels,
                     role:   Filter(Low, FilterGain),
                     values: filter_gain_values_15(), }
}

fn freq_s() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Hertz,
                   role:   Filter(Low, Frequency),
                   values: vec![values::integer(50),
                                values::integer(52),
                                values::integer(53),
                                values::integer(55),
                                values::integer(56),
                                values::integer(57),
                                values::integer(58),
                                values::integer(60),
                                values::integer(75),
                                values::integer(90),
                                values::integer(100),
                                values::integer(120),
                                values::integer(150),
                                values::integer(180),
                                values::integer(210),
                                values::integer(230),
                                values::integer(260),
                                values::integer(280),
                                values::integer(300),
                                values::integer(330),
                                values::integer(360),
                                values::integer(390),
                                values::integer(420),
                                values::integer(470),
                                values::integer(520),
                                values::integer(560),
                                values::integer(600),
                                values::integer(750),
                                values::integer(900),
                                values::integer(1100),
                                values::integer(1300),
                                values::integer(1700),
                                values::integer(2100),
                                values::integer(2500),
                                values::integer(4000),
                                values::integer(5500),
                                values::integer(7000),
                                values::integer(8500),
                                values::integer(1000),
                                values::integer(1300),
                                values::integer(15000)], }
}

fn eq_mode() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}

fn full_range() -> ModelParameter {
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

fn auto_gain() -> ModelParameter {
  ModelParameter { scope:  Size(1),
                   unit:   Toggle,
                   role:   Filter(High, Bandwidth),
                   values: values::toggle(), }
}

fn filter_gain_values_15() -> Vec<ModelValueOption> {
  vec![values::numbers(-15_f64, 15_f64)]
}
