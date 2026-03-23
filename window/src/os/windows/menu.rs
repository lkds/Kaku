//! Windows Menu handling (Stub)

/// Menu item representation (stub)
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
}

impl MenuItem {
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
        }
    }
}

/// Represented item (stub)
#[derive(Debug, Clone)]
pub enum RepresentedItem {
    None,
}