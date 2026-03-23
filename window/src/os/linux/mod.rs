//! Linux-specific window and event handling (Stub for compilation)

mod connection;
mod event;
mod window;

pub use connection::*;
pub use event::*;
pub use window::*;

/// Initialize Linux-specific resources
pub fn init() {
    // No-op for stub
}