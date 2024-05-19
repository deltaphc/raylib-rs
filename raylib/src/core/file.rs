//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use crate::core::RaylibHandle;
use std::ffi::{CStr, CString, OsString};

make_thin_wrapper!(FilePathList, ffi::FilePathList, ffi::UnloadDirectoryFiles);
make_thin_wrapper!(
    DroppedFilePathList,
    ffi::FilePathList,
    ffi::UnloadDroppedFiles
);

impl FilePathList {
    /// Length of the file path list
    pub const fn count(&self) -> u32 {
        self.0.count
    }
    /// The amount of files that can be held in this list.
    pub const fn capacity(&self) -> u32 {
        self.0.capacity
    }
    /// The paths held in this list.
    /// This function is NOT constant and the inner array will be copied into the returned Vec every time you call this.
    pub fn paths(&self) -> Vec<&str> {
        unsafe { std::slice::from_raw_parts(self.0.paths, self.count() as usize) }
            .iter()
            .map(|f| unsafe { CStr::from_ptr(*f) }.to_str().unwrap())
            .collect()
    }
}

impl DroppedFilePathList {
    /// Length of the file path list
    pub const fn count(&self) -> u32 {
        self.0.count
    }
    /// The amount of files that can be held in this list.
    pub const fn capacity(&self) -> u32 {
        self.0.capacity
    }
    /// The paths held in this list.
    /// This function is NOT constant and the inner array will be copied into the returned Vec every time you call this.
    pub fn paths(&self) -> Vec<&str> {
        unsafe { std::slice::from_raw_parts(self.0.paths, self.count() as usize) }
            .iter()
            .map(|f| unsafe { CStr::from_ptr(*f) }.to_str().unwrap())
            .collect()
    }
}

impl RaylibHandle {
    /// Checks if a file has been dropped into the window.
    #[inline]
    pub fn is_file_dropped(&self) -> bool {
        unsafe { ffi::IsFileDropped() }
    }

    /// Checks a file's extension.
    #[inline]
    pub fn is_file_extension<A>(&self, file_name: A, file_ext: A) -> bool
    where
        A: Into<OsString>,
    {
        let file_name = CString::new(file_name.into().to_string_lossy().as_bytes()).unwrap();
        let file_ext = CString::new(file_ext.into().to_string_lossy().as_bytes()).unwrap();
        unsafe { ffi::IsFileExtension(file_name.as_ptr(), file_ext.as_ptr()) }
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
    pub fn get_file_length<A>(&self, filename: A) -> i32
    where
        A: Into<OsString>,
    {
        let c_str = CString::new(filename.into().to_string_lossy().as_bytes()).unwrap();
        unsafe { ffi::GetFileLength(c_str.as_ptr()) }
    }

    /// Check if a given path is a file or a directory
    /// # Errors
    /// This function will return an error if the supplied bytes contain an internal 0 byte. The NulError returned will contain the bytes as well as the position of the nul byte.
    pub fn is_path_file<A>(&self, filename: A) -> bool
    where
        A: Into<OsString>,
    {
        let c_str = CString::new(filename.into().to_string_lossy().as_bytes()).unwrap();
        unsafe { ffi::IsPathFile(c_str.as_ptr()) }
    }

    /// Load directory filepaths
    pub fn load_directory_files<A>(&self, dir_path: A) -> FilePathList
    where
        A: Into<OsString>,
    {
        unsafe {
            let c_str = CString::new(dir_path.into().to_string_lossy().as_bytes()).unwrap(); // .unwrap() is okay here because any nul bytes placed into the actual string should be cleared out by to_string_lossy.
            FilePathList(ffi::LoadDirectoryFiles(c_str.as_ptr()))
        }
    }

    /// Load directory filepaths with extension filtering and recursive directory scan
    pub fn load_directory_files_ex<A>(
        &self,
        dir_path: A,
        filter: String,
        scan_sub_dirs: bool,
    ) -> FilePathList
    where
        A: Into<OsString>,
    {
        unsafe {
            let dir_c_str = CString::new(dir_path.into().to_string_lossy().as_bytes()).unwrap(); // .unwrap() is okay here because any nul bytes placed into the actual string should be cleared out by to_string_lossy.
            let filter_c_str = CString::new(filter.replace("\0", "").as_bytes()).unwrap();
            FilePathList(ffi::LoadDirectoryFilesEx(
                dir_c_str.as_ptr(),
                filter_c_str.as_ptr(),
                scan_sub_dirs,
            ))
        }
    }

    /// Check if a file has been dropped into window
    pub fn load_dropped_files(&self) -> DroppedFilePathList {
        unsafe { DroppedFilePathList(ffi::LoadDroppedFiles()) }
    }
}
