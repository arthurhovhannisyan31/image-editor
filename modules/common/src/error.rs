use image::ImageError;
use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageProcessorError {
  #[error("IO Error")]
  Io(#[from] io::Error),
  #[error("Failed locating path: `{source_path:?}`")]
  NotFound {
    err: io::Error,
    source_path: PathBuf,
  },
  #[error("Image error")]
  ImageError(#[from] ImageError),
  #[error("Failed loading library")]
  LibLoadingError(#[from] libloading::Error),
  #[error("Failed calling plugin: {0}")]
  PluginError(String),
  #[error(transparent)]
  OtherError(#[from] anyhow::Error),
}
