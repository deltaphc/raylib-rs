//! File manipulation functions. Should be parity with std::fs except on emscripten
use crate::ffi;

use crate::core::RaylibHandle;
use std::ffi::{c_char, CStr, CString, OsString};

#[derive(Debug, Clone)]
pub struct FilePathIter<'a> {
    iter: std::slice::Iter<'a, Option<&'a c_char>>,
}
impl<'a> FilePathIter<'a> {
    /// # Safety
    /// The memory pointed to by `list` must not be mutated for `'a`.
    /// Every `*mut c_char` in `list` must outlive `'a`.
    ///
    /// ## Examples
    ///
    /// The following is invalid, because `list` is dropped while `it` is still borrowing it.
    /// ```compile_fail
    /// # use raylib::{ffi, file::*};
    /// # use std::{mem::ManuallyDrop, ffi::CStr};
    /// let mut it;
    /// let s;
    /// {
    ///     let mut paths = [
    ///         CStr::from_bytes_with_nul(b"apple\0").unwrap().as_ptr().cast_mut(),
    ///     ];
    ///     let mut list = ManuallyDrop::new(unsafe {
    ///         FilePathList::from_raw(ffi::FilePathList {
    ///             capacity: 1,
    ///             count: 1,
    ///             paths: paths.as_mut_ptr(),
    ///         })
    ///     });
    ///     it = list.iter(); // expect error[E0597]
    ///     //   ^^^^ borrowed value does not live long enough
    ///     s = it.next();
    ///     assert_eq!(s, Some("apple"));
    /// } // `list` dropped here while still borrowed
    /// assert_eq!(s, Some("apple")); // borrow later used here
    /// ```
    ///
    /// The following is invalid, because `list` is mutated while `it` is still borrowing it.
    /// ```compile_fail
    /// # use raylib::{ffi, file::*};
    /// # use std::{mem::ManuallyDrop, ffi::CStr};
    /// let mut paths = [
    ///     CStr::from_bytes_with_nul(b"apple\0").unwrap().as_ptr().cast_mut(),
    /// ];
    /// let mut list = ManuallyDrop::new(unsafe {
    ///     FilePathList::from_raw(ffi::FilePathList {
    ///         capacity: 1,
    ///         count: 1,
    ///         paths: paths.as_mut_ptr(),
    ///     })
    /// });
    /// let mut it = list.iter();
    /// //           ---- immutable borrow occurs here
    /// let s = it.next();
    /// assert_eq!(s, Some("apple"));
    /// unsafe { *(*list.paths) = b'@' as std::ffi::c_char; } // expect error[E0502]
    /// //          ^^^^ mutable borrow occurs here
    /// assert_eq!(s, Some("apple")); // immutable borrow later used here
    /// ```
    unsafe fn new(list: *mut *mut c_char, count: u32) -> Self {
        // No new items are being created that get dropped here, these are just changes in perspective of how to borrow-check the pointers.
        assert!(!list.is_null(), "file path array cannot be null");
        assert!(list.is_aligned(), "file path array must be aligned");
        let list = list.cast::<Option<&'a c_char>>();
        let iter = unsafe { std::slice::from_raw_parts(list, count as usize) }.iter();
        Self { iter }
    }
    fn func(f: &Option<&'a c_char>) -> &'a str {
        // CStr isn't being "constructed", it's essentially an adapter on &[c_char]
        let s = std::slice::from_ref(f.expect("file path string cannot be null"));
        unsafe { CStr::from_ptr(s.as_ptr()) }.to_str().unwrap()
    }
}
impl<'a> Iterator for FilePathIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Self::func)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(Self::func)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(Self::func)
    }
}
impl<'a> DoubleEndedIterator for FilePathIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Self::func)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Self::func)
    }
}
impl<'a> ExactSizeIterator for FilePathIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

make_thin_wrapper!(FilePathList, ffi::FilePathList, ffi::UnloadDirectoryFiles);
make_thin_wrapper!(
    DroppedFilePathList,
    ffi::FilePathList,
    ffi::UnloadDroppedFiles
);

impl FilePathList {
    /// Length of the file path list
    #[inline]
    pub const fn count(&self) -> u32 {
        self.0.count
    }
    /// The amount of files that can be held in this list.
    #[inline]
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
    /// An iterator over the paths held in this list.
    pub fn iter<'a>(&'a self) -> FilePathIter<'a> {
        unsafe { FilePathIter::new(self.0.paths, self.count()) }
    }
}

impl DroppedFilePathList {
    /// Length of the file path list
    #[inline]
    pub const fn count(&self) -> u32 {
        self.0.count
    }
    /// The amount of files that can be held in this list.
    #[inline]
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
    /// An iterator over the paths held in this list.
    pub fn iter<'a>(&'a self) -> FilePathIter<'a> {
        unsafe { FilePathIter::new(self.0.paths, self.count()) }
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
    ///
    /// # Errors
    /// This function will return an error if the supplied bytes contain an internal 0 byte. The NulError returned will contain the bytes as well as the position of the nul byte.
    #[must_use]
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
    #[inline]
    pub fn load_dropped_files(&self) -> DroppedFilePathList {
        unsafe { DroppedFilePathList(ffi::LoadDroppedFiles()) }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::ManuallyDrop;
    use super::*;

    #[test]
    #[should_panic(expected = "file path array cannot be null")]
    fn test_null_list() {
        let list = ManuallyDrop::new(FilePathList(ffi::FilePathList {
            capacity: 0,
            count: 0,
            paths: std::ptr::null_mut(),
        }));
        let _it = list.iter();
        // should have panicked while calling .iter()
    }

    #[test]
    #[should_panic(expected = "file path string cannot be null")]
    fn test_null_item() {
        let mut paths = [std::ptr::null_mut()];
        let list = ManuallyDrop::new(FilePathList(ffi::FilePathList {
            capacity: 1,
            count: 1,
            paths: paths.as_mut_ptr(),
        }));
        let mut it = list.iter();
        let _f = it.next();
        // should have panicked while calling .next()
    }

    #[test]
    #[should_panic(expected = "file path string cannot be null")]
    fn test_null_item_double_ended() {
        let mut paths = [std::ptr::null_mut()];
        let list = ManuallyDrop::new(FilePathList(ffi::FilePathList {
            capacity: 1,
            count: 1,
            paths: paths.as_mut_ptr(),
        }));
        let mut it = list.iter();
        let _f = it.next_back();
        // should have panicked while calling .next_back()
    }

    #[test]
    fn test_len() {
        let mut paths = [
            CStr::from_bytes_with_nul(b"apple\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"orange\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"banana\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"mango\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"pineapple\0").unwrap().as_ptr().cast_mut(),
        ];
        let list = ManuallyDrop::new(FilePathList(ffi::FilePathList {
            capacity: 5,
            count: 5,
            paths: paths.as_mut_ptr(),
        }));
        let mut it = list.iter();
        assert_eq!(it.len(), 5);
        assert_eq!(it.next(), Some("apple"));
        assert_eq!(it.len(), 4);
        assert_eq!(it.next(), Some("orange"));
        assert_eq!(it.len(), 3);
        assert_eq!(it.next(), Some("banana"));
        assert_eq!(it.len(), 2);
        assert_eq!(it.next(), Some("mango"));
        assert_eq!(it.len(), 1);
        assert_eq!(it.next(), Some("pineapple"));
        assert_eq!(it.len(), 0);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_len_double_ended() {
        let mut paths = [
            CStr::from_bytes_with_nul(b"apple\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"orange\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"banana\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"mango\0").unwrap().as_ptr().cast_mut(),
            CStr::from_bytes_with_nul(b"pineapple\0").unwrap().as_ptr().cast_mut(),
        ];
        let list = ManuallyDrop::new(FilePathList(ffi::FilePathList {
            capacity: 5,
            count: 5,
            paths: paths.as_mut_ptr(),
        }));
        let mut it = list.iter();
        assert_eq!(it.len(), 5);
        assert_eq!(it.next_back(), Some("pineapple"));
        assert_eq!(it.len(), 4);
        assert_eq!(it.next_back(), Some("mango"));
        assert_eq!(it.len(), 3);
        assert_eq!(it.next_back(), Some("banana"));
        assert_eq!(it.len(), 2);
        assert_eq!(it.next_back(), Some("orange"));
        assert_eq!(it.len(), 1);
        assert_eq!(it.next_back(), Some("apple"));
        assert_eq!(it.len(), 0);
        assert_eq!(it.next_back(), None);
    }
}
