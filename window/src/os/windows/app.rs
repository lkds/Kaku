//! Windows Application lifecycle management
//!
//! Handles Windows message loop, window class registration, and
//! application initialization.

use anyhow::Result;
use log::{info, error};
use std::sync::{Arc, Mutex};
use winapi::um::winuser::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::LRESULT;
use winapi::shared::windef::HWND;

use crate::Window;

/// Application singleton for Windows
pub struct Application {
    windows: Arc<Mutex<Vec<Arc<Window>>>>,
    running: bool,
}

impl Application {
    /// Create a new application instance
    pub fn new() -> Result<Self> {
        super::init();
        
        Ok(Self {
            windows: Arc::new(Mutex::new(Vec::new())),
            running: false,
        })
    }
    
    /// Run the main event loop
    pub fn run(&mut self) -> Result<()> {
        self.running = true;
        
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            
            while self.running {
                let ret = GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);
                
                if ret == 0 {
                    // WM_QUIT received
                    break;
                }
                
                if ret == -1 {
                    error!("GetMessage error");
                    break;
                }
                
                // Process window messages
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        
        Ok(())
    }
    
    /// Request application termination
    pub fn quit(&mut self) {
        self.running = false;
        unsafe {
            PostQuitMessage(0);
        }
    }
    
    /// Register a window with the application
    pub fn register_window(&self, window: Arc<Window>) {
        if let Ok(mut windows) = self.windows.lock() {
            windows.push(window);
        }
    }
    
    /// Unregister a window from the application
    pub fn unregister_window(&self, hwnd: HWND) {
        if let Ok(mut windows) = self.windows.lock() {
            windows.retain(|w| w.hwnd() != hwnd);
        }
    }
    
    /// Get window count
    pub fn window_count(&self) -> usize {
        self.windows.lock().map(|w| w.len()).unwrap_or(0)
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // Clean up COM
        #[cfg(windows)]
        unsafe {
            winapi::um::ole2::OleUninitialize();
        }
    }
}

/// Window procedure callback
pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            PostQuitMessage(0);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_SIZE => {
            // Handle window resize
            let width = (lparam & 0xFFFF) as u32;
            let height = ((lparam >> 16) & 0xFFFF) as u32;
            log::debug!("Window resized to {}x{}", width, height);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            BeginPaint(hwnd, &mut ps);
            EndPaint(hwnd, &ps);
            0
        }
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            // Handle key press
            let vk = wparam as u32;
            let key = super::vk_to_key(vk);
            let mods = super::get_modifiers();
            
            if let Some(key) = key {
                log::debug!("Key pressed: {:?} with modifiers {:?}", key, mods);
            }
            0
        }
        WM_CHAR => {
            // Handle character input
            let ch = wparam as u32;
            if let Some(c) = char::from_u32(ch) {
                log::trace!("Char input: {:?}", c);
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}