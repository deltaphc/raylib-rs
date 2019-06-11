use crate::core::*;
use crate::ffi;
use std::ffi::{CStr, CString, IntoStringError, NulError};

#[derive(Clone, Debug)]
pub struct MonitorInfo {
    width: i32,
    height: i32,
    physical_width: i32,
    physical_height: i32,
    name: String,
}

/// Get number of connected monitors
pub fn get_monitor_count() -> i32 {
    unsafe { ffi::GetMonitorCount() }
}
/// Gets the attributes of the monitor as well as the name
/// fails if monitor name is not a utf8 string
/// ```rust
/// use std::ffi::IntoStringError;
/// use raylib::core::*;
/// fn main() -> Result<(), IntoStringError> {
///     let count = get_monitor_count();
///     for i in (0..count) {
///         println!("{:?}", get_monitor_info(i)?);
///     }
///     Ok(())
/// }
/// ```
pub fn get_monitor_info(index: i32) -> Result<MonitorInfo, IntoStringError> {
    let len = get_monitor_count();
    debug_assert!(index < len && index >= 0, "monitor index out of range");
    let (width, height, physical_width, physical_height) = unsafe {
        (
            ffi::GetMonitorWidth(index),
            ffi::GetMonitorHeight(index),
            ffi::GetMonitorPhysicalWidth(index),
            ffi::GetMonitorPhysicalHeight(index),
        )
    };
    let name = unsafe {
        let c = CString::from_raw(ffi::GetMonitorName(index) as *mut i8);
        c.into_string()?
    };
    Ok(MonitorInfo {
        width,
        height,
        physical_height,
        physical_width,
        name,
    })
}

/// Returns camera transform matrix (view matrix)
/// ```rust
/// use raylib::core::*;
/// fn main() {
///     let c = Camera::orthographic(
///            Vector3::zero(),
///            Vector3::new(0.0, 0.0, 1.0),
///            Vector3::up(),
///            90.0,
///        );
///        let m = get_camera_matrix(&c);
/// }
/// ```
pub fn get_camera_matrix(camera: &Camera) -> Matrix {
    unsafe { ffi::GetCameraMatrix(camera.into()).into() }
}

impl RaylibHandle {
    /// Get clipboard text content
    pub fn get_clipboard_text(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let c = ffi::GetClipboardText();
            let c = CStr::from_ptr(c as *mut i8);
            c.to_str().map(|s| s.to_owned())
        }
    }

    /// Set clipboard text content
    pub fn set_clipboard_text(&mut self, text: &str) -> Result<(), NulError> {
        let s = CString::new(text)?;
        unsafe {
            ffi::SetClipboardText(s.as_ptr());
        }
        Ok(())
    }
}

// Screen-space-related functions
impl RaylibHandle {
    /// Returns a ray trace from mouse position
    pub fn get_mouse_ray(&self, mouse_position: &Vector2, camera: &Camera) -> Ray {
        unsafe { ffi::GetMouseRay(mouse_position.into(), camera.into()).into() }
    }

    /// Returns the screen space position for a 3d world space position
    pub fn get_world_to_screen(&self, position: &Vector3, camera: &Camera) -> Vector2 {
        unsafe { ffi::GetWorldToScreen(position.into(), camera.into()).into() }
    }
}

// Timing related functions
impl RaylibHandle {
    /// Set target FPS (maximum)
    pub fn set_target_fps(&mut self, fps: u32) {
        unsafe {
            ffi::SetTargetFPS(fps as i32);
        }
    }

    /// Returns current FPS
    pub fn get_fps(&self) -> u32 {
        unsafe { ffi::GetFPS() as u32 }
    }

    /// Returns time in seconds for last frame drawn
    pub fn get_frame_time(&self) -> f32 {
        unsafe { ffi::GetFrameTime() }
    }

    /// Returns elapsed time in seconds since InitWindow()
    pub fn get_time(&self) -> f64 {
        unsafe { ffi::GetTime() }
    }
}

#[cfg(test)]
mod core_test {
    use crate::core::*;
    use crate::test::*;
    #[test_case]
    fn test_clipboard() {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let s = "Hello, world!";
        rl.set_clipboard_text("Hello, world!").unwrap();
        let other = rl.get_clipboard_text().unwrap();
        assert_eq!(s, other);
    }

    #[test_case]
    fn test_screen_space() {
        let handle = TEST_HANDLE.read().unwrap();
        let rl = handle.as_ref().unwrap();
        let c = Camera::orthographic(
            Vector3::zero(),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::up(),
            90.0,
        );
        let _ = rl.get_mouse_ray(&Vector2::zero(), &c);
        // Should be the middle of the screen
        let _ = rl.get_world_to_screen(&Vector3::zero(), &c);
    }

    #[test_case]
    fn test_timing_functions() {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.set_target_fps(24);
        let _fps = rl.get_fps();
        // TODO uncomment this once drawing is possible
        // assert_eq!(fps, 24, "fps doeesn't match up after set");
        // make sure they don't panic
        rl.get_frame_time();
        rl.get_time();
    }
}
