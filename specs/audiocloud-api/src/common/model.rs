/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::collections::{HashMap, HashSet};
use std::iter;

use anyhow::anyhow;
use derive_more::{Display, IsVariant, Unwrap};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::{FilterId, ParameterId, ReportId};

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, IsVariant, JsonSchema)]
pub enum ModelValueUnit {
    #[serde(rename = "no")]
    Unitless,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "dB")]
    Decibels,
    #[serde(rename = "hz")]
    Hertz,
    #[serde(rename = "oct")]
    Octaves,
    #[serde(rename = "toggle")]
    Toggle,
    #[serde(rename = "amps")]
    Amperes,
    #[serde(rename = "watthrs")]
    WattHours,
}

impl Default for ModelValueUnit {
    fn default() -> Self {
        Self::Unitless
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd, IsVariant, Unwrap, JsonSchema)]
#[serde(untagged)]
pub enum ModelValueOption {
    Single(ModelValue),
    Range(ModelValue, ModelValue),
}

impl ModelValueOption {
    pub fn num_range(min: f64, max: f64) -> Self {
        Self::Range(ModelValue::Number(min), ModelValue::Number(max))
    }

    pub fn zero_to(max: f64) -> Self {
        Self::num_range(0f64, max)
    }

    pub fn to_zero(min: f64) -> Self {
        Self::num_range(min, 0f64)
    }

    pub fn get_simple_type(&self) -> anyhow::Result<SimpleModelValueType> {
        match self {
            ModelValueOption::Single(value) => Ok(value.get_simple_type()),
            ModelValueOption::Range(first, second) => {
                if first.is_number() && second.is_number() {
                    Ok(SimpleModelValueType::Number { signed:  true,
                                                      integer: false, })
                } else {
                    Err(anyhow!("Only numeric ranges supported"))
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd, IsVariant, Unwrap, JsonSchema)]
#[serde(untagged)]
pub enum ModelValue {
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SimpleModelValueType {
    String,
    Number { integer: bool, signed: bool },
    Bool,
}

impl SimpleModelValueType {
    pub fn from_numeric_value(number: f64) -> SimpleModelValueType {
        let mut integer = false;
        let mut signed = false;

        if number.fract() == 0.0 {
            if number.is_sign_negative() {
                signed = true;
            }

            integer = true;
        }

        return Self::Number { integer, signed };
    }

    pub fn try_widen(self, other: SimpleModelValueType) -> anyhow::Result<SimpleModelValueType> {
        match (self, other) {
            (Self::Number { signed: s1, integer: i1 }, Self::Number { signed: s2, integer: i2 }) => Ok(Self::Number { signed:  s1 || s2,
                                                                                                                      integer: i1 && i2, }),
            _ => Err(anyhow!("Only numeric types may be widened")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelValueType {
    Single(SimpleModelValueType),
    Either(SimpleModelValueType, SimpleModelValueType),
    Any,
}

impl ModelValue {
    pub fn into_f64(self) -> Option<f64> {
        self.to_f64()
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some(*v),
            ModelValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        }
    }

    pub fn into_i64(self) -> Option<i64> {
        self.to_i64()
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some(*v as i64),
            ModelValue::Bool(b) => Some(if *b { 1 } else { 0 }),
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        self.to_bool()
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some({
                if *v == 0.0 {
                    false
                } else {
                    true
                }
            }),
            ModelValue::Bool(b) => Some(*b),
        }
    }

    pub fn get_simple_type(&self) -> SimpleModelValueType {
        match self {
            ModelValue::String(_) => SimpleModelValueType::String,
            ModelValue::Number(value) => SimpleModelValueType::from_numeric_value(*value),
            ModelValue::Bool(_) => SimpleModelValueType::Bool,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            ModelValue::String(s) => s.clone().into(),
            ModelValue::Number(n) => (*n).into(),
            ModelValue::Bool(b) => (*b).into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelInput {
    Audio(ControlChannels),
    Sidechain,
    Midi,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelOutput {
    Audio(ControlChannels),
    Midi,
}

pub type ModelInputs = Vec<ModelInput>;
pub type ModelOutputs = Vec<ModelOutput>;

pub type ModelParameters = HashMap<ParameterId, ModelParameter>;
pub type ModelReports = HashMap<ReportId, ModelReport>;

/// A model describes the parameters and reprots of a processor
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, JsonSchema)]
pub struct Model {
    #[serde(default)]
    pub resources:    HashMap<ResourceId, f64>,
    pub inputs:       ModelInputs,
    pub outputs:      ModelOutputs,
    pub parameters:   ModelParameters,
    pub reports:      ModelReports,
    pub media:        bool,
    #[serde(default)]
    pub capabilities: HashSet<ModelCapability>,
}

impl Model {
    pub fn get_audio_input_channel_count(&self) -> usize {
        self.inputs
            .iter()
            .map(|input| match input {
                ModelInput::Audio(_) => 1,
                _ => 0,
            })
            .sum()
    }

    pub fn get_audio_output_channel_count(&self) -> usize {
        self.outputs
            .iter()
            .map(|output| match output {
                ModelOutput::Audio(_) => 1,
                _ => 0,
            })
            .sum()
    }

    pub fn default_parameter_values(&self) -> serde_json::Value {
        let mut rv = serde_json::Map::new();
        for (k, v) in self.parameters.iter() {
            let value = match (v.default.as_ref(), v.values.first()) {
                (Some(value), _) => value.to_json(),
                (_, Some(ModelValueOption::Single(value))) => value.to_json(),
                (_, Some(ModelValueOption::Range(low, _))) => low.to_json(),
                (_, _) => serde_json::Value::default(),
            };

            let count = v.scope.len(self);
            let values = serde_json::Value::Array(iter::repeat(value).take(count).collect::<Vec<_>>());

            rv.insert(k.as_str().to_owned(), values);
        }

        serde_json::Value::Object(rv)
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelCapability {
    PowerDistributor,
    AudioRouter,
    AudioMixer,
    DigitalInputOutput,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PowerDistributorReports {
    pub power: Option<Vec<bool>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelParameterRole {
    #[unwrap(ignore)]
    NoRole,
    Power,
    Global(GlobalParameterRole),
    Channel(ChannelParameterRole),
    Amplifier(AmplifierId, AmplifierParameterRole),
    Dynamics(DynamicsId, DynamicsParameterRole),
    Filter(FilterId, FilterParameterRole),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChannelParameterRole {
    Pan,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GlobalParameterRole {
    Enable,
    Bypass,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierParameterRole {
    Enable,
    Gain,
    Distortion,
    SlewRate,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsParameterRole {
    Ratio,
    Threshold,
    Ceiling,
    Attack,
    Release,
    AutoRelease,
    AutoAttack,
    AutoRatio,
    Knee,
    DetectorInput,
    DetectorMaterial,
    DetectorFilter,
    MidEmphasis,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FilterParameterRole {
    Gain,
    GainDirection,
    Frequency,
    Bandwidth,
    Type,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelReportRole {
    #[unwrap(ignore)]
    NoRole,
    Power(PowerReportRole),
    Amplifier(AmplifierId, AmplifierReportRole),
    Dynamics(DynamicsId, DynamicsReportRole),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PowerReportRole {
    Powered,
    Current,
    PowerFactor,
    TotalEnergy,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierReportRole {
    PeakVolume,
    RmsVolume,
    LufsVolumeMomentary,
    LufsVolumeShortTerm,
    LufsVolumeIntegrated,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsReportRole {
    GainReduction,
    GainReductionLimitHit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd, JsonSchema)]
pub struct ModelParameter {
    pub scope:   ModelElementScope,
    #[serde(default)]
    pub unit:    ModelValueUnit,
    pub role:    ModelParameterRole,
    pub values:  Vec<ModelValueOption>,
    #[serde(default)]
    pub default: Option<ModelValue>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelElementScope {
    Global,
    AllInputs,
    AllOutputs,
    Count(usize),
}

impl ModelElementScope {
    pub fn len(self, model: &Model) -> usize {
        match self {
            ModelElementScope::Global => 1,
            ModelElementScope::AllInputs => model.inputs.len(),
            ModelElementScope::AllOutputs => model.outputs.len(),
            ModelElementScope::Count(num) => num,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ModelReport {
    pub scope:    ModelElementScope,
    #[serde(default)]
    pub unit:     ModelValueUnit,
    pub role:     ModelReportRole,
    pub values:   Vec<ModelValueOption>,
    #[serde(default)]
    pub public:   bool,
    #[serde(default)]
    pub volatile: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ControlChannels {
    Global,
    Left,
    Right,
    Generic,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Hash, Display, JsonSchema)]
pub enum ResourceId {
    // in GiB
    #[serde(rename = "ram")]
    Memory,
    // in Ghz
    #[serde(rename = "cpu")]
    CPU,
    // in cuda cores
    #[serde(rename = "gpu")]
    GPU,
    // in percent?
    #[serde(rename = "antelope_dsp")]
    AntelopeDSP,
    // in percent?
    #[serde(rename = "universal_audio_dsp")]
    UniversalAudioDSP,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierId {
    Input,
    Output,
    Global,
    InsertInput,
    InsertOutput,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsId {
    Total,
    Compressor,
    Gate,
    Limiter,
    DeEsser,
}

pub fn get_values_type(options: &Vec<ModelValueOption>) -> anyhow::Result<ModelValueType> {
    let simple_options = options.iter()
                                .map(ModelValueOption::get_simple_type)
                                .filter_map(Result::ok)
                                .collect::<HashSet<_>>();

    let maybe_numeric_type = simple_options.iter()
                                           .filter(|x| x.is_number())
                                           .copied()
                                           .reduce(|a, b| a.try_widen(b).unwrap());

    let mut other_types = simple_options.into_iter()
                                        .filter(|x| !x.is_number())
                                        .collect::<HashSet<_>>()
                                        .into_iter();

    let first = other_types.next();
    let second = other_types.next();
    let third = other_types.next();

    Ok(match (maybe_numeric_type, first, second, third) {
        (None, None, None, None) => return Err(anyhow!("value without any times, illegal")),
        (Some(numeric), None, None, None) => ModelValueType::Single(numeric),
        (None, Some(first), None, None) => ModelValueType::Single(first),
        (Some(numeric), Some(first), None, None) => ModelValueType::Either(numeric, first),
        (None, Some(first), Some(second), None) => ModelValueType::Either(first, second),
        _ => ModelValueType::Any, //
    })
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant, JsonSchema)]
#[serde(untagged)]
pub enum ToggleOr<T> {
    Toggle(bool),
    Value(T),
}

impl ToggleOr<i64> {
    pub fn to_f64(self) -> ToggleOr<f64> {
        match self {
            Self::Toggle(value) => ToggleOr::Toggle(value),
            Self::Value(value) => ToggleOr::Value(value as f64),
        }
    }
}

impl ToggleOr<u64> {
    pub fn to_f64(self) -> ToggleOr<f64> {
        match self {
            Self::Toggle(value) => ToggleOr::Toggle(value),
            Self::Value(value) => ToggleOr::Value(value as f64),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub struct Stereo<T> {
    pub left:  T,
    pub right: T,
}

impl<T> Stereo<T> {
    pub fn both(value: T) -> Self
        where T: Clone
    {
        Self { left:  { value.clone() },
               right: { value }, }
    }
}

pub fn toggle_off<T>() -> ToggleOr<T> {
    ToggleOr::Toggle(false)
}

pub fn toggle_value<T>(value: T) -> ToggleOr<T> {
    ToggleOr::Value(value)
}
