use actix::Recipient;
use serde::{Deserialize, Serialize};

use audiocloud_api::newtypes::FixedInstanceId;

use crate::{Command, InstanceConfig};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {}

impl InstanceConfig for Config {
    fn create(self, _id: FixedInstanceId) -> anyhow::Result<Recipient<Command>> {
        todo!()
    }
}

// struct Dual1084 {
//     id:             FixedInstanceId,
//     config:         Config,
//     last_update:    Timestamp,
//     raw_fd:         RawFd,
//     io_exp_data:    [[u16; 6]; 8],

//     input_gain:             [UnipotRegion; 2],//[L,D]
//     input_gain_param:       ModelParameter,   //  1
//     high_pass_filter:       [UnipotRegion; 4],
//     high_pass_filter_param: ModelParameter,
//     low_freq:               [UnipotRegion; 4],//[L,D,L,D]
//     low_freq_param:         ModelParameter,   //  1   2
//     low_gain:               [UnipotRegion; 2],
//     low_gain_param:         ModelParameter,
//     low_mid_freq:           [UnipotRegion; 2],
//     low_mid_freq_param:     ModelParameter,
//     low_mid_gain:           [UnipotRegion; 2],
//     low_mid_gain_param:     ModelParameter,
//     low_mid_width:          [UnipotRegion; 2],
//     low_mid_width_param:    ModelParameter,
//     high_mid_freq:          [UnipotRegion; 2],
//     high_mid_freq_param:    ModelParameter,
//     high_mid_gain:          [UnipotRegion; 2],
//     high_mid_gain_param:    ModelParameter,
//     high_mid_width:         [UnipotRegion; 2],
//     high_mid_width_param:   ModelParameter,
//     high_freq:              [UnipotRegion; 4],
//     high_freq_param:        ModelParameter,
//     high_gain:              [UnipotRegion; 2],
//     high_gain_param:        ModelParameter,
//     output_pad:             [UnipotRegion; 2],
//     output_pad_param:       ModelParameter,
//     eql_toggle:             [UnipotRegion; 2],
//     eql_toggle_param:       ModelParameter,
// }

// // TODO: move to separate utils file, as we think there will be many more "unipot" etc drivers
// struct UnipotRegion {
//     bits: Vec<usize>,
//     pot_id: usize,
// }

// impl UnipotRegion {
//     pub fn new(pot_id: usize, bits: impl Iterator<Item = usize>) -> Self {
//         Self {
//             bits: bits.collect(),
//             pot_id,
//         }
//     }

//     pub fn write(&self, memory: &mut [[u16; 6]; 8], value: u16) {
//         // for every i-th bit set in the value, we set self.bits[i]-th bit in `memory`
//         // bits can be arbitrarily far from the beginning of the buffer, as long as there
//         // is space
//         for (i, bit) in self.bits.iter().copied().enumerate() {
//             let bit_value = value & (1 << i);
//             write_bit_16(&mut memory[self.pot_id][(bit / 16)+1], (bit % 16) as u16, bit_value);
//         }
//         memory[self.pot_id][0] = 1;
//     }
// }

// struct UnirelRegion {
//     bits: Vec<usize>,
//     pot_id: usize,
// }

// impl UnirelRegion {
//     pub fn new(pot_id: usize, bits: impl Iterator<Item = usize>) -> Self {
//         Self {
//             bits: bits.collect(),
//             pot_id,
//         }
//     }

//     pub fn write_switch(&self, memory: &mut [[u16; 6]; 8], value: u16) {
//         //writes a bit to a correct location in memory
//         for (i, bit) in self.bits.iter().copied().enumerate() {
//             write_bit_16(&mut memory[self.pot_id][(bit / 16) + 1], (bit % 16) as u16, value);
//         }
//         memory[self.pot_id][0] = 1;
//     }
//     pub fn write_rot_switch(&self, memory: &mut [[u16; 6]; 8], value: u16) {
//         // rotation switches use moving bits 0000 -> 0001 -> 0010 -> 0100...
//         for (i, bit) in self.bits.iter().copied().enumerate() {
//             write_bit_16(&mut memory[self.pot_id][(bit / 16) + 1], (bit % 16) as u16 , (value == i as u16) as u16);
//         }
//         memory[self.pot_id][0] = 1;
//     }
//     pub fn write_nrot_switch(&self, memory: &mut [[u16; 6]; 8], value: u16) {
//         // negated rot switch has negated first bit/switch
//         for (i, bit) in self.bits.iter().copied().enumerate() {
//             let mut temp: bool = false;
//             if value != 0 && i == 0 {
//               temp = true
//             }
//             if value == 2 && i == 1 {
//               temp = true
//             }
//             write_bit_16(&mut memory[self.pot_id][(bit / 16) + 1], (bit % 16) as u16, temp as u16);
//         }
//         memory[self.pot_id][0] = 1;
//     }
// }

// impl Dual1084 {
//     pub fn new(id: FixedInstanceId, config: Config) -> anyhow::Result<Self> {
//         let raw_fd = File::options().read(true).write(true).open("/dev/PIVO")?.into_raw_fd();
//         let model = dual_1084::distopik_dual_1084_model();

//         Ok(Dual1084 { id,
//                       config,
//                       raw_fd,
//                       last_update:            Utc::now(),
//                       io_exp_data:            [[0; 6]; 8],
//                       input_gain:             [UnipotRegion::new(3, 72..=79), UnipotRegion::new(1, 72..=79)],
//                       input_gain_param:       dual_1084::input_gain(),
//                       high_pass_filter:       [UnipotRegion::new(3, 56..=61), UnipotRegion::new(1, 56..=61),
//                                                UnipotRegion::new(3, [62,63,48,49,50,51].into_iter()), UnipotRegion::new(1, [62,63,48,49,50,51].into_iter())],
//                       high_pass_filter_param: dual_1084::high_pass_filter(),
//                       low_freq:               [UnipotRegion::new(3, 16..=21), UnipotRegion::new(1, (16..=21).rev()),
//                                                UnipotRegion::new(3, [22,23,40,41,42,43].into_iter()), UnipotRegion::new(1, [22,23,40,41,42,43].into_iter())],
//                       low_freq_param:         dual_1084::low_freq(),
//                       low_gain:               [UnipotRegion::new(5, 57..=63), UnipotRegion::new(5, [28,29,30,31,23,22,21].into_iter())],
//                       low_gain_param:         dual_1084::low_gain(),
//                       low_mid_freq:           [UnipotRegion::new(3, [4,5,6,7,24,25,26,27,28,29,30,31].into_iter()), UnipotRegion::new(1, [4,5,6,7,24,25,26,27,28,29,30,31].into_iter())],
//                       low_mid_freq_param:     dual_1084::low_mid_freq(),
//                       low_mid_gain:           [UnipotRegion::new(5, (49..=55).rev()), UnipotRegion::new(5, [20,19,18,17,16,8,9].into_iter())],
//                       low_mid_gain_param:     dual_1084::low_mid_gain(),
//                       low_mid_width:          [UnipotRegion::new(3, 54..=54), UnipotRegion::new(1, 54..=54)],
//                       low_mid_width_param:    dual_1084::low_mid_width(),
//                       high_mid_freq:          [UnipotRegion::new(3, [8,9,10,11,12,13,14,15,0,1,2,3].into_iter()), UnipotRegion::new(1, [8,9,10,11,12,13,14,15,0,1,2,3].into_iter())],
//                       high_mid_freq_param:    dual_1084::high_mid_freq(),
//                       high_mid_gain:          [UnipotRegion::new(5, [48,40,41,42,43,44,45].into_iter()), UnipotRegion::new(5, [10,11,12,13,14,15,7].into_iter())],
//                       high_mid_gain_param:    dual_1084::high_mid_gain(),
//                       high_mid_width:         [UnipotRegion::new(3, 53..=53), UnipotRegion::new(1, 53..=53)],
//                       high_mid_width_param:   dual_1084::high_mid_width(),
//                       high_freq:              [UnipotRegion::new(3, [44,45,46,47,32,33].into_iter()), UnipotRegion::new(1, [44,45,46,47,32,33].into_iter()),
//                                                UnipotRegion::new(3, 34..=39), UnipotRegion::new(1, 34..=39)],
//                       high_freq_param:        dual_1084::high_freq(),
//                       high_gain:              [UnipotRegion::new(5, [46,47,39,38,37,36,35].into_iter()), UnipotRegion::new(5, (0..=6).rev())],
//                       high_gain_param:        dual_1084::high_gain(),
//                       output_pad:             [UnipotRegion::new(7, 1..2), UnipotRegion::new(7, 3..4)],
//                       output_pad_param:       dual_1084::output_pad(),
//                       eql_toggle:             [UnipotRegion::new(3, 52..=52), UnipotRegion::new(1, 52..=52)],
//                       eql_toggle_param:       dual_1084::eql_toggle(),

//                     })
//     }
// }

// impl Actor for Dual1084 {
//     type Context = Context<Self>;
// }

// impl Handler<Command> for Dual1084 {
//     type Result = Result<(), InstanceDriverError>;

//     fn handle(&mut self, msg: Command, ctx: &mut Self::Context) -> Self::Result {
//         match msg.command {
//             InstanceDriverCommand::CheckConnection => Ok(()),
//             InstanceDriverCommand::Stop
//             | InstanceDriverCommand::Play { .. }
//             | InstanceDriverCommand::Render { .. }
//             | InstanceDriverCommand::Rewind { .. } => Err(InstanceDriverError::MediaNotPresent),
//             InstanceDriverCommand::SetParameters(mut params) => {
//                 if let Some(input_gain) = params.remove(&dual_1084::INPUT_GAIN) {
//                     for (ch, value) in input_gain.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = repoint_param(value, &self.input_gain_param, ch);
//                             self.input_gain[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(high_pass_filter) = params.remove(&dual_1084::HIGH_PASS_FILTER) {
//                     for (ch, value) in high_pass_filter.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = repoint_param(value, &self.high_pass_filter_param, ch);
//                             self.high_pass_filter[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(low_gain) = params.remove(&dual_1084::LOW_GAIN) {
//                     for (ch, value) in low_gain.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = rescale_param(value, &self.low_gain_param, ch, 128.0);
//                             self.low_gain[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(low_freq) = params.remove(&dual_1084::LOW_FREQ) {
//                     for (ch, value) in low_freq.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = repoint_param(value, &self.low_freq_param, ch);
//                             self.low_freq[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(low_mid_gain) = params.remove(&dual_1084::LOW_MID_GAIN) {
//                     for (ch, value) in low_mid_gain.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = rescale_param(value, &self.low_mid_gain_param, ch, 128.0);
//                             self.low_mid_gain[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(low_mid_freq) = params.remove(&dual_1084::LOW_MID_FREQ) {
//                     for (ch, value) in low_mid_freq.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = repoint_param(value, &self.low_mid_freq_param, ch);
//                             self.low_mid_freq[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(low_mid_width) = params.remove(&dual_1084::LOW_MID_WIDTH) {
//                     for (ch, value) in low_mid_width.into_iter().enumerate() {
//                         if let Some(ModelValue::Bool(value)) = value{
//                             //let rescaled = rescale_param(value, &self.low_mid_width_param, ch, 128.0);
//                             self.low_mid_width[ch].write(&mut self.io_exp_data, value as u16);
//                         }
//                     }
//                 }
//                 if let Some(high_mid_gain) = params.remove(&dual_1084::HIGH_MID_GAIN) {
//                     for (ch, value) in high_mid_gain.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = rescale_param(value, &self.high_mid_gain_param, ch, 128.0);
//                             self.high_mid_gain[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(high_mid_freq) = params.remove(&dual_1084::HIGH_MID_FREQ) {
//                     for (ch, value) in high_mid_freq.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = repoint_param(value, &self.high_mid_freq_param, ch);
//                             self.high_mid_freq[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(high_mid_width) = params.remove(&dual_1084::HIGH_MID_WIDTH) {
//                     for (ch, value) in high_mid_width.into_iter().enumerate() {
//                         if let Some(ModelValue::Bool(value)) = value{
//                             self.high_mid_width[ch].write(&mut self.io_exp_data, value as u16);
//                         }
//                     }
//                 }
//                 if let Some(high_gain) = params.remove(&dual_1084::HIGH_GAIN) {
//                     for (ch, value) in high_gain.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = rescale_param(value, &self.high_gain_param, ch, 128.0);
//                             self.high_gain[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(high_freq) = params.remove(&dual_1084::HIGH_FREQ) {
//                     for (ch, value) in high_freq.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = repoint_param(value, &self.high_freq_param, ch);
//                             self.high_freq[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(output_pad) = params.remove(&dual_1084::OUTPUT_PAD) {
//                     for (ch, value) in output_pad.into_iter().enumerate() {
//                         if let Some(ModelValue::Number(value)) = value{
//                             let rescaled = repoint_param(value, &self.output_pad_param, ch);
//                             self.output_pad[ch].write(&mut self.io_exp_data, rescaled as u16);
//                         }
//                     }
//                 }
//                 if let Some(eql_toggle) = params.remove(&dual_1084::EQL_TOGGLE) {
//                     for (ch, value) in eql_toggle.into_iter().enumerate() {
//                         if let Some(ModelValue::Bool(value)) = value{
//                             self.eql_toggle[ch].write(&mut self.io_exp_data, value as u16);
//                         }
//                     }
//                 }

//                 // TODO: implement
//                 // self.write_io_expanders();

//                 Ok(())
//             }
//         }
//     }
// }
