//  SPEC.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 16:54:12
//  Last edited:
//    06 Aug 2022, 17:08:29
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines public interfaces and structs for the `rust-win` crate.
// 

use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use crate::errors::WindowModeError as Error;


/***** LIBRARY *****/
/// The WindowMode defines how to size and place a Window.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case", tag = "mode"))]
pub enum WindowMode {
    /// Draws the window as a, well, window.
    Windowed{ resolution: (u32, u32) },
    /// Draws the window in windowed fullscreen mode.
    WindowedFullscreen{ monitor: usize },
    /// Draws the window in fullscreen mode.
    Fullscreen{ monitor: usize, resolution: (u32, u32), refresh_rate: u16 },
}

impl FromStr for WindowMode {
    type Err = Error;

    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "windowed"            => Ok(WindowMode::Windowed{ resolution: (0, 0) }),
            "windowed_fullscreen" => Ok(WindowMode::WindowedFullscreen{ monitor: 0 }),
            "fullscreen"          => Ok(WindowMode::Fullscreen{ monitor: 0, resolution: (0, 0), refresh_rate: 0 }),
            value                 => Err(Error::UnknownWindowMode{ got: value.into() }),
        }
    }
}



/// Defines a struct with parameters to create a new Winit Window.
#[derive(Clone, Debug)]
pub struct WindowInfo {
    /// The title of the new Window.
    pub title       : String,
    /// The WindowMode of the new Window, which decides the size and location of it.
    pub window_mode : WindowMode,
}
