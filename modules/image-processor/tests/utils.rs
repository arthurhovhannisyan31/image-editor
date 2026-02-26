use assert_fs::TempDir;

#[cfg(unix)]
pub fn get_temp_dir() -> TempDir {
  TempDir::new().unwrap()
}
#[cfg(windows)]
pub fn get_temp_dir() -> TempDir {
  TempDir::new_in(".").unwrap()
}
