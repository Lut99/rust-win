# Changelog
This file will maintain a list of changes per release of the rust-win crate.


## [1.0.0] - 2022-08-06
### Added
- Initial set of objects, based on the `window.rs` file from the [Game-Rust](https://github.com/Lut99/Game-Rust) project.
- A README.md.
- A CHANGELOG.md.
- A .gitignore file.

### Changed
- `log`-related stuff as a feature.
- Various function interfaces (e.g., `Window::extent()` now returns an owned Extent2D instead of a referenced).
- The constructor to use a `WindowInfo` struct for some of its parameters.
