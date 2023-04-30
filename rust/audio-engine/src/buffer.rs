use std::ops::Add;
use std::ptr::{null, null_mut};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub fn fill_slice<S>(slice: &mut [S], value: impl Iterator<Item = S>) {
  slice.iter_mut().zip(value).for_each(|(a, b)| *a = b);
}

pub fn add_slice<S>(slice: &mut [S], value: impl Iterator<Item = S>)
  where S: Add<S>,
        S: Copy,
        <S as Add<S>>::Output: Into<S>
{
  slice.iter_mut().zip(value).for_each(|(a, b)| *a = (*a + b).into());
}

pub fn zero_slice(slice: &mut [f64]) {
  slice.iter_mut().for_each(|a| *a = 0.0);
}

#[derive(Copy, Clone, Debug)]
pub struct DeviceBuffers {
  pub inputs:      *const *const f32,
  pub outputs:     *mut *mut f32,
  pub num_inputs:  usize,
  pub num_outputs: usize,
  pub buffer_size: usize,
}

impl Default for DeviceBuffers {
  fn default() -> Self {
    Self { inputs:      null(),
           outputs:     null_mut(),
           num_inputs:  0,
           num_outputs: 0,
           buffer_size: 0, }
  }
}

unsafe impl Send for DeviceBuffers {}

unsafe impl Sync for DeviceBuffers {}

impl DeviceBuffers {
  pub fn get_input_plane(&self, plane: usize) -> &[f32] {
    assert!(plane < self.num_inputs);
    unsafe { from_raw_parts(*self.inputs.add(plane), self.buffer_size) }
  }

  pub fn get_output_plane(&self, plane: usize) -> &mut [f32] {
    assert!(plane < self.num_outputs);
    unsafe { from_raw_parts_mut(*self.outputs.add(plane), self.buffer_size) }
  }
}
