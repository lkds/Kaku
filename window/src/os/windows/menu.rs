//! Windows Menu support
//!
//! Provides context menu and menu bar functionality for Windows.

use anyhow::Result;
use log::debug;
use winapi::um::winuser::*;
use winapi::shared::windef::{HMENU, HWND};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, BOOL};
use winapi::um::shellapi::{POINT, TrackPopupMenu, TPM_LEFTALIGN, TPM_RIGHTBUTTON, TPM_TOPALIGN};

/// Menu item identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuItemId(pub u16);

/// Menu item definition
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: MenuItemId,
    pub label: String,
    pub enabled: bool,
    pub checked: bool,
    pub separator: bool,
    pub submenu: Option<Vec<MenuItem>>,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(id: u16, label: &str) -> Self {
        Self {
            id: MenuItemId(id),
            label: label.to_string(),
            enabled: true,
            checked: false,
            separator: false,
            submenu: None,
        }
    }
    
    /// Create a separator
    pub fn separator() -> Self {
        Self {
            id: MenuItemId(0),
            label: String::new(),
            enabled: true,
            checked: false,
            separator: true,
            submenu: None,
        }
    }
    
    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Set checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    
    /// Add submenu
    pub fn submenu(mut self, items: Vec<MenuItem>) -> Self {
        self.submenu = Some(items);
        self
    }
}

/// Windows menu wrapper
pub struct Menu {
    hmenu: HMENU,
}

impl Menu {
    /// Create a new popup menu
    pub fn new_popup() -> Result<Self> {
        unsafe {
            let hmenu = CreatePopupMenu();
            if hmenu.is_null() {
                anyhow::bail!("Failed to create popup menu");
            }
            Ok(Self { hmenu })
        }
    }
    
    /// Create a new menu bar
    pub fn new_menu() -> Result<Self> {
        unsafe {
            let hmenu = CreateMenu();
            if hmenu.is_null() {
                anyhow::bail!("Failed to create menu");
            }
            Ok(Self { hmenu })
        }
    }
    
    /// Add an item to the menu
    pub fn add_item(&self, item: &MenuItem) -> Result<()> {
        unsafe {
            if item.separator {
                AppendMenuW(self.hmenu, MF_SEPARATOR, 0, std::ptr::null());
            } else {
                let mut flags = MF_STRING;
                if !item.enabled {
                    flags |= MF_GRAYED;
                }
                if item.checked {
                    flags |= MF_CHECKED;
                }
                
                let label_wide = wide_str(&item.label);
                AppendMenuW(
                    self.hmenu,
                    flags,
                    item.id.0 as usize,
                    label_wide.as_ptr() as *const i8,
                );
            }
        }
        Ok(())
    }
    
    /// Add multiple items
    pub fn add_items(&self, items: &[MenuItem]) -> Result<()> {
        for item in items {
            self.add_item(item)?;
        }
        Ok(())
    }
    
    /// Show as context menu at the given position
    pub fn show_context_menu(&self, hwnd: HWND, x: i32, y: i32) -> Option<MenuItemId> {
        unsafe {
            let mut point = POINT { x, y };
            ClientToScreen(hwnd, &mut point);
            
            let cmd = TrackPopupMenu(
                self.hmenu,
                TPM_LEFTALIGN | TPM_TOPALIGN | TPM_RIGHTBUTTON,
                point.x,
                point.y,
                0,
                hwnd,
                std::ptr::null_mut(),
            );
            
            if cmd != 0 {
                Some(MenuItemId(cmd as u16))
            } else {
                None
            }
        }
    }
    
    /// Get the menu handle
    pub fn handle(&self) -> HMENU {
        self.hmenu
    }
    
    /// Set as window menu bar
    pub fn set_as_menu_bar(&self, hwnd: HWND) {
        unsafe {
            SetMenu(hwnd, self.hmenu);
        }
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        unsafe {
            if !self.hmenu.is_null() {
                DestroyMenu(self.hmenu);
            }
        }
    }
}

/// Standard Kaku context menu items
pub fn standard_context_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::new(1, "Copy"),
        MenuItem::new(2, "Paste"),
        MenuItem::separator(),
        MenuItem::new(3, "Select All"),
        MenuItem::separator(),
        MenuItem::new(4, "New Tab"),
        MenuItem::new(5, "New Window"),
        MenuItem::separator(),
        MenuItem::new(6, "Split Pane Vertical"),
        MenuItem::new(7, "Split Pane Horizontal"),
        MenuItem::separator(),
        MenuItem::new(10, "Settings..."),
    ]
}

/// Convert string to Windows wide string
fn wide_str(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStringExt;
    std::ffi::OsString::from(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

unsafe impl Send for Menu {}
unsafe impl Sync for Menu {}