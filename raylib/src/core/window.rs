//! Window manipulation functions
use crate::core::math::{Matrix, Ray, Vector2};
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::{CStr, CString, IntoStringError, NulError};

// MonitorInfo grabs the sizes (virtual and physical) of your monitor
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
/// use raylib::prelude::*;
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
/// use raylib::prelude::*;
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

    /// Get the raw window Handle
    #[inline]
    pub unsafe fn get_window_handle(&mut self) -> *mut ::std::os::raw::c_void {
        ffi::GetWindowHandle()
    }
}
