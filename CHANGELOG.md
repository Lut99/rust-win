# Changelog
This file will maintain a list of changes per release of the rust-win crate.


## [2.1.5] - 2022-08-13
### Changed
- Bumped `rust-vk` to 4.0.0.


## [2.1.4] - 2022-08-13
### Changed
- Bumped `rust-vk` to 3.0.1.


## [2.1.3] - 2022-08-13
### Changed
- Bumped `rust-vk` to 3.0.0.


## [2.1.2] - 2022-08-09
### Changed
- Bumped `rust-vk` to 2.0.2.


## [2.1.1] - 2022-08-09
### Changed
- `Window::extent()` now returns the accurate Window size instead of the (possibly) outdated Swapchain extent. To get the old size, use `Window::swapchain()` and then the Swapchain's function.
- Bumped `rust-vk` to 2.0.1.


## [2.1.0] - 2022-08-07
### Added
- A convenience constructor for the `WindowInfo` struct that does some implicit conversion.


## [2.0.0] - 2022-08-07
### Added
- A new `Window::request_redraw()` shortcut for triggering new winit redraw events for the underlying Window.
- Proper comments for `Window::new()`'s generic type.
- Comments warning for device idleness in `Window::rebuild()`.

### Changed
- Windows now no longer create ImageViews. Instead, this is left for the parent implementation. **[breaking]**


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
