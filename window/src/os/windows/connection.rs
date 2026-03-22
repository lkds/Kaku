//! Windows Connection implementation

use crate::connection::ConnectionOps;
use crate::screen::Screens;
use crate::Appearance;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

use super::window::WindowInner;

pub struct Connection {
    pub(crate) windows: RefCell<HashMap<usize, Rc<RefCell<WindowInner>>>>,
    pub(crate) next_window_id: AtomicUsize,
}

impl Connection {
    pub(crate) fn create_new() -> anyhow::Result<Self> {
        Ok(Self {
            windows: RefCell::new(HashMap::new()),
            next_window_id: AtomicUsize::new(1),
        })
    }

    pub(crate) fn next_window_id(&self) -> usize {
        self.next_window_id
            .fetch_add(1, ::std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn window_by_id(&self, window_id: usize) -> Option<Rc<RefCell<WindowInner>>> {
        self.windows.borrow().get(&window_id).map(Rc::clone)
    }
}

impl ConnectionOps for Connection {
    fn name(&self) -> String {
        "Windows".to_string()
    }

    fn terminate_message_loop(&self) {
        // Post quit message
        #[cfg(windows)]
        unsafe {
            winapi::um::winuser::PostQuitMessage(0);
        }
    }

    fn run_message_loop(&self) -> Result<()> {
        #[cfg(windows)]
        {
            use winapi::um::winuser::{GetMessageW, DispatchMessageW, TranslateMessage, MSG};
            
            unsafe {
                let mut msg: MSG = std::mem::zeroed();
                while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
        Ok(())
    }

    fn get_appearance(&self) -> Appearance {
        // TODO: Query Windows for dark/light mode
        Appearance::Light
    }

    fn screens(&self) -> anyhow::Result<Screens> {
        // TODO: Implement using EnumDisplayMonitors
        anyhow::bail!("Screen enumeration not yet implemented on Windows")
    }
}