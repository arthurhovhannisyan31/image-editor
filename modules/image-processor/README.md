<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>image-processor</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>

[![main](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/image-editor/actions/workflows/packages-validation.yml)

## Overview

This is an image editing CLI tool that uses dynamically loaded libraries for data processing.

## Description

The `image processor` is a flexible CLI tool that allows you to use any kind of library, which
implements [PluginInterface](../common/src/plugin.rs). The `image processor` takes an image as a source and outputs
processed
file in the same location with the `_1` suffix.

`Image processor` performs a set of validations on provided arguments to ensure files and configs are valid.
Please ensure you provide `plugin_dir` as a path to the plugin file directory and the `plugin_name` as a file name
without
`lib` prefix and file extension.
Filename resolution is done by [libloading](https://docs.rs/libloading/latest/libloading/index.html) and it
will [construct filename](https://docs.rs/libloading/latest/libloading/index.html) specific to the host OS.
The config should be a valid JSON string for a selected plugin.

## Synopsis

- `-i, --input <PathBuf>` Path to image file
- `-o --output <PathBuf>` Output directory
- `-p --plugin_dir <PathBuf>` Plugin directory
- `-P --plugin_name <String>` Plugin name
- `-c --config <String>` Plugin config as JSON string


- `--help`  Print help
- `-V, --version`  Print version

## Usage

```shell
image-processor -i <image_path> -o <output_dir> -p <plugin_dir> -P <plugin_name> -c <JSON string>

image-processor -i ./test.png -o . -p ./target/debug -P blur_plugin -c '{"radius":5, "iterations": 3}'
image-processor -i ./test.png -o . -p ./target/debug -P mirror_plugin -c '{"horizontal":true, "vertical":true}'
```

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
