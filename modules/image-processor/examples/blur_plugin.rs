use common::error::ImageProcessorError;
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
      "blur_plugin",
      "-c",
      "{\"radius\": 5, \"iterations\": 5}",
    ])
    .status()?;

  assert!(status.success());

  Ok(())
}
