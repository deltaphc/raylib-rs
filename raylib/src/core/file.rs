//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use crate::core::RaylibHandle;
use std::ffi::CStr;

impl RaylibHandle {
    /// Checks if a file has been dropped into the window.
    #[inline]
    pub fn is_file_dropped(&self) -> bool {
        unsafe { ffi::IsFileDropped() }
    }

    /// Gets dropped filenames.
    pub fn get_dropped_files(&self) -> Vec<String> {
        let mut v = Vec::new();
        unsafe {
            let mut count: i32 = 0;
            let dropfiles = ffi::LoadDroppedFiles(&mut count);
            for i in 0..count {
                let filestr = CStr::from_ptr(*dropfiles.offset(i as isize))
                    .to_str()
                    .unwrap();
                let file = String::from(filestr);
                v.push(file);
            }
        }
        v
    }

    /// Clears dropped files paths buffer.
    #[inline]
    pub fn clear_dropped_files(&mut self) {
        unsafe {
            ffi::ClearDroppedFiles();
        }
    }
}
