#![allow(unused_imports)]

use audiocloud_api::api::*;
use audiocloud_api::model::*;
use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

pub mod audiocloud {
    use super::*;

    pub const NAME: &str = "audiocloud";

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct Insert1X1Preset {}
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Insert1X1Parameters {}
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Insert1X1Reports {
        pub insert_input:  Option<f64>,
        pub insert_output: Option<f64>,
    }
    pub mod insert_1x1 {
        use super::*;
        pub const NAME: &str = "insert_1x1";
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct Insert24X2Preset {}
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Insert24X2Parameters {}
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Insert24X2Reports {
        pub insert_input:  Option<Vec<f64>>,
        pub insert_output: Option<Stereo<f64>>,
    }
    pub mod insert_24x2 {
        use super::*;
        pub const NAME: &str = "insert_24x2";
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct Insert2X2Preset {}
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Insert2X2Parameters {}
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Insert2X2Reports {
        pub insert_input:  Option<Stereo<f64>>,
        pub insert_output: Option<Stereo<f64>>,
    }
    pub mod insert_2x2 {
        use super::*;
        pub const NAME: &str = "insert_2x2";
    }
}

pub mod distopik {
    use super::*;

    pub const NAME: &str = "distopik";

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct Dual1084Preset {
        pub eql_toggle:       Stereo<bool>,
        pub high_freq:        Stereo<ToggleOr<u64>>,
        pub high_gain:        Stereo<f64>,
        pub high_mid_freq:    Stereo<ToggleOr<u64>>,
        pub high_mid_gain:    Stereo<f64>,
        pub high_mid_width:   Stereo<bool>,
        pub high_pass_filter: Stereo<ToggleOr<u64>>,
        pub input_gain:       Stereo<ToggleOr<i64>>,
        pub low_freq:         Stereo<ToggleOr<u64>>,
        pub low_gain:         Stereo<f64>,
        pub low_mid_freq:     Stereo<ToggleOr<u64>>,
        pub low_mid_gain:     Stereo<f64>,
        pub low_mid_width:    Stereo<bool>,
        pub output_pad:       Stereo<ToggleOr<i64>>,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Dual1084Parameters {
        pub eql_toggle:       Option<Stereo<bool>>,
        pub high_freq:        Option<Stereo<ToggleOr<u64>>>,
        pub high_gain:        Option<Stereo<f64>>,
        pub high_mid_freq:    Option<Stereo<ToggleOr<u64>>>,
        pub high_mid_gain:    Option<Stereo<f64>>,
        pub high_mid_width:   Option<Stereo<bool>>,
        pub high_pass_filter: Option<Stereo<ToggleOr<u64>>>,
        pub input_gain:       Option<Stereo<ToggleOr<i64>>>,
        pub low_freq:         Option<Stereo<ToggleOr<u64>>>,
        pub low_gain:         Option<Stereo<f64>>,
        pub low_mid_freq:     Option<Stereo<ToggleOr<u64>>>,
        pub low_mid_gain:     Option<Stereo<f64>>,
        pub low_mid_width:    Option<Stereo<bool>>,
        pub output_pad:       Option<Stereo<ToggleOr<i64>>>,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct Dual1084Reports {}
    pub mod dual1084 {
        use super::*;
        pub const NAME: &str = "dual1084";

        pub const EQL_TOGGLE_NAME: &str = "eql_toggle";
        pub const EQL_TOGGLE_VALUES: [ModelValueOption; 2] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                              ModelValueOption::Single(ModelValue::Bool(true))];
        pub const HIGH_FREQ_NAME: &str = "high_freq";
        pub const HIGH_FREQ_VALUES: [ModelValueOption; 6] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                             ModelValueOption::Single(ModelValue::Number(8000_f64)),
                                                             ModelValueOption::Single(ModelValue::Number(10000_f64)),
                                                             ModelValueOption::Single(ModelValue::Number(12000_f64)),
                                                             ModelValueOption::Single(ModelValue::Number(16000_f64)),
                                                             ModelValueOption::Single(ModelValue::Number(20000_f64))];
        pub const HIGH_GAIN_NAME: &str = "high_gain";
        pub const HIGH_GAIN_VALUES: [ModelValueOption; 1] =
            [ModelValueOption::Range(ModelValue::Number(-16_f64), ModelValue::Number(16_f64))];
        pub const HIGH_MID_FREQ_NAME: &str = "high_mid_freq";
        pub const HIGH_MID_FREQ_VALUES: [ModelValueOption; 12] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                                  ModelValueOption::Single(ModelValue::Number(360_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(480_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(720_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(1600_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(2400_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(3200_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(3900_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(4800_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(6400_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(7200_f64)),
                                                                  ModelValueOption::Single(ModelValue::Number(8400_f64))];
        pub const HIGH_MID_GAIN_NAME: &str = "high_mid_gain";
        pub const HIGH_MID_GAIN_VALUES: [ModelValueOption; 1] =
            [ModelValueOption::Range(ModelValue::Number(-12_f64), ModelValue::Number(12_f64))];
        pub const HIGH_MID_WIDTH_NAME: &str = "high_mid_width";
        pub const HIGH_MID_WIDTH_VALUES: [ModelValueOption; 2] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                                  ModelValueOption::Single(ModelValue::Bool(true))];
        pub const HIGH_PASS_FILTER_NAME: &str = "high_pass_filter";
        pub const HIGH_PASS_FILTER_VALUES: [ModelValueOption; 6] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                                    ModelValueOption::Single(ModelValue::Number(22_f64)),
                                                                    ModelValueOption::Single(ModelValue::Number(45_f64)),
                                                                    ModelValueOption::Single(ModelValue::Number(70_f64)),
                                                                    ModelValueOption::Single(ModelValue::Number(160_f64)),
                                                                    ModelValueOption::Single(ModelValue::Number(360_f64))];
        pub const INPUT_GAIN_NAME: &str = "input_gain";
        pub const INPUT_GAIN_VALUES: [ModelValueOption; 8] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                              ModelValueOption::Single(ModelValue::Number(-10_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(-5_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(0_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(5_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(10_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(15_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(20_f64))];
        pub const LOW_FREQ_NAME: &str = "low_freq";
        pub const LOW_FREQ_VALUES: [ModelValueOption; 6] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                            ModelValueOption::Single(ModelValue::Number(20_f64)),
                                                            ModelValueOption::Single(ModelValue::Number(35_f64)),
                                                            ModelValueOption::Single(ModelValue::Number(60_f64)),
                                                            ModelValueOption::Single(ModelValue::Number(110_f64)),
                                                            ModelValueOption::Single(ModelValue::Number(220_f64))];
        pub const LOW_GAIN_NAME: &str = "low_gain";
        pub const LOW_GAIN_VALUES: [ModelValueOption; 1] =
            [ModelValueOption::Range(ModelValue::Number(-16_f64), ModelValue::Number(16_f64))];
        pub const LOW_MID_FREQ_NAME: &str = "low_mid_freq";
        pub const LOW_MID_FREQ_VALUES: [ModelValueOption; 12] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                                 ModelValueOption::Single(ModelValue::Number(120_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(180_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(240_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(360_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(480_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(720_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(1600_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(2400_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(3200_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(4800_f64)),
                                                                 ModelValueOption::Single(ModelValue::Number(7200_f64))];
        pub const LOW_MID_GAIN_NAME: &str = "low_mid_gain";
        pub const LOW_MID_GAIN_VALUES: [ModelValueOption; 1] =
            [ModelValueOption::Range(ModelValue::Number(-12_f64), ModelValue::Number(12_f64))];
        pub const LOW_MID_WIDTH_NAME: &str = "low_mid_width";
        pub const LOW_MID_WIDTH_VALUES: [ModelValueOption; 2] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                                 ModelValueOption::Single(ModelValue::Bool(true))];
        pub const OUTPUT_PAD_NAME: &str = "output_pad";
        pub const OUTPUT_PAD_VALUES: [ModelValueOption; 3] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                              ModelValueOption::Single(ModelValue::Number(-10_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(-20_f64))];
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct SummatraPreset {
        pub bus_assign: Vec<u64>,
        pub input:      Vec<f64>,
        pub pan:        Vec<f64>,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct SummatraParameters {
        pub bus_assign: Option<Vec<u64>>,
        pub input:      Option<Vec<f64>>,
        pub pan:        Option<Vec<f64>>,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct SummatraReports {}
    pub mod summatra {
        use super::*;
        pub const NAME: &str = "summatra";

        pub const BUS_ASSIGN_NAME: &str = "bus_assign";
        pub const BUS_ASSIGN_VALUES: [ModelValueOption; 3] = [ModelValueOption::Single(ModelValue::Number(0_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(1_f64)),
                                                              ModelValueOption::Single(ModelValue::Number(2_f64))];
        pub const INPUT_NAME: &str = "input";
        pub const INPUT_VALUES: [ModelValueOption; 1] = [ModelValueOption::Range(ModelValue::Number(-48_f64), ModelValue::Number(10_f64))];
        pub const PAN_NAME: &str = "pan";
        pub const PAN_VALUES: [ModelValueOption; 1] = [ModelValueOption::Range(ModelValue::Number(-1_f64), ModelValue::Number(1_f64))];
    }
}

pub mod netio {
    use super::*;

    pub const NAME: &str = "netio";

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct PowerPdu4CPreset {
        pub power: Vec<bool>,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct PowerPdu4CParameters {
        pub power: Option<Vec<bool>>,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
    pub struct PowerPdu4CReports {
        pub current:      Option<Vec<f64>>,
        pub energy:       Option<Vec<f64>>,
        pub power:        Option<Vec<bool>>,
        pub power_factor: Option<Vec<f64>>,
    }
    pub mod power_pdu_4c {
        use super::*;
        pub const NAME: &str = "power_pdu_4c";

        pub const POWER_NAME: &str = "power";
        pub const POWER_VALUES: [ModelValueOption; 2] = [ModelValueOption::Single(ModelValue::Bool(false)),
                                                         ModelValueOption::Single(ModelValue::Bool(true))];
    }
}

pub fn schemas() -> RootSchema {
    merge_schemas([schema_for!(self::audiocloud::Insert1X1Preset),
                   schema_for!(self::audiocloud::Insert1X1Parameters),
                   schema_for!(self::audiocloud::Insert1X1Reports),
                   schema_for!(self::audiocloud::Insert24X2Preset),
                   schema_for!(self::audiocloud::Insert24X2Parameters),
                   schema_for!(self::audiocloud::Insert24X2Reports),
                   schema_for!(self::audiocloud::Insert2X2Preset),
                   schema_for!(self::audiocloud::Insert2X2Parameters),
                   schema_for!(self::audiocloud::Insert2X2Reports),
                   schema_for!(self::distopik::Dual1084Preset),
                   schema_for!(self::distopik::Dual1084Parameters),
                   schema_for!(self::distopik::Dual1084Reports),
                   schema_for!(self::distopik::SummatraPreset),
                   schema_for!(self::distopik::SummatraParameters),
                   schema_for!(self::distopik::SummatraReports),
                   schema_for!(self::netio::PowerPdu4CPreset),
                   schema_for!(self::netio::PowerPdu4CParameters),
                   schema_for!(self::netio::PowerPdu4CReports)].into_iter())
}
