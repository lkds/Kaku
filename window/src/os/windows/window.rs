//! Windows Window implementation

use crate::{
    Clipboard, ClipboardData, Connection, Dimensions, MouseCursor, Rect, ResizeIncrement,
    ScreenPoint, WindowEvent, WindowEventSender, WindowOps, WindowState,
    RequestedWindowGeometry, ResolvedGeometry, Point, ULength, Size,
};
use crate::connection::ConnectionOps;
use anyhow::Result;
use async_trait::async_trait;
use promise::Future;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WindowHandle,
};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WindowInner {
    pub(crate) hwnd: Option<usize>, // HWND as usize
    event_sender: RefCell<Option<WindowEventSender>>,
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
            hwnd: None,
            event_sender: RefCell::new(None),
        }
    }
}

// Like macOS, we use just an ID for the Window struct to make it Send+Sync
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Window {
    id: usize,
}

impl Window {
    pub async fn new_window<F>(
        _class_name: &str,
        _name: &str,
        geometry: RequestedWindowGeometry,
        _config: Option<&config::ConfigHandle>,
        _font_config: Rc<wezterm_font::FontConfiguration>,
        event_handler: F,
    ) -> Result<Window>
    where
        F: 'static + FnMut(WindowEvent, &Window),
    {
        let conn = Connection::get()
            .ok_or_else(|| anyhow::anyhow!("Connection not initialized"))?;
        
        let ResolvedGeometry { width, height, x, y } = conn.resolve_geometry(geometry);
        
        let window_id = conn.next_window_id();
        let inner = Rc::new(RefCell::new(WindowInner::new()));
        
        let mut sender = WindowEventSender::new(event_handler);
        let window = Window { id: window_id };
        sender.assign_window(window.clone());
        inner.borrow_mut().event_sender.replace(Some(sender));
        
        // Register with connection
        conn.windows.borrow_mut().insert(window_id, inner);

        // TODO: Create actual Windows window with CreateWindowExW
        
        Ok(window)
    }

    pub fn window_id(&self) -> usize {
        self.id
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        // TODO: Return proper Windows display handle
        Err(HandleError::Unavailable)
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        // TODO: Return proper HWND handle
        Err(HandleError::Unavailable)
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

    fn get_clipboard(&self, _clipboard: crate::Clipboard) -> Future<String> {
        Future::result(Err(anyhow::anyhow!("Clipboard not implemented")))
    }

    fn get_clipboard_data(&self, _clipboard: crate::Clipboard) -> Future<ClipboardData> {
        Future::result(Err(anyhow::anyhow!("Clipboard not implemented")))
    }

    fn set_clipboard(&self, _clipboard: crate::Clipboard, _text: String) {
        // TODO: SetClipboardData
    }
}