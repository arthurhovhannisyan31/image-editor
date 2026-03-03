use common::error::ImageProcessorError;
use std::process::Command;

fn main() -> Result<(), ImageProcessorError> {
  let mut cmd = Command::new("./target/debug/image-processor");

  let status = cmd
    .args([
      "-i",
      "./static/img/image.png",
      "-o",
      "./static/img/",
      "-p",
      "./target/debug",
      "-P",
      "blur_plugin",
      "-c",
      "{\"radius\": 5, \"iterations\": 5}",
    ])
    .status()?;

  assert!(status.success());

  Ok(())
}
