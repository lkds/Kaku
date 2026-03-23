//! Windows-specific window and event handling
//!
//! This module provides Windows implementation for Kaku terminal,
//! including window management, clipboard, keyboard handling, and
//! application lifecycle.

#[cfg(target_os = "windows")]
mod app;
#[cfg(target_os = "windows")]
mod bitmap;
#[cfg(target_os = "windows")]
mod clipboard;
#[cfg(target_os = "windows")]
mod connection;
#[cfg(target_os = "windows")]
mod gl;
mod keycodes;
#[cfg(target_os = "windows")]
mod menu;
#[cfg(target_os = "windows")]
mod window;

#[cfg(target_os = "windows")]
pub use app::*;
#[cfg(target_os = "windows")]
pub use bitmap::*;
#[cfg(target_os = "windows")]
pub use clipboard::*;
#[cfg(target_os = "windows")]
pub use connection::*;
#[cfg(target_os = "windows")]
pub use gl::*;
pub use keycodes::*;
#[cfg(target_os = "windows")]
pub use menu::*;
#[cfg(target_os = "windows")]
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