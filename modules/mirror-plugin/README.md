<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>mirror-plugin</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>

[![main](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/packages-validation.yml)

## Overview

This is image mirror plugin which is loaded during program run.
Crate is compiled to platform specific library format, which implements `C ABI` and used as
`Foreign Function Interface`.

## Description

This is runtime linked library file with exposed `C` like ABI.
The plugin exposes `process_image` symbol according to [PluginInterface](../common/src/plugin.rs).

The plugin applies blur effect to provided image data in place, no data is returned.

## Error codes

- `1` Arguments validation failed
- `2` Failed reading image data

## Usage

To use a shared library file (.so, .dll) provide the library name without the `lib` prefix and file extension.
The [libloading](https://docs.rs/libloading/latest/libloading/index.html)
will [construct filename](https://docs.rs/libloading/latest/libloading/index.html) specific to host OS.
Please see [Plugin](../common/src/plugin.rs) docs for usage details.

```rust
use std::path::PathBuf;
use common::plugin::Plugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let plugin_dir = PathBuf::from("plugin_dir");
  let plugin_name = String::from("mirror_plugin");

  Ok(())
}
```

## Stack

- [Rust](https://rust-lang.org/)
- [Serde](https://docs.rs/serde/latest/serde/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
