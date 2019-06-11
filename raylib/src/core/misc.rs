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
    /// Takes a screenshot of current screen (saved a .png)
    pub fn take_screenshot(&self, file_name: &str) -> Result<(), std::ffi::NulError> {
        let s = CString::new(file_name)?;
        unsafe {
            ffi::TakeScreenshot(s.as_ptr());
        }
        Ok(())
    }
}

#[cfg(test)]
mod core_test {
    use crate::core::*;
    use crate::test::*;
    #[test_case]
    fn test_screenshot() {
        let handle = TEST_HANDLE.read().unwrap();
        let rl = handle.as_ref().unwrap();
        let filename = std::env::temp_dir()
            .join("screenshot.png")
            .to_str()
            .expect("no tempdir available")
            .to_owned();
        // println!("Out : {}", filename);
        rl.take_screenshot(&filename)
            .expect("couldn't take screenshot");
    }
}
