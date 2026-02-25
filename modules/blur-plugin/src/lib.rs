use libblur::{
  AnisotropicRadius, BlurImageMut, EdgeMode2D, FastBlurChannels,
  ThreadingPolicy, fast_gaussian,
};
use std::ffi::{CString, c_char};
use std::process;

mod params;

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

  let Params { iterations, radius } =
    serde_json::from_str::<Params>(params_string).unwrap_or_default();

  let mut image = BlurImageMut {
    data: libblur::BufferStore::Borrowed(buf),
    width,
    height,
    stride: width,
    channels: FastBlurChannels::Channels3,
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
      println!("Error: {e:?}");

      process::exit(1);
    }
  }
}
