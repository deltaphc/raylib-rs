use crate::core::RaylibHandle;
use crate::ffi;

impl RaylibHandle {
    /// Saves integer value to storage file (to defined `position`).
    #[inline]
    pub fn storage_save_value(&mut self, position: i32, value: i32) {
        unsafe {
            ffi::StorageSaveValue(position, value);
        }
    }

    /// Loads integer value from storage file (from defined `position`).
    #[inline]
    pub fn storage_load_value(&self, position: i32) -> i32 {
        unsafe { ffi::StorageLoadValue(position) }
    }
}
