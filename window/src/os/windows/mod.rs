//! Windows-specific window and event handling
//!
//! This module provides Windows implementation for Kaku terminal,
//! including window management, clipboard, keyboard handling, and
//! application lifecycle.

mod app;
mod bitmap;
mod clipboard;
mod connection;
mod gl;
mod keycodes;
mod menu;
mod window;

pub use app::*;
pub use bitmap::*;
pub use clipboard::*;
pub use connection::*;
pub use gl::*;
pub use keycodes::*;
pub use menu::*;
pub use window::*;

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize Windows-specific resources
pub fn init() {
    INIT.call_once(|| {
        // Initialize COM for clipboard and other Windows APIs
        #[cfg(windows)]
        unsafe {
            winapi::um::ole2::OleInitialize(std::ptr::null_mut());
        }
    });
}