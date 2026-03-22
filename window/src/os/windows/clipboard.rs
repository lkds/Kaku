//! Windows Clipboard support
//!
//! Provides clipboard operations using Windows API.

use anyhow::{bail, Context, Result};
use std::io::Read;
use winapi::um::winuser::*;
use winapi::um::winbase::GlobalAlloc;
use winapi::um::minwinbase::GMEM_MOVEABLE;
use winapi::shared::ntdef::HANDLE;

/// Windows clipboard implementation
pub struct Clipboard {
    clipboard_open: bool,
}

impl Clipboard {
    pub fn new() -> Result<Self> {
        Ok(Self { clipboard_open: false })
    }
    
    /// Get clipboard contents as string
    pub fn get(&mut self) -> Result<String> {
        unsafe {
            if OpenClipboard(std::ptr::null_mut()) == 0 {
                bail!("Failed to open clipboard");
            }
            self.clipboard_open = true;
            
            let handle = GetClipboardData(CF_UNICODETEXT);
            if handle.is_null() {
                CloseClipboard();
                self.clipboard_open = false;
                bail!("Failed to get clipboard data");
            }
            
            let ptr = winapi::um::winbase::GlobalLock(handle);
            if ptr.is_null() {
                CloseClipboard();
                self.clipboard_open = false;
                bail!("Failed to lock clipboard memory");
            }
            
            let text = std::ffi::CStr::from_ptr(ptr as *const i8)
                .to_string_lossy()
                .into_owned();
            
            winapi::um::winbase::GlobalUnlock(handle);
            CloseClipboard();
            self.clipboard_open = false;
            
            Ok(text)
        }
    }
    
    /// Set clipboard contents from string
    pub fn set(&mut self, text: &str) -> Result<()> {
        unsafe {
            if OpenClipboard(std::ptr::null_mut()) == 0 {
                bail!("Failed to open clipboard");
            }
            self.clipboard_open = true;
            
            EmptyClipboard();
            
            let text_utf16: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
            let size = text_utf16.len() * std::mem::size_of::<u16>();
            
            let handle = GlobalAlloc(GMEM_MOVEABLE, size);
            if handle.is_null() {
                CloseClipboard();
                self.clipboard_open = false;
                bail!("Failed to allocate clipboard memory");
            }
            
            let ptr = winapi::um::winbase::GlobalLock(handle);
            if ptr.is_null() {
                CloseClipboard();
                self.clipboard_open = false;
                bail!("Failed to lock clipboard memory");
            }
            
            std::ptr::copy_nonoverlapping(text_utf16.as_ptr(), ptr as *mut u16, text_utf16.len());
            
            winapi::um::winbase::GlobalUnlock(handle);
            SetClipboardData(CF_UNICODETEXT, handle);
            CloseClipboard();
            self.clipboard_open = false;
            
            Ok(())
        }
    }
    
    /// Check if clipboard has text content
    pub fn has_text(&self) -> bool {
        unsafe { IsClipboardFormatAvailable(CF_UNICODETEXT) != 0 }
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        if self.clipboard_open {
            unsafe { CloseClipboard() };
        }
    }
}

/// Clipboard content for advanced operations
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Files(Vec<std::path::PathBuf>),
}

impl Clipboard {
    /// Get clipboard format and content
    pub fn get_content(&mut self) -> Result<ClipboardContent> {
        // For now, just get text
        self.get().map(ClipboardContent::Text)
    }
}