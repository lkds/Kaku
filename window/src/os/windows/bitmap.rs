//! Windows Bitmap handling (Stub)

use crate::bitmaps::BitmapImage;

/// Bitmap reference for Windows (stub)
pub struct BitmapRef<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> BitmapRef<'a> {
    pub fn with_image(_image: &'a dyn BitmapImage) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}