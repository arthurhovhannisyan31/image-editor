use std::ffi::{CString, c_char};
use std::process;

use libblur::{
  AnisotropicRadius, BlurImageMut, EdgeMode2D, FastBlurChannels,
  ThreadingPolicy, fast_gaussian,
};

mod params;
mod utils;

use params::Params;
use utils::validate_pointers;

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

      process::exit(1);
    }
  }
}
