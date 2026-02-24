use std::io;
use std::path::PathBuf;

use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ImageProcessorError {
  #[error("IO Error")]
  Io(#[from] io::Error),
  #[error("Failed locating path: `{source_path:?}`")]
  NotFound {
    err: io::Error,
    source_path: PathBuf,
  },
  #[error(transparent)]
  OtherError(#[from] anyhow::Error),
  #[error("Image error")]
  ImageError(#[from] ImageError),
  #[error("Failed loading library")]
  LibLoadingError(#[from] libloading::Error),
}
