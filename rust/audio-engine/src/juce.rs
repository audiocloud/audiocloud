use std::ffi::{c_char, c_void};

extern "C" {
  pub fn create_mgr(type_name: *const c_char,
                    input_name: *const c_char,
                    output_name: *const c_char,
                    input_channel_count: i32,
                    output_channel_count: i32,
                    sample_rate: i32,
                    buffer_size: i32)
                    -> *mut c_void;

  pub fn mgr_start(mgr: *mut c_void,
                   callback: extern "C" fn(*mut c_void, *const *const f32, i32, *mut *mut f32, i32, i32),
                   user_data: *mut c_void);

  pub fn mgr_stop(mgr: *mut c_void);

  pub fn mgr_latency(mgr: *mut c_void) -> u32;

  pub fn delete_mgr(mgr: *mut c_void);

  pub fn engine_init();

  pub fn engine_shutdown();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_ffi() {
    unsafe {
      engine_init();
      engine_shutdown();
    }
  }
}
