use std::ffi::c_char;
use std::path::PathBuf;

use libloading::{Library, Symbol, library_filename};

pub struct PluginInterface<'a> {
  pub process_image: Symbol<
    'a,
    extern "C" fn(
      width: u32,
      height: u32,
      data: *const u8,
      params: *const c_char, // empty string or
    ),
  >,
}

pub struct Plugin(Library);

impl Plugin {
  pub fn new(path: PathBuf, name: &str) -> Result<Self, libloading::Error> {
    let lib_name = library_filename(name);
    let path = path.join(lib_name);

    Ok(Plugin(unsafe { Library::new(path) }?))
  }
  pub fn interface(&self) -> Result<PluginInterface<'_>, libloading::Error> {
    Ok(PluginInterface {
      // load process_image symbol from shared object / library file
      process_image: unsafe { self.0.get("process_image") }?,
    })
  }
}
