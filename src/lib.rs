//  LIB.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 16:38:45
//  Last edited:
//    06 Aug 2022, 17:24:58
//  Auto updated?
//    Yes
// 
//  Description:
//!   The `rust-win` crate provides an OOP, RAII-wrapper around winit
//!   Windows and window-dependent Vulkan structs.
// 

// Declare modules
pub mod errors;
pub mod spec;
pub mod window;


// Export useful structs and such
pub use window::{Error, Window};


// Declare useful crate-local macros
/// Performs a `log`-crate `debug`, but only if that feature is defined
#[cfg(feature = "log")]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        log::debug!($target, $($arg)+)
    };

    ($($arg:tt)+) => {
        log::debug!($($arg)+)
    };
}
#[cfg(not(feature = "log"))]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => { () };

    ($($arg:tt)+) => { () };
}
pub(crate) use debug;

/// Performs a `log`-crate `info`, but only if that feature is defined
#[cfg(feature = "log")]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => {
        log::info!($target, $($arg)+)
    };

    ($($arg:tt)+) => {
        log::info!($($arg)+)
    };
}
#[cfg(not(feature = "log"))]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => { () };

    ($($arg:tt)+) => { () };
}
pub(crate) use info;
