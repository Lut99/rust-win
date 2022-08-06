# rust-win: A winit Window wrapper
A [winit](https://github.com/rust-windowing/winit)-based, [ash](https://github.com/ash-rs/ash)-compatible Window implementation in Rust. Builds on the `rust-vk` crate as a more convenient wrapper for ash.


## Installation
To use this crate in one of your projects, simply add:
```toml
rust-win = { git = "https://github.com/Lut99/rust-win" }
```
to your `Cargo.toml` file.

Optionally, you can select a specific version by adding:
```toml
rust-win = { git = "https://github.com/Lut99/rust-win", tag = "<VERSION>" }
```
where `<VERSION>` is the required version number.

Note that this crate uses [semantic versioning](https://semver.org). That means that any breaking change will always be reflected in the major version number.


## Usage
The documentation of this crate may be automatically generated by running:
```bash
cargo doc
```
in the source of this crate. Then, in your browser, navigate to:
```
file://<path-to-repo>/target/doc/rust_win/index.html
```
You can then browse the documentation at your leisure.


## Contributing
If you like to contribute to this library or have any suggestions / bugs to report, leave an issue over at the [issues](https://github.com/Lut99/rust-win/issues) page. Please tag them appropriate if you want the quickest support.