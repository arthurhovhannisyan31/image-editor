use std::ffi::c_char;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use common::error::ImageProcessorError;

pub fn get_output_file_name(input_file: &Path, output_path: &Path) -> PathBuf {
  let input_filename = input_file
    .file_stem()
    .unwrap_or("result".as_ref())
    .to_str()
    .unwrap_or_default();
  let input_ext = input_file
    .extension()
    .unwrap_or(".png".as_ref())
    .to_str()
    .unwrap_or_default();
  output_path.join(format!("{input_filename}_1.{input_ext}"))
}

pub fn validate_plugin_arguments(
  width: u32,
  height: u32,
  data: *mut u8,
  config: *const c_char,
) -> Result<(), ImageProcessorError> {
  if width == 0 || height == 0 {
    return Err(ImageProcessorError::Io(io::Error::new(
      ErrorKind::InvalidData,
      "Image dimensions should not be zero".to_string(),
    )));
  }
  if data.is_null() {
    return Err(ImageProcessorError::Io(io::Error::new(
      ErrorKind::InvalidData,
      "The `data` pointer is null, cannot process.".to_string(),
    )));
  }
  if config.is_null() {
    return Err(ImageProcessorError::Io(io::Error::new(
      ErrorKind::InvalidData,
      "The `config` pointer is null, cannot process.".to_string(),
    )));
  }

  Ok(())
}
