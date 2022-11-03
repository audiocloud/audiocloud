/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::ops::Range;

use audiocloud_api::{ModelValue, ModelValueOption, ToggleOr};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(0.0, 0.0..1.0), 0.0);
        assert_eq!(clamp(1.0, 0.0..1.0), 1.0);

        assert_eq!(clamp(-0.5, 0.0..1.0), 0.0);
        assert_eq!(clamp(1.5, 0.0..1.0), 1.0);
    }

    #[test]
    fn test_swap_u16() {
        assert_eq!(swap_u16(0x1234), 0x3412);
    }

    #[test]
    fn test_db_to_gain_factor() {
        assert_eq!(db_to_gain_factor(0.0), 1.0);
        assert_eq!(db_to_gain_factor(20.0), 10.0);
        assert_eq!(db_to_gain_factor(-20.0), 0.1);
    }

    #[test]
    fn test_repoint() {
        let options = vec![ModelValueOption::Single(ModelValue::Bool(true)),
                           ModelValueOption::Single(ModelValue::Bool(false)),];
        assert_eq!(repoint(ToggleOr::Toggle(true), &options), 0);
        assert_eq!(repoint(ToggleOr::Toggle(false), &options), 1);
    }

    #[test]
    fn test_rescale() {
        let options = vec![ModelValueOption::Range(ModelValue::Number(0.0), ModelValue::Number(1.0)),];
        assert_eq!(rescale(0.0, &options, 1.0), 0.0);
        assert_eq!(rescale(0.5, &options, 1.0), 0.5);
        assert_eq!(rescale(1.0, &options, 4.0), 4.0);
    }
}
