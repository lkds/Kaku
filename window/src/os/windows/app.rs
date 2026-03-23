//! Windows Application lifecycle management (Stub)
//!
//! Handles application lifecycle for Windows.

/// Request application termination (stub)
pub fn request_app_termination(_origin: super::connection::QuitOrigin, _detail: Option<&str>) {
    // TODO: Implement Windows termination
}

/// Create app delegate (stub - not needed on Windows)
pub fn create_app_delegate() -> *mut std::ffi::c_void {
    std::ptr::null_mut()
}

/// Flush pending service opens (stub)
pub fn flush_pending_service_opens() {
    // No-op on Windows
}

/// Sync global hotkey registration (stub)
pub fn sync_global_hotkey_registration() {
    // TODO: Implement Windows global hotkeys
}

/// Check if system is sleeping (stub)
pub fn is_system_sleeping() -> bool {
    false
}