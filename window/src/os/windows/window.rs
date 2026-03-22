//! Windows Window management
//!
//! Provides window creation, management, and rendering for Windows.

use anyhow::{Context, Result};
use async_trait::async_trait;
use log::{info, debug};
use parking_lot::RwLock;
use promise::Future;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle};
use std::sync::Arc;
use winapi::um::winuser::*;
use winapi::shared::windef::{HWND, HDC};
use winapi::shared::minwindef::*;
use winapi::um::dwmapi::DwmExtendFrameIntoClientArea;
use winapi::shared::dwmapi::MARGINS;
use winapi::um::wingdi::{GetDeviceCaps, LOGPIXELSX};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::os::parameters::*;
use crate::{
    Clipboard, ClipboardData, Dimensions, MouseCursor, RawKeyEvent, KeyEvent,
    Point, ScreenPoint, Rect, ResizeIncrement, RequestedWindowGeometry,
    ResolvedGeometry, WindowEvent, WindowEventSender, WindowOps, WindowState,
};
use config::window::WindowLevel;
use config::ConfigHandle;

// Forward declare trait for callbacks
pub trait WindowCallbacks: Send + Sync {
    fn on_event(&self, event: WindowEvent);
}

/// Window configuration for creating new windows
#[derive(Debug, Clone, Default)]
pub struct WindowConfig {
    pub title: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub transparent: Option<bool>,
    pub fullscreen: Option<bool>,
}

/// Window state and configuration
struct InternalWindowState {
    title: String,
    width: u32,
    height: u32,
    fullscreen: bool,
    visible: bool,
    transparent: bool,
}

/// Windows window implementation
pub struct Window {
    hwnd: HWND,
    hdc: HDC,
    state: RwLock<InternalWindowState>,
    callbacks: RwLock<Option<Arc<dyn WindowCallbacks>>>,
}

impl Window {
    /// Create a new window
    pub fn new(config: &WindowConfig) -> Result<Arc<Self>> {
        let class_name = wide_str("KakuWindowClass");
        
        // Register window class if not already registered
        unsafe {
            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
                style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
                lpfnWndProc: Some(super::window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null_mut()),
                hIcon: LoadIconW(std::ptr::null_mut(), winapi::um::winuser::IDI_APPLICATION),
                hCursor: LoadCursorW(std::ptr::null_mut(), winapi::um::winuser::IDC_ARROW),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null(),
                lpszClassName: class_name.as_ptr(),
                hIconSm: LoadIconW(std::ptr::null_mut(), winapi::um::winuser::IDI_APPLICATION),
            };
            
            if RegisterClassExW(&wc) == 0 {
                let err = winapi::um::errhandlingapi::GetLastError();
                if err != 1410 { // Class already registered
                    debug!("RegisterClassExW returned {}, error {}", 
                           "already registered", err);
                }
            }
        }
        
        let title = config.title.clone().unwrap_or_else(|| "Kaku".to_string());
        let title_wide = wide_str(&title);
        let class_name = wide_str("KakuWindowClass");
        
        // Calculate window size
        let width = config.width.unwrap_or(1200);
        let height = config.height.unwrap_or(800);
        
        let (window_width, window_height) = unsafe {
            let mut rect = winapi::shared::windef::RECT {
                left: 0,
                top: 0,
                right: width as i32,
                bottom: height as i32,
            };
            AdjustWindowRect(&mut rect, WS_OVERLAPPEDWINDOW, 0);
            (rect.right - rect.left, rect.bottom - rect.top)
        };
        
        // Create window
        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW | WS_EX_LAYERED, // Layered for transparency
                class_name.as_ptr(),
                title_wide.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                window_width,
                window_height,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null_mut()),
                std::ptr::null_mut(),
            )
        };
        
        if hwnd.is_null() {
            anyhow::bail!("Failed to create window");
        }
        
        let hdc = unsafe { GetDC(hwnd) };
        if hdc.is_null() {
            anyhow::bail!("Failed to get device context");
        }
        
        // Enable transparency
        if config.transparent.unwrap_or(false) {
            unsafe {
                let margins = MARGINS {
                    cxLeftWidth: -1,
                    cxRightWidth: -1,
                    cyTopHeight: -1,
                    cyBottomHeight: -1,
                };
                DwmExtendFrameIntoClientArea(hwnd, &margins);
            }
        }
        
        let state = RwLock::new(WindowState {
            title,
            width: window_width as u32,
            height: window_height as u32,
            fullscreen: false,
            visible: true,
            transparent: config.transparent.unwrap_or(false),
        });
        
        info!("Created Windows window: {:?} {}x{}", hwnd, window_width, window_height);
        
        Ok(Arc::new(Self {
            hwnd,
            hdc,
            state,
            callbacks: RwLock::new(None),
        }))
    }
    
    /// Get the window handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
    
    /// Get the device context
    pub fn hdc(&self) -> HDC {
        self.hdc
    }
    
    /// Set window title
    pub fn set_title(&self, title: &str) {
        let title_wide = wide_str(title);
        unsafe {
            SetWindowTextW(self.hwnd, title_wide.as_ptr());
        }
        if let Ok(mut state) = self.state.write() {
            state.title = title.to_string();
        }
    }
    
    /// Get window title
    pub fn title(&self) -> String {
        self.state.read().map(|s| s.title.clone()).unwrap_or_default()
    }
    
    /// Show window
    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
        if let Ok(mut state) = self.state.write() {
            state.visible = true;
        }
    }
    
    /// Hide window
    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
        if let Ok(mut state) = self.state.write() {
            state.visible = false;
        }
    }
    
    /// Set window size
    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                std::ptr::null_mut(),
                0, 0,
                width as i32,
                height as i32,
                SWP_NOMOVE | SWP_NOZORDER,
            );
        }
        if let Ok(mut state) = self.state.write() {
            state.width = width;
            state.height = height;
        }
    }
    
    /// Get window size
    pub fn size(&self) -> (u32, u32) {
        self.state.read().map(|s| (s.width, s.height)).unwrap_or((800, 600))
    }
    
    /// Toggle fullscreen
    pub fn toggle_fullscreen(&self) {
        let fullscreen = self.state.read().map(|s| s.fullscreen).unwrap_or(false);
        
        unsafe {
            if fullscreen {
                // Exit fullscreen
                let style = WS_OVERLAPPEDWINDOW;
                SetWindowLongPtrW(self.hwnd, GWL_STYLE, style as LONG_PTR);
                SetWindowPos(
                    self.hwnd,
                    std::ptr::null_mut(),
                    100, 100,
                    1200, 800,
                    SWP_FRAMECHANGED | SWP_NOZORDER,
                );
            } else {
                // Enter fullscreen
                let style = WS_POPUP | WS_CLIPCHILDREN | WS_CLIPSIBLINGS;
                SetWindowLongPtrW(self.hwnd, GWL_STYLE, style as LONG_PTR);
                SetWindowPos(
                    self.hwnd,
                    std::ptr::null_mut(),
                    0, 0,
                    GetSystemMetrics(SM_CXSCREEN),
                    GetSystemMetrics(SM_CYSCREEN),
                    SWP_FRAMECHANGED | SWP_NOZORDER,
                );
            }
        }
        
        if let Ok(mut state) = self.state.write() {
            state.fullscreen = !fullscreen;
        }
    }
    
    /// Check if fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.state.read().map(|s| s.fullscreen).unwrap_or(false)
    }
    
    /// Set window opacity
    pub fn set_opacity(&self, opacity: f32) {
        unsafe {
            let alpha = (opacity * 255.0) as u8;
            SetLayeredWindowAttributes(
                self.hwnd,
                0,
                alpha,
                winapi::um::winuser::LWA_ALPHA,
            );
        }
    }
    
    /// Set callbacks
    pub fn set_callbacks(&self, callbacks: Arc<dyn WindowCallbacks>) {
        if let Ok(mut cbs) = self.callbacks.write() {
            *cbs = Some(callbacks);
        }
    }
    
    /// Request repaint
    pub fn request_redraw(&self) {
        unsafe {
            InvalidateRect(self.hwnd, std::ptr::null(), 0);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if !self.hdc.is_null() {
                ReleaseDC(self.hwnd, self.hdc);
            }
            if !self.hwnd.is_null() {
                DestroyWindow(self.hwnd);
            }
        }
    }
}

/// Convert string to Windows wide string
fn wide_str(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStringExt;
    std::ffi::OsString::from(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

// Implement raw_window_handle traits for wgpu integration
unsafe impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        let handle = raw_window_handle::Win32WindowHandle::new(
            std::num::NonZeroIsize::new(self.hwnd as isize).unwrap()
        );
        Ok(unsafe { WindowHandle::new_raw(RawWindowHandle::Win32(handle)) })
    }
}

unsafe impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        Ok(DisplayHandle::new_raw(RawDisplayHandle::Windows(
            raw_window_handle::WindowsDisplayHandle::new()
        )))
    }
}

/// High DPI support
impl Window {
    /// Get the current DPI for the window
    pub fn dpi(&self) -> f64 {
        unsafe {
            // Try to use GetDpiForWindow (Windows 10 1607+)
            let dpi = GetDpiForWindow(self.hwnd);
            if dpi != 0 {
                return dpi as f64;
            }
            
            // Fallback to system DPI
            let hdc = GetDC(std::ptr::null_mut());
            if !hdc.is_null() {
                let dpi = GetDeviceCaps(hdc, LOGPIXELSX);
                ReleaseDC(std::ptr::null_mut(), hdc);
                return dpi as f64;
            }
            
            96.0 // Default DPI
        }
    }
    
    /// Get DPI scale factor (1.0 = 100%)
    pub fn dpi_scale(&self) -> f64 {
        self.dpi() / 96.0
    }
    
    /// Convert logical pixels to physical pixels
    pub fn logical_to_physical(&self, logical: f64) -> i32 {
        (logical * self.dpi_scale()) as i32
    }
    
    /// Convert physical pixels to logical pixels
    pub fn physical_to_logical(&self, physical: i32) -> f64 {
        physical as f64 / self.dpi_scale()
    }
}

/// Window positioning
impl Window {
    /// Get window position on screen
    pub fn position(&self) -> (i32, i32) {
        unsafe {
            let mut rect: winapi::shared::windef::RECT = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut rect);
            (rect.left, rect.top)
        }
    }
    
    /// Set window position
    pub fn set_position(&self, x: i32, y: i32) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                std::ptr::null_mut(),
                x, y,
                0, 0,
                SWP_NOSIZE | SWP_NOZORDER,
            );
        }
    }
    
    /// Center window on screen
    pub fn center(&self) {
        unsafe {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            
            let (width, height) = self.size();
            
            let x = (screen_width - width as i32) / 2;
            let y = (screen_height - height as i32) / 2;
            
            self.set_position(x, y);
        }
    }
}

/// Window state queries
impl Window {
    /// Check if window is visible
    pub fn is_visible(&self) -> bool {
        self.state.read().map(|s| s.visible).unwrap_or(false)
    }
    
    /// Check if window is maximized
    pub fn is_maximized(&self) -> bool {
        unsafe {
            let style = GetWindowLongPtrW(self.hwnd, GWL_STYLE);
            (style & WS_MAXIMIZE as LONG_PTR) != 0
        }
    }
    
    /// Check if window is minimized
    pub fn is_minimized(&self) -> bool {
        unsafe {
            IsIconic(self.hwnd) != 0
        }
    }
    
    /// Maximize window
    pub fn maximize(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_MAXIMIZE);
        }
    }
    
    /// Restore window from minimized/maximized state
    pub fn restore(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_RESTORE);
        }
    }
    
    /// Minimize window
    pub fn minimize(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_MINIMIZE);
        }
    }
    
    /// Bring window to front
    pub fn bring_to_front(&self) {
        unsafe {
            SetForegroundWindow(self.hwnd);
        }
    }
    
    /// Focus this window
    pub fn focus(&self) {
        unsafe {
            SetFocus(self.hwnd);
        }
    }
}

/// Cursor management
impl Window {
    /// Set mouse cursor
    pub fn set_cursor(&self, cursor: Option<MouseCursor>) {
        unsafe {
            let cursor_id = match cursor {
                Some(MouseCursor::Arrow) => IDC_ARROW,
                Some(MouseCursor::Hand) => IDC_HAND,
                Some(MouseCursor::Text) => IDC_IBEAM,
                Some(MouseCursor::SizeUpDown) => IDC_SIZENS,
                Some(MouseCursor::SizeLeftRight) => IDC_SIZEWE,
                None => {
                    SetCursor(std::ptr::null_mut());
                    return;
                }
            };
            
            let hcursor = LoadCursorW(std::ptr::null_mut(), cursor_id);
            SetCursor(hcursor);
        }
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}