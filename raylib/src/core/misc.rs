//! Useful functions that don't fit anywhere else
use crate::core::texture::Image;
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::{CString};

/// Returns a random value between min and max (both included)
/// ```rust
/// use raylib::*;
/// fn main() {
///     let r = get_random_value(0, 10);
///     println!("random value: {}", r);
/// }
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { ffi::GetRandomValue(min, max) }
}

/// Open URL with default system browser (if available)
/// ```ignore
/// use raylib::*;
/// fn main() {
///     open_url("https://google.com");
/// }
pub fn open_url(url: &str) {
    let s = CString::new(url).expect("Not a string");
    unsafe {
        ffi::OpenURL(s.as_ptr());
    }
}

impl RaylibHandle {
    pub fn get_screen_data(&mut self, _: &RaylibThread) -> Image {
        unsafe { Image(ffi::GetScreenData()) }
    }

    /// Takes a screenshot of current screen (saved a .png)
    pub fn take_screenshot(&mut self, _: &RaylibThread, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::TakeScreenshot(c_filename.as_ptr());
        }
    }
}
