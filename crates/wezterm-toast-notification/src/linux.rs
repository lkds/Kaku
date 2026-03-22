//! Linux notification support via D-Bus
#![cfg(target_os = "linux")]

use crate::ToastNotification;

pub fn show_notif(_toast: ToastNotification) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement Linux notifications via D-Bus (org.freedesktop.Notifications)
    // Could use the `notify-rust` crate or direct D-Bus calls
    // For now, just log the notification
    log::info!(
        "Toast notification: {} - {}",
        _toast.title,
        _toast.message
    );
    Ok(())
}

pub fn initialize() {
    // Linux notifications via D-Bus don't require explicit initialization
}