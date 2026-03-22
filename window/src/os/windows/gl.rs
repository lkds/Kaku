//! Windows OpenGL/WGL support
//!
//! Provides OpenGL context creation and management using WGL.

use anyhow::{bail, Context, Result};
use log::{info, debug};
use std::rc::Rc;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use winapi::shared::windef::{HWND, HDC, HGLRC};
use winapi::shared::minwindef::*;

/// OpenGL context wrapper for Windows
pub struct GLContext {
    hwnd: HWND,
    hdc: HDC,
    hglrc: HGLRC,
}

impl GLContext {
    /// Create a new OpenGL context for the given window
    pub fn new(hwnd: HWND) -> Result<Self> {
        unsafe {
            let hdc = GetDC(hwnd);
            if hdc.is_null() {
                bail!("Failed to get device context");
            }
            
            // Set pixel format
            let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
            pfd.nSize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD;
            pfd.nVersion = 1;
            pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
            pfd.iPixelType = PFD_TYPE_RGBA;
            pfd.cColorBits = 32;
            pfd.cDepthBits = 24;
            pfd.cStencilBits = 8;
            pfd.iLayerType = PFD_MAIN_PLANE;
            
            let pixel_format = ChoosePixelFormat(hdc, &pfd);
            if pixel_format == 0 {
                ReleaseDC(hwnd, hdc);
                bail!("Failed to choose pixel format");
            }
            
            if SetPixelFormat(hdc, pixel_format, &pfd) == 0 {
                ReleaseDC(hwnd, hdc);
                bail!("Failed to set pixel format");
            }
            
            // Create OpenGL context
            let hglrc = wglCreateContext(hdc);
            if hglrc.is_null() {
                ReleaseDC(hwnd, hdc);
                bail!("Failed to create OpenGL context");
            }
            
            // Make context current
            if wglMakeCurrent(hdc, hglrc) == 0 {
                wglDeleteContext(hglrc);
                ReleaseDC(hwnd, hdc);
                bail!("Failed to make OpenGL context current");
            }
            
            info!("Created OpenGL context for Windows");
            
            Ok(Self { hwnd, hdc, hglrc })
        }
    }
    
    /// Create a modern OpenGL 3.2+ core context (requires WGL_ARB_create_context)
    pub fn new_core(hwnd: HWND, major: u8, minor: u8) -> Result<Self> {
        // First create a dummy context to load extensions
        let dummy = Self::new(hwnd)?;
        
        // Load wglCreateContextAttribsARB
        let wgl_create_context_attribs: Option<unsafe extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC> = unsafe {
            let proc = wglGetProcAddress(b"wglCreateContextAttribsARB\0".as_ptr() as *const i8);
            if proc.is_null() {
                None
            } else {
                Some(std::mem::transmute(proc))
            }
        };
        
        let core_hglrc = if let Some(create_fn) = wgl_create_context_attribs {
            unsafe {
                let attribs: [i32; 5] = [
                    WGL_CONTEXT_MAJOR_VERSION_ARB as i32, major as i32,
                    WGL_CONTEXT_MINOR_VERSION_ARB as i32, minor as i32,
                    0, // Terminate
                ];
                
                let hglrc = create_fn(dummy.hdc, std::ptr::null_mut(), attribs.as_ptr());
                if hglrc.is_null() {
                    debug!("Failed to create core OpenGL context, falling back to compatibility");
                    std::ptr::null_mut()
                } else {
                    hglrc
                }
            }
        } else {
            std::ptr::null_mut()
        };
        
        if !core_hglrc.is_null() {
            // Make the new context current
            unsafe {
                wglMakeCurrent(dummy.hdc, core_hglrc);
                wglDeleteContext(dummy.hglrc);
            }
            
            info!("Created OpenGL {}.{} core context", major, minor);
            
            Ok(Self {
                hwnd,
                hdc: dummy.hdc,
                hglrc: core_hglrc,
            })
        } else {
            // Fall back to the dummy context
            info!("Using compatibility OpenGL context");
            Ok(dummy)
        }
    }
    
    /// Make this context current
    pub fn make_current(&self) -> Result<()> {
        unsafe {
            if wglMakeCurrent(self.hdc, self.hglrc) == 0 {
                bail!("Failed to make OpenGL context current");
            }
            Ok(())
        }
    }
    
    /// Swap buffers (present)
    pub fn swap_buffers(&self) {
        unsafe {
            SwapBuffers(self.hdc);
        }
    }
    
    /// Set vertical sync
    pub fn set_vsync(&self, enabled: bool) {
        unsafe {
            // Load wglSwapIntervalEXT
            let swap_interval: Option<unsafe extern "system" fn(i32) -> i32> = {
                let proc = wglGetProcAddress(b"wglSwapIntervalEXT\0".as_ptr() as *const i8);
                if proc.is_null() {
                    None
                } else {
                    Some(std::mem::transmute(proc))
                }
            };
            
            if let Some(swap_fn) = swap_interval {
                swap_fn(if enabled { 1 } else { 0 });
            }
        }
    }
    
    /// Get the device context
    pub fn hdc(&self) -> HDC {
        self.hdc
    }
    
    /// Get the OpenGL context handle
    pub fn hglrc(&self) -> HGLRC {
        self.hglrc
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(std::ptr::null_mut(), std::ptr::null_mut());
            wglDeleteContext(self.hglrc);
            ReleaseDC(self.hwnd, self.hdc);
        }
    }
}

// WGL constants not defined in winapi
const WGL_CONTEXT_MAJOR_VERSION_ARB: DWORD = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: DWORD = 0x2092;

/// Create a glium context for the window
pub fn create_glium_context(hwnd: HWND) -> Result<Rc<glium::backend::Context>> {
    let gl_context = GLContext::new_core(hwnd, 3, 2)?;
    gl_context.make_current()?;
    
    // Create glium context
    let context = unsafe {
        glium::backend::Context::new(
            gl_context,
            glium::debug::DebugCallbackBehavior::default(),
            false, // NotDebugEnabled
        )
    }.map_err(|e| anyhow::anyhow!("Failed to create glium context: {:?}", e))?;
    
    Ok(context)
}