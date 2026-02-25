use std::ffi::CString;
use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use common::plugin::Plugin;
use image::{
  DynamicImage, GenericImageView, ImageFormat, ImageReader, RgbaImage,
};

mod configs;
mod error;
mod logging;

use configs::CliArgs;
use error::ImageProcessorError;
use logging::init_logging;

fn main() -> Result<(), ImageProcessorError> {
  init_logging();

  let CliArgs {
    config,
    input,
    output,
    plugin_dir,
    plugin_name,
  } = CliArgs::parse();

  let input_filename = input.file_name().unwrap_or("result".as_ref());
  let output_path = output.join(input_filename);

  let image = ImageReader::open(input)?.decode()?;
  let (width, height) = image.dimensions();
  let rgb_image = image.to_rgba8();
  let mut buf = rgb_image.into_raw();

  let plugin = Plugin::new(plugin_dir, &plugin_name)?;
  let process_image = plugin.interface()?.process_image;

  let config_str = CString::from_str(&config.clone()).unwrap_or_default();

  process_image(
    width,
    height,
    buf.as_mut_ptr(),
    config_str.into_raw(), // Memory freed on plugin side
  );

  let Some(image_buf) = RgbaImage::from_raw(width, height, buf) else {
    return Err(ImageProcessorError::OtherError(anyhow!("Some error")));
  };

  DynamicImage::ImageRgba8(image_buf)
    .save_with_format(output_path, ImageFormat::Png)?;

  Ok(())
}
