//! Windows toast notification support
#![cfg(target_os = "windows")]

use crate::ToastNotification;

pub fn show_notif(_toast: ToastNotification) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement Windows toast notifications using Windows.Data.Xml.Dom.XmlDocument
    // and Windows.UI.Notifications.ToastNotification
    // For now, just log the notification
    log::info!(
        "Toast notification: {} - {}",
        _toast.title,
        _toast.message
    );
    Ok(())
}

pub fn initialize() {
    // Windows toast notifications don't require explicit initialization
    // like macOS does
}