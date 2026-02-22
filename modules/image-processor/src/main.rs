use crate::configs::CliArgs;
use clap::Parser;

mod configs;
mod error;

use error::ImageProcessorError;

fn main() -> Result<(), ImageProcessorError> {
  // env_logger || tracing

  let CliArgs {
    config,
    input,
    output,
    plugin,
  } = CliArgs::parse();

  Ok(())
}
