use std::ffi::CString;

use clap::Parser;
use common::plugin::Plugin;
use image::{GenericImageView, ImageReader};
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
  info!(image = ?image);

  let (width, height) = image.dimensions();
  info!(width = ?width, height = ?height);

  let rgb_image = image.to_rgba8();
  let buf = rgb_image.into_raw();
  info!(buf = ?buf);

  let plugin = Plugin::new(plugin_dir, &plugin_name)?;
  let process_image = plugin.interface()?.process_image;
  let buf_len = buf.len();

  process_image(
    width,
    height,
    buf.as_ptr(),
    buf_len,
    CString::new("").unwrap_or_default().as_ptr(),
  );

  Ok(())
}
