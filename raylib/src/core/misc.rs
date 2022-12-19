//! Useful functions that don't fit anywhere else
use crate::core::texture::Image;
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::CString;
/// Returns a random value between min and max (both included)
/// ```rust
/// use raylib::*;
/// fn main() {
///     let r = get_random_value::<i32>(0, 10);
///     println!("random value: {}", r);
/// }
pub fn get_random_value<T: From<i32>>(min: i32, max: i32) -> T {
    unsafe { (ffi::GetRandomValue(min, max) as i32).into() }
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
    /// Load pixels from the screen into a CPU image
    pub fn load_image_from_screen(&self, _: &RaylibThread) -> Image {
        unsafe { Image(ffi::LoadImageFromScreen()) }
    }

    /// Takes a screenshot of current screen (saved a .png)
    pub fn take_screenshot(&mut self, _: &RaylibThread, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::TakeScreenshot(c_filename.as_ptr());
        }
    }

    /// Returns a random value between min and max (both included)
    /// ```rust
    /// use raylib::*;
    /// fn main() {
    ///     let (mut rl, thread) = ...;
    ///     let r = rl.get_random_value(0, 10);
    ///     println!("random value: {}", r);
    /// }
    pub fn get_random_value<T: From<i32>>(&self, min: i32, max: i32) -> T {
        unsafe { (ffi::GetRandomValue(min, max) as i32).into() }
    }

    /// Set the seed for random number generation
    pub fn set_random_seed(&mut self, seed: u32) {
        unsafe { ffi::SetRandomSeed(seed); }
    }
}

// lossy conversion to an f32
pub trait AsF32: Copy {
    fn as_f32(self) -> f32;
}

macro_rules! as_f32 {
    ($ty:ty) => {
        impl AsF32 for $ty {
            fn as_f32(self) -> f32 {
                self as f32
            }
        }
    };
}

as_f32!(u8);
as_f32!(u16);
as_f32!(u32);
as_f32!(i8);
as_f32!(i16);
as_f32!(i32);
as_f32!(f32);
