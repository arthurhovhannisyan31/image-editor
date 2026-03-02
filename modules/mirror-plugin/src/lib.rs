use std::ffi::{CStr, c_char, c_int};

use common::utils::{get_rgba_buffer_length, validate_plugin_arguments};

mod params;
mod utils;

use params::Params;
use utils::{flip_horizontal_in_place, flip_vertical_in_place};

/// Mutates data buffer by applying image filter
/// # Safety
/// * `width` and `height` should not be zero
/// * `data` must be a valid pointer to existing data buffer
/// * `config` must be a valid point to existing JSON string
#[unsafe(no_mangle)]
unsafe extern "C" fn process_image(
  width: u32,
  height: u32,
  data: *mut u8,
  config: *const c_char,
) -> c_int {
  validate_plugin_arguments(width, height, data, config);

  let data_len: usize = match get_rgba_buffer_length(width, height) {
    Ok(val) => val,
    Err(e) => {
      eprintln!("{e:?}");
      return 2;
    }
  };

  // SAFETY
  // Data is not a null pointer
  // Data length is not zero and fits into `usize` limits
  let buf = unsafe { core::slice::from_raw_parts_mut(data, data_len) };
  // SAFETY
  // Config is not a null pointer
  let params_str = unsafe { CStr::from_ptr(config) };
  let params_string = params_str.to_str().unwrap_or_default();

  // In case of wrong JSON content default values will be used instead
  let Params {
    horizontal,
    vertical,
  } = serde_json::from_str::<Params>(params_string).unwrap_or_default();

  if horizontal {
    flip_horizontal_in_place(width, height, buf);
  }

  if vertical {
    flip_vertical_in_place(width, height, buf);
  }

  0
}
