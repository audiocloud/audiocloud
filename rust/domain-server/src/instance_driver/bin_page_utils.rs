use std::io::Cursor;

use anyhow::bail;
use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};

use api::driver::{BinaryPosition, Clamp, Remap, Rescale, ValuePacking};

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
      page[*byte as usize] = value[0];
    }
    | BinaryPosition::Bytes(from, to) => {
      let from = *from as usize;
      let to = *to as usize;

      for (i, byte) in page[from..to].iter_mut().take(8).enumerate() {
        *byte = value[i];
      }
    }
    | BinaryPosition::Bit(byte, bit) => {
      page[*byte as usize] |= match value[0] {
        | 0 => 0,
        | _ => 1 << (*bit % 8),
      };
    }
    | BinaryPosition::BitRange(ranges) => {
      let mut bit_pos = 0;
      for (start, end) in ranges {
        for bit in *start..*end {
          page[(bit / 8) as usize] |= match value[bit_pos / 8] & (1 << (bit_pos % 8)) {
            | 0 => 0,
            | _ => 1 << (bit % 8),
          };

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
