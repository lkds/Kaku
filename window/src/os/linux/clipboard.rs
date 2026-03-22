//! Linux Clipboard implementation (Stub)

use std::result::Result;

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Self { Self }
    pub fn get(&mut self) -> Result<String, ()> { Ok(String::new()) }
    pub fn set(&mut self, _text: &str) -> Result<(), ()> { Ok(()) }
}