//! Windows-specific window and event handling
//!
//! This module provides Windows implementation for Kaku terminal.

mod app;
mod bitmap;
mod clipboard;
mod connection;
mod event;
mod keycodes;
mod menu;
mod window;

pub use app::*;
pub use bitmap::*;
pub use clipboard::*;
pub use connection::*;
pub use event::*;
pub use keycodes::*;
pub use menu::*;
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

/// Check if running in a Remote Desktop Protocol session
#[cfg(target_os = "windows")]
pub fn is_running_in_rdp_session() -> bool {
    use winapi::um::winuser::GetSystemMetrics;
    use winapi::um::winuser::SM_REMOTESESSION;
    unsafe { GetSystemMetrics(SM_REMOTESESSION) != 0 }
}

/// Check if running in a Remote Desktop Protocol session (stub for non-Windows)
#[cfg(not(target_os = "windows"))]
pub fn is_running_in_rdp_session() -> bool {
    false
}