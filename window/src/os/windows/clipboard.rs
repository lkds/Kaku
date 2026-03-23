//! Windows Clipboard handling (Stub)

use crate::ClipboardData;
use std::path::PathBuf;

/// Windows Clipboard (stub)
pub struct Clipboard {}

impl Clipboard {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read(&self) -> anyhow::Result<String> {
        anyhow::bail!("Clipboard not implemented on Windows stub")
    }

    pub fn read_data(&self) -> anyhow::Result<ClipboardData> {
        anyhow::bail!("Clipboard not implemented on Windows stub")
    }

    pub fn write(&self, _text: String) -> anyhow::Result<()> {
        anyhow::bail!("Clipboard not implemented on Windows stub")
    }

    pub fn write_files(&self, _files: &[PathBuf]) -> anyhow::Result<()> {
        anyhow::bail!("Clipboard not implemented on Windows stub")
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}