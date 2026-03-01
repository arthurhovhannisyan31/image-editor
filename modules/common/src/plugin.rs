use std::ffi::{c_char, c_int};
use std::path::PathBuf;

use libloading::{Library, Symbol, library_filename};

/// Plugin interface access
///
/// # Example
///
/// ```
/// use std::ffi::CString;
/// use std::str::FromStr;
/// use common::plugin::Plugin;
///
/// fn process(plugin: Plugin) -> Result<(), Box<dyn std::error::Error>> {
///   let process_image = plugin.interface()?.process_image;
///
///   let width = 0;
///   let height = 0;
///   let mut buf = vec![];
///   let config = String::from("{\"radius\":5, \"iterations\": 3}");
///   let config_str = CString::from_str(&config.clone()).unwrap_or_default();
///
///   process_image(
///     width,
///     height,
///     buf.as_mut_ptr(),
///     config_str.into_raw()
///   );
///
///   Ok(())
/// }
/// ```
pub struct PluginInterface<'a> {
  pub process_image: Symbol<
    'a,
    extern "C" fn(
      width: u32,
      height: u32,
      data: *mut u8,         // size is width * height * 4
      config: *const c_char, // JSON or empty string
    ) -> c_int,
  >,
}

/// Plugin instantiation and usage
///
/// # Example
///
/// ```rust,no_run
/// use std::path::PathBuf;
/// use common::plugin::Plugin;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let plugin_dir = PathBuf::from("target/release");
///   let plugin_name = String::from("mirror_plugin");
///   let plugin = Plugin::new(plugin_dir, &plugin_name)?;
///
///   Ok(())
/// }
/// ```
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
