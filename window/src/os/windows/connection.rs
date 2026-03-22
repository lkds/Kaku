//! Windows connection and event handling
//!
//! Provides event loop and connection management for Windows.

use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;

use crate::os::Event;

/// Windows event handle for signaling
pub struct EventHandle {
    handle: HANDLE,
}

impl EventHandle {
    /// Create a new event handle
    pub fn new_manual_reset() -> Result<Self> {
        unsafe {
            let handle = CreateEventW(
                std::ptr::null_mut(),
                1, // Manual reset
                0, // Initial state: not signaled
                std::ptr::null(),
            );
            
            if handle.is_null() {
                anyhow::bail!("Failed to create event handle");
            }
            
            Ok(Self { handle })
        }
    }
    
    /// Set the event (signal)
    pub fn set_event(&self) {
        unsafe {
            SetEvent(self.handle);
        }
    }
    
    /// Reset the event
    pub fn reset_event(&self) {
        unsafe {
            ResetEvent(self.handle);
        }
    }
    
    /// Wait for the event
    pub fn wait(&self, timeout_ms: u32) -> bool {
        unsafe {
            WaitForSingleObject(self.handle, timeout_ms) == 0
        }
    }
    
    /// Get the handle
    pub fn handle(&self) -> HANDLE {
        self.handle
    }
}

impl Drop for EventHandle {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                CloseHandle(self.handle);
            }
        }
    }
}

unsafe impl Send for EventHandle {}
unsafe impl Sync for EventHandle {}

/// Connection state for a terminal session
pub struct Connection {
    window: HWND,
    events: Arc<Mutex<VecDeque<Event>>>,
}

impl Connection {
    /// Create a new connection
    pub fn new(window: HWND) -> Result<Self> {
        Ok(Self {
            window,
            events: Arc::new(Mutex::new(VecDeque::new())),
        })
    }
    
    /// Queue an event
    pub fn queue_event(&self, event: Event) {
        if let Ok(mut events) = self.events.lock() {
            events.push_back(event);
        }
    }
    
    /// Get pending events
    pub fn drain_events(&self) -> Vec<Event> {
        if let Ok(mut events) = self.events.lock() {
            events.drain(..).collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get the window handle
    pub fn window(&self) -> HWND {
        self.window
    }
    
    /// Notify the window to repaint
    pub fn request_redraw(&self) {
        unsafe {
            InvalidateRect(self.window, std::ptr::null(), 0);
        }
    }
    
    /// Show a toast notification (Windows 10+)
    pub fn show_notification(&self, title: &str, body: &str) -> Result<()> {
        // Use PowerShell for toast notifications on Windows
        let script = format!(
            r#"
            [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
            [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] | Out-Null
            
            $template = @"
            <toast>
                <visual>
                    <binding template='ToastText02'>
                        <text id='1'>{}</text>
                        <text id='2'>{}</text>
                    </binding>
                </visual>
            </toast>
"@
            
            $xml = New-Object Windows.Data.Xml.Dom.XmlDocument
            $xml.LoadXml($template)
            $toast = New-Object Windows.UI.Notifications.ToastNotification $xml
            [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('Kaku').Show($toast)
            "#,
            title.replace("'", "''"),
            body.replace("'", "''")
        );
        
        std::process::Command::new("powershell")
            .args(["-Command", &script])
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("Failed to show notification: {}", e))
    }
}

/// Event types for Windows
#[derive(Debug, Clone)]
pub enum EventType {
    KeyPress { vk: u32, mods: u32 },
    KeyRelease { vk: u32 },
    Char { ch: char },
    MouseMove { x: i32, y: i32 },
    MousePress { button: u32, x: i32, y: i32 },
    MouseRelease { button: u32, x: i32, y: i32 },
    Resize { width: u32, height: u32 },
    FocusGained,
    FocusLost,
    Close,
}