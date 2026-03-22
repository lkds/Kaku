//! Windows Virtual Key Code to Key mapping
//!
//! Maps Windows VK_* codes to Kaku's internal Key representation.

use config::keys::Key;
use wezterm_input_types::Modifiers;
use winapi::um::winuser::*;

/// Convert Windows virtual key code to Kaku Key
pub fn vk_to_key(vk: u32) -> Option<Key> {
    // Standard ASCII printable characters
    if (0x30..=0x39).contains(&vk) {
        // 0-9
        return Some(Key::Char((vk as u8) as char));
    }
    if (0x41..=0x5A).contains(&vk) {
        // A-Z (convert to lowercase)
        return Some(Key::Char(((vk as u8) + 32) as char));
    }

    // Special keys mapping
    let key = match vk {
        VK_RETURN => Key::Enter,
        VK_TAB => Key::Tab,
        VK_BACK => Key::Backspace,
        VK_DELETE => Key::Delete,
        VK_INSERT => Key::Insert,
        VK_ESCAPE => Key::Escape,
        VK_SPACE => Key::Char(' '),
        
        // Arrow keys
        VK_UP => Key::UpArrow,
        VK_DOWN => Key::DownArrow,
        VK_LEFT => Key::LeftArrow,
        VK_RIGHT => Key::RightArrow,
        
        // Navigation keys
        VK_HOME => Key::Home,
        VK_END => Key::End,
        VK_PRIOR => Key::PageUp,
        VK_NEXT => Key::PageDown,
        
        // Function keys
        VK_F1 => Key::F(1),
        VK_F2 => Key::F(2),
        VK_F3 => Key::F(3),
        VK_F4 => Key::F(4),
        VK_F5 => Key::F(5),
        VK_F6 => Key::F(6),
        VK_F7 => Key::F(7),
        VK_F8 => Key::F(8),
        VK_F9 => Key::F(9),
        VK_F10 => Key::F(10),
        VK_F11 => Key::F(11),
        VK_F12 => Key::F(12),
        
        // Numpad
        VK_NUMPAD0 => Key::Keypad0,
        VK_NUMPAD1 => Key::Keypad1,
        VK_NUMPAD2 => Key::Keypad2,
        VK_NUMPAD3 => Key::Keypad3,
        VK_NUMPAD4 => Key::Keypad4,
        VK_NUMPAD5 => Key::Keypad5,
        VK_NUMPAD6 => Key::Keypad6,
        VK_NUMPAD7 => Key::Keypad7,
        VK_NUMPAD8 => Key::Keypad8,
        VK_NUMPAD9 => Key::Keypad9,
        VK_MULTIPLY => Key::KeypadMultiply,
        VK_ADD => Key::KeypadAdd,
        VK_SUBTRACT => Key::KeypadSubtract,
        VK_DECIMAL => Key::KeypadDecimal,
        VK_DIVIDE => Key::KeypadDivide,
        VK_SEPARATOR => Key::KeypadEnter,
        
        // Other special keys
        VK_CAPITAL => Key::CapsLock,
        VK_NUMLOCK => Key::NumLock,
        VK_SCROLL => Key::ScrollLock,
        VK_PAUSE => Key::Pause,
        VK_SNAPSHOT => Key::PrintScreen,
        VK_APPS => Key::Menu, // Menu key
        
        _ => return None,
    };
    
    Some(key)
}

/// Get modifier state from Windows keyboard state
pub fn get_modifiers() -> Modifiers {
    use winapi::um::winuser::GetKeyState;
    
    let mut mods = Modifiers::NONE;
    
    unsafe {
        if GetKeyState(VK_SHIFT) & 0x8000 != 0 {
            mods |= Modifiers::SHIFT;
        }
        if GetKeyState(VK_CONTROL) & 0x8000 != 0 {
            mods |= Modifiers::CTRL;
        }
        if GetKeyState(VK_MENU) & 0x8000 != 0 {
            mods |= Modifiers::ALT;
        }
        if GetKeyState(VK_LWIN) & 0x8000 != 0 || GetKeyState(VK_RWIN) & 0x8000 != 0 {
            mods |= Modifiers::SUPER;
        }
    }
    
    mods
}