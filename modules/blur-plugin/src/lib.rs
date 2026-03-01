use std::ffi::{CStr, c_char, c_int};

use common::utils::get_rgba_buffer_length;
use libblur::{
  AnisotropicRadius, BlurImageMut, EdgeMode2D, FastBlurChannels,
  ThreadingPolicy, fast_gaussian,
};

mod params;
use params::Params;

/// Mutates data buffer by applying image filter
/// # Safety
/// * `data` must be a valid pointer to existing data buffer
/// * `config` must be a valid point to existing JSON string
#[unsafe(no_mangle)]
unsafe extern "C" fn process_image(
  width: u32,
  height: u32,
  data: *mut u8,
  config: *mut c_char,
) -> c_int {
  let data_len: usize = match get_rgba_buffer_length(width, height) {
    Ok(val) => val,
    Err(e) => {
      eprintln!("{e:?}");
      return 1;
    }
  };

  let buf = unsafe { core::slice::from_raw_parts_mut(data, data_len) };
  let params_str = unsafe { CStr::from_ptr(config) };
  let params_string = params_str.to_str().unwrap_or_default();

  // In case of wrong JSON content default values will be used instead
  let Params { iterations, radius } =
    serde_json::from_str::<Params>(params_string).unwrap_or_default();

  let mut image = BlurImageMut {
    data: libblur::BufferStore::Borrowed(buf),
    width,
    height,
    stride: width * 4,
    channels: FastBlurChannels::Channels4,
  };

  let radius = AnisotropicRadius {
    x_axis: radius,
    y_axis: radius,
  };

  for _ in 0..iterations {
    if let Err(e) = fast_gaussian(
      &mut image,
      radius,
      ThreadingPolicy::Adaptive,
      EdgeMode2D::default(),
    ) {
      eprintln!("Error: {e:?}");

      return 1;
    }
  }

  0
}
