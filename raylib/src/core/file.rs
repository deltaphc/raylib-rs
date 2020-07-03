//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use std::ffi::CStr;

/// Checks if a file has been dropped into the window.
pub fn is_file_dropped() -> bool {
    unsafe { ffi::IsFileDropped() }
}

/// Gets dropped filenames.
pub fn get_dropped_files() -> Vec<String> {
    let mut v = Vec::new();
    unsafe {
        let mut count: i32 = 0;
        let dropfiles = ffi::GetDroppedFiles(&mut count);
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
pub fn clear_dropped_files() {
    unsafe {
        ffi::ClearDroppedFiles();
    }
}
