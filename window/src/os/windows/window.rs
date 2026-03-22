//! Windows Window implementation

use crate::{
    Clipboard, ClipboardData, Connection, Dimensions, MouseCursor, Rect, ResizeIncrement,
    ScreenPoint, WindowEvent, WindowEventSender, WindowOps, WindowState,
};
use crate::connection::ConnectionOps;
use anyhow::Result;
use async_trait::async_trait;
use promise::Future;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WindowInner {
    pub(crate) hwnd: Option<usize>, // HWND as usize
    event_sender: RefCell<Option<WindowEventSender>>,
}

impl WindowInner {
    pub fn new() -> Self {
        Self {
            hwnd: None,
            event_sender: RefCell::new(None),
        }
    }
}

#[derive(Clone)]
pub struct Window {
    inner: Rc<RefCell<WindowInner>>,
    window_id: usize,
}

impl Window {
    pub fn new<F>(_event_handler: F) -> Result<Self>
    where
        F: 'static + FnMut(WindowEvent, &Window),
    {
        let inner = Rc::new(RefCell::new(WindowInner::new()));
        let window_id = Connection::get()
            .ok_or_else(|| anyhow::anyhow!("Connection not initialized"))?
            .next_window_id();

        let window = Self {
            inner: inner.clone(),
            window_id,
        };

        // Register with connection
        if let Some(conn) = Connection::get() {
            conn.windows.borrow_mut().insert(window_id, inner.clone());
        }

        Ok(window)
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }
}

#[async_trait(?Send)]
impl WindowOps for Window {
    fn show(&self) {
        // TODO: ShowWindow
    }

    fn notify<T: Any + Send + Sync>(&self, _t: T) {}

    async fn enable_opengl(&self) -> Result<Rc<glium::backend::Context>> {
        anyhow::bail!("OpenGL not yet implemented on Windows")
    }

    fn hide(&self) {
        // TODO: ShowWindow(SW_HIDE)
    }

    fn close(&self) {
        // TODO: DestroyWindow
    }

    fn set_cursor(&self, _cursor: Option<MouseCursor>) {
        // TODO: SetCursor
    }

    fn invalidate(&self) {
        // TODO: InvalidateRect
    }

    fn set_title(&self, _title: &str) {
        // TODO: SetWindowText
    }

    fn set_inner_size(&self, _width: usize, _height: usize) {
        // TODO: SetWindowPos
    }

    fn get_clipboard(&self, _clipboard: Clipboard) -> Future<String> {
        Future::result(Err(anyhow::anyhow!("Clipboard not implemented")))
    }

    fn get_clipboard_data(&self, _clipboard: Clipboard) -> Future<ClipboardData> {
        Future::result(Err(anyhow::anyhow!("Clipboard not implemented")))
    }

    fn set_clipboard(&self, _clipboard: Clipboard, _text: String) {
        // TODO: SetClipboardData
    }
}