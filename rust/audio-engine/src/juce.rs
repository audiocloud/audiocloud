use std::ffi::{c_char, c_void, CString};
use std::ptr::null_mut;

use anyhow::anyhow;
use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct AudioMgr(*mut c_void);

unsafe impl Send for AudioMgr {}
unsafe impl Sync for AudioMgr {}

impl AudioMgr {
  pub fn is_valid(&self) -> bool {
    self.0 != null_mut()
  }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct AudioReader(*mut c_void);

unsafe impl Send for AudioReader {}
unsafe impl Sync for AudioReader {}

impl AudioReader {
  pub fn is_valid(&self) -> bool {
    self.0 != null_mut()
  }
}

extern "C" {
  pub fn create_audio_mgr(type_name: *const c_char,
                          input_name: *const c_char,
                          output_name: *const c_char,
                          input_channel_count: i32,
                          output_channel_count: i32,
                          sample_rate: i32,
                          buffer_size: i32)
                          -> AudioMgr;

  pub fn audio_mgr_start(mgr: AudioMgr,
                         callback: extern "C" fn(*mut c_void, *const *const f32, i32, *mut *mut f32, i32, i32),
                         user_data: *mut c_void);

  pub fn audio_mgr_stop(mgr: AudioMgr);

  pub fn audio_mgr_latency(mgr: AudioMgr) -> u32;

  pub fn delete_audio_mgr(mgr: AudioMgr);

  fn juce_engine_init();

  fn juce_engine_shutdown();

  fn create_file_reader(path: *const c_char) -> AudioReader;

  fn delete_file_reader(reader: AudioReader);

  fn file_reader_get_total_length(reader: AudioReader) -> i64;

  fn file_reader_get_sample_rate(reader: AudioReader) -> i32;

  fn file_reader_get_channels(reader: AudioReader) -> i32;

  fn file_reader_set_read_position(reader: AudioReader, position: i64) -> i64;

  fn file_reader_read_samples(reader: AudioReader, buffer: *const *mut f32, num_channels: i32, num_samples: i32) -> i32;
}

pub struct JuceEngineGuard;

impl JuceEngineGuard {
  pub fn new() -> Self {
    unsafe {
      juce_engine_init();
    }
    Self
  }
}

impl Drop for JuceEngineGuard {
  fn drop(&mut self) {
    unsafe {
      juce_engine_shutdown();
    }
  }
}

lazy_static! {
  pub static ref JUCE_GUARD: JuceEngineGuard = JuceEngineGuard::new();
}

pub fn assure_juce_init() {
  let _ = &JUCE_GUARD;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_ffi() {
    unsafe {
      juce_engine_init();
      juce_engine_shutdown();
    }
  }
}

pub struct JuceAudioReader {
  reader: AudioReader,
}

impl JuceAudioReader {
  pub fn new(path: &str) -> crate::Result<Self> {
    assure_juce_init();

    let c_path = CString::new(path)?;

    let reader = unsafe { create_file_reader(c_path.as_ptr()) };

    if !reader.is_valid() {
      return Err(anyhow!("Failed to create file reader"));
    }

    Ok(Self { reader })
  }

  pub fn get_sample_rate(&self) -> i32 {
    unsafe { file_reader_get_sample_rate(self.reader) }
  }

  pub fn get_channel_count(&self) -> i32 {
    unsafe { file_reader_get_channels(self.reader) }
  }

  pub fn get_total_length(&self) -> i64 {
    unsafe { file_reader_get_total_length(self.reader) }
  }

  pub fn set_read_position(&self, position: i64) -> i64 {
    unsafe { file_reader_set_read_position(self.reader, position) }
  }

  pub fn read_samples(&self, buffers: &mut [&mut [f32]]) -> i32 {
    let mut len = -1;
    let mut count = 0;
    let mut slice_ptrs = [null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut()];

    for (dest, src) in slice_ptrs.iter_mut().zip(buffers.iter_mut()) {
      len = src.len() as i32;
      *dest = src.as_mut_ptr();
      count += 1;
    }

    unsafe { file_reader_read_samples(self.reader, slice_ptrs.as_mut_ptr(), count, len) }
  }
}

impl Drop for JuceAudioReader {
  fn drop(&mut self) {
    unsafe {
      delete_file_reader(self.reader);
    }
  }
}
