use crate::ffi;
use std::ffi::CString;

make_thin_wrapper!(Image, ffi::Image, ffi::UnloadImage);

/// Loads image from file into CPU memory (RAM).
#[inline]
pub fn load_image(filename: &str) -> Image {
    let c_filename = CString::new(filename).unwrap();
    unsafe { Image(ffi::LoadImage(c_filename.as_ptr())) }
}
