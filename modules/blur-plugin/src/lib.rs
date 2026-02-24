use std::ffi::c_char;

pub fn process_image(
  width: u32,
  height: u32,
  data: *const u8,       // size is width * height * 4
  params: *const c_char, // empty string or
) {
  //
}
