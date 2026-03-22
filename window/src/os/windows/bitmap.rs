//! Windows bitmap utilities
//!
//! Provides bitmap creation and manipulation for Windows.

use anyhow::Result;
use winapi::shared::windef::{HBITMAP, HDC, HWND};
use winapi::um::wingdi::*;
use winapi::shared::minwindef::*;

/// Bitmap wrapper for Windows
pub struct Bitmap {
    handle: HBITMAP,
    width: u32,
    height: u32,
}

impl Bitmap {
    /// Create a new bitmap
    pub fn new(width: u32, height: u32) -> Result<Self> {
        unsafe {
            let hdc = winapi::um::winuser::GetDC(std::ptr::null_mut());
            let handle = CreateCompatibleBitmap(hdc, width as i32, height as i32);
            winapi::um::winuser::ReleaseDC(std::ptr::null_mut(), hdc);
            
            if handle.is_null() {
                anyhow::bail!("Failed to create bitmap");
            }
            
            Ok(Self { handle, width, height })
        }
    }
    
    /// Create from RGBA data
    pub fn from_rgba(width: u32, height: u32, data: &[u8]) -> Result<Self> {
        let bitmap = Self::new(width, height)?;
        
        // Convert RGBA to BGRA for Windows
        let mut bgra_data = data.to_vec();
        for chunk in bgra_data.chunks_mut(4) {
            chunk.swap(0, 2); // Swap R and B
        }
        
        unsafe {
            let hdc = winapi::um::winuser::GetDC(std::ptr::null_mut());
            let mem_dc = CreateCompatibleDC(hdc);
            let old_bitmap = SelectObject(mem_dc, bitmap.handle);
            
            // Create BITMAPINFO
            let mut bmi: BITMAPINFO = std::mem::zeroed();
            bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = width as i32;
            bmi.bmiHeader.biHeight = -(height as i32); // Top-down
            bmi.bmiHeader.biPlanes = 1;
            bmi.bmiHeader.biBitCount = 32;
            bmi.bmiHeader.biCompression = BI_RGB;
            
            SetDIBits(
                hdc,
                bitmap.handle,
                0,
                height,
                bgra_data.as_ptr() as *const _,
                &bmi,
                DIB_RGB_COLORS,
            );
            
            SelectObject(mem_dc, old_bitmap);
            DeleteDC(mem_dc);
            winapi::um::winuser::ReleaseDC(std::ptr::null_mut(), hdc);
        }
        
        Ok(bitmap)
    }
    
    /// Get the bitmap handle
    pub fn handle(&self) -> HBITMAP {
        self.handle
    }
    
    /// Get dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                DeleteObject(self.handle as _);
            }
        }
    }
}

unsafe impl Send for Bitmap {}