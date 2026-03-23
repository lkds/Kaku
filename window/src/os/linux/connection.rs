//! Linux connection and event handling (Stub)

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

/// Connection state for the Linux application (stub)
pub struct Connection {
    pub(crate) windows: RefCell<HashMap<usize, Rc<RefCell<WindowInner>>>>,
    pub(crate) next_window_id: AtomicUsize,
    pub(crate) gl_connection: RefCell<Option<Rc<crate::egl::GlConnection>>>,
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
            gl_connection: RefCell::new(None),
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
        "Linux (stub)".to_string()
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