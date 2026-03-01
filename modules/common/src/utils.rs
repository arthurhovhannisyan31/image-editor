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
