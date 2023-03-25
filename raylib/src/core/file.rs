//! File manipulation functions. Should be parity with std::fs except on emscripten
use core::slice;
use std::ffi::CStr;

use crate::ffi;
use super::RaylibHandle;

impl<'a> RaylibHandle<'a> {
    /// Checks if a file has been dropped into the window.
    #[inline]
    pub fn is_file_dropped(&self) -> bool {
        unsafe { ffi::IsFileDropped() }
    }

    /// Gets dropped filenames.
    pub fn get_dropped_files(&self) -> Vec<String> {
        let dropfiles = unsafe { ffi::LoadDroppedFiles() };

        let v: Vec<String> =
            unsafe { slice::from_raw_parts(dropfiles.paths, dropfiles.count as usize) }
                .iter()
                .map(|p| unsafe { CStr::from_ptr(*p) })
                .map(|cstr| String::from(cstr.to_string_lossy()))
                .collect();

        unsafe { ffi::UnloadDroppedFiles(dropfiles) };

        v
    }
}
