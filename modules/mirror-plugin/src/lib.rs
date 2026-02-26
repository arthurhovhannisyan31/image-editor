use std::ffi::{c_char, CString};

mod params;

mod utils;

use params::Params;

use utils::{
  flip_horizontal_in_place, flip_vertical_in_place, validate_pointers,
};

#[unsafe(no_mangle)]
extern "C" fn process_image(
  width: u32,
  height: u32,
  data: *mut u8,       // size is width * height * 4
  config: *mut c_char, // JSON or empty string
) {
  debug_assert!(validate_pointers(data, config));

  let data_len = (width * height * 4) as usize;
  // SAFETY: Pointer is valid, data length is calculated as image width * height * rgba segment width
  // Buffer deallocates memory when exits scope
  let buf = unsafe { core::slice::from_raw_parts_mut(data, data_len) };

  // SAFETY: Pointer is valid, the caller validates string as JSON
  let params_str = unsafe { CString::from_raw(config) };
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
}
