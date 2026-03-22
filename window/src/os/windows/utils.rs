//! Windows utility functions

/// Check if the application is running in an RDP session
pub fn is_running_in_rdp_session() -> bool {
    #[cfg(target_os = "windows")]
    unsafe {
        use winapi::um::winuser::GetSystemMetrics;
        use winapi::um::winuser::SM_REMOTESESSION;
        GetSystemMetrics(SM_REMOTESESSION) != 0
    }
    #[cfg(not(target_os = "windows"))]
    false
}