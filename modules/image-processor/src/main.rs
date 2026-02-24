use std::ffi::CString;
use std::str::FromStr;

use clap::Parser;
use common::plugin::Plugin;
use image::{GenericImageView, ImageReader, Rgba, RgbaImage};
use tracing::info;

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

  let image = ImageReader::open(input)?.decode()?;

  // let mut image = RgbaImage::new(2, 2);
  // image.put_pixel(0, 0, Rgba([001, 001, 001, 001]));
  // image.put_pixel(1, 0, Rgba([002, 002, 002, 002]));
  // image.put_pixel(0, 1, Rgba([003, 003, 003, 003]));
  // image.put_pixel(1, 1, Rgba([004, 004, 004, 004]));
  // let image = DynamicImage::from(image);

  let (width, height) = image.dimensions();

  let rgb_image = image.to_rgba8();
  let mut buf = rgb_image.into_raw();

  let plugin = Plugin::new(plugin_dir, &plugin_name)?;
  let process_image = plugin.interface()?.process_image;

  let config_str = CString::from_str(&config.clone()).unwrap_or_default();

  info!("buf: {buf:?}");

  process_image(
    width,
    height,
    buf.as_mut_ptr(),
    config_str.into_raw(), // Delegate memory drop to plugin
  );

  info!("buf: {buf:?}");

  Ok(())
}
