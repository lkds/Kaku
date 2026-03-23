//! Windows Window management
//!
//! Provides window creation, management, and rendering for Windows.
//! Architecture mirrors macOS: Window is just an ID, WindowInner holds the state.

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use parking_lot::RwLock;
use promise::Future;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use winapi::shared::windef::{HWND, HDC};
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;

use crate::os::parameters::*;
use crate::{
    Clipboard, ClipboardData, Connection, Dimensions, MouseCursor,
    Point, ScreenPoint, Rect, ResizeIncrement, RequestedWindowGeometry,
    ResolvedGeometry, WindowEvent, WindowEventSender, WindowOps, WindowState,
    ULength, Size,
};
use crate::connection::ConnectionOps;
use config::ConfigHandle;

/// Window inner state - stored in Connection's windows HashMap
pub(crate) struct WindowInner {
    pub(crate) hwnd: HWND,
    pub(crate) hdc: HDC,
    event_sender: RefCell<Option<WindowEventSender>>,
    title: RefCell<String>,
}

impl std::fmt::Debug for WindowInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowInner")
            .field("hwnd", &self.hwnd)
            .finish()
    }
}

impl WindowInner {
    pub fn new() -> Self {
        Self {
            hwnd: std::ptr::null_mut(),
            hdc: std::ptr::null_mut(),
            event_sender: RefCell::new(None),
            title: RefCell::new(String::new()),
        }
    }
}

// Window is just an ID, making it Send+Sync
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Window {
    id: usize,
}

impl Window {
    pub fn window_id(&self) -> usize {
        self.id
    }

    pub async fn new_window<F>(
        _class_name: &str,
        _name: &str,
        _geometry: RequestedWindowGeometry,
        _config: Option<&ConfigHandle>,
        _font_config: Rc<wezterm_font::FontConfiguration>,
        event_handler: F,
    ) -> Result<Window>
    where
        F: 'static + FnMut(WindowEvent, &Window),
    {
        let conn = Connection::get()
            .ok_or_else(|| anyhow::anyhow!("Connection not initialized"))?;

        let window_id = conn.next_window_id();
        let inner = Rc::new(RefCell::new(WindowInner::new()));

        let mut sender = WindowEventSender::new(event_handler);
        let window = Window { id: window_id };
        sender.assign_window(window.clone());
        inner.borrow_mut().event_sender.replace(Some(sender));

        // Register with connection
        conn.windows.borrow_mut().insert(window_id, inner);

        // TODO: Create actual Win32 window

        Ok(window)
    }
}

impl raw_window_handle::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::Unavailable)
    }
}

impl raw_window_handle::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::Unavailable)
    }
}

#[async_trait(?Send)]
impl WindowOps for Window {
    fn show(&self) {
        // TODO: Show Win32 window
    }

    fn notify<T: Any + Send + Sync>(&self, _t: T) {}

    async fn enable_opengl(&self) -> Result<Rc<glium::backend::Context>> {
        anyhow::bail!("OpenGL not yet implemented on Windows")
    }

    fn finish_frame(&self, _frame: glium::Frame) -> Result<()> {
        Ok(())
    }

    fn hide(&self) {
        // TODO: Hide Win32 window
    }

    fn close(&self) {
        // TODO: Close Win32 window
    }

    fn set_cursor(&self, _cursor: Option<MouseCursor>) {}

    fn invalidate(&self) {
        // TODO: Invalidate Win32 window rect
    }

    fn set_title(&self, _title: &str) {}

    fn set_inner_size(&self, _width: usize, _height: usize) {}

    fn get_clipboard(&self, _clipboard: crate::Clipboard) -> Future<String> {
        Future::result(Err(anyhow::anyhow!("Clipboard not implemented")))
    }

    fn get_clipboard_data(&self, _clipboard: crate::Clipboard) -> Future<ClipboardData> {
        Future::result(Err(anyhow::anyhow!("Clipboard not implemented")))
    }

    fn set_clipboard(&self, _clipboard: crate::Clipboard, _text: String) {}
}