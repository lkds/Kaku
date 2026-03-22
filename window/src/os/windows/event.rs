//! Windows Event Handle for spawn queue

use std::result::Result;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct EventHandle {
    signaled: AtomicBool,
    manual_reset: bool,
}

impl EventHandle {
    pub fn new_manual_reset() -> Result<Self, ()> {
        Ok(Self {
            signaled: AtomicBool::new(false),
            manual_reset: true,
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