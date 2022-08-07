//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 16:39:19
//  Last edited:
//    07 Aug 2022, 13:35:32
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines errors for the `rust-win` crate.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use rust_vk::auxillary::structs::Extent2D;


/***** LIBRARY *****/
/// Defines the toplevel errors that occur in the crate.
#[derive(Debug)]
pub enum WindowError {
    /// Unknown monitor index given.
    UnknownMonitor{ got: usize, expected: usize },
    /// No monitors found
    NoMonitors,
    /// The given VideoMode was not supported by the target monitor.
    UnknownVideoMode{ monitor: usize, resolution: (u32, u32), refresh_rate: u16, bit_depth: u16 },
    /// Failed to create a new Window.
    WindowCreateError{ title: String, err: winit::error::OsError },
    /// Failed to create a new Surface for a Window.
    SurfaceCreateError{ title: String, err: rust_vk::surface::Error },
    /// Failed to create a new Swapchain for a Window.
    SwapchainCreateError{ title: String, err: rust_vk::swapchain::Error },

    /// Failed to rebuild the swapchain.
    SwapchainRecreateError{ title: String, old_size: Extent2D<u32>, new_size: Extent2D<u32>, err: rust_vk::swapchain::Error },
}

impl Display for WindowError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowError::*;
        match self {
            UnknownMonitor{ got, expected }                                  => write!(f, "Unknown monitor index '{}' (only {} monitors known)", got, expected),
            NoMonitors                                                       => write!(f, "No monitors found to create a Window on"),
            UnknownVideoMode{ monitor, resolution, refresh_rate, bit_depth } => write!(f, "Monitor {} does not support {}x{}@{} ({} bpp)", monitor, resolution.0, resolution.1, refresh_rate, bit_depth),
            WindowCreateError{ title, err }                                  => write!(f, "Could not create window with title '{}': {}", title, err),
            SurfaceCreateError{ title, err }                                 => write!(f, "Could not create new Surface for window with title '{}': {}", title, err),
            SwapchainCreateError{ title, err }                               => write!(f, "Could not create new Swapchain for window with title '{}': {}", title, err),

            SwapchainRecreateError{ title, old_size, new_size, err } => write!(f, "Could not re-create Swapchain from {} to {} for window with title '{}': {}", old_size, new_size, title, err),
        }
    }
}

impl Error for WindowError {}



/// Defines errors that relate to a WindowMode (one error, to be precise).
#[derive(Debug)]
pub enum WindowModeError {
    /// Failed to parse a WindowError.
    UnknownWindowMode{ got: String },
}

impl Display for WindowModeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowModeError::*;
        match self {
            UnknownWindowMode{ got } => write!(f, "Unknown window mode identifier '{}'", got),
        }
    }
}

impl Error for WindowModeError {}
