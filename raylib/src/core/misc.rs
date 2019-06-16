use crate::core::*;
use std::ffi::CString;

/// Returns a random value between min and max (both included)
/// ```rust
/// use raylib::core::*;
/// fn main() {
///     let r = get_random_value(0, 10);
///     println!("random value: {}", r);
/// }
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { ffi::GetRandomValue(min, max) }
}

/// Open URL with default system browser (if available)
/// ```ignore
/// use raylib::core::*;
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

#[cfg(test)]
mod core_test {
    use crate::core::*;
    use crate::tests::*;
    ray_test!(test_screenshot);
    fn test_screenshot(t: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.take_screenshot(t, "test_out/screenshot.png");
        assert!(std::path::Path::new("test_out/screenshot.png").exists());
    }

    ray_test!(test_screendata);
    fn test_screendata(t: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        // make sure it doesn't seg fault
        let i = rl.get_screen_data(t);
    }
}
