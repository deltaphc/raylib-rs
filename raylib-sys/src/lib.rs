#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[allow(dead_code)]
/// Runs test on raylib functions that should not require a window
#[cfg(test)]
mod core_no_window_test {
    use super::*;
    use std::ffi::CString;

    #[test]
    /// None of these should panic/segfault
    fn test_monitor_info() {
        unsafe {
            GetMonitorCount();
            GetMonitorWidth(0);
            GetMonitorHeight(0);
            GetMonitorPhysicalWidth(0);
            GetMonitorPhysicalHeight(0);
            GetMonitorName(0);
        }
    }

    #[test]
    fn test_file_ops() {
        let file_name = CString::new("doesnt_exist.txt").unwrap();
        let file_ext = CString::new("txt").unwrap();
        let current = CString::new(".").unwrap();
        unsafe {
            assert!(!FileExists(file_name.as_ptr()));
            assert!(!IsFileExtension(file_name.as_ptr(), file_ext.as_ptr()));
            GetDirectoryPath(current.as_ptr());
            GetWorkingDirectory();
            ChangeDirectory(current.as_ptr());
            GetFileModTime(current.as_ptr());
            StorageSaveValue(0, 5);
            assert_eq!(5, StorageLoadValue(0));
        }
    }
}