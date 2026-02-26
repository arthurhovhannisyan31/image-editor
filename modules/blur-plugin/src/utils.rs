use std::ffi::c_char;

pub fn validate_pointers(data: *mut u8, config: *mut c_char) -> bool {
  if data.is_null() {
    eprintln!("Received a null `data` pointer, cannot process.");

    return false;
  }
  if config.is_null() {
    eprintln!("Received a null `config` pointer, cannot process.");

    return false;
  }

  true
}
