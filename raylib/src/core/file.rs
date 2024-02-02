//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use crate::core::RaylibHandle;
use std::ffi::{CStr, CString, NulError};
use std::path::Path;
use std::str::Utf8Error;

impl RaylibHandle {
    /// Checks if a file has been dropped into the window.
    #[inline]
    pub fn is_file_dropped(&self) -> bool {
        unsafe { ffi::IsFileDropped() }
    }

    /// Gets dropped filenames.
    pub fn load_dropped_files(&self) -> Vec<String> {
        let mut v = Vec::new();
        unsafe {
            let dropfiles = ffi::LoadDroppedFiles();
            for i in 0..dropfiles.count {
                let filestr = CStr::from_ptr(*dropfiles.paths.offset(i as isize))
                    .to_str()
                    .unwrap();
                let file = String::from(filestr);
                v.push(file);
            }
            ffi::UnloadDroppedFiles(dropfiles)
        }
        v
    }
    /// Get the directory of the running application.
    pub fn application_directory(&self) -> String {
        unsafe {
            let st = ffi::GetApplicationDirectory();
            let c_str = CStr::from_ptr(st);

            // If this ever errors out, yell at @ioi_xd on Discord,
            c_str.to_str().unwrap().to_string()
        }
    }

    /// Get file length in bytes.
    ///
    /// # Errors
    /// This function will return an error if the supplied bytes contain an internal 0 byte. The NulError returned will contain the bytes as well as the position of the nul byte.
    pub fn get_file_length<A>(&self, filename: A) -> Result<i32, NulError>
    where
        A: Into<String>,
    {
        let c_str = CString::new(filename.into())?;
        unsafe { Ok(ffi::GetFileLength(c_str.as_ptr())) }
    }

    /// Check if a given path is a file or a directory
    /// # Errors
    /// This function will return an error if the supplied bytes contain an internal 0 byte. The NulError returned will contain the bytes as well as the position of the nul byte.
    pub fn is_path_file<A>(&self, filename: A) -> Result<bool, NulError>
    where
        A: Into<String>,
    {
        let c_str = CString::new(filename.into())?;
        unsafe { Ok(ffi::IsPathFile(c_str.as_ptr())) }
    }
}
