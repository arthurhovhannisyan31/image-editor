use std::ffi::{CString, c_char};

mod params;
mod utils;

use crate::utils::{flip_horizontal_in_place, flip_vertical_in_place};
use params::Params;

#[unsafe(no_mangle)]
extern "C" fn process_image(
  width: u32,
  height: u32,
  data: *mut u8,       // size is width * height * 4
  config: *mut c_char, // JSON or empty string
) {
  let data_len = (width * height * 4) as usize;
  let buf = unsafe { core::slice::from_raw_parts_mut(data, data_len) };

  let params_str = unsafe { CString::from_raw(config) };
  let params_string = params_str.to_str().unwrap_or_default();

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
