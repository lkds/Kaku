//! Linux Connection implementation (Stub)

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

    pub(crate) fn window_by_id(&self, _window_id: usize) -> Option<Rc<RefCell<WindowInner>>> {
        None
    }
}

impl ConnectionOps for Connection {
    fn name(&self) -> String {
        "Linux".to_string()
    }

    fn terminate_message_loop(&self) {
        // TODO: Implement
    }

    fn run_message_loop(&self) -> Result<()> {
        // TODO: Implement
        Ok(())
    }

    fn get_appearance(&self) -> Appearance {
        // TODO: Query GTK/Qt for dark mode
        Appearance::Light
    }

    fn screens(&self) -> anyhow::Result<Screens> {
        // TODO: Implement using X11/Wayland
        anyhow::bail!("Screen enumeration not yet implemented on Linux")
    }
}