use std::ops::Range;

use audiocloud_api::{ModelParameter, ModelValue, ModelValueOption, ToggleOr};

pub fn rescale(value: f64, options: &[ModelValueOption], scale: f64) -> f64 {
    for (i, value_opt) in options.iter().enumerate() {
        let start_range = i as f64 / options.len() as f64 * scale;
        let end_range = (i + 1) as f64 / options.len() as f64 * scale;

        match value_opt {
            ModelValueOption::Single(ModelValue::Number(single)) if single == &value => {
                return start_range;
            }
            ModelValueOption::Range(ModelValue::Number(left), ModelValue::Number(right)) if left <= &value && &value <= right => {
                return rescale_range(value, *left..*right, start_range..end_range);
            }
            _ => {}
        }
    }

    0_f64
}

pub fn repoint(value: ToggleOr<f64>, options: &[ModelValueOption]) -> usize {
    for (i, option) in options.iter().enumerate() {
        match (&value, option) {
            (ToggleOr::Toggle(value), ModelValueOption::Single(ModelValue::Bool(opt_value))) if value == opt_value => return i,
            (ToggleOr::Value(value), ModelValueOption::Single(ModelValue::Number(opt_value))) if value == opt_value => return i,
            _ => {}
        }
    }

    0
}

pub fn clamp(value: f64, to: Range<f64>) -> f64 {
    value.min(to.end).max(to.start)
}

pub fn write_bit_16(dest: &mut u16, position: u16, val: u16) {
    //  let val = val.round() as u16;
    if val != 0 {
        *dest |= 1 << position;
    } else {
        *dest &= !(1 << position);
    }
}

pub fn swap_u16(val: u16) -> u16 {
    (val << 8) | (val >> 8)
}

pub fn db_to_gain_factor(x: f64) -> f64 {
    10_f64.powf(x / 20_f64)
}

fn rescale_range(value: f64, from: Range<f64>, to: Range<f64>) -> f64 {
    let value_from = value.max(from.start) - from.start;
    let from_len = from.end - from.start;
    let to_len = to.end - to.start;
    (value_from / from_len) * to_len + to.start
}