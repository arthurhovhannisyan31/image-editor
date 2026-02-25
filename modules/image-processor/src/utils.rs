use std::path::{Path, PathBuf};

pub fn get_output_file_name(input_file: &Path, output_path: &Path) -> PathBuf {
  let input_filename = input_file
    .file_stem()
    .unwrap_or("result".as_ref())
    .to_str()
    .unwrap_or_default();
  let input_ext = input_file
    .extension()
    .unwrap_or(".png".as_ref())
    .to_str()
    .unwrap_or_default();
  output_path.join(format!("{input_filename}_1.{input_ext}"))
}
