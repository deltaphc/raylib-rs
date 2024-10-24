//! Window manipulation functions
use crate::core::math::{Matrix, Ray, Vector2};
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::{CStr, CString, IntoStringError, NulError};
use std::os::raw::c_char;

#[cfg(not(feature = "with_serde"))]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "with_serde")]
#[cfg(not(feature = "serde"))]
use serde::{Deserialize, Serialize};

#[cfg(feature = "with_serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// MonitorInfo grabs the sizes (virtual and physical) of your monitor
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonitorInfo {
    pub width: i32,
    pub height: i32,
    pub physical_width: i32,
    pub physical_height: i32,
    pub name: String,
    pub position: Vector2,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WindowState(i32);

impl WindowState {
    pub fn vsync_hint(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_VSYNC_HINT as i32) != 0
    }
    /// Set to try enabling V-Sync on GPU
    pub fn set_vsync_hint(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_VSYNC_HINT as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_VSYNC_HINT as i32);
        }
        self
    }

    pub fn fullscreen_mode(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_FULLSCREEN_MODE as i32) != 0
    }
    /// Set to run program in fullscreen
    pub fn set_fullscreen_mode(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_FULLSCREEN_MODE as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_FULLSCREEN_MODE as i32);
        }
        self
    }

    pub fn window_resizable(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as i32) != 0
    }
    /// Set to allow resizable window
    pub fn set_window_resizable(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as i32);
        }
        self
    }

    pub fn window_undecorated(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_UNDECORATED as i32) != 0
    }
    /// Set to disable window decoration (frame and buttons)
    pub fn set_window_undecorated(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_UNDECORATED as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_UNDECORATED as i32);
        }
        self
    }

    pub fn window_hidden(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_HIDDEN as i32) != 0
    }
    /// Set to hide window
    pub fn set_window_hidden(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_HIDDEN as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_HIDDEN as i32);
        }
        self
    }

    pub fn window_minimized(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_MINIMIZED as i32) != 0
    }
    /// Set to minimize window (iconify)
    pub fn set_window_minimized(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_MINIMIZED as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_MINIMIZED as i32);
        }
        self
    }

    pub fn window_maximized(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_MAXIMIZED as i32) != 0
    }
    /// Set to maximize window (expanded to monitor)
    pub fn set_window_maximized(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_MAXIMIZED as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_MAXIMIZED as i32);
        }
        self
    }

    pub fn window_unfocused(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_UNFOCUSED as i32) != 0
    }
    /// Set to window non focused
    pub fn set_window_unfocused(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_UNFOCUSED as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_UNFOCUSED as i32);
        }
        self
    }

    pub fn window_topmost(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_TOPMOST as i32) != 0
    }
    /// Set to window always on top
    pub fn set_window_topmost(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_TOPMOST as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_TOPMOST as i32);
        }
        self
    }

    pub fn window_always_run(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_ALWAYS_RUN as i32) != 0
    }
    /// Set to allow windows running while minimized
    pub fn set_window_always_run(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_ALWAYS_RUN as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_ALWAYS_RUN as i32);
        }
        self
    }

    pub fn window_transparent(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_TRANSPARENT as i32) != 0
    }
    /// Set to allow transparent framebuffer
    pub fn set_window_transparent(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_TRANSPARENT as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_TRANSPARENT as i32);
        }
        self
    }

    pub fn window_highdpi(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as i32) != 0
    }
    /// Set to support HighDPI
    pub fn set_window_highdpi(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as i32);
        }
        self
    }

    pub fn msaa(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_MSAA_4X_HINT as i32) != 0
    }
    /// Set to try enabling MSAA 4X
    pub fn set_msaa(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_MSAA_4X_HINT as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_MSAA_4X_HINT as i32);
        }
        self
    }

    pub fn interlaced_hint(&self) -> bool {
        self.0 & (ffi::ConfigFlags::FLAG_INTERLACED_HINT as i32) != 0
    }
    /// Set to try enabling interlaced video format (for V3D)
    pub fn set_interlaced_hint(mut self, enabled: bool) -> Self {
        if enabled {
            // set the bit
            self.0 |= ffi::ConfigFlags::FLAG_INTERLACED_HINT as i32;
        } else {
            // enable the bit
            self.0 &= !(ffi::ConfigFlags::FLAG_INTERLACED_HINT as i32);
        }
        self
    }
}

/// Get number of connected monitors
#[inline]
pub fn get_monitor_count() -> i32 {
    unsafe { ffi::GetMonitorCount() }
}

// Get current connected monitor
#[inline]
pub fn get_current_monitor() -> i32 {
    unsafe { ffi::GetCurrentMonitor() }
}

// Get current connected monitor
#[inline]
pub fn get_current_monitor_index() -> i32 {
    unsafe { ffi::GetCurrentMonitor() }
}

/// Get specified monitor refresh rate
#[inline]
pub fn get_monitor_refresh_rate(monitor: i32) -> i32 {
    debug_assert!(
        monitor < get_monitor_count() && monitor >= 0,
        "monitor index out of range"
    );

    unsafe { ffi::GetMonitorRefreshRate(monitor) }
}

/// Get width of monitor
/// Only checks that monitor index is in range in debug mode
#[inline]
pub fn get_monitor_width(monitor: i32) -> i32 {
    let len = get_monitor_count();
    debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");

    unsafe { ffi::GetMonitorWidth(monitor) }
}

/// Get height of monitor
/// Only checks that monitor index is in range in debug mode
#[inline]
pub fn get_monitor_height(monitor: i32) -> i32 {
    let len = get_monitor_count();
    debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");

    unsafe { ffi::GetMonitorHeight(monitor) }
}

/// Get physical width of monitor
/// Only checks that monitor index is in range in debug mode
#[inline]
pub fn get_monitor_physical_width(monitor: i32) -> i32 {
    let len = get_monitor_count();
    debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");

    unsafe { ffi::GetMonitorPhysicalWidth(monitor) }
}

/// Get physical height of monitor
/// Only checks that monitor index is in range in debug mode
#[inline]
pub fn get_monitor_physical_height(monitor: i32) -> i32 {
    let len = get_monitor_count();
    debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");

    unsafe { ffi::GetMonitorPhysicalHeight(monitor) }
}

/// Get name of monitor
/// Only checks that monitor index is in range in debug mode
#[inline]
pub fn get_monitor_name(monitor: i32) -> Result<String, IntoStringError> {
    let len = get_monitor_count();
    debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");

    Ok(unsafe {
        let c = CString::from_raw(ffi::GetMonitorName(monitor) as *mut c_char);
        c.into_string()?
    })
}

/// Get position of monitor
/// Only checks that monitor index is in range in debug mode
#[inline]
pub fn get_monitor_position(monitor: i32) -> Vector2 {
    let len = get_monitor_count();
    debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");

    unsafe { ffi::GetMonitorPosition(monitor).into() }
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
pub fn get_monitor_info(monitor: i32) -> Result<MonitorInfo, IntoStringError> {
    let len = get_monitor_count();
    debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");

    Ok(MonitorInfo {
        width: get_monitor_width(monitor),
        height: get_monitor_height(monitor),
        physical_height: get_monitor_physical_height(monitor),
        physical_width: get_monitor_physical_width(monitor),
        name: get_monitor_name(monitor)?,
        position: get_monitor_position(monitor),
    })
}

/// Returns camera transform matrix (view matrix)
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let c = Camera::perspective(
///            Vector3::zero(),
///            Vector3::new(0.0, 0.0, -1.0),
///            Vector3::up(),
///            90.0,
///        );
///        let m = get_camera_matrix(&c);
///        assert_eq!(m, Matrix::identity());
/// }
/// ```
pub fn get_camera_matrix(camera: impl Into<ffi::Camera>) -> Matrix {
    unsafe { ffi::GetCameraMatrix(camera.into()).into() }
}

/// Returns camera 2D transform matrix (view matrix)
/// ```rust
/// use raylib::prelude::*;
/// fn main() {
///     let c = Camera2D::default();
///     let m = get_camera_matrix2D(&c);
///     let mut check = Matrix::zero();
///     check.m10 = 1.0;
///     check.m15 = 1.0;
///     assert_eq!(m, check);
/// }
/// ```
#[allow(non_snake_case)]
pub fn get_camera_matrix2D(camera: impl Into<ffi::Camera2D>) -> Matrix {
    unsafe { ffi::GetCameraMatrix2D(camera.into()).into() }
}

impl RaylibHandle {
    /// Get clipboard text content
    pub fn get_clipboard_text(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let c = ffi::GetClipboardText();
            let c = CStr::from_ptr(c as *mut c_char);
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
    /// Get a ray trace from screen position (i.e mouse)
    pub fn get_screen_to_world_ray(
        &self,
        mouse_position: impl Into<ffi::Vector2>,
        camera: impl Into<ffi::Camera>,
    ) -> Ray {
        unsafe { ffi::GetScreenToWorldRay(mouse_position.into(), camera.into()).into() }
    }

    /// Get a ray trace from screen position (i.e mouse) in a viewport
    pub fn get_screen_to_world_ray_ex(
        &self,
        mouse_position: impl Into<ffi::Vector2>,
        camera: impl Into<ffi::Camera>,
        width: i32,
        height: i32,
    ) -> Ray {
        unsafe {
            ffi::GetScreenToWorldRayEx(mouse_position.into(), camera.into(), width, height).into()
        }
    }

    /// Returns the screen space position for a 3d world space position
    pub fn get_world_to_screen(
        &self,
        position: impl Into<ffi::Vector3>,
        camera: impl Into<ffi::Camera>,
    ) -> Vector2 {
        unsafe { ffi::GetWorldToScreen(position.into(), camera.into()).into() }
    }

    /// Returns the screen space position for a 2d camera world space position
    #[allow(non_snake_case)]
    pub fn get_world_to_screen2D(
        &self,
        position: impl Into<ffi::Vector2>,
        camera: impl Into<ffi::Camera2D>,
    ) -> Vector2 {
        unsafe { ffi::GetWorldToScreen2D(position.into(), camera.into()).into() }
    }

    /// Returns size position for a 3d world space position
    pub fn get_world_to_screen_ex(
        &self,
        position: impl Into<ffi::Vector3>,
        camera: impl Into<ffi::Camera>,
        width: i32,
        height: i32,
    ) -> Vector2 {
        unsafe { ffi::GetWorldToScreenEx(position.into(), camera.into(), width, height).into() }
    }

    /// Returns the world space position for a 2d camera screen space position
    #[allow(non_snake_case)]
    pub fn get_screen_to_world2D(
        &self,
        position: impl Into<ffi::Vector2>,
        camera: impl Into<ffi::Camera2D>,
    ) -> Vector2 {
        unsafe { ffi::GetScreenToWorld2D(position.into(), camera.into()).into() }
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
    /// Do not call on web unless you are compiling with asyncify.
    #[inline]
    pub fn window_should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    /// Checks if window has been initialized successfully.
    #[inline]
    pub fn is_window_ready(&self) -> bool {
        unsafe { ffi::IsWindowReady() }
    }

    /// Set window state: maximized, if resizable
    #[inline]
    pub fn maximize_window(&mut self) {
        unsafe { ffi::MaximizeWindow() }
    }

    /// Set window state: minimized, if resizable
    #[inline]
    pub fn minimize_window(&mut self) {
        unsafe { ffi::MinimizeWindow() }
    }

    /// Set window state: not minimized/maximized
    #[inline]
    pub fn restore_window(&mut self) {
        unsafe { ffi::RestoreWindow() }
    }

    /// Check if window is currently maximized
    #[inline]
    pub fn is_window_maximized(&self) -> bool {
        unsafe { ffi::IsWindowMaximized() }
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
        unsafe { ffi::IsWindowHidden() }
    }

    /// Returns whether or not window is in fullscreen mode
    #[inline]
    pub fn is_window_fullscreen(&self) -> bool {
        unsafe { ffi::IsWindowFullscreen() }
    }

    // Check if window is currently focused (only PLATFORM_DESKTOP)
    #[inline]
    pub fn is_window_focused(&self) -> bool {
        unsafe { ffi::IsWindowFocused() }
    }

    /// Check if window is currently focused (only PLATFORM_DESKTOP)
    #[inline]
    pub fn get_window_scale_dpi(&self) -> Vector2 {
        unsafe { ffi::GetWindowScaleDPI().into() }
    }

    /// Check if cursor is on the current screen.
    #[inline]
    pub fn is_cursor_on_screen(&self) -> bool {
        unsafe { ffi::IsCursorOnScreen() }
    }

    /// Set mouse cursor
    #[inline]
    pub fn set_mouse_cursor(&self, cursor: crate::consts::MouseCursor) {
        unsafe { ffi::SetMouseCursor(cursor as i32) }
    }

    /// Toggles fullscreen mode (only on desktop platforms).
    #[inline]
    pub fn toggle_fullscreen(&mut self) {
        unsafe {
            ffi::ToggleFullscreen();
        }
    }

    /// Set window configuration state using flags
    pub fn set_window_state(&mut self, state: WindowState) {
        unsafe { ffi::SetWindowState(state.0 as u32) }
    }

    /// Clear window configuration state flags
    pub fn clear_window_state(&mut self, state: WindowState) {
        unsafe { ffi::ClearWindowState(state.0 as u32) }
    }

    /// Get the window config state
    pub fn get_window_state(&self) -> WindowState {
        let state = WindowState::default();
        unsafe {
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_VSYNC_HINT as u32) {
                state.set_vsync_hint(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_FULLSCREEN_MODE as u32) {
                state.set_fullscreen_mode(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32) {
                state.set_window_resizable(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_UNDECORATED as u32) {
                state.set_window_undecorated(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_HIDDEN as u32) {
                state.set_window_hidden(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_MINIMIZED as u32) {
                state.set_window_minimized(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_MAXIMIZED as u32) {
                state.set_window_maximized(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_UNFOCUSED as u32) {
                state.set_window_unfocused(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_TOPMOST as u32) {
                state.set_window_topmost(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_ALWAYS_RUN as u32) {
                state.set_window_always_run(true);
            }

            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_TRANSPARENT as u32) {
                state.set_window_transparent(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32) {
                state.set_window_highdpi(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_MSAA_4X_HINT as u32) {
                state.set_msaa(true);
            }
            if ffi::IsWindowState(ffi::ConfigFlags::FLAG_INTERLACED_HINT as u32) {
                state.set_interlaced_hint(true);
            }
        }
        state
    }

    /// Sets icon for window (only on desktop platforms).
    #[inline]
    pub fn set_window_icon(&mut self, image: impl AsRef<ffi::Image>) {
        unsafe {
            ffi::SetWindowIcon(*image.as_ref());
        }
    }

    #[inline]
    pub fn set_window_icons(&mut self, images: &mut [raylib_sys::Image]) {
        use std::convert::TryInto;
        unsafe { ffi::SetWindowIcons(images.as_mut_ptr(), images.len().try_into().unwrap()) }
    }

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
        let len = get_monitor_count();
        debug_assert!(monitor < len && monitor >= 0, "monitor index out of range");
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

    /// Sets maximum window dimensions (for `FLAG_WINDOW_RESIZABLE`).
    #[inline]
    pub fn set_window_max_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::SetWindowMaxSize(width, height);
        }
    }

    /// Sets window dimensions.
    #[inline]
    pub fn set_window_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::SetWindowSize(width, height);
        }
    }

    /// Set window opacity, value opacity is between 0.0 and 1.0
    #[inline]
    pub fn set_window_opacity(&mut self, opacity: f32) {
        unsafe { ffi::SetWindowOpacity(opacity) }
    }

    /// Get current render width which is equal to screen width * dpi scale
    #[inline]
    pub fn get_render_width(&self) -> i32 {
        unsafe { ffi::GetRenderWidth() }
    }

    /// Get current render width which is equal to screen height * dpi scale
    #[inline]
    pub fn get_render_height(&self) -> i32 {
        unsafe { ffi::GetRenderHeight() }
    }

    /// Get current screen height which is equal to screen height * dpi scale
    #[inline]
    pub fn get_screen_width(&self) -> i32 {
        unsafe { ffi::GetScreenWidth() }
    }

    /// Gets current screen height.
    #[inline]
    pub fn get_screen_height(&self) -> i32 {
        unsafe { ffi::GetScreenHeight() }
    }

    /// Get window position
    #[inline]
    pub fn get_window_position(&self) -> Vector2 {
        unsafe { ffi::GetWindowPosition().into() }
    }

    // Toggle window state: borderless windowed (only on desktop platforms).
    pub fn toggle_borderless_windowed(&self) {
        unsafe { ffi::ToggleBorderlessWindowed() }
    }

    // Focus the window (only on desktop platforms)
    pub fn set_window_focused(&self) {
        unsafe { ffi::SetWindowFocused() }
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

    /// Get native window handle
    #[inline]
    pub unsafe fn get_window_handle(&mut self) -> *mut ::std::os::raw::c_void {
        ffi::GetWindowHandle()
    }
}

// Advanced "frame control" functions.
impl RaylibHandle {
    #[cfg(feature = "custom_frame_control")]
    /// Swap back buffer with front buffer (screen drawing)
    /// This function, by default, is already done when the handle is dropped.
    pub fn swap_screen_buffer(&self) {
        unsafe { ffi::SwapScreenBuffer() }
    }

    #[cfg(feature = "custom_frame_control")]
    pub fn poll_input_events(&self) {
        unsafe { ffi::PollInputEvents() }
    }

    #[cfg(feature = "custom_frame_control")]
    pub fn wait_time(&self, seconds: f64) {
        unsafe { ffi::WaitTime(seconds) }
    }
}
