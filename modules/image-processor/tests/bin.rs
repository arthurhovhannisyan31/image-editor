mod stub;
mod utils;

#[cfg(test)]
mod test_bin {
  use std::env;

  use assert_cmd::Command;
  use image::{EncodableLayout, ImageFormat, ImageReader, RgbaImage};

  use crate::stub::{
    PluginTestCase, get_blur_plugin_test_cases, get_mirror_plugin_test_cases,
  };
  use crate::utils::get_temp_dir;

  static TARGET_DIR: &str = "target/debug";
  static TARGET_BINARY_NAME: &str = "image-processor";

  #[test]
  #[ignore] // E2E tests require release build
  fn test_mirror_plugin() {
    for PluginTestCase {
      source_buf,
      target_buf,
      plugin_name,
      config_json,
    } in get_mirror_plugin_test_cases()
    {
      let crate_root = env::current_dir()
        .unwrap()
        .join("../../") // have to go to workspace level
        .canonicalize()
        .unwrap();
      let target_dir = crate_root.join(TARGET_DIR);
      let bin_path = target_dir.join(TARGET_BINARY_NAME);
      let mut cmd = Command::new(bin_path.to_str().unwrap());

      let temp_dir = get_temp_dir();
      let source_image_path = temp_dir.join("test.png");
      let source_image_str = source_image_path.to_str().unwrap();
      let image_buf = RgbaImage::from_raw(3, 2, source_buf).unwrap();
      image_buf
        .save_with_format(source_image_str, ImageFormat::Png)
        .unwrap();

      cmd
        .current_dir(crate_root)
        .args([
          "-i",
          source_image_str,
          "-o",
          temp_dir.to_str().unwrap(),
          "-p",
          target_dir.to_str().unwrap(),
          "-P",
          &plugin_name,
          "-c",
          &config_json,
        ])
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

  #[test]
  #[ignore] // E2E tests require release build
  fn test_blur_plugin() {
    for PluginTestCase {
      source_buf,
      target_buf,
      plugin_name,
      config_json,
    } in get_blur_plugin_test_cases()
    {
      let crate_root = env::current_dir()
        .unwrap()
        .join("../../") // have to go to workspace level
        .canonicalize()
        .unwrap();
      let target_dir = crate_root.join(TARGET_DIR);
      let bin_path = target_dir.join(TARGET_BINARY_NAME);
      let mut cmd = Command::new(bin_path.to_str().unwrap());

      let temp_dir = get_temp_dir();
      let source_image_path = temp_dir.join("test.png");
      let source_image_str = source_image_path.to_str().unwrap();
      let image_buf = RgbaImage::from_raw(3, 2, source_buf).unwrap();
      image_buf
        .save_with_format(source_image_str, ImageFormat::Png)
        .unwrap();

      cmd
        .current_dir(crate_root)
        .args([
          "-i",
          source_image_str,
          "-o",
          temp_dir.to_str().unwrap(),
          "-p",
          target_dir.to_str().unwrap(),
          "-P",
          &plugin_name,
          "-c",
          &config_json,
        ])
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
