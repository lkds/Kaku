//! Windows Clipboard implementation

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Self {
        Self
    }

    pub fn get(&mut self) -> Result<String, ()> {
        // TODO: Implement using GetClipboardData
        Ok(String::new())
    }

    pub fn set(&mut self, _text: &str) -> Result<(), ()> {
        // TODO: Implement using SetClipboardData
        Ok(())
    }
}