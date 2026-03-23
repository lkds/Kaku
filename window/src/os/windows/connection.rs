//! Windows connection and event handling
//!
//! Provides event loop and connection management for Windows.

use crate::connection::ConnectionOps;
use crate::screen::Screens;
use crate::Appearance;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use winapi::um::handleapi::CloseHandle;
use winapi::um::synchapi::{CreateEventW, SetEvent, ResetEvent, WaitForSingleObject};

use super::window::WindowInner;

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

/// Connection state for the Windows application
pub struct Connection {
    pub(crate) windows: RefCell<HashMap<usize, Rc<RefCell<WindowInner>>>>,
    pub(crate) next_window_id: AtomicUsize,
    running: RefCell<bool>,
}

impl Connection {
    /// Create a new connection instance
    pub(crate) fn create_new() -> Result<Self> {
        Ok(Self {
            windows: RefCell::new(HashMap::new()),
            next_window_id: AtomicUsize::new(1),
            running: RefCell::new(false),
        })
    }

    /// Get the next window ID
    pub(crate) fn next_window_id(&self) -> usize {
        self.next_window_id
            .fetch_add(1, Ordering::Relaxed)
    }

    /// Get a window by its ID
    pub(crate) fn window_by_id(&self, window_id: usize) -> Option<Rc<RefCell<WindowInner>>> {
        self.windows.borrow().get(&window_id).map(Rc::clone)
    }
}

impl ConnectionOps for Connection {
    fn name(&self) -> String {
        "Windows".to_string()
    }

    fn terminate_message_loop(&self) {
        *self.running.borrow_mut() = false;
        unsafe {
            PostQuitMessage(0);
        }
    }

    fn run_message_loop(&self) -> Result<()> {
        *self.running.borrow_mut() = true;
        
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            
            while *self.running.borrow() {
                let ret = GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);
                
                if ret == 0 {
                    // WM_QUIT received
                    break;
                }
                
                if ret == -1 {
                    log::error!("GetMessage error");
                    break;
                }
                
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        
        Ok(())
    }

    fn get_appearance(&self) -> Appearance {
        // TODO: Query Windows for dark/light mode
        // For now, default to Light
        Appearance::Light
    }

    fn screens(&self) -> anyhow::Result<Screens> {
        // TODO: Implement screen enumeration using EnumDisplayMonitors
        anyhow::bail!("Screen enumeration not yet implemented on Windows")
    }
    
    fn default_dpi(&self) -> f64 {
        // TODO: Query actual DPI from Windows
        crate::DEFAULT_DPI
    }
}