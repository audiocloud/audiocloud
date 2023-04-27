use std::ffi::{c_char, c_void, CString};
use std::ptr::null_mut;

use anyhow::anyhow;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct JuceAudioReaderPtr(*mut c_void);

unsafe impl Send for JuceAudioReaderPtr {}
unsafe impl Sync for JuceAudioReaderPtr {}

impl JuceAudioReaderPtr {
  pub fn is_valid(&self) -> bool {
    self.0 != null_mut()
  }
}

type AudioMgrCallback = extern "C" fn(*mut c_void, *const *const f32, i32, *mut *mut f32, i32, i32);

extern "C" {
  pub fn create_audio_mgr(type_name: *const c_char,
                          input_name: *const c_char,
                          output_name: *const c_char,
                          input_channel_count: i32,
                          output_channel_count: i32,
                          sample_rate: i32,
                          buffer_size: i32)
                          -> i32;

  pub fn audio_mgr_start(callback: AudioMgrCallback, user_data: *mut c_void);

  pub fn audio_mgr_stop();

  pub fn audio_mgr_latency() -> u32;

  fn create_file_reader(path: *const c_char) -> JuceAudioReaderPtr;

  fn delete_file_reader(reader: JuceAudioReaderPtr);

  fn file_reader_get_total_length(reader: JuceAudioReaderPtr) -> i64;

  fn file_reader_get_sample_rate(reader: JuceAudioReaderPtr) -> i32;

  fn file_reader_get_channels(reader: JuceAudioReaderPtr) -> i32;

  fn file_reader_read_samples(reader: JuceAudioReaderPtr,
                              buffer: *const *mut f32,
                              num_channels: i32,
                              start_pos: i64,
                              num_samples: i32)
                              -> i32;
}

pub struct JuceAudioReader {
  reader_ptr: JuceAudioReaderPtr,
}

impl JuceAudioReader {
  pub fn new(path: &str) -> crate::Result<Self> {
    let c_path = CString::new(path)?;

    let reader_ptr = unsafe { create_file_reader(c_path.as_ptr()) };

    if !reader_ptr.is_valid() {
      return Err(anyhow!("Failed to create file reader"));
    }

    Ok(Self { reader_ptr })
  }

  pub fn get_sample_rate(&self) -> i32 {
    unsafe { file_reader_get_sample_rate(self.reader_ptr) }
  }

  pub fn get_channel_count(&self) -> i32 {
    unsafe { file_reader_get_channels(self.reader_ptr) }
  }

  pub fn get_total_length(&self) -> i64 {
    unsafe { file_reader_get_total_length(self.reader_ptr) }
  }

  pub fn read_samples(&self, buffers: &mut [&mut [f32]], pos: i64, len: i32) -> i32 {
    let mut slice_ptrs = [null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut(),
                          null_mut()];

    for (i, buffer) in buffers.iter_mut().enumerate() {
      slice_ptrs[i] = buffer.as_mut_ptr();
    }

    unsafe { file_reader_read_samples(self.reader_ptr, slice_ptrs.as_ptr(), buffers.len() as i32, pos, len as i32) }
  }
}

impl Drop for JuceAudioReader {
  fn drop(&mut self) {
    unsafe {
      delete_file_reader(self.reader_ptr);
    }
  }
}
