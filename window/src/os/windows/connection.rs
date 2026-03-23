//! Windows connection and event handling
//!
//! Provides event loop and connection management for Windows.

use crate::connection::ConnectionOps;
use crate::screen::Screens;
use crate::spawn::SPAWN_QUEUE;
use crate::Appearance;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::window::WindowInner;

/// Origin of quit request
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuitOrigin {
    UserRequested,
    SystemRequested,
}

/// Request application termination
pub fn request_terminate(_origin: QuitOrigin) {
    // TODO: Implement Windows termination
}

/// Connection state for the Windows application
pub struct Connection {
    pub(crate) windows: RefCell<HashMap<usize, Rc<RefCell<WindowInner>>>>,
    pub(crate) next_window_id: AtomicUsize,
    running: RefCell<bool>,
}

impl Connection {
    /// Create a new connection instance
    pub(crate) fn create_new() -> Result<Self> {
        // Ensure SPAWN_QUEUE is created
        SPAWN_QUEUE.run();
        
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

#[cfg(target_os = "windows")]
impl ConnectionOps for Connection {
    fn name(&self) -> String {
        "Windows".to_string()
    }

    fn terminate_message_loop(&self) {
        *self.running.borrow_mut() = false;
        unsafe {
            winapi::um::winuser::PostQuitMessage(0);
        }
    }

    fn run_message_loop(&self) -> Result<()> {
        use winapi::um::winuser::{GetMessageW, TranslateMessage, DispatchMessageW, MSG};
        
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

#[cfg(not(target_os = "windows"))]
impl ConnectionOps for Connection {
    fn name(&self) -> String {
        "Windows (stub)".to_string()
    }

    fn terminate_message_loop(&self) {
        *self.running.borrow_mut() = false;
    }

    fn run_message_loop(&self) -> Result<()> {
        // Stub: just spin
        *self.running.borrow_mut() = true;
        while *self.running.borrow() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(())
    }

    fn get_appearance(&self) -> Appearance {
        Appearance::Light
    }

    fn screens(&self) -> anyhow::Result<Screens> {
        anyhow::bail!("Screen enumeration not available")
    }
}