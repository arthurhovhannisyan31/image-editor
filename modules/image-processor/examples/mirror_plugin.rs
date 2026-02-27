use image_processor::error::ImageProcessorError;
use std::process::Command;

fn main() -> Result<(), ImageProcessorError> {
  let mut cmd = Command::new("./target/release/image-processor");

  let status = cmd
    .args([
      "-i",
      "./static/img/image.png",
      "-o",
      "./static/img/",
      "-p",
      "./target/release",
      "-P",
      "mirror_plugin",
      "-c",
      "{\"horizontal\":true, \"vertical\":true}",
    ])
    .status()?;

  assert!(status.success());

  Ok(())
}
