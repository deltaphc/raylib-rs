//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use std::ffi::{CStr, CString};
use std::path::Path;

/// Check if file exists
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let check = file_exists("resources/just_exists.txt");
///     assert!(!check, "file unexpectedly exists");
/// }
pub fn file_exists(file_name: impl AsRef<Path>) -> bool {
    if let Some(s) = file_name.as_ref().to_str() {
        if let Ok(c) = CString::new(s) {
            return unsafe { ffi::FileExists(c.as_ptr()) };
        }
    }
    false
}

// Check file extension
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let check = is_file_extension("resources/just_exists.txt", ".txt");
///     assert!(check, "extension mismatch");
/// }
pub fn is_file_extension(filename: &str, ext: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    let c_ext = CString::new(ext).unwrap();
    unsafe { ffi::IsFileExtension(c_filename.as_ptr(), c_ext.as_ptr()) }
}

/// Gets the extension for a `filename` string.
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let check = get_extension("resources/just_exists.txt");
///     assert_eq!(check, "txt", "extension mismatch");
/// }
pub fn get_extension(filename: &str) -> String {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let ext = ffi::GetExtension(c_filename.as_ptr());
        CStr::from_ptr(ext).to_str().unwrap().to_owned()
    }
}

/// Gets the filename for a path string.
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let check = get_file_name("resources/just_exists.txt");
///     assert_eq!(check, "just_exists.txt", "extension mismatch");
/// }
pub fn get_file_name(file_path: &str) -> String {
    let c_file_path = CString::new(file_path).unwrap();
    unsafe {
        let filename = ffi::GetFileName(c_file_path.as_ptr());
        CStr::from_ptr(filename).to_str().unwrap().to_owned()
    }
}

/// Gets full path for a given `filename`.
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let check = get_directory_path("resources/just_exists.txt");
///     assert_eq!(check, "resources", "extension mismatch");
/// }
pub fn get_directory_path(filename: &str) -> String {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let dirpath = ffi::GetDirectoryPath(c_filename.as_ptr());
        CStr::from_ptr(dirpath).to_str().unwrap().to_owned()
    }
}

/// Gets all the files in a Directory
pub fn get_directory_files(dir: &str) -> Vec<String> {
    let c_dir = CString::new(dir).unwrap();
    let mut count = 0;
    let files = unsafe { ffi::GetDirectoryFiles(c_dir.as_ptr(), &mut count) };
    let mut f_vec = Vec::with_capacity(count as usize);
    for i in 0..count {
        unsafe {
            f_vec.push(
                CString::from_raw(*files.offset(i as isize))
                    .to_str()
                    .unwrap()
                    .to_owned(),
            );
        }
    }
    unsafe {
        libc::free(files as *mut libc::c_void);
    }
    f_vec
}

/// Gets current working directory.
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let check = get_working_directory();
///     let current = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
///     assert_eq!(check, current, "extension mismatch");
/// }
pub fn get_working_directory() -> String {
    unsafe {
        let workdir = ffi::GetWorkingDirectory();
        CStr::from_ptr(workdir).to_str().unwrap().to_owned()
    }
}

/// Changes working directory, returns true on success.
pub fn change_directory(dir: &str) -> bool {
    let c_dir = CString::new(dir).unwrap();
    unsafe { ffi::ChangeDirectory(c_dir.as_ptr()) }
}

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
