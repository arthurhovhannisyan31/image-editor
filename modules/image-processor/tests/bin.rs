mod stub;
mod utils;

#[cfg(test)]
mod test_bin {
  use std::env;

  use assert_cmd::Command;
  use image::{EncodableLayout, ImageFormat, ImageReader, RgbaImage};

  use crate::stub::get_mirror_plugin_test_cases;
  use crate::utils::get_temp_dir;

  static TARGET_DIR: &str = "target/release";
  static TARGET_BINARY_NAME: &str = "image-processor";

  #[test]
  fn test_mirror_plugin() {
    for (source_buf, target_buf, json_config) in get_mirror_plugin_test_cases()
    {
      let temp_dir = get_temp_dir();
      let cur_dir = env::current_dir().unwrap();
      let absolute_target_dir = cur_dir
        .join("../../") // have to go to workspace level
        .join(TARGET_DIR)
        .canonicalize()
        .unwrap();
      let absolute_bin_path = absolute_target_dir.join(TARGET_BINARY_NAME);
      let mut cmd = Command::new(absolute_bin_path.to_str().unwrap());

      // Setup image
      let source_image_path = temp_dir.join("test.png");
      let source_image_str = source_image_path.to_str().unwrap();
      let image_buf = RgbaImage::from_raw(3, 2, source_buf).unwrap();
      image_buf
        .save_with_format(source_image_str, ImageFormat::Png)
        .unwrap();

      cmd
        .arg("-i")
        .arg(source_image_str)
        .arg("-o")
        .arg(temp_dir.to_str().unwrap())
        .arg("-p")
        .arg(absolute_target_dir)
        .arg("-P")
        .arg("mirror_plugin")
        .arg("-c")
        .arg(json_config)
        .assert()
        .success();

      let result_img_path = temp_dir.join("test_1.png");
      let result_img = ImageReader::open(result_img_path.to_str().unwrap())
        .unwrap()
        .decode()
        .unwrap();
      let result_img_buf = result_img.to_rgba8();

      assert_eq!(target_buf, result_img_buf.as_bytes());

      temp_dir.close().unwrap();
    }
  }
}
