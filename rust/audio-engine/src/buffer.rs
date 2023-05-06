use std::collections::HashMap;
use std::ops::Add;
use std::ptr::{null, null_mut};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::sync::Arc;

use anyhow::anyhow;
use dasp::sample::FromSample;

use super::Result;

pub fn fill_slice<S>(slice: &mut [S], value: impl Iterator<Item = S>) {
  slice.iter_mut().zip(value).for_each(|(a, b)| *a = b);
}

pub fn cast_sample_ref<S, D>() -> impl Fn(&S) -> D
  where D: FromSample<S>,
        S: Copy
{
  |a| D::from_sample_(*a)
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
  pub generation:  u64,
}

impl Default for DeviceBuffers {
  fn default() -> Self {
    Self { inputs:      null(),
           outputs:     null_mut(),
           num_inputs:  0,
           num_outputs: 0,
           buffer_size: 0,
           generation:  0, }
  }
}

unsafe impl Send for DeviceBuffers {}

unsafe impl Sync for DeviceBuffers {}

#[derive(Default, Clone, Debug)]
pub struct DevicesBuffers(pub(crate) HashMap<String, DeviceBuffers>);

impl DevicesBuffers {
  pub fn device(&self, name: &str) -> Result<DeviceBuffers> {
    let rv = self.0.get(name).ok_or_else(|| anyhow!("No device named {name}"))?;
    Ok(*rv)
  }
}

impl DeviceBuffers {
  pub fn input_plane(&self, plane: usize) -> &[f32] {
    assert!(plane < self.num_inputs);
    unsafe { from_raw_parts(*self.inputs.add(plane), self.buffer_size) }
  }

  pub fn output_plane(&self, plane: usize) -> &mut [f32] {
    assert!(plane < self.num_outputs);
    unsafe { from_raw_parts_mut(*self.outputs.add(plane), self.buffer_size) }
  }
}

#[derive(Clone, Debug)]
pub struct NodeBuffers {
  inputs:          Arc<Vec<Vec<f64>>>,
  outputs:         Arc<Vec<Vec<f64>>>,
  pub num_inputs:  usize,
  pub num_outputs: usize,
  pub buffer_size: usize,
}

impl NodeBuffers {
  pub fn new(inputs: Vec<Vec<f64>>, outputs: Vec<Vec<f64>>, buffer_size: usize) -> Self {
    Self { num_inputs: inputs.len(),
           num_outputs: outputs.len(),
           inputs: Arc::new(inputs),
           outputs: Arc::new(outputs),
           buffer_size }
  }

  pub fn input_plane(&self, plane: usize) -> &mut [f64] {
    assert!(plane < self.num_inputs);
    unsafe { from_raw_parts_mut(self.inputs[plane].as_ptr() as *mut _, self.buffer_size) }
  }

  pub fn output_plane(&self, plane: usize) -> &mut [f64] {
    assert!(plane < self.num_outputs);
    unsafe { from_raw_parts_mut(self.outputs[plane].as_ptr() as *mut _, self.buffer_size) }
  }

  pub fn inputs(&self) -> impl Iterator<Item = &mut [f64]> + '_ {
    (0..self.num_inputs).map(|i| self.input_plane(i))
  }

  pub fn outputs(&self) -> impl Iterator<Item = &mut [f64]> + '_ {
    (0..self.num_outputs).map(|i| self.output_plane(i))
  }
}

unsafe impl Send for NodeBuffers {}

unsafe impl Sync for NodeBuffers {}
