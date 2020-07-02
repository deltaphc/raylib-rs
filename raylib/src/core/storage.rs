//! Code for saving raw data to permanent (file based) storage
use crate::core::RaylibHandle;
use crate::ffi;

impl RaylibHandle {
    /// Saves integer value to storage file (to defined `position`).
    #[inline]
    pub fn save_storage_value(&mut self, position: u32, value: i32) {
        unsafe {
            ffi::SaveStorageValue(position, value);
        }
    }

    /// Loads integer value from storage file (from defined `position`).
    #[inline]
    pub fn load_storage_value(&self, position: u32) -> i32 {
        unsafe { ffi::LoadStorageValue(position) }
    }
}
