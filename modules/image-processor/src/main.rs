use crate::configs::CliArgs;
use clap::Parser;

mod configs;
mod error;
mod logging;

use error::ImageProcessorError;
use logging::init_logging;

fn main() -> Result<(), ImageProcessorError> {
  init_logging();

  let CliArgs {
    config,
    input,
    output,
    plugin,
  } = CliArgs::parse();

  Ok(())
}
