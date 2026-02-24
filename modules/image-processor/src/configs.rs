use std::ffi::OsStr;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, builder::NonEmptyStringValueParser};

use crate::error::ImageProcessorError;

pub(crate) const EXTENSION_WHITELIST: &[&str] = &["png"];
// ImageFormat from image

#[derive(Debug, Parser)]
#[command(version, about, next_line_help = true)]
pub struct CliArgs {
  #[arg(long, short = 'i', value_name = "Input file", value_parser = image_path_validation)]
  pub input: PathBuf,
  #[arg(long, short = 'o', value_name = "Output path", value_parser = path_validation)]
  pub output: PathBuf,
  #[arg(long, short = 'd', value_name = "Plugin directory", value_parser = path_validation)]
  pub plugin_dir: PathBuf,
  #[arg(long, short = 'p', value_name = "Plugin name without platform extension", value_parser = NonEmptyStringValueParser::new())]
  pub plugin_name: String,
  #[arg(long, short = 'c', value_name = "Config file", value_parser = path_validation)]
  // TODO Maybe add config extension whitelist
  pub config: PathBuf,
}

fn path_validation(path: &str) -> Result<PathBuf, ImageProcessorError> {
  let path = PathBuf::from_str(path).map_err(|_| {
    ImageProcessorError::Io(io::Error::new(
      ErrorKind::NotFound,
      format!("Failed reading provided file path: {path}"),
    ))
  })?;

  if !path.exists() {
    return Err(ImageProcessorError::NotFound {
      err: io::Error::new(ErrorKind::NotFound, "File path does not exist"),
      source_path: path,
    });
  }

  Ok(path)
}

fn image_path_validation(path: &str) -> Result<PathBuf, ImageProcessorError> {
  let path = path_validation(path)?;

  if let Some(extension) = path.extension().and_then(OsStr::to_str)
    && EXTENSION_WHITELIST.contains(&extension)
  {
    return Ok(path);
  }

  Err(ImageProcessorError::Io(io::Error::new(
    ErrorKind::InvalidFilename,
    "Failed reading file extension",
  )))
}
