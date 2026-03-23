//! Windows-specific window and event handling
//!
//! This module provides Windows implementation for Kaku terminal.

mod connection;
mod event;
mod keycodes;
mod window;

pub use connection::*;
pub use event::*;
pub use keycodes::*;
pub use window::*;

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize Windows-specific resources
pub fn init() {
    INIT.call_once(|| {
        // Initialize COM for clipboard and other Windows APIs
        #[cfg(target_os = "windows")]
        unsafe {
            winapi::um::ole2::OleInitialize(std::ptr::null_mut());
        }
    });
}