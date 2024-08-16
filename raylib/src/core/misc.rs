//! Useful functions that don't fit anywhere else

use crate::core::texture::Image;
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::CString;
use std::ops::{Deref, DerefMut, Range};
use std::usize;

/// Struct for holding the result of RaylibHandle::load_random_sequence.
/// This is a thin wrapper for an array of i32. The reason it exists is because Raylib expects you
/// to unload the sequence it creates manually, and this struct does it for you.
pub struct RandomSequence<'a>(&'a mut [i32]);

impl<'a> Deref for RandomSequence<'a> {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> DerefMut for RandomSequence<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Drop for RandomSequence<'a> {
    fn drop(&mut self) {
        unsafe { ffi::UnloadRandomSequence(self.0.as_mut_ptr()) }
    }
}

impl<'a> IntoIterator for RandomSequence<'a> {
    type Item = i32;

    type IntoIter = RandSeqIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RandSeqIterator(self, 0)
    }
}
pub struct RandSeqIterator<'a>(RandomSequence<'a>, usize);

impl<'a> Iterator for RandSeqIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0.get(self.1);
        self.1 += 1;
        match ret {
            Some(a) => Some(*a),
            None => None,
        }
    }
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
    /// Load random values sequence, no values repeated
    pub fn load_random_sequence<'a>(&self, num: Range<i32>, count: u32) -> RandomSequence<'a> {
        unsafe {
            let ptr = ffi::LoadRandomSequence(count, num.start, num.end.into());
            RandomSequence(std::slice::from_raw_parts_mut(ptr, count as usize))
        }
    }
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
    /// ```ignore
    /// use raylib::*;
    /// fn main() {
    ///     let (mut rl, thread) = ...;
    ///     let r = rl.get_random_value(0, 10);
    ///     println!("random value: {}", r);
    /// }
    pub fn get_random_value<T: From<i32>>(&self, num: Range<i32>) -> T {
        unsafe { (ffi::GetRandomValue(num.start, num.end.into()) as i32).into() }
    }

    /// Set the seed for random number generation
    pub fn set_random_seed(&mut self, seed: u32) {
        unsafe {
            ffi::SetRandomSeed(seed);
        }
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
