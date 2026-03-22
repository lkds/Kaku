//! Windows Virtual Key code mappings

use crate::KeyCode;

/// Convert a Windows Virtual Key code to KeyCode
pub fn vkey_to_keycode(_vkey: u16) -> Option<KeyCode> {
    // TODO: Implement VK to KeyCode mapping
    None
}

/// Convert KeyCode to Windows Virtual Key code
pub fn keycode_to_vkey(_key: KeyCode) -> Option<u16> {
    // TODO: Implement KeyCode to VK mapping
    None
}