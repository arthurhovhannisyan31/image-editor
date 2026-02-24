use std::ffi::c_char;

#[unsafe(no_mangle)]
extern "C" fn process_image(
  width: u32,
  height: u32,
  data: *const u8,
  data_len: usize,
  params: *const c_char, // empty string or
) {
  println!("width: {width}");
  println!("height: {height}");
  println!("data: {data:?}");
  println!("data_len: {data_len:?}");
  println!("params: {params:?}");

  let buf = unsafe { core::slice::from_raw_parts(data, data_len) };
  println!("buf: {buf:?}");
}
