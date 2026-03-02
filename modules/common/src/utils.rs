use std::ffi::{c_char, c_int};
use std::io;
use std::io::ErrorKind;

use crate::constants::MAX_IMAGE_SIZE_MB;
use crate::error::ImageProcessorError;

// Size is calculated as width * height * bytes len (which is 4 for rgba format)
pub fn get_rgba_buffer_length(
  width: u32,
  height: u32,
) -> Result<usize, ImageProcessorError> {
  (width as usize)
    .checked_mul(4)
    .and_then(|rgba_width| rgba_width.checked_mul(height as usize)).ok_or_else(||{
    ImageProcessorError::Io(io::Error::new(
      ErrorKind::InvalidInput,
      format!("Image size is too big, max supported image size is : {MAX_IMAGE_SIZE_MB} Mb.")
    ))
  })
}

pub fn validate_plugin_arguments(
  width: u32,
  height: u32,
  data: *mut u8,
  config: *const c_char,
) -> c_int {
  if width == 0 || height == 0 {
    eprintln!("Image dimensions should not be zero");

    return 1;
  }
  if data.is_null() {
    eprintln!(
      "Couldn't read image data. The `data` pointer is null, cannot process."
    );

    return 1;
  }
  if config.is_null() {
    eprintln!(
      "Couldn't read plugin config. The `config` pointer is null, cannot process."
    );

    return 1;
  }

  0
}
