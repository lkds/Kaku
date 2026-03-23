//! Linux Event Handle for spawn queue (Stub)

use std::result::Result;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct EventHandle {
    signaled: AtomicBool,
}

impl EventHandle {
    pub fn new_manual_reset() -> Result<Self, ()> {
        Ok(Self {
            signaled: AtomicBool::new(false),
        })
    }

    pub fn set_event(&self) {
        self.signaled.store(true, Ordering::Release);
    }

    pub fn reset_event(&self) {
        self.signaled.store(false, Ordering::Release);
    }

    pub fn is_signaled(&self) -> bool {
        self.signaled.load(Ordering::Acquire)
    }
}