use std::ffi::CString;
use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use common::{error::ImageProcessorError, plugin::Plugin};
use image::{
  DynamicImage, GenericImageView, ImageFormat, ImageReader, RgbaImage,
};

mod configs;
mod logging;
mod utils;

use configs::CliArgs;
use logging::init_logging;
use utils::{get_output_file_name, validate_plugin_arguments};

fn main() -> Result<(), ImageProcessorError> {
  init_logging();

  let CliArgs {
    config,
    input,
    output,
    plugin_dir,
    plugin_name,
  } = CliArgs::parse();

  let output_path = get_output_file_name(&input, &output);

  let image = ImageReader::open(input)?.decode()?;
  let (width, height) = image.dimensions();
  let rgb_image = image.to_rgba8();
  let mut buf = rgb_image.into_raw();

  let plugin = Plugin::new(plugin_dir, &plugin_name)?;
  let process_image = plugin.interface()?.process_image;

  let config_str = CString::from_str(&config.clone()).unwrap_or_default();
  let buf_ptr = buf.as_mut_ptr();
  let config_ptr = config_str.as_ptr();

  validate_plugin_arguments(width, height, buf_ptr, config_ptr)?;

  unsafe {
    // SAFETY
    // Image has valid dimension
    // Buffer pointer is a valid pointer to existing data buffer
    // Config pointer is a valid point to existing JSON string
    let result = process_image(width, height, buf_ptr, config_ptr);
    if result != 0 {
      return Err(ImageProcessorError::PluginError(plugin_name));
    }
  }

  let Some(image_buf) = RgbaImage::from_raw(width, height, buf) else {
    return Err(ImageProcessorError::OtherError(anyhow!("Some error")));
  };

  DynamicImage::ImageRgba8(image_buf)
    .save_with_format(output_path, ImageFormat::Png)?;

  Ok(())
}
