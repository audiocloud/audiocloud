use std::io::Cursor;

use anyhow::bail;
use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};
use tracing::trace;

use api::instance::driver::config::{BinaryPosition, Clamp, Remap, Rescale, ValuePacking};

use super::Result;

pub fn read_packed_value(buffer: &[u8; 8], packing: &ValuePacking) -> f64 {
  let mut cursor = Cursor::new(buffer);

  match packing {
    | ValuePacking::UInt8 => cursor.read_u8().unwrap() as f64,
    | ValuePacking::UInt16LE => cursor.read_u16::<LE>().unwrap() as f64,
    | ValuePacking::UInt16BE => cursor.read_u16::<BE>().unwrap() as f64,
    | ValuePacking::UInt32LE => cursor.read_u32::<LE>().unwrap() as f64,
    | ValuePacking::UInt32BE => cursor.read_u32::<BE>().unwrap() as f64,
    | ValuePacking::Int8 => cursor.read_i8().unwrap() as f64,
    | ValuePacking::Int16LE => cursor.read_i16::<LE>().unwrap() as f64,
    | ValuePacking::Int16BE => cursor.read_i16::<BE>().unwrap() as f64,
    | ValuePacking::Int32LE => cursor.read_i32::<LE>().unwrap() as f64,
    | ValuePacking::Int32BE => cursor.read_i32::<BE>().unwrap() as f64,
    | ValuePacking::Float32LE => cursor.read_f32::<LE>().unwrap() as f64,
    | ValuePacking::Float32BE => cursor.read_f32::<BE>().unwrap() as f64,
    | ValuePacking::Float64LE => cursor.read_f64::<LE>().unwrap() as f64,
    | ValuePacking::Float64BE => cursor.read_f64::<BE>().unwrap() as f64,
  }
}

pub fn write_packed_value(value: f64, packing: &ValuePacking) -> [u8; 8] {
  let mut buffer = [0u8; 8];
  let mut cursor = Cursor::new(&mut buffer[..]);

  match packing {
    | ValuePacking::UInt8 => cursor.write_u8(value as u8).unwrap(),
    | ValuePacking::UInt16LE => cursor.write_u16::<LE>(value as u16).unwrap(),
    | ValuePacking::UInt16BE => cursor.write_u16::<BE>(value as u16).unwrap(),
    | ValuePacking::UInt32LE => cursor.write_u32::<LE>(value as u32).unwrap(),
    | ValuePacking::UInt32BE => cursor.write_u32::<BE>(value as u32).unwrap(),
    | ValuePacking::Int8 => cursor.write_i8(value as i8).unwrap(),
    | ValuePacking::Int16LE => cursor.write_i16::<LE>(value as i16).unwrap(),
    | ValuePacking::Int16BE => cursor.write_i16::<BE>(value as i16).unwrap(),
    | ValuePacking::Int32LE => cursor.write_i32::<LE>(value as i32).unwrap(),
    | ValuePacking::Int32BE => cursor.write_i32::<BE>(value as i32).unwrap(),
    | ValuePacking::Float32LE => cursor.write_f32::<LE>(value as f32).unwrap(),
    | ValuePacking::Float32BE => cursor.write_f32::<BE>(value as f32).unwrap(),
    | ValuePacking::Float64LE => cursor.write_f64::<LE>(value as f64).unwrap(),
    | ValuePacking::Float64BE => cursor.write_f64::<BE>(value as f64).unwrap(),
  }

  buffer
}

pub fn write_binary_within_page(page: &mut [u8], value: [u8; 8], position: &BinaryPosition) {
  match position {
    | BinaryPosition::Byte(byte) => {
      trace!(byte, value = value[0], "writing byte");
      page[*byte as usize] = value[0];
    }
    | BinaryPosition::Bytes(from, to) => {
      let from = *from as usize;
      let to = *to as usize;

      for (i, byte) in page[from..=to].iter_mut().take(8).enumerate() {
        trace!(byte = from + i, value = value[i], "writing byte");
        *byte = value[i];
      }
    }
    | BinaryPosition::Bit(byte, bit) => {
      let mask = 1 << (*bit % 8);
      let data = &mut page[*byte as usize];
      let before = *data;

      *data = match value[0] {
        | 0 => *data & !mask,
        | _ => *data | mask,
      };

      trace!(mask = mask, before, now = *data, "writing bit");
    }
    | BinaryPosition::BitRange(ranges) => {
      let mut bit_pos = 0;
      for (start, end) in ranges {
        for bit in *start..*end {
          let mask = 1 << (bit % 8);
          let dest_byte = &mut page[(bit / 8) as usize];
          let before = *dest_byte;
          *dest_byte = match value[bit_pos / 8] & (1 << (bit_pos % 8)) {
            | 0 => *dest_byte & !mask,
            | _ => *dest_byte | mask,
          };

          trace!(mask = mask, before, now = *dest_byte, "writing bit");

          bit_pos += 1;
        }
      }
    }
  }
}

pub fn read_binary_within_page(page: &[u8], position: &BinaryPosition) -> [u8; 8] {
  let mut buffer = [0u8; 8];
  match position {
    | BinaryPosition::Byte(byte) => {
      buffer[0] = page[*byte as usize];
    }
    | BinaryPosition::Bytes(from, to) => {
      let from = *from as usize;
      let to = *to as usize;

      for (i, byte) in page[from..to].iter().take(8).enumerate() {
        buffer[i] = *byte;
      }
    }
    | BinaryPosition::Bit(byte, bit) => {
      let byte = *byte as usize;
      buffer[0] = match page[byte] & (1 << (*bit % 8)) {
        | 0 => 0,
        | _ => 1,
      };
    }
    | BinaryPosition::BitRange(ranges) => {
      let mut bit_pos = 0;
      for (start, end) in ranges {
        for bit in *start..*end {
          buffer[bit_pos / 8] |= match page[(bit / 8) as usize] & (1 << (bit % 8)) {
            | 0 => 0,
            | _ => 1 << (bit_pos % 8),
          };

          bit_pos += 1;
        }
      }
    }
  }

  buffer
}

pub fn remap_and_rescale_value(mut value: f64, remap: Option<&Remap>, rescale: Option<&Rescale>, clamp: Option<&Clamp>) -> Result<f64> {
  match remap {
    | None => {}
    | Some(remap) => match remap {
      | Remap::Linear { values } => {
        let mut found = false;
        for (i, eq_value) in values.iter().enumerate() {
          if value == *eq_value {
            value = i as f64;
            found = true;
            break;
          }
        }

        if !found {
          bail!("Value {value} not found in linear remap values");
        }
      }
      | Remap::Pairs { pairs } => {
        let mut found = false;
        for (remap_from, remap_to) in pairs.iter() {
          if value == *remap_from {
            value = *remap_to;
            found = true;
            break;
          }
        }

        if !found {
          bail!("Value {value} not found in remap pairs");
        }
      }
    },
  }

  match rescale {
    | None => {}
    | Some(Rescale { from, to }) => {
      value = value.min(from.1).max(from.0);
      value = (value - from.0) / (from.1 - from.0) * (to.1 - to.0) + to.0;
    }
  }

  match clamp {
    | None => {}
    | Some(Clamp { min, max }) => {
      value = value.min(*max).max(*min);
    }
  }

  Ok(value)
}

#[cfg(test)]
mod test {
  use api::instance::driver::config::{BinaryPosition, Clamp, Remap, Rescale};

  use super::*;

  #[test]
  fn test_rescale_0_1_0_2() {
    let rescale = Rescale { from: (0.0, 1.0),
                            to:   (0.0, 2.0), };

    let result = remap_and_rescale_value(0.5, None, Some(&rescale), None).unwrap();

    assert_eq!(result, 1.0);
  }

  #[test]
  fn test_rescale_m10_10_0_200() {
    let rescale = Rescale { from: (-10.0, 10.0),
                            to:   (0.0, 200.0), };

    let result = remap_and_rescale_value(0.0, None, Some(&rescale), None).unwrap();

    assert_eq!(result, 100.0);

    let result = remap_and_rescale_value(10.0, None, Some(&rescale), None).unwrap();

    assert_eq!(result, 200.0);

    let result = remap_and_rescale_value(-10.0, None, Some(&rescale), None).unwrap();

    assert_eq!(result, 0.0);
  }

  #[test]
  fn test_clamp_0_1() {
    let clamp = Clamp { min: 0.0, max: 1.0 };

    let result = remap_and_rescale_value(0.5, None, None, Some(&clamp)).unwrap();
    assert_eq!(result, 0.5);

    let result = remap_and_rescale_value(-0.5, None, None, Some(&clamp)).unwrap();
    assert_eq!(result, 0.0);

    let result = remap_and_rescale_value(1.5, None, None, Some(&clamp)).unwrap();
    assert_eq!(result, 1.0);
  }

  #[test]
  fn test_remap_linear() {
    let remap = Remap::Linear { values: vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0], };

    let result = remap_and_rescale_value(0.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(0.0));

    let result = remap_and_rescale_value(1.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(1.0));

    let result = remap_and_rescale_value(2.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(2.0));

    let result = remap_and_rescale_value(3.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(3.0));

    let result = remap_and_rescale_value(4.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(4.0));

    let result = remap_and_rescale_value(5.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(5.0));

    let result = remap_and_rescale_value(0.5, Some(&remap), None, None).ok();
    assert_eq!(result, None);

    let result = remap_and_rescale_value(-4.5, Some(&remap), None, None).ok();
    assert_eq!(result, None);
  }

  #[test]
  fn test_remap_pairs() {
    let remap = Remap::Pairs { pairs: vec![(0.0, 0.5), (1.0, 1.5), (2.0, 2.5), (3.0, 3.5), (4.0, 4.5), (5.0, 5.5)], };

    let result = remap_and_rescale_value(0.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(0.5));

    let result = remap_and_rescale_value(1.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(1.5));

    let result = remap_and_rescale_value(2.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(2.5));

    let result = remap_and_rescale_value(3.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(3.5));

    let result = remap_and_rescale_value(4.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(4.5));

    let result = remap_and_rescale_value(5.0, Some(&remap), None, None).ok();
    assert_eq!(result, Some(5.5));

    let result = remap_and_rescale_value(0.5, Some(&remap), None, None).ok();
    assert_eq!(result, None);

    let result = remap_and_rescale_value(-4.5, Some(&remap), None, None).ok();
    assert_eq!(result, None);
  }

  #[test]
  fn test_packing_u8() {
    let value = write_packed_value(1.0, &ValuePacking::UInt8);
    assert_eq!(value, [1, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(-1.0, &ValuePacking::UInt8);
    assert_eq!(value, [0, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(0.9, &ValuePacking::UInt8);
    assert_eq!(value, [0, 0, 0, 0, 0, 0, 0, 0]);
  }

  #[test]
  fn test_packing_i8() {
    let value = write_packed_value(1.0, &ValuePacking::Int8);
    assert_eq!(value, [1, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(-1.0, &ValuePacking::Int8);
    assert_eq!(value, [0xff, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(-2.0, &ValuePacking::Int8);
    assert_eq!(value, [0xfe, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(0.9, &ValuePacking::Int8);
    assert_eq!(value, [0, 0, 0, 0, 0, 0, 0, 0]);
  }

  #[test]
  fn test_packing_u16_le() {
    let value = write_packed_value(1.0, &ValuePacking::UInt16LE);
    assert_eq!(value, [1, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(-1.0, &ValuePacking::UInt16LE);
    assert_eq!(value, [0, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(0.9, &ValuePacking::UInt16LE);
    assert_eq!(value, [0, 0, 0, 0, 0, 0, 0, 0]);
  }

  #[test]
  fn test_packing_u16_be() {
    let value = write_packed_value(1.0, &ValuePacking::UInt16BE);
    assert_eq!(value, [0, 1, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(-1.0, &ValuePacking::UInt16BE);
    assert_eq!(value, [0, 0, 0, 0, 0, 0, 0, 0]);

    let value = write_packed_value(0.9, &ValuePacking::UInt16BE);
    assert_eq!(value, [0, 0, 0, 0, 0, 0, 0, 0]);
  }

  #[test]
  fn test_position_bit() {
    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Bit(0, 0));
    assert_eq!(page, [0b1, 0b1, 0b10, 0b100]);

    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Bit(1, 0));
    assert_eq!(page, [0, 0b1, 0b10, 0b100]);

    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Bit(2, 0));
    assert_eq!(page, [0, 0b1, 0b11, 0b100]);

    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Bit(1, 1));
    assert_eq!(page, [0, 0b11, 0b10, 0b100]);
  }

  #[test]
  fn test_position_byte() {
    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Byte(0));
    assert_eq!(page, [1, 0b1, 0b10, 0b100]);

    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Byte(1));
    assert_eq!(page, [0, 1, 0b10, 0b100]);

    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Byte(2));
    assert_eq!(page, [0, 0b1, 1, 0b100]);

    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page, write_packed_value(1.0, &ValuePacking::UInt8), &BinaryPosition::Byte(3));
    assert_eq!(page, [0, 0b1, 0b10, 1]);
  }

  #[test]
  fn test_position_bytes() {
    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page,
                             write_packed_value(1.0, &ValuePacking::UInt16LE),
                             &BinaryPosition::Bytes(0, 1));
    assert_eq!(page, [1, 0, 0b10, 0b100]);

    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page,
                             write_packed_value(1.0, &ValuePacking::UInt16BE),
                             &BinaryPosition::Bytes(0, 1));
    assert_eq!(page, [0, 1, 0b10, 0b100]);

    // putting bytes in the wrong order should not work
    let mut page = [0, 0b1, 0b10, 0b100];
    write_binary_within_page(&mut page,
                             write_packed_value(1.0, &ValuePacking::UInt16BE),
                             &BinaryPosition::Bytes(1, 0));
    assert_eq!(page, [0, 0b1, 0b10, 0b100]);
  }
}
