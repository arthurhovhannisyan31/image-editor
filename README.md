<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>image-editor</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>

[![main](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/packages-validation.yml)

## Overview

This is an image editor workspace, which
includes [image-processor](./modules/image-processor), [blur-plugin](./modules/blur-plugin), [mirror-plugin](./modules/mirror-plugin).
The image editor provides a CLI tool to edit images by applying different filters, which
implement [PluginInterface](./modules/common/src/plugin.rs).

## Description

- [Image-processor](./modules/image-processor) is an image editing CLI tool that uses dynamically loaded libraries for
  data processing. `Image processor` performs a set of validations on provided arguments to ensure files and configs are
  valid.
- [Blur-plugin](./modules/blur-plugin), [Mirror-plugin](./modules/mirror-plugin) are shared library files that are
  loaded
  on demand during program runtime. Plugins implement [PluginInterface](./modules/common/src/plugin.rs) and apply
  changes to
  provided image data in place, no data is returned. The plugin performs safety validation
  checks for provided data pointers.

In its core, the following steps are performed:

- Plugin is compiled as a library file: `Rust -> C ABI`
- Image processor loads provided plugin at runtime
  using [libloading](https://docs.rs/libloading/latest/libloading/index.html)
- The `process_image` symbol is looked up in plugin: `C ABI <- Rust FFI`
- The FFI function `process_image()` is called with arguments: `Rust FFI <- Rust`
- A new image is created with processed data and written to `fs`

`Rust -> C ABI <- Rust FFI <- Rust`

## Usage

Please find the latest build binary for `image-processor` and plugin files in
the [GH Releases](https://github.com/arthurhovhannisyan31/image-editor/releases).
Download the archived files for your OS and run them from the `target/release` folder.

Please
see [Image-processor](./modules/image-processor), [Blur-plugin](./modules/blur-plugin), [Mirror-plugin](./modules/mirror-plugin)
and [PluginInterface](./modules/common/src/plugin.rs) documentation for details.

## Stack

- [Rust](https://rust-lang.org/)
- [Clap](https://docs.rs/clap/latest/clap/)
- [Image](https://docs.rs/image/latest/image/index.html)
- [Libloading](https://docs.rs/libloading/latest/libloading/)
- [Serde](https://docs.rs/serde/latest/serde/)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
