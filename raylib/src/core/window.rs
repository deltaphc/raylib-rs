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
pub fn get_camera_matrix(camera: impl Into<ffi::Camera>) -> Matrix {
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
    pub fn get_mouse_ray(
        &self,
        mouse_position: impl Into<ffi::Vector2>,
        camera: impl Into<ffi::Camera>,
    ) -> Ray {
        unsafe { ffi::GetMouseRay(mouse_position.into(), camera.into()).into() }
    }

    /// Returns the screen space position for a 3d world space position
    pub fn get_world_to_screen(
        &self,
        position: impl Into<ffi::Vector3>,
        camera: impl Into<ffi::Camera>,
    ) -> Vector2 {
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

// Window handling functions
impl RaylibHandle {
    /// Checks if `KEY_ESCAPE` or Close icon was pressed.
    #[inline]
    pub fn window_should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    /// Checks if window has been initialized successfully.
    #[inline]
    pub fn is_window_ready(&self) -> bool {
        unsafe { ffi::IsWindowReady() }
    }

    /// Checks if window has been minimized (or lost focus).
    #[inline]
    pub fn is_window_minimized(&self) -> bool {
        unsafe { ffi::IsWindowMinimized() }
    }

    /// Checks if window has been resized.
    #[inline]
    pub fn is_window_resized(&self) -> bool {
        unsafe { ffi::IsWindowResized() }
    }

    /// Checks if window has been hidden.
    #[inline]
    pub fn is_window_hidden(&self) -> bool {
        unsafe { ffi::IsWindowResized() }
    }

    /// Toggles fullscreen mode (only on desktop platforms).
    #[inline]
    pub fn toggle_fullscreen(&mut self) {
        unsafe {
            ffi::ToggleFullscreen();
        }
    }

    /// Show the window.
    #[inline]
    pub fn unhide_window(&mut self) {
        unsafe {
            ffi::UnhideWindow();
        }
    }

    /// Hide the window.
    #[inline]
    pub fn hide_window(&mut self) {
        unsafe {
            ffi::HideWindow();
        }
    }

    /// TODO uncomment this when imaging is up
    // /// Sets icon for window (only on desktop platforms).
    // #[inline]
    // pub fn set_window_icon(&self, image: &Image) {
    //     unsafe {
    //         ffi::SetWindowIcon(image.0);
    //     }
    // }

    /// Sets title for window (only on desktop platforms).
    #[inline]
    pub fn set_window_title(&self, _: &RaylibThread, title: &str) {
        let c_title = CString::new(title).unwrap();
        unsafe {
            ffi::SetWindowTitle(c_title.as_ptr());
        }
    }

    /// Sets window position on screen (only on desktop platforms).
    #[inline]
    pub fn set_window_position(&mut self, x: i32, y: i32) {
        unsafe {
            ffi::SetWindowPosition(x, y);
        }
    }

    /// Sets monitor for the current window (fullscreen mode).
    #[inline]
    pub fn set_window_monitor(&mut self, monitor: i32) {
        unsafe {
            ffi::SetWindowMonitor(monitor);
        }
    }

    /// Sets minimum window dimensions (for `FLAG_WINDOW_RESIZABLE`).
    #[inline]
    pub fn set_window_min_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::SetWindowMinSize(width, height);
        }
    }

    /// Sets window dimensions.
    #[inline]
    pub fn set_window_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::SetWindowSize(width, height);
        }
    }

    /// Gets current screen width.
    #[inline]
    pub fn get_screen_width(&self) -> i32 {
        unsafe { ffi::GetScreenWidth() }
    }

    /// Gets current screen height.
    #[inline]
    pub fn get_screen_height(&self) -> i32 {
        unsafe { ffi::GetScreenHeight() }
    }
}

// Cursor-related functions
impl RaylibHandle {
    /// Shows mouse cursor.
    #[inline]
    pub fn show_cursor(&mut self) {
        unsafe {
            ffi::ShowCursor();
        }
    }

    /// Hides mouse cursor.
    #[inline]
    pub fn hide_cursor(&mut self) {
        unsafe {
            ffi::HideCursor();
        }
    }

    /// Checks if mouse cursor is not visible.
    #[inline]
    pub fn is_cursor_hidden(&self) -> bool {
        unsafe { ffi::IsCursorHidden() }
    }

    /// Enables mouse cursor (unlock cursor).
    #[inline]
    pub fn enable_cursor(&mut self) {
        unsafe {
            ffi::EnableCursor();
        }
    }

    /// Disables mouse cursor (lock cursor).
    #[inline]
    pub fn disable_cursor(&mut self) {
        unsafe {
            ffi::DisableCursor();
        }
    }
}

#[cfg(test)]
mod core_test {
    use crate::core::*;
    use crate::tests::*;
    #[test]
    fn test_clipboard() {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let s = "Hello, world!";
        rl.set_clipboard_text("Hello, world!").unwrap();
        let other = rl.get_clipboard_text().unwrap();
        assert_eq!(s, other);
    }

    #[test]
    fn test_screen_space() {
        let handle = TEST_HANDLE.read().unwrap();
        let rl = handle.as_ref().unwrap();
        let c = Camera::orthographic(
            Vector3::zero(),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::up(),
            90.0,
        );
        let _ = rl.get_mouse_ray(Vector2::zero(), &c);
        // Should be the middle of the screen
        let _ = rl.get_world_to_screen(Vector3::zero(), &c);
    }

    #[test]
    fn test_timing_functions() {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.set_target_fps(24);
        let _fps = rl.get_fps();
        rl.get_frame_time();
        rl.get_time();
    }

    #[test]
    fn test_window_ops() {
        // Call twice to make sure multiple calls won't panic
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        // double hide double show
        rl.hide_window();
        rl.hide_window();
        // TODO uncomment this when we can draw a frame
        // assert!(rl.is_window_hidden(), "window is not hidden!");

        rl.unhide_window();
        rl.unhide_window();
        // assert!(!rl.is_window_hidden(), "window is hidden!");
    }

    ray_test!(test_set_window_name);
    fn test_set_window_name(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        rl.set_window_title(thread, "raylib test");
        assert_eq!(
            rl.get_screen_width(),
            TEST_WIDTH,
            "screen width is not the expected size!"
        );
        assert_eq!(
            rl.get_screen_height(),
            TEST_HEIGHT,
            "screen height is not the expected size!"
        );
    }

    // #[test]
    fn test_cursor() {
        // Call twice to make sure multiple calls won't panic
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        // double hide double show
        rl.hide_cursor();
        rl.hide_cursor();
        // TODO uncomment this when we can draw a frame
        // assert!(rl.is_cursor_hidden(), "window is not hidden!");

        rl.show_cursor();
        rl.show_cursor();
        // assert!(!rl.is_cursor_hidden(), "window is hidden!");

        rl.disable_cursor();
        rl.disable_cursor();
        rl.enable_cursor();
        rl.enable_cursor();
    }
}
