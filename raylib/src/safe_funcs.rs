// /* raylib-rs
//    safe_funcs.rs - Safe versions of raylib functions

// Copyright (c) 2018-2019 Paul Clement (@deltaphc)

// This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

// Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

//   1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

//   2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

//   3. This notice may not be removed or altered from any source distribution.
// */

// use crate::ffi;
// use crate::ffi::{CharInfo, Rectangle};
// use crate::raymath::*;
// use crate::safe_types::*;
// use lazy_static::lazy_static;
// use std::ffi::{CStr, CString};
// use std::mem;
// use std::sync::atomic::{AtomicBool, Ordering};

// /// A convenience function for making a new `Color` from RGB values.
// #[inline]
// pub fn rgb(r: u8, g: u8, b: u8) -> Color {
//     Color { r, g, b, a: 255 }
// }

// /// A convenience function for making a new `Color` from RGBA values.
// #[inline]
// pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
//     Color { r, g, b, a }
// }

// /// A marker trait specifying an audio sample (`u8`, `i16`, or `f32`).
// pub trait AudioSample {}
// impl AudioSample for u8 {}
// impl AudioSample for i16 {}
// impl AudioSample for f32 {}

// static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

// lazy_static! {
//     static ref FONT_DEFAULT: Font = { unsafe { Font(ffi::GetFontDefault()) } };
// }

// lazy_static! {
//     static ref MATERIAL_DEFAULT: Material = { unsafe { Material(ffi::LoadMaterialDefault()) } };
// }

// lazy_static! {
//     static ref SHADER_DEFAULT: Shader = { unsafe { Shader(ffi::GetShaderDefault()) } };
// }

// lazy_static! {
//     static ref TEXTURE_DEFAULT: Texture2D = { unsafe { Texture2D(ffi::GetTextureDefault()) } };
// }

// /// A builder that allows more customization of the game window shown to the user before the `RaylibHandle` is created.
// #[derive(Debug, Default)]
// pub struct RaylibBuilder {
//     show_logo: bool,
//     fullscreen_mode: bool,
//     window_resizable: bool,
//     window_undecorated: bool,
//     window_transparent: bool,
//     msaa_4x_hint: bool,
//     vsync_hint: bool,
//     width: i32,
//     height: i32,
//     title: String,
// }

// impl RaylibBuilder {
//     /// Shows the raylib logo at startup.
//     pub fn with_logo(&mut self) -> &mut Self {
//         self.show_logo = true;
//         self
//     }

//     /// Sets the window to be fullscreen.
//     pub fn fullscreen(&mut self) -> &mut Self {
//         self.fullscreen_mode = true;
//         self
//     }

//     /// Sets the window to be resizable.
//     pub fn resizable(&mut self) -> &mut Self {
//         self.window_resizable = true;
//         self
//     }

//     /// Sets the window to be undecorated (without a border).
//     pub fn undecorated(&mut self) -> &mut Self {
//         self.window_undecorated = true;
//         self
//     }

//     /// Sets the window to be transparent.
//     pub fn transparent(&mut self) -> &mut Self {
//         self.window_transparent = true;
//         self
//     }

//     /// Hints that 4x MSAA (anti-aliasing) should be enabled. The system's graphics drivers may override this setting.
//     pub fn msaa_4x(&mut self) -> &mut Self {
//         self.msaa_4x_hint = true;
//         self
//     }

//     /// Hints that vertical sync (VSync) should be enabled. The system's graphics drivers may override this setting.
//     pub fn vsync(&mut self) -> &mut Self {
//         self.vsync_hint = true;
//         self
//     }

//     /// Sets the window's width.
//     pub fn width(&mut self, w: i32) -> &mut Self {
//         self.width = w;
//         self
//     }

//     /// Sets the window's height.
//     pub fn height(&mut self, h: i32) -> &mut Self {
//         self.height = h;
//         self
//     }

//     /// Sets the window's width and height.
//     pub fn size(&mut self, w: i32, h: i32) -> &mut Self {
//         self.width = w;
//         self.height = h;
//         self
//     }

//     /// Sets the window title.
//     pub fn title(&mut self, text: &str) -> &mut Self {
//         self.title = text.to_string();
//         self
//     }

//     /// Builds and initializes a Raylib window.
//     ///
//     /// # Panics
//     ///
//     /// Attempting to initialize Raylib more than once will result in a panic.
//     pub fn build(&self) -> RaylibHandle {
//         use crate::consts::ConfigFlag::*;
//         let mut flags = 0u32;
//         if self.show_logo {
//             flags |= FLAG_SHOW_LOGO as u32;
//         }
//         if self.fullscreen_mode {
//             flags |= FLAG_FULLSCREEN_MODE as u32;
//         }
//         if self.window_resizable {
//             flags |= FLAG_WINDOW_RESIZABLE as u32;
//         }
//         if self.window_undecorated {
//             flags |= FLAG_WINDOW_UNDECORATED as u32;
//         }
//         if self.window_transparent {
//             flags |= FLAG_WINDOW_TRANSPARENT as u32;
//         }
//         if self.msaa_4x_hint {
//             flags |= FLAG_MSAA_4X_HINT as u32;
//         }
//         if self.vsync_hint {
//             flags |= FLAG_VSYNC_HINT as u32;
//         }

//         unsafe {
//             ffi::SetConfigFlags(flags as u8);
//         }
//         init_window(self.width, self.height, &self.title)
//     }
// }

// /// Enables trace log message types (bit flags based).
// #[inline]
// pub fn set_trace_log(types: Log) {
//     unsafe {
//         ffi::SetTraceLogLevel(types as i32);
//     }
// }

// /// Writes a trace log message (`Log::INFO`, `Log::WARNING`, `Log::ERROR`, `Log::DEBUG`).
// #[inline]
// pub fn trace_log(msg_type: Log, text: &str) {
//     unsafe {
//         let text = CString::new(text).unwrap();
//         ffi::TraceLog(msg_type as i32, text.as_ptr());
//     }
// }

// /// The main interface into the Raylib API.
// ///
// /// This is the way in which you will use the vast majority of Raylib's functionality. A `RaylibHandle` can be constructed using the [`init_window`] function or through a [`RaylibBuilder`] obtained with the [`init`] function.
// ///
// /// [`init_window`]: fn.init_window.html
// /// [`RaylibBuilder`]: struct.RaylibBuilder.html
// /// [`init`]: fn.init.html
// pub struct RaylibHandle(()); // inner field is private, preventing manual construction

// /// Creates a `RaylibBuilder` for choosing window options before initialization.
// pub fn init() -> RaylibBuilder {
//     RaylibBuilder {
//         width: 640,
//         height: 480,
//         title: "raylib-rs".to_string(),
//         ..Default::default()
//     }
// }

// /// Initializes window and OpenGL context.
// ///
// /// # Panics
// ///
// /// Attempting to initialize Raylib more than once will result in a panic.
// pub fn init_window(width: i32, height: i32, title: &str) -> RaylibHandle {
//     if IS_INITIALIZED.load(Ordering::Relaxed) {
//         panic!("Attempted to initialize raylib-rs more than once");
//     } else {
//         unsafe {
//             let c_title = CString::new(title).unwrap();
//             ffi::InitWindow(width, height, c_title.as_ptr());
//         }
//         IS_INITIALIZED.store(true, Ordering::Relaxed);
//         RaylibHandle(())
//     }
// }

// impl Drop for RaylibHandle {
//     fn drop(&mut self) {
//         if IS_INITIALIZED.load(Ordering::Relaxed) {
//             unsafe {
//                 ffi::CloseWindow();
//             }
//             IS_INITIALIZED.store(false, Ordering::Relaxed);
//         }
//     }
// }

// impl RaylibHandle {
//     /// Checks if window has been initialized successfully.
//     #[inline]
//     pub fn is_window_ready(&self) -> bool {
//         unsafe { ffi::IsWindowReady() }
//     }

//     /// Checks if `KEY_ESCAPE` or Close icon was pressed.
//     #[inline]
//     pub fn window_should_close(&self) -> bool {
//         unsafe { ffi::WindowShouldClose() }
//     }

//     /// Checks if window has been minimized (or lost focus).
//     #[inline]
//     pub fn is_window_minimized(&self) -> bool {
//         unsafe { ffi::IsWindowMinimized() }
//     }

//     /// Toggles fullscreen mode (only on desktop platforms).
//     #[inline]
//     pub fn toggle_fullscreen(&self) {
//         unsafe {
//             ffi::ToggleFullscreen();
//         }
//     }

//     /// Sets icon for window (only on desktop platforms).
//     #[inline]
//     pub fn set_window_icon(&self, image: &Image) {
//         unsafe {
//             ffi::SetWindowIcon(image.0);
//         }
//     }

//     /// Sets title for window (only on desktop platforms).
//     #[inline]
//     pub fn set_window_title(&self, title: &str) {
//         let c_title = CString::new(title).unwrap();
//         unsafe {
//             ffi::SetWindowTitle(c_title.as_ptr());
//         }
//     }

//     /// Sets window position on screen (only on desktop platforms).
//     #[inline]
//     pub fn set_window_position(&self, x: i32, y: i32) {
//         unsafe {
//             ffi::SetWindowPosition(x, y);
//         }
//     }

//     /// Sets monitor for the current window (fullscreen mode).
//     #[inline]
//     pub fn set_window_monitor(&self, monitor: i32) {
//         unsafe {
//             ffi::SetWindowMonitor(monitor);
//         }
//     }

//     /// Sets minimum window dimensions (for `FLAG_WINDOW_RESIZABLE`).
//     #[inline]
//     pub fn set_window_min_size(&self, width: i32, height: i32) {
//         unsafe {
//             ffi::SetWindowMinSize(width, height);
//         }
//     }

//     /// Sets window dimensions.
//     #[inline]
//     pub fn set_window_size(&self, width: i32, height: i32) {
//         unsafe {
//             ffi::SetWindowSize(width, height);
//         }
//     }

//     /// Gets current screen width.
//     #[inline]
//     pub fn get_screen_width(&self) -> i32 {
//         unsafe { ffi::GetScreenWidth() }
//     }

//     /// Gets current screen height.
//     #[inline]
//     pub fn get_screen_height(&self) -> i32 {
//         unsafe { ffi::GetScreenHeight() }
//     }

//     /// Shows mouse cursor.
//     #[inline]
//     pub fn show_cursor(&self) {
//         unsafe {
//             ffi::ShowCursor();
//         }
//     }

//     /// Hides mouse cursor.
//     #[inline]
//     pub fn hide_cursor(&self) {
//         unsafe {
//             ffi::HideCursor();
//         }
//     }

//     /// Checks if mouse cursor is not visible.
//     #[inline]
//     pub fn is_cursor_hidden(&self) -> bool {
//         unsafe { ffi::IsCursorHidden() }
//     }

//     /// Enables mouse cursor (unlock cursor).
//     #[inline]
//     pub fn enable_cursor(&self) {
//         unsafe {
//             ffi::EnableCursor();
//         }
//     }

//     /// Disables mouse cursor (lock cursor).
//     #[inline]
//     pub fn disable_cursor(&self) {
//         unsafe {
//             ffi::DisableCursor();
//         }
//     }

//     /// Sets background color (framebuffer clear color).
//     #[inline]
//     pub fn clear_background(&self, color: impl Into<Color>) {
//         unsafe {
//             ffi::ClearBackground(color.into().into());
//         }
//     }

//     /// Sets up canvas (framebuffer) to start drawing.
//     #[inline]
//     pub fn begin_drawing(&self) {
//         unsafe {
//             ffi::BeginDrawing();
//         }
//     }

//     /// Ends canvas drawing and swaps buffers (double buffering).
//     #[inline]
//     pub fn end_drawing(&self) {
//         unsafe {
//             ffi::EndDrawing();
//         }
//     }

//     /// Initializes 2D mode with custom camera (2D).
//     #[inline]
//     pub fn begin_mode_2d(&self, camera: Camera2D) {
//         unsafe {
//             ffi::BeginMode2D(camera.into());
//         }
//     }

//     /// Ends 2D mode with custom camera.
//     #[inline]
//     pub fn end_mode_2d(&self) {
//         unsafe {
//             ffi::EndMode2D();
//         }
//     }

//     /// Initializes 3D mode with custom camera (3D).
//     #[inline]
//     pub fn begin_mode_3d(&self, camera: Camera3D) {
//         unsafe {
//             ffi::BeginMode3D(camera.into());
//         }
//     }

//     /// Ends 3D mode and returns to default 2D orthographic mode.
//     #[inline]
//     pub fn end_mode_3d(&self) {
//         unsafe {
//             ffi::EndMode3D();
//         }
//     }

//     /// Initializes render texture for drawing.
//     #[inline]
//     pub fn begin_texture_mode(&self, target: &RenderTexture2D) {
//         unsafe {
//             ffi::BeginTextureMode(target.0);
//         }
//     }

//     /// Ends drawing to render texture.
//     #[inline]
//     pub fn end_texture_mode(&self) {
//         unsafe {
//             ffi::EndTextureMode();
//         }
//     }

//     /// Returns a ray trace from mouse position.
//     #[inline]
//     pub fn get_mouse_ray(&self, mouse_position: impl Into<Vector2>, camera: Camera3D) -> Ray {
//         unsafe { ffi::GetMouseRay(mouse_position.into().into(), camera.into()).into() }
//     }

//     /// Returns the screen space position for a 3D world space position.
//     #[inline]
//     pub fn get_world_to_screen(&self, position: impl Into<Vector3>, camera: Camera3D) -> Vector2 {
//         unsafe { ffi::GetWorldToScreen(position.into().into(), camera.into()).into() }
//     }

//     /// Returns camera transform matrix (view matrix).
//     #[inline]
//     pub fn get_camera_matrix(&self, camera: Camera3D) -> Matrix {
//         unsafe { ffi::GetCameraMatrix(camera.into()).into() }
//     }

//     /// Sets target FPS (maximum).
//     #[inline]
//     pub fn set_target_fps(&self, fps: i32) {
//         unsafe {
//             ffi::SetTargetFPS(fps);
//         }
//     }

//     /// Returns current FPS.
//     #[inline]
//     pub fn get_fps(&self) -> i32 {
//         unsafe { ffi::GetFPS() }
//     }

//     /// Returns time in seconds for last frame drawn.
//     #[inline]
//     pub fn get_frame_time(&self) -> f32 {
//         unsafe { ffi::GetFrameTime() }
//     }

//     /// Returns elapsed time in seconds since `init_window` was called.
//     #[inline]
//     pub fn get_time(&self) -> f64 {
//         unsafe { ffi::GetTime() }
//     }

//     /// Returns hexadecimal value for a Color.
//     #[inline]
//     pub fn color_to_int(&self, color: impl Into<Color>) -> i32 {
//         unsafe { ffi::ColorToInt(color.into().into()) }
//     }

//     /// Returns color normalized as `f32` [0..1].
//     #[inline]
//     pub fn color_normalize(&self, color: impl Into<Color>) -> Vector4 {
//         unsafe { ffi::ColorNormalize(color.into().into()).into() }
//     }

//     /// Returns HSV values for a Color.
//     #[inline]
//     pub fn color_to_hsv(&self, color: impl Into<Color>) -> Vector3 {
//         unsafe { ffi::ColorToHSV(color.into().into()).into() }
//     }

//     /// Returns a Color struct from hexadecimal value.
//     #[inline]
//     pub fn get_color(&self, hex_value: i32) -> Color {
//         unsafe { ffi::GetColor(hex_value).into() }
//     }

//     /// Color fade-in or fade-out, `alpha` goes from `0.0` to `1.0`.
//     #[inline]
//     pub fn fade(&self, color: impl Into<Color>, alpha: f32) -> Color {
//         unsafe { ffi::Fade(color.into().into(), alpha).into() }
//     }

//     /// Takes a screenshot of current screen (in PNG format).
//     #[inline]
//     pub fn take_screenshot(&self, filename: &str) {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             ffi::TakeScreenshot(c_filename.as_ptr());
//         }
//     }

//     /// Returns a random value between min and max (both included).
//     #[inline]
//     pub fn get_random_value(&self, min: i32, max: i32) -> i32 {
//         unsafe { ffi::GetRandomValue(min, max) }
//     }

//     /// Checks if `filename` has an `ext` extension.
//     #[inline]
//     pub fn is_file_extension(&self, filename: &str, ext: &str) -> bool {
//         let c_filename = CString::new(filename).unwrap();
//         let c_ext = CString::new(ext).unwrap();
//         unsafe { ffi::IsFileExtension(c_filename.as_ptr(), c_ext.as_ptr()) }
//     }

//     /// Gets the extension for a `filename` string.
//     #[inline]
//     pub fn get_extension(&self, filename: &str) -> String {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             let ext = ffi::GetExtension(c_filename.as_ptr());
//             CStr::from_ptr(ext).to_str().unwrap().to_owned()
//         }
//     }

//     /// Gets the filename for a path string.
//     #[inline]
//     pub fn get_file_name(&self, file_path: &str) -> String {
//         let c_file_path = CString::new(file_path).unwrap();
//         unsafe {
//             let filename = ffi::GetFileName(c_file_path.as_ptr());
//             CStr::from_ptr(filename).to_str().unwrap().to_owned()
//         }
//     }

//     /// Gets full path for a given `filename`.
//     #[inline]
//     pub fn get_directory_path(&self, filename: &str) -> String {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             let dirpath = ffi::GetDirectoryPath(c_filename.as_ptr());
//             CStr::from_ptr(dirpath).to_str().unwrap().to_owned()
//         }
//     }

//     /// Gets current working directory.
//     #[inline]
//     pub fn get_working_directory(&self) -> String {
//         unsafe {
//             let workdir = ffi::GetWorkingDirectory();
//             CStr::from_ptr(workdir).to_str().unwrap().to_owned()
//         }
//     }

//     /// Changes working directory, returns true on success.
//     #[inline]
//     pub fn change_directory(&self, dir: &str) -> bool {
//         let c_dir = CString::new(dir).unwrap();
//         unsafe { ffi::ChangeDirectory(c_dir.as_ptr()) }
//     }

//     /// Checks if a file has been dropped into the window.
//     #[inline]
//     pub fn is_file_dropped(&self) -> bool {
//         unsafe { ffi::IsFileDropped() }
//     }

//     /// Gets dropped filenames.
//     #[inline]
//     pub fn get_dropped_files(&self) -> Vec<String> {
//         let mut v = Vec::new();
//         unsafe {
//             let mut count: i32 = 0;
//             let dropfiles = ffi::GetDroppedFiles(&mut count);
//             for i in 0..count {
//                 let filestr = CStr::from_ptr(*dropfiles.offset(i as isize))
//                     .to_str()
//                     .unwrap();
//                 let file = String::from(filestr);
//                 v.push(file);
//             }
//         }
//         v
//     }

//     /// Clears dropped files paths buffer.
//     #[inline]
//     pub fn clear_dropped_files(&self) {
//         unsafe {
//             ffi::ClearDroppedFiles();
//         }
//     }

//     /// Saves integer value to storage file (to defined `position`).
//     #[inline]
//     pub fn storage_save_value(&self, position: i32, value: i32) {
//         unsafe {
//             ffi::StorageSaveValue(position, value);
//         }
//     }

//     /// Loads integer value from storage file (from defined `position`).
//     #[inline]
//     pub fn storage_load_value(&self, position: i32) -> i32 {
//         unsafe { ffi::StorageLoadValue(position) }
//     }

//     /// Detect if a key has been pressed once.
//     #[inline]
//     pub fn is_key_pressed(&self, key: u32) -> bool {
//         unsafe { ffi::IsKeyPressed(key as i32) }
//     }

//     /// Detect if a key is being pressed.
//     #[inline]
//     pub fn is_key_down(&self, key: u32) -> bool {
//         unsafe { ffi::IsKeyDown(key as i32) }
//     }

//     /// Detect if a key has been released once.
//     #[inline]
//     pub fn is_key_released(&self, key: u32) -> bool {
//         unsafe { ffi::IsKeyReleased(key as i32) }
//     }

//     /// Detect if a key is NOT being pressed.
//     #[inline]
//     pub fn is_key_up(&self, key: u32) -> bool {
//         unsafe { ffi::IsKeyUp(key as i32) }
//     }

//     /// Gets latest key pressed.
//     #[inline]
//     pub fn get_key_pressed(&self) -> u32 {
//         unsafe { ffi::GetKeyPressed() as u32 }
//     }

//     /// Sets a custom key to exit program (default is ESC).
//     #[inline]
//     pub fn set_exit_key(&self, key: u32) {
//         unsafe {
//             ffi::SetExitKey(key as i32);
//         }
//     }

//     /// Detect if a gamepad is available.
//     #[inline]
//     pub fn is_gamepad_available(&self, gamepad: u32) -> bool {
//         unsafe { ffi::IsGamepadAvailable(gamepad as i32) }
//     }

//     /// Checks gamepad name (if available).
//     #[inline]
//     pub fn is_gamepad_name(&self, gamepad: u32, name: &str) -> bool {
//         let c_name = CString::new(name).unwrap();
//         unsafe { ffi::IsGamepadName(gamepad as i32, c_name.as_ptr()) }
//     }

//     /// Returns gamepad internal name id.
//     #[inline]
//     pub fn get_gamepad_name(&self, gamepad: u32) -> Option<String> {
//         unsafe {
//             let name = ffi::GetGamepadName(gamepad as i32);
//             match name.is_null() {
//                 false => Some(CStr::from_ptr(name).to_str().unwrap().to_owned()),
//                 true => None,
//             }
//         }
//     }

//     /// Detect if a gamepad button has been pressed once.
//     #[inline]
//     pub fn is_gamepad_button_pressed(&self, gamepad: u32, button: i32) -> bool {
//         unsafe { ffi::IsGamepadButtonPressed(gamepad as i32, button) }
//     }

//     /// Detect if a gamepad button is being pressed.
//     #[inline]
//     pub fn is_gamepad_button_down(&self, gamepad: u32, button: i32) -> bool {
//         unsafe { ffi::IsGamepadButtonDown(gamepad as i32, button) }
//     }

//     /// Detect if a gamepad button has been released once.
//     #[inline]
//     pub fn is_gamepad_button_released(&self, gamepad: u32, button: i32) -> bool {
//         unsafe { ffi::IsGamepadButtonReleased(gamepad as i32, button) }
//     }

//     /// Detect if a gamepad button is NOT being pressed.
//     #[inline]
//     pub fn is_gamepad_button_up(&self, gamepad: u32, button: i32) -> bool {
//         unsafe { ffi::IsGamepadButtonUp(gamepad as i32, button) }
//     }

//     /// Gets the last gamepad button pressed.
//     #[inline]
//     pub fn get_gamepad_button_pressed(&self) -> u32 {
//         unsafe { ffi::GetGamepadButtonPressed() as u32 }
//     }

//     /// Returns gamepad axis count for a gamepad.
//     #[inline]
//     pub fn get_gamepad_axis_count(&self, gamepad: u32) -> i32 {
//         unsafe { ffi::GetGamepadAxisCount(gamepad as i32) }
//     }

//     /// Returns axis movement value for a gamepad axis.
//     #[inline]
//     pub fn get_gamepad_axis_movement(&self, gamepad: u32, axis: u32) -> f32 {
//         unsafe { ffi::GetGamepadAxisMovement(gamepad as i32, axis as i32) }
//     }

//     /// Detect if a mouse button has been pressed once.
//     #[inline]
//     pub fn is_mouse_button_pressed(&self, button: u32) -> bool {
//         unsafe { ffi::IsMouseButtonPressed(button as i32) }
//     }

//     /// Detect if a mouse button is being pressed.
//     #[inline]
//     pub fn is_mouse_button_down(&self, button: u32) -> bool {
//         unsafe { ffi::IsMouseButtonDown(button as i32) }
//     }

//     /// Detect if a mouse button has been released once.
//     #[inline]
//     pub fn is_mouse_button_released(&self, button: u32) -> bool {
//         unsafe { ffi::IsMouseButtonReleased(button as i32) }
//     }

//     /// Detect if a mouse button is NOT being pressed.
//     #[inline]
//     pub fn is_mouse_button_up(&self, button: u32) -> bool {
//         unsafe { ffi::IsMouseButtonUp(button as i32) }
//     }

//     /// Returns mouse position X.
//     #[inline]
//     pub fn get_mouse_x(&self) -> i32 {
//         unsafe { ffi::GetMouseX() }
//     }

//     /// Returns mouse position Y.
//     #[inline]
//     pub fn get_mouse_y(&self) -> i32 {
//         unsafe { ffi::GetMouseY() }
//     }

//     /// Returns mouse position.
//     #[inline]
//     pub fn get_mouse_position(&self) -> Vector2 {
//         unsafe { ffi::GetMousePosition().into() }
//     }

//     /// Sets mouse position.
//     #[inline]
//     pub fn set_mouse_position(&self, position: impl Into<Vector2>) {
//         unsafe {
//             let Vector2 { x, y } = position.into();
//             ffi::SetMousePosition(x as i32, y as i32);
//         }
//     }

//     /// Sets mouse scaling.
//     #[inline]
//     pub fn set_mouse_scale(&self, scale_x: f32, scale_y: f32) {
//         unsafe {
//             ffi::SetMouseScale(scale_x, scale_y);
//         }
//     }

//     /// Returns mouse wheel movement Y.
//     #[inline]
//     pub fn get_mouse_wheel_move(&self) -> i32 {
//         unsafe { ffi::GetMouseWheelMove() }
//     }

//     /// Returns touch position X for touch point 0 (relative to screen size).
//     #[inline]
//     pub fn get_touch_x(&self) -> i32 {
//         unsafe { ffi::GetTouchX() }
//     }

//     /// Returns touch position Y for touch point 0 (relative to screen size).
//     #[inline]
//     pub fn get_touch_y(&self) -> i32 {
//         unsafe { ffi::GetTouchY() }
//     }

//     /// Returns touch position XY for a touch point index (relative to screen size).
//     #[inline]
//     pub fn get_touch_position(&self, index: u32) -> Vector2 {
//         unsafe { ffi::GetTouchPosition(index as i32).into() }
//     }

//     /// Enables a set of gestures using flags.
//     #[inline]
//     pub fn set_gestures_enabled(&self, gesture_flags: Gesture) {
//         unsafe {
//             ffi::SetGesturesEnabled(gesture_flags as u32);
//         }
//     }

//     /// Checks if a gesture have been detected.
//     #[inline]
//     pub fn is_gesture_detected(&self, gesture: Gesture) -> bool {
//         unsafe { ffi::IsGestureDetected(gesture as i32) }
//     }

//     /// Gets latest detected gesture.
//     #[inline]
//     pub fn get_gesture_detected(&self) -> Gesture {
//         unsafe { mem::transmute::<i32, Gesture>(ffi::GetGestureDetected()) }
//     }

//     /// Gets touch points count.
//     #[inline]
//     pub fn get_touch_points_count(&self) -> u32 {
//         unsafe { ffi::GetTouchPointsCount() as u32 }
//     }

//     /// Gets gesture hold time in milliseconds.
//     #[inline]
//     pub fn get_gesture_hold_duration(&self) -> f32 {
//         unsafe { ffi::GetGestureHoldDuration() }
//     }

//     /// Gets gesture drag vector.
//     #[inline]
//     pub fn get_gesture_drag_vector(&self) -> Vector2 {
//         unsafe { ffi::GetGestureDragVector().into() }
//     }

//     /// Gets gesture drag angle.
//     #[inline]
//     pub fn get_gesture_drag_angle(&self) -> f32 {
//         unsafe { ffi::GetGestureDragAngle() }
//     }

//     /// Gets gesture pinch delta.
//     #[inline]
//     pub fn get_gesture_pinch_vector(&self) -> Vector2 {
//         unsafe { ffi::GetGesturePinchVector().into() }
//     }

//     /// Gets gesture pinch angle.
//     #[inline]
//     pub fn get_gesture_pinch_angle(&self) -> f32 {
//         unsafe { ffi::GetGesturePinchAngle() }
//     }

//     /// Sets camera mode.
//     #[inline]
//     pub fn set_camera_mode(&self, camera: Camera3D, mode: ffi::CameraMode) {
//         unsafe {
//             ffi::SetCameraMode(camera.into(), mode as i32);
//         }
//     }

//     /// Updates camera position for selected mode.
//     #[inline]
//     pub fn update_camera(&self, camera: &mut Camera3D) {
//         unsafe {
//             let mut fficam: ffi::Camera3D = (*camera).into();
//             ffi::UpdateCamera(&mut fficam);
//             *camera = fficam.into();
//         }
//     }

//     /// Sets camera pan key to combine with mouse movement (free camera).
//     #[inline]
//     pub fn set_camera_pan_control(&self, pan_key: u32) {
//         unsafe {
//             ffi::SetCameraPanControl(pan_key as i32);
//         }
//     }

//     /// Sets camera alt key to combine with mouse movement (free camera).
//     #[inline]
//     pub fn set_camera_alt_control(&self, alt_key: u32) {
//         unsafe {
//             ffi::SetCameraAltControl(alt_key as i32);
//         }
//     }

//     /// Sets camera smooth zoom key to combine with mouse (free camera).
//     #[inline]
//     pub fn set_camera_smooth_zoom_control(&self, sz_key: u32) {
//         unsafe {
//             ffi::SetCameraSmoothZoomControl(sz_key as i32);
//         }
//     }

//     /// Sets camera move controls (1st person and 3rd person cameras).
//     #[inline]
//     pub fn set_camera_move_controls(
//         &self,
//         front_key: u32,
//         back_key: u32,
//         right_key: u32,
//         left_key: u32,
//         up_key: u32,
//         down_key: u32,
//     ) {
//         unsafe {
//             ffi::SetCameraMoveControls(
//                 front_key as i32,
//                 back_key as i32,
//                 right_key as i32,
//                 left_key as i32,
//                 up_key as i32,
//                 down_key as i32,
//             );
//         }
//     }

//     /// Draws a pixel.
//     #[inline]
//     pub fn draw_pixel(&self, x: i32, y: i32, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawPixel(x, y, color.into().into());
//         }
//     }

//     /// Draws a pixel (Vector version).
//     #[inline]
//     pub fn draw_pixel_v(&self, position: impl Into<Vector2>, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawPixelV(position.into().into(), color.into().into());
//         }
//     }

//     /// Draws a line.
//     #[inline]
//     pub fn draw_line(
//         &self,
//         start_pos_x: i32,
//         start_pos_y: i32,
//         end_pos_x: i32,
//         end_pos_y: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawLine(
//                 start_pos_x,
//                 start_pos_y,
//                 end_pos_x,
//                 end_pos_y,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a line (Vector version).
//     #[inline]
//     pub fn draw_line_v(
//         &self,
//         start_pos: impl Into<Vector2>,
//         end_pos: impl Into<Vector2>,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawLineV(
//                 start_pos.into().into(),
//                 end_pos.into().into(),
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a line with thickness.
//     #[inline]
//     pub fn draw_line_ex(
//         &self,
//         start_pos: impl Into<Vector2>,
//         end_pos: impl Into<Vector2>,
//         thick: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawLineEx(
//                 start_pos.into().into(),
//                 end_pos.into().into(),
//                 thick,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a line using cubic-bezier curves in-out.
//     #[inline]
//     pub fn draw_line_bezier(
//         &self,
//         start_pos: impl Into<Vector2>,
//         end_pos: impl Into<Vector2>,
//         thick: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawLineBezier(
//                 start_pos.into().into(),
//                 end_pos.into().into(),
//                 thick,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a color-filled circle.
//     #[inline]
//     pub fn draw_circle(&self, center_x: i32, center_y: i32, radius: f32, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawCircle(center_x, center_y, radius, color.into().into());
//         }
//     }

//     /// Draws a gradient-filled circle.
//     #[inline]
//     pub fn draw_circle_gradient(
//         &self,
//         center_x: i32,
//         center_y: i32,
//         radius: f32,
//         color1: impl Into<Color>,
//         color2: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCircleGradient(
//                 center_x,
//                 center_y,
//                 radius,
//                 color1.into().into(),
//                 color2.into().into(),
//             );
//         }
//     }

//     /// Draws a color-filled circle (Vector version).
//     #[inline]
//     pub fn draw_circle_v(&self, center: impl Into<Vector2>, radius: f32, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawCircleV(center.into().into(), radius, color.into().into());
//         }
//     }

//     /// Draws circle outline.
//     #[inline]
//     pub fn draw_circle_lines(
//         &self,
//         center_x: i32,
//         center_y: i32,
//         radius: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCircleLines(center_x, center_y, radius, color.into().into());
//         }
//     }

//     /// Draws a color-filled rectangle.
//     #[inline]
//     pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawRectangle(x, y, width, height, color.into().into());
//         }
//     }

//     /// Draws a color-filled rectangle (Vector version).
//     #[inline]
//     pub fn draw_rectangle_v(
//         &self,
//         position: impl Into<Vector2>,
//         size: impl Into<Vector2>,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawRectangleV(
//                 position.into().into(),
//                 size.into().into(),
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a color-filled rectangle from `rec`.
//     #[inline]
//     pub fn draw_rectangle_rec(&self, rec: Rectangle, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawRectangleRec(rec, color.into().into());
//         }
//     }

//     /// Draws a color-filled rectangle with pro parameters.
//     #[inline]
//     pub fn draw_rectangle_pro(
//         &self,
//         rec: Rectangle,
//         origin: impl Into<Vector2>,
//         rotation: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawRectanglePro(rec, origin.into().into(), rotation, color.into().into());
//         }
//     }

//     /// Draws a vertical-gradient-filled rectangle.
//     ///
//     /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
//     #[inline]
//     pub fn draw_rectangle_gradient_v(
//         &self,
//         x: i32,
//         y: i32,
//         width: i32,
//         height: i32,
//         color1: impl Into<Color>,
//         color2: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawRectangleGradientV(
//                 x,
//                 y,
//                 width,
//                 height,
//                 color1.into().into(),
//                 color2.into().into(),
//             );
//         }
//     }

//     /// Draws a horizontal-gradient-filled rectangle.
//     ///
//     /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
//     #[inline]
//     pub fn draw_rectangle_gradient_h(
//         &self,
//         x: i32,
//         y: i32,
//         width: i32,
//         height: i32,
//         color1: impl Into<Color>,
//         color2: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawRectangleGradientH(
//                 x,
//                 y,
//                 width,
//                 height,
//                 color1.into().into(),
//                 color2.into().into(),
//             );
//         }
//     }

//     /// Draws a gradient-filled rectangle with custom vertex colors.
//     ///
//     /// **NOTE**: Colors refer to corners, starting at top-left corner and going counter-clockwise.
//     #[inline]
//     pub fn draw_rectangle_gradient_ex(
//         &self,
//         rec: Rectangle,
//         col1: impl Into<Color>,
//         col2: impl Into<Color>,
//         col3: impl Into<Color>,
//         col4: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawRectangleGradientEx(
//                 rec,
//                 col1.into().into(),
//                 col2.into().into(),
//                 col3.into().into(),
//                 col4.into().into(),
//             );
//         }
//     }

//     /// Draws rectangle outline.
//     #[inline]
//     pub fn draw_rectangle_lines(
//         &self,
//         x: i32,
//         y: i32,
//         width: i32,
//         height: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawRectangleLines(x, y, width, height, color.into().into());
//         }
//     }

//     /// Draws rectangle outline with extended parameters.
//     #[inline]
//     pub fn draw_rectangle_lines_ex(
//         &self,
//         rec: Rectangle,
//         line_thick: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawRectangleLinesEx(rec, line_thick, color.into().into());
//         }
//     }

//     /// Draws a triangle.
//     #[inline]
//     pub fn draw_triangle(
//         &self,
//         v1: impl Into<Vector2>,
//         v2: impl Into<Vector2>,
//         v3: impl Into<Vector2>,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawTriangle(
//                 v1.into().into(),
//                 v2.into().into(),
//                 v3.into().into(),
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a triangle using lines.
//     #[inline]
//     pub fn draw_triangle_lines(
//         &self,
//         v1: impl Into<Vector2>,
//         v2: impl Into<Vector2>,
//         v3: impl Into<Vector2>,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawTriangleLines(
//                 v1.into().into(),
//                 v2.into().into(),
//                 v3.into().into(),
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a regular polygon of n sides (Vector version).
//     #[inline]
//     pub fn draw_poly(
//         &self,
//         center: impl Into<Vector2>,
//         sides: i32,
//         radius: f32,
//         rotation: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawPoly(
//                 center.into().into(),
//                 sides,
//                 radius,
//                 rotation,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Checks collision between two rectangles.
//     #[inline]
//     pub fn check_collision_recs(&self, rec1: Rectangle, rec2: Rectangle) -> bool {
//         unsafe { ffi::CheckCollisionRecs(rec1, rec2) }
//     }

//     /// Checks collision between two circles.
//     #[inline]
//     pub fn check_collision_circles(
//         &self,
//         center1: impl Into<Vector2>,
//         radius1: f32,
//         center2: impl Into<Vector2>,
//         radius2: f32,
//     ) -> bool {
//         unsafe {
//             ffi::CheckCollisionCircles(
//                 center1.into().into(),
//                 radius1,
//                 center2.into().into(),
//                 radius2,
//             )
//         }
//     }

//     /// Checks collision between circle and rectangle.
//     #[inline]
//     pub fn check_collision_circle_rec(
//         &self,
//         center: impl Into<Vector2>,
//         radius: f32,
//         rec: Rectangle,
//     ) -> bool {
//         unsafe { ffi::CheckCollisionCircleRec(center.into().into(), radius, rec) }
//     }

//     /// Gets the overlap between two colliding rectangles.
//     #[inline]
//     pub fn get_collision_rec(&self, rec1: Rectangle, rec2: Rectangle) -> Rectangle {
//         unsafe { ffi::GetCollisionRec(rec1, rec2) }
//     }

//     /// Checks if point is inside rectangle.
//     #[inline]
//     pub fn check_collision_point_rec(&self, point: impl Into<Vector2>, rec: Rectangle) -> bool {
//         unsafe { ffi::CheckCollisionPointRec(point.into().into(), rec) }
//     }

//     /// Checks if point is inside circle.
//     #[inline]
//     pub fn check_collision_point_circle(
//         &self,
//         point: impl Into<Vector2>,
//         center: impl Into<Vector2>,
//         radius: f32,
//     ) -> bool {
//         unsafe { ffi::CheckCollisionPointCircle(point.into().into(), center.into().into(), radius) }
//     }

//     /// Checks if point is inside a triangle.
//     #[inline]
//     pub fn check_collision_point_triangle(
//         &self,
//         point: impl Into<Vector2>,
//         p1: impl Into<Vector2>,
//         p2: impl Into<Vector2>,
//         p3: impl Into<Vector2>,
//     ) -> bool {
//         unsafe {
//             ffi::CheckCollisionPointTriangle(
//                 point.into().into(),
//                 p1.into().into(),
//                 p2.into().into(),
//                 p3.into().into(),
//             )
//         }
//     }

//     /// Loads image from file into CPU memory (RAM).
//     #[inline]
//     pub fn load_image(&self, filename: &str) -> Image {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe { Image(ffi::LoadImage(c_filename.as_ptr())) }
//     }

//     /// Loads image from Color array data (RGBA - 32bit).
//     #[inline]
//     pub fn load_image_ex(&self, pixels: &[Color], width: i32, height: i32) -> Image {
//         let expected_len = (width * height) as usize;
//         if pixels.len() != expected_len {
//             panic!(
//                 "load_image_ex: Data is wrong size. Expected {}, got {}",
//                 expected_len,
//                 pixels.len()
//             );
//         }
//         unsafe {
//             // An examination of Raylib source (textures.c) shows that it does not mutate the given pixels
//             Image(ffi::LoadImageEx(
//                 pixels.as_ptr() as *mut ffi::Color,
//                 width,
//                 height,
//             ))
//         }
//     }

//     /// Loads image from raw data with parameters.
//     #[inline]
//     pub fn load_image_pro(
//         &self,
//         data: &[u8],
//         width: i32,
//         height: i32,
//         format: ffi::PixelFormat,
//     ) -> Image {
//         let expected_len = self.get_pixel_data_size(width, height, format) as usize;
//         if data.len() != expected_len {
//             panic!(
//                 "load_image_pro: Data is wrong size. Expected {}, got {}",
//                 expected_len,
//                 data.len()
//             );
//         }
//         unsafe {
//             Image(ffi::LoadImagePro(
//                 data.as_ptr() as *mut std::os::raw::c_void,
//                 width,
//                 height,
//                 format as i32,
//             ))
//         }
//     }

//     /// Loads image from RAW file data.
//     #[inline]
//     pub fn load_image_raw(
//         &self,
//         filename: &str,
//         width: i32,
//         height: i32,
//         format: i32,
//         header_size: i32,
//     ) -> Image {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             Image(ffi::LoadImageRaw(
//                 c_filename.as_ptr(),
//                 width,
//                 height,
//                 format,
//                 header_size,
//             ))
//         }
//     }

//     /// Exports image as a PNG file.
//     #[inline]
//     pub fn export_image(&self, image: Image, filename: &str) {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             ffi::ExportImage(image.0, c_filename.as_ptr());
//         }
//     }

//     /// Loads texture from file into GPU memory (VRAM).
//     #[inline]
//     pub fn load_texture(&self, filename: &str) -> Texture2D {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe { Texture2D(ffi::LoadTexture(c_filename.as_ptr())) }
//     }

//     /// Loads texture from image data.
//     #[inline]
//     pub fn load_texture_from_image(&self, image: &Image) -> Texture2D {
//         unsafe { Texture2D(ffi::LoadTextureFromImage(image.0)) }
//     }

//     /// Loads texture for rendering (framebuffer).
//     #[inline]
//     pub fn load_render_texture(&self, width: i32, height: i32) -> RenderTexture2D {
//         unsafe { RenderTexture2D(ffi::LoadRenderTexture(width, height)) }
//     }

//     /// Gets pixel data from `image` as a Vec of Color structs.
//     #[inline]
//     pub fn get_image_data(&self, image: &Image) -> Vec<Color> {
//         unsafe {
//             let image_data = ffi::GetImageData(image.0);
//             let image_data_len = (image.width * image.height) as usize;
//             let mut safe_image_data: Vec<Color> = Vec::with_capacity(image_data_len);
//             safe_image_data.set_len(image_data_len);
//             std::ptr::copy(
//                 image_data,
//                 safe_image_data.as_mut_ptr() as *mut ffi::Color,
//                 image_data_len,
//             );
//             libc::free(image_data as *mut libc::c_void);
//             safe_image_data
//         }
//     }

//     /// Gets normalized (`0.0` to `1.0`) pixel data from `image` as a Vec of Vector4 structs.
//     #[inline]
//     pub fn get_image_data_normalized(&self, image: &Image) -> Vec<Vector4> {
//         unsafe {
//             let image_data = ffi::GetImageDataNormalized(image.0);
//             let image_data_len = (image.width * image.height) as usize;
//             let mut safe_image_data: Vec<Vector4> = Vec::with_capacity(image_data_len);
//             safe_image_data.set_len(image_data_len);
//             std::ptr::copy(
//                 image_data,
//                 safe_image_data.as_mut_ptr() as *mut ffi::Vector4,
//                 image_data_len,
//             );
//             libc::free(image_data as *mut libc::c_void);
//             safe_image_data
//         }
//     }

//     /// Gets pixel data size in bytes (image or texture).
//     #[inline]
//     pub fn get_pixel_data_size(&self, width: i32, height: i32, format: ffi::PixelFormat) -> i32 {
//         unsafe { ffi::GetPixelDataSize(width, height, format as i32) }
//     }

//     /// Gets pixel data from GPU texture and returns an `Image`.
//     #[inline]
//     pub fn get_texture_data(&self, texture: &Texture2D) -> Image {
//         unsafe { Image(ffi::GetTextureData(texture.0)) }
//     }

//     /// Updates GPU texture with new data.
//     #[inline]
//     pub fn update_texture(&self, texture: &mut Texture2D, pixels: &[u8]) {
//         let expected_len = unsafe {
//             self.get_pixel_data_size(
//                 texture.width,
//                 texture.height,
//                 mem::transmute::<i32, ffi::PixelFormat>(texture.format),
//             ) as usize
//         };
//         if pixels.len() != expected_len {
//             panic!(
//                 "update_texture: Data is wrong size. Expected {}, got {}",
//                 expected_len,
//                 pixels.len()
//             );
//         }
//         unsafe {
//             ffi::UpdateTexture(texture.0, pixels.as_ptr() as *const std::os::raw::c_void);
//         }
//     }

//     /// Creates an image duplicate (useful for transformations).
//     #[inline]
//     pub fn image_copy(&self, image: &Image) -> Image {
//         unsafe { Image(ffi::ImageCopy(image.0)) }
//     }

//     /// Converts `image` to POT (power-of-two).
//     #[inline]
//     pub fn image_to_pot(&self, image: &mut Image, fill_color: impl Into<Color>) {
//         unsafe {
//             ffi::ImageToPOT(&mut image.0, fill_color.into().into());
//         }
//     }

//     /// Converts `image` data to desired pixel format.
//     #[inline]
//     pub fn image_format(&self, image: &mut Image, new_format: ffi::PixelFormat) {
//         unsafe {
//             ffi::ImageFormat(&mut image.0, new_format as i32);
//         }
//     }

//     /// Applies alpha mask to `image`.
//     #[inline]
//     pub fn image_alpha_mask(&self, image: &mut Image, alpha_mask: &Image) {
//         unsafe {
//             ffi::ImageAlphaMask(&mut image.0, alpha_mask.0);
//         }
//     }

//     /// Clears alpha channel on `image` to desired color.
//     #[inline]
//     pub fn image_alpha_clear(&self, image: &mut Image, color: impl Into<Color>, threshold: f32) {
//         unsafe {
//             ffi::ImageAlphaClear(&mut image.0, color.into().into(), threshold);
//         }
//     }

//     /// Crops `image` depending on alpha value.
//     #[inline]
//     pub fn image_alpha_crop(&self, image: &mut Image, threshold: f32) {
//         unsafe {
//             ffi::ImageAlphaCrop(&mut image.0, threshold);
//         }
//     }

//     /// Premultiplies alpha channel on `image`.
//     #[inline]
//     pub fn image_alpha_premultiply(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageAlphaPremultiply(&mut image.0);
//         }
//     }

//     /// Crops `image` to a defined rectangle.
//     #[inline]
//     pub fn image_crop(&self, image: &mut Image, crop: Rectangle) {
//         unsafe {
//             ffi::ImageCrop(&mut image.0, crop);
//         }
//     }

//     /// Resizes `image` (bilinear filtering).
//     #[inline]
//     pub fn image_resize(&self, image: &mut Image, new_width: i32, new_height: i32) {
//         unsafe {
//             ffi::ImageResize(&mut image.0, new_width, new_height);
//         }
//     }

//     /// Resizes `image` (nearest-neighbor scaling).
//     #[inline]
//     pub fn image_resize_nn(&self, image: &mut Image, new_width: i32, new_height: i32) {
//         unsafe {
//             ffi::ImageResizeNN(&mut image.0, new_width, new_height);
//         }
//     }

//     /// Resizes `image` canvas and fills with `color`.
//     #[inline]
//     pub fn image_resize_canvas(
//         &self,
//         image: &mut Image,
//         new_width: i32,
//         new_height: i32,
//         offset_x: i32,
//         offset_y: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::ImageResizeCanvas(
//                 &mut image.0,
//                 new_width,
//                 new_height,
//                 offset_x,
//                 offset_y,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Generates all mipmap levels for a provided `image`.
//     #[inline]
//     pub fn image_mipmaps(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageMipmaps(&mut image.0);
//         }
//     }

//     /// Dithers `image` data to 16bpp or lower (Floyd-Steinberg dithering).
//     #[inline]
//     pub fn image_dither(&self, image: &mut Image, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
//         unsafe {
//             ffi::ImageDither(&mut image.0, r_bpp, g_bpp, b_bpp, a_bpp);
//         }
//     }

//     /// Creates an image from `text` (default font).
//     #[inline]
//     pub fn image_text(&self, text: &str, font_size: i32, color: impl Into<Color>) -> Image {
//         let c_text = CString::new(text).unwrap();
//         unsafe {
//             Image(ffi::ImageText(
//                 c_text.as_ptr(),
//                 font_size,
//                 color.into().into(),
//             ))
//         }
//     }

//     /// Creates an image from `text` (custom font).
//     #[inline]
//     pub fn image_text_ex(
//         &self,
//         font: &Font,
//         text: &str,
//         font_size: f32,
//         spacing: f32,
//         tint: impl Into<Color>,
//     ) -> Image {
//         let c_text = CString::new(text).unwrap();
//         unsafe {
//             Image(ffi::ImageTextEx(
//                 font.0,
//                 c_text.as_ptr(),
//                 font_size,
//                 spacing,
//                 tint.into().into(),
//             ))
//         }
//     }

//     /// Draws a source image within a destination image.
//     #[inline]
//     pub fn image_draw(&self, dst: &mut Image, src: &Image, src_rec: Rectangle, dst_rec: Rectangle) {
//         unsafe {
//             ffi::ImageDraw(&mut dst.0, src.0, src_rec, dst_rec);
//         }
//     }

//     /// Draws a rectangle within an image.
//     #[inline]
//     pub fn image_draw_rectangle(&self, dst: &mut Image, rec: Rectangle, color: impl Into<Color>) {
//         unsafe {
//             ffi::ImageDrawRectangle(&mut dst.0, rec, color.into().into());
//         }
//     }

//     /// Draws text (default font) within an image (destination).
//     #[inline]
//     pub fn image_draw_text(
//         &self,
//         dst: &mut Image,
//         position: impl Into<Vector2>,
//         text: &str,
//         font_size: i32,
//         color: impl Into<Color>,
//     ) {
//         let c_text = CString::new(text).unwrap();
//         unsafe {
//             ffi::ImageDrawText(
//                 &mut dst.0,
//                 position.into().into(),
//                 c_text.as_ptr(),
//                 font_size,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws text (custom font) within an image (destination).
//     #[inline]
//     pub fn image_draw_text_ex(
//         &self,
//         dst: &mut Image,
//         position: impl Into<Vector2>,
//         font: &Font,
//         text: &str,
//         font_size: f32,
//         spacing: f32,
//         color: impl Into<Color>,
//     ) {
//         let c_text = CString::new(text).unwrap();
//         unsafe {
//             ffi::ImageDrawTextEx(
//                 &mut dst.0,
//                 position.into().into(),
//                 font.0,
//                 c_text.as_ptr(),
//                 font_size,
//                 spacing,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Flips `image` vertically.
//     #[inline]
//     pub fn image_flip_vertical(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageFlipVertical(&mut image.0);
//         }
//     }

//     /// Flips `image` horizontally.
//     #[inline]
//     pub fn image_flip_horizontal(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageFlipHorizontal(&mut image.0);
//         }
//     }

//     /// Rotates `image` clockwise by 90 degrees (PI/2 radians).
//     #[inline]
//     pub fn image_rotate_cw(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageRotateCW(&mut image.0);
//         }
//     }

//     /// Rotates `image` counterclockwise by 90 degrees (PI/2 radians).
//     #[inline]
//     pub fn image_rotate_ccw(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageRotateCCW(&mut image.0);
//         }
//     }

//     /// Tints colors in `image` using specified `color`.
//     #[inline]
//     pub fn image_color_tint(&self, image: &mut Image, color: impl Into<Color>) {
//         unsafe {
//             ffi::ImageColorTint(&mut image.0, color.into().into());
//         }
//     }

//     /// Inverts the colors in `image`.
//     #[inline]
//     pub fn image_color_invert(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageColorInvert(&mut image.0);
//         }
//     }

//     /// Converts `image color to grayscale.
//     #[inline]
//     pub fn image_color_grayscale(&self, image: &mut Image) {
//         unsafe {
//             ffi::ImageColorGrayscale(&mut image.0);
//         }
//     }

//     /// Adjusts the contrast of `image`.
//     #[inline]
//     pub fn image_color_contrast(&self, image: &mut Image, contrast: f32) {
//         unsafe {
//             ffi::ImageColorContrast(&mut image.0, contrast);
//         }
//     }

//     /// Adjusts the brightness of `image`.
//     #[inline]
//     pub fn image_color_brightness(&self, image: &mut Image, brightness: i32) {
//         unsafe {
//             ffi::ImageColorBrightness(&mut image.0, brightness);
//         }
//     }

//     /// Searches `image` for all occurences of `color` and replaces them with `replace` color.
//     #[inline]
//     pub fn image_color_replace(
//         &self,
//         image: &mut Image,
//         color: impl Into<Color>,
//         replace: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::ImageColorReplace(&mut image.0, color.into().into(), replace.into().into());
//         }
//     }

//     /// Generates a plain `color` Image.
//     #[inline]
//     pub fn gen_image_color(&self, width: i32, height: i32, color: impl Into<Color>) -> Image {
//         unsafe { Image(ffi::GenImageColor(width, height, color.into().into())) }
//     }

//     /// Generates an Image containing a vertical gradient.
//     #[inline]
//     pub fn gen_image_gradient_v(
//         &self,
//         width: i32,
//         height: i32,
//         top: impl Into<Color>,
//         bottom: impl Into<Color>,
//     ) -> Image {
//         unsafe {
//             Image(ffi::GenImageGradientV(
//                 width,
//                 height,
//                 top.into().into(),
//                 bottom.into().into(),
//             ))
//         }
//     }

//     /// Generates an Image containing a horizonal gradient.
//     #[inline]
//     pub fn gen_image_gradient_h(
//         &self,
//         width: i32,
//         height: i32,
//         left: impl Into<Color>,
//         right: impl Into<Color>,
//     ) -> Image {
//         unsafe {
//             Image(ffi::GenImageGradientH(
//                 width,
//                 height,
//                 left.into().into(),
//                 right.into().into(),
//             ))
//         }
//     }

//     /// Generates an Image containing a radial gradient.
//     #[inline]
//     pub fn gen_image_gradient_radial(
//         &self,
//         width: i32,
//         height: i32,
//         density: f32,
//         inner: impl Into<Color>,
//         outer: impl Into<Color>,
//     ) -> Image {
//         unsafe {
//             Image(ffi::GenImageGradientRadial(
//                 width,
//                 height,
//                 density,
//                 inner.into().into(),
//                 outer.into().into(),
//             ))
//         }
//     }

//     /// Generates an Image containing a checkerboard pattern.
//     #[inline]
//     pub fn gen_image_checked(
//         &self,
//         width: i32,
//         height: i32,
//         checks_x: i32,
//         checks_y: i32,
//         col1: impl Into<Color>,
//         col2: impl Into<Color>,
//     ) -> Image {
//         unsafe {
//             Image(ffi::GenImageChecked(
//                 width,
//                 height,
//                 checks_x,
//                 checks_y,
//                 col1.into().into(),
//                 col2.into().into(),
//             ))
//         }
//     }

//     /// Generates an Image containing white noise.
//     #[inline]
//     pub fn gen_image_white_noise(&self, width: i32, height: i32, factor: f32) -> Image {
//         unsafe { Image(ffi::GenImageWhiteNoise(width, height, factor)) }
//     }

//     /// Generates an Image containing perlin noise.
//     #[inline]
//     pub fn gen_image_perlin_noise(
//         &self,
//         width: i32,
//         height: i32,
//         offset_x: i32,
//         offset_y: i32,
//         scale: f32,
//     ) -> Image {
//         unsafe {
//             Image(ffi::GenImagePerlinNoise(
//                 width, height, offset_x, offset_y, scale,
//             ))
//         }
//     }

//     /// Generates an Image using a cellular algorithm. Bigger `tile_size` means bigger cells.
//     #[inline]
//     pub fn gen_image_cellular(&self, width: i32, height: i32, tile_size: i32) -> Image {
//         unsafe { Image(ffi::GenImageCellular(width, height, tile_size)) }
//     }

//     /// Generates GPU mipmaps for a `texture`.
//     #[inline]
//     pub fn gen_texture_mipmaps(&self, texture: &mut Texture2D) {
//         unsafe {
//             ffi::GenTextureMipmaps(&mut texture.0);
//         }
//     }

//     /// Sets `texture` scaling filter mode.
//     #[inline]
//     pub fn set_texture_filter(&self, texture: &mut Texture2D, filter_mode: TextureFilter) {
//         unsafe {
//             ffi::SetTextureFilter(texture.0, filter_mode as i32);
//         }
//     }

//     /// Sets texture wrapping mode.
//     #[inline]
//     pub fn set_texture_wrap(&self, texture: &mut Texture2D, wrap_mode: TextureWrap) {
//         unsafe {
//             ffi::SetTextureWrap(texture.0, wrap_mode as i32);
//         }
//     }

//     /// Draws a `texture` using specified position and `tint` color.
//     #[inline]
//     pub fn draw_texture(&self, texture: &Texture2D, x: i32, y: i32, tint: impl Into<Color>) {
//         unsafe {
//             ffi::DrawTexture(texture.0, x, y, tint.into().into());
//         }
//     }

//     /// Draws a `texture` using specified `position` vector and `tint` color.
//     #[inline]
//     pub fn draw_texture_v(
//         &self,
//         texture: &Texture2D,
//         position: impl Into<Vector2>,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawTextureV(texture.0, position.into().into(), tint.into().into());
//         }
//     }

//     /// Draws a `texture` with extended parameters.
//     #[inline]
//     pub fn draw_texture_ex(
//         &self,
//         texture: &Texture2D,
//         position: impl Into<Vector2>,
//         rotation: f32,
//         scale: f32,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawTextureEx(
//                 texture.0,
//                 position.into().into(),
//                 rotation,
//                 scale,
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Draws from a region of `texture` defined by the `source_rec` rectangle.
//     #[inline]
//     pub fn draw_texture_rec(
//         &self,
//         texture: &Texture2D,
//         source_rec: Rectangle,
//         position: impl Into<Vector2>,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawTextureRec(
//                 texture.0,
//                 source_rec,
//                 position.into().into(),
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Draw from a region of `texture` defined by the `source_rec` rectangle with pro parameters.
//     #[inline]
//     pub fn draw_texture_pro(
//         &self,
//         texture: &Texture2D,
//         source_rec: Rectangle,
//         dest_rec: Rectangle,
//         origin: impl Into<Vector2>,
//         rotation: f32,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawTexturePro(
//                 texture.0,
//                 source_rec,
//                 dest_rec,
//                 origin.into().into(),
//                 rotation,
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Gets the default font.
//     #[inline]
//     pub fn get_font_default(&self) -> &'static Font {
//         &FONT_DEFAULT
//     }

//     /// Loads font from file into GPU memory (VRAM).
//     #[inline]
//     pub fn load_font(&self, filename: &str) -> Font {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe { Font(ffi::LoadFont(c_filename.as_ptr())) }
//     }

//     /// Loads font from file with extended parameters.
//     #[inline]
//     pub fn load_font_ex(&self, filename: &str, font_size: i32, chars: Option<&[i32]>) -> Font {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             match chars {
//                 Some(c) => Font(ffi::LoadFontEx(
//                     c_filename.as_ptr(),
//                     font_size,
//                     c.as_ptr() as *mut i32,
//                     c.len() as i32,
//                 )),
//                 None => Font(ffi::LoadFontEx(
//                     c_filename.as_ptr(),
//                     font_size,
//                     std::ptr::null_mut(),
//                     0,
//                 )),
//             }
//         }
//     }

//     /// Loads font data for further use (see also `Font::from_data`).
//     #[inline]
//     pub fn load_font_data(
//         &self,
//         filename: &str,
//         font_size: i32,
//         chars: Option<&[i32]>,
//         sdf: i32,
//     ) -> Vec<CharInfo> {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             let ci_arr_ptr = match chars {
//                 Some(c) => ffi::LoadFontData(
//                     c_filename.as_ptr(),
//                     font_size,
//                     c.as_ptr() as *mut i32,
//                     c.len() as i32,
//                     sdf,
//                 ),
//                 None => {
//                     ffi::LoadFontData(c_filename.as_ptr(), font_size, std::ptr::null_mut(), 0, sdf)
//                 }
//             };
//             let ci_size = if let Some(c) = chars { c.len() } else { 95 }; // raylib assumes 95 if none given
//             let mut ci_vec = Vec::with_capacity(ci_size);
//             for i in 0..ci_size {
//                 ci_vec.push(*ci_arr_ptr.offset(i as isize));
//             }
//             libc::free(ci_arr_ptr as *mut libc::c_void);
//             ci_vec
//         }
//     }

//     /// Generates image font atlas using `chars` info.
//     #[inline]
//     pub fn gen_image_font_atlas(
//         &self,
//         chars: &mut [CharInfo],
//         font_size: i32,
//         padding: i32,
//         pack_method: i32,
//     ) -> Image {
//         unsafe {
//             Image(ffi::GenImageFontAtlas(
//                 chars.as_mut_ptr(),
//                 font_size,
//                 chars.len() as i32,
//                 padding,
//                 pack_method,
//             ))
//         }
//     }

//     /// Shows current FPS.
//     #[inline]
//     pub fn draw_fps(&self, x: i32, y: i32) {
//         unsafe {
//             ffi::DrawFPS(x, y);
//         }
//     }

//     /// Draws text (using default font).
//     #[inline]
//     pub fn draw_text(&self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<Color>) {
//         let c_text = CString::new(text).unwrap();
//         unsafe {
//             ffi::DrawText(c_text.as_ptr(), x, y, font_size, color.into().into());
//         }
//     }

//     /// Draws text using `font` and additional parameters.
//     #[inline]
//     pub fn draw_text_ex(
//         &self,
//         font: &Font,
//         text: &str,
//         position: impl Into<Vector2>,
//         font_size: f32,
//         spacing: f32,
//         tint: impl Into<Color>,
//     ) {
//         let c_text = CString::new(text).unwrap();
//         unsafe {
//             ffi::DrawTextEx(
//                 font.0,
//                 c_text.as_ptr(),
//                 position.into().into(),
//                 font_size,
//                 spacing,
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Measures string width in pixels for default font.
//     #[inline]
//     pub fn measure_text(&self, text: &str, font_size: i32) -> i32 {
//         let c_text = CString::new(text).unwrap();
//         unsafe { ffi::MeasureText(c_text.as_ptr(), font_size) }
//     }

//     /// Measures string width in pixels for `font`.
//     #[inline]
//     pub fn measure_text_ex(
//         &self,
//         font: &Font,
//         text: &str,
//         font_size: f32,
//         spacing: f32,
//     ) -> Vector2 {
//         let c_text = CString::new(text).unwrap();
//         unsafe { ffi::MeasureTextEx(font.0, c_text.as_ptr(), font_size, spacing).into() }
//     }

//     /// Gets index position for a unicode character on `font`.
//     #[inline]
//     pub fn get_glyph_index(&self, font: &Font, character: i32) -> i32 {
//         unsafe { ffi::GetGlyphIndex(font.0, character) }
//     }

//     /// Draws a line in 3D world space.
//     #[inline]
//     pub fn draw_line_3d(
//         &self,
//         start_pos: impl Into<Vector3>,
//         end_pos: impl Into<Vector3>,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawLine3D(
//                 start_pos.into().into(),
//                 end_pos.into().into(),
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a circle in 3D world space.
//     #[inline]
//     pub fn draw_circle_3d(
//         &self,
//         center: impl Into<Vector3>,
//         radius: f32,
//         rotation_axis: impl Into<Vector3>,
//         rotation_angle: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCircle3D(
//                 center.into().into(),
//                 radius,
//                 rotation_axis.into().into(),
//                 rotation_angle,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a cube.
//     #[inline]
//     pub fn draw_cube(
//         &self,
//         position: impl Into<Vector3>,
//         width: f32,
//         height: f32,
//         length: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCube(
//                 position.into().into(),
//                 width,
//                 height,
//                 length,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a cube (Vector version).
//     #[inline]
//     pub fn draw_cube_v(
//         &self,
//         position: impl Into<Vector3>,
//         size: impl Into<Vector3>,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCubeV(
//                 position.into().into(),
//                 size.into().into(),
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a cube in wireframe.
//     #[inline]
//     pub fn draw_cube_wires(
//         &self,
//         position: impl Into<Vector3>,
//         width: f32,
//         height: f32,
//         length: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCubeWires(
//                 position.into().into(),
//                 width,
//                 height,
//                 length,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a textured cube.
//     #[inline]
//     pub fn draw_cube_texture(
//         &self,
//         texture: &Texture2D,
//         position: impl Into<Vector3>,
//         width: f32,
//         height: f32,
//         length: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCubeTexture(
//                 texture.0,
//                 position.into().into(),
//                 width,
//                 height,
//                 length,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a sphere.
//     #[inline]
//     pub fn draw_sphere(
//         &self,
//         center_pos: impl Into<Vector3>,
//         radius: f32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawSphere(center_pos.into().into(), radius, color.into().into());
//         }
//     }

//     /// Draws a sphere with extended parameters.
//     #[inline]
//     pub fn draw_sphere_ex(
//         &self,
//         center_pos: impl Into<Vector3>,
//         radius: f32,
//         rings: i32,
//         slices: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawSphereEx(
//                 center_pos.into().into(),
//                 radius,
//                 rings,
//                 slices,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a sphere in wireframe.
//     #[inline]
//     pub fn draw_sphere_wires(
//         &self,
//         center_pos: impl Into<Vector3>,
//         radius: f32,
//         rings: i32,
//         slices: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawSphereWires(
//                 center_pos.into().into(),
//                 radius,
//                 rings,
//                 slices,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a cylinder.
//     #[inline]
//     pub fn draw_cylinder(
//         &self,
//         position: impl Into<Vector3>,
//         radius_top: f32,
//         radius_bottom: f32,
//         height: f32,
//         slices: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCylinder(
//                 position.into().into(),
//                 radius_top,
//                 radius_bottom,
//                 height,
//                 slices,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a cylinder in wireframe.
//     #[inline]
//     pub fn draw_cylinder_wires(
//         &self,
//         position: impl Into<Vector3>,
//         radius_top: f32,
//         radius_bottom: f32,
//         height: f32,
//         slices: i32,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawCylinderWires(
//                 position.into().into(),
//                 radius_top,
//                 radius_bottom,
//                 height,
//                 slices,
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws an X/Z plane.
//     #[inline]
//     pub fn draw_plane(
//         &self,
//         center_pos: impl Into<Vector3>,
//         size: impl Into<Vector2>,
//         color: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawPlane(
//                 center_pos.into().into(),
//                 size.into().into(),
//                 color.into().into(),
//             );
//         }
//     }

//     /// Draws a ray line.
//     #[inline]
//     pub fn draw_ray(&self, ray: Ray, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawRay(ray.into(), color.into().into());
//         }
//     }

//     /// Draws a grid (centered at (0, 0, 0)).
//     #[inline]
//     pub fn draw_grid(&self, slices: i32, spacing: f32) {
//         unsafe {
//             ffi::DrawGrid(slices, spacing);
//         }
//     }

//     /// Draws a simple gizmo.
//     #[inline]
//     pub fn draw_gizmo(&self, position: impl Into<Vector3>) {
//         unsafe {
//             ffi::DrawGizmo(position.into().into());
//         }
//     }

//     /// Loads model from files (mesh and material).
//     #[inline]
//     pub fn load_model(&self, filename: &str) -> Model {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe { Model(ffi::LoadModel(c_filename.as_ptr())) }
//     }

//     /// Loads model from generated mesh. Returned Model takes ownership of `mesh`.
//     #[inline]
//     pub fn load_model_from_mesh(&self, mesh: Mesh) -> Model {
//         unsafe {
//             let m = mesh.0;
//             std::mem::forget(mesh);
//             Model(ffi::LoadModelFromMesh(m))
//         }
//     }

//     /// Exports mesh as an OBJ file.
//     #[inline]
//     pub fn export_mesh(&self, filename: &str, mesh: &Mesh) {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             ffi::ExportMesh(mesh.0, c_filename.as_ptr());
//         }
//     }

//     /// Computes mesh bounding box limits.
//     #[inline]
//     pub fn mesh_bounding_box(&self, mesh: &Mesh) -> BoundingBox {
//         unsafe { ffi::MeshBoundingBox(mesh.0).into() }
//     }

//     /// Computes mesh tangents.
//     #[inline]
//     pub fn mesh_tangents(&self, mesh: &mut Mesh) {
//         unsafe {
//             ffi::MeshTangents(&mut mesh.0);
//         }
//     }

//     /// Computes mesh binormals.
//     #[inline]
//     pub fn mesh_binormals(&self, mesh: &mut Mesh) {
//         unsafe {
//             ffi::MeshBinormals(&mut mesh.0);
//         }
//     }

//     /// Generates plane mesh (with subdivisions).
//     #[inline]
//     pub fn gen_mesh_plane(&self, width: f32, length: f32, res_x: i32, res_z: i32) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshPlane(width, length, res_x, res_z)) }
//     }

//     /// Generates cuboid mesh.
//     #[inline]
//     pub fn gen_mesh_cube(&self, width: f32, height: f32, length: f32) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshCube(width, height, length)) }
//     }

//     /// Generates sphere mesh (standard sphere).
//     #[inline]
//     pub fn gen_mesh_sphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshSphere(radius, rings, slices)) }
//     }

//     /// Generates half-sphere mesh (no bottom cap).
//     #[inline]
//     pub fn gen_mesh_hemisphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshHemiSphere(radius, rings, slices)) }
//     }

//     /// Generates cylinder mesh.
//     #[inline]
//     pub fn gen_mesh_cylinder(&self, radius: f32, height: f32, slices: i32) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshCylinder(radius, height, slices)) }
//     }

//     /// Generates torus mesh.
//     #[inline]
//     pub fn gen_mesh_torus(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshTorus(radius, size, rad_seg, sides)) }
//     }

//     /// Generates trefoil knot mesh.
//     #[inline]
//     pub fn gen_mesh_knot(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshKnot(radius, size, rad_seg, sides)) }
//     }

//     /// Generates heightmap mesh from image data.
//     #[inline]
//     pub fn gen_mesh_heightmap(&self, heightmap: &Image, size: impl Into<Vector3>) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshHeightmap(heightmap.0, size.into().into())) }
//     }

//     /// Generates cubes-based map mesh from image data.
//     #[inline]
//     pub fn gen_mesh_cubicmap(&self, cubicmap: &Image, cube_size: impl Into<Vector3>) -> Mesh {
//         unsafe { Mesh(ffi::GenMeshCubicmap(cubicmap.0, cube_size.into().into())) }
//     }

//     /// Loads default material (supports `DIFFUSE`, `SPECULAR`, and `NORMAL` maps).
//     #[inline]
//     pub fn load_material_default(&self) -> &'static Material {
//         &MATERIAL_DEFAULT
//     }

//     /// Draws a model (with texture if set).
//     #[inline]
//     pub fn draw_model(
//         &self,
//         model: &Model,
//         position: impl Into<Vector3>,
//         scale: f32,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawModel(model.0, position.into().into(), scale, tint.into().into());
//         }
//     }

//     /// Draws a model with extended parameters.
//     #[inline]
//     pub fn draw_model_ex(
//         &self,
//         model: &Model,
//         position: impl Into<Vector3>,
//         rotation_axis: impl Into<Vector3>,
//         rotation_angle: f32,
//         scale: impl Into<Vector3>,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawModelEx(
//                 model.0,
//                 position.into().into(),
//                 rotation_axis.into().into(),
//                 rotation_angle,
//                 scale.into().into(),
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Draws a model with wires (with texture if set).
//     #[inline]
//     pub fn draw_model_wires(
//         &self,
//         model: &Model,
//         position: impl Into<Vector3>,
//         scale: f32,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawModelWires(model.0, position.into().into(), scale, tint.into().into());
//         }
//     }

//     /// Draws a model with wires.
//     #[inline]
//     pub fn draw_model_wires_ex(
//         &self,
//         model: &Model,
//         position: impl Into<Vector3>,
//         rotation_axis: impl Into<Vector3>,
//         rotation_angle: f32,
//         scale: impl Into<Vector3>,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawModelWiresEx(
//                 model.0,
//                 position.into().into(),
//                 rotation_axis.into().into(),
//                 rotation_angle,
//                 scale.into().into(),
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Draws a bounding box (wires).
//     #[inline]
//     pub fn draw_bounding_box(&self, bbox: BoundingBox, color: impl Into<Color>) {
//         unsafe {
//             ffi::DrawBoundingBox(bbox.into(), color.into().into());
//         }
//     }

//     /// Draws a billboard texture.
//     #[inline]
//     pub fn draw_billboard(
//         &self,
//         camera: Camera3D,
//         texture: &Texture2D,
//         center: impl Into<Vector3>,
//         size: f32,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawBillboard(
//                 camera.into(),
//                 texture.0,
//                 center.into().into(),
//                 size,
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Draws a billboard texture defined by `source_rec`.
//     #[inline]
//     pub fn draw_billboard_rec(
//         &self,
//         camera: Camera3D,
//         texture: &Texture2D,
//         source_rec: Rectangle,
//         center: impl Into<Vector3>,
//         size: f32,
//         tint: impl Into<Color>,
//     ) {
//         unsafe {
//             ffi::DrawBillboardRec(
//                 camera.into(),
//                 texture.0,
//                 source_rec,
//                 center.into().into(),
//                 size,
//                 tint.into().into(),
//             );
//         }
//     }

//     /// Detects collision between two spheres.
//     #[inline]
//     pub fn check_collision_spheres(
//         &self,
//         center_a: impl Into<Vector3>,
//         radius_a: f32,
//         center_b: impl Into<Vector3>,
//         radius_b: f32,
//     ) -> bool {
//         unsafe {
//             ffi::CheckCollisionSpheres(
//                 center_a.into().into(),
//                 radius_a,
//                 center_b.into().into(),
//                 radius_b,
//             )
//         }
//     }

//     /// Detects collision between two boxes.
//     #[inline]
//     pub fn check_collision_boxes(&self, box1: BoundingBox, box2: BoundingBox) -> bool {
//         unsafe { ffi::CheckCollisionBoxes(box1.into(), box2.into()) }
//     }

//     /// Detects collision between box and sphere.
//     #[inline]
//     pub fn check_collision_box_sphere(
//         &self,
//         bbox: BoundingBox,
//         center_sphere: impl Into<Vector3>,
//         radius_sphere: f32,
//     ) -> bool {
//         unsafe {
//             ffi::CheckCollisionBoxSphere(bbox.into(), center_sphere.into().into(), radius_sphere)
//         }
//     }

//     /// Detects collision between ray and sphere.
//     #[inline]
//     pub fn check_collision_ray_sphere(
//         &self,
//         ray: Ray,
//         sphere_position: impl Into<Vector3>,
//         sphere_radius: f32,
//     ) -> bool {
//         unsafe {
//             ffi::CheckCollisionRaySphere(ray.into(), sphere_position.into().into(), sphere_radius)
//         }
//     }

//     /// Detects collision between ray and sphere, and returns the collision point.
//     #[inline]
//     pub fn check_collision_ray_sphere_ex(
//         &self,
//         ray: Ray,
//         sphere_position: impl Into<Vector3>,
//         sphere_radius: f32,
//     ) -> Option<Vector3> {
//         unsafe {
//             let mut col_point = ffi::Vector3 {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//             };
//             let collision = ffi::CheckCollisionRaySphereEx(
//                 ray.into(),
//                 sphere_position.into().into(),
//                 sphere_radius,
//                 &mut col_point,
//             );
//             if collision {
//                 Some(col_point.into())
//             } else {
//                 None
//             }
//         }
//     }

//     /// Detects collision between ray and box.
//     #[inline]
//     pub fn check_collision_ray_box(&self, ray: Ray, bbox: BoundingBox) -> bool {
//         unsafe { ffi::CheckCollisionRayBox(ray.into(), bbox.into()) }
//     }

//     /// Gets collision info between ray and model.
//     #[inline]
//     pub fn get_collision_ray_model(&self, ray: Ray, model: &Model) -> RayHitInfo {
//         unsafe { ffi::GetCollisionRayModel(ray.into(), &mut { model.0 }).into() }
//     }

//     /// Gets collision info between ray and triangle.
//     #[inline]
//     pub fn get_collision_ray_triangle(
//         &self,
//         ray: Ray,
//         p1: impl Into<Vector3>,
//         p2: impl Into<Vector3>,
//         p3: impl Into<Vector3>,
//     ) -> RayHitInfo {
//         unsafe {
//             ffi::GetCollisionRayTriangle(
//                 ray.into(),
//                 p1.into().into(),
//                 p2.into().into(),
//                 p3.into().into(),
//             )
//             .into()
//         }
//     }

//     /// Gets collision info between ray and ground plane (Y-normal plane).
//     #[inline]
//     pub fn get_collision_ray_ground(&self, ray: Ray, ground_height: f32) -> RayHitInfo {
//         unsafe { ffi::GetCollisionRayGround(ray.into(), ground_height).into() }
//     }

//     /// Loads a text file and returns its contents in a string.
//     #[inline]
//     pub fn load_text(&self, filename: &str) -> String {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe {
//             let text = ffi::LoadText(c_filename.as_ptr());
//             let safe_text = CStr::from_ptr(text).to_str().unwrap().to_owned();
//             libc::free(text as *mut libc::c_void);
//             safe_text
//         }
//     }

//     /// Loads a custom shader and binds default locations.
//     #[inline]
//     pub fn load_shader(&self, vs_filename: &str, fs_filename: &str) -> Shader {
//         let c_vs_filename = CString::new(vs_filename).unwrap();
//         let c_fs_filename = CString::new(fs_filename).unwrap();
//         unsafe {
//             Shader(ffi::LoadShader(
//                 c_vs_filename.as_ptr(),
//                 c_fs_filename.as_ptr(),
//             ))
//         }
//     }

//     /// Loads shader from code strings and binds default locations.
//     #[inline]
//     pub fn load_shader_code(&self, vs_code: &str, fs_code: &str) -> Shader {
//         let c_vs_code = CString::new(vs_code).unwrap();
//         let c_fs_code = CString::new(fs_code).unwrap();
//         unsafe {
//             Shader(ffi::LoadShaderCode(
//                 c_vs_code.as_ptr() as *mut i8,
//                 c_fs_code.as_ptr() as *mut i8,
//             ))
//         }
//     }

//     /// Gets default shader.
//     #[inline]
//     pub fn get_shader_default(&self) -> &'static Shader {
//         &SHADER_DEFAULT
//     }

//     /// Gets default texture.
//     #[inline]
//     pub fn get_texture_default(&self) -> &'static Texture2D {
//         &TEXTURE_DEFAULT
//     }

//     /// Gets shader uniform location by name.
//     #[inline]
//     pub fn get_shader_location(&self, shader: &Shader, uniform_name: &str) -> i32 {
//         let c_uniform_name = CString::new(uniform_name).unwrap();
//         unsafe { ffi::GetShaderLocation(shader.0, c_uniform_name.as_ptr()) }
//     }

//     /// Sets shader uniform value (`f32`).
//     #[inline]
//     pub fn set_shader_value(&self, shader: &mut Shader, uniform_loc: i32, value: &[f32]) {
//         unsafe {
//             ffi::SetShaderValue(
//                 shader.0,
//                 uniform_loc,
//                 value.as_ptr() as *const ::std::os::raw::c_void,
//                 value.len() as i32,
//             );
//         }
//     }

//     /// Sets shader uniform value (matrix 4x4).
//     #[inline]
//     pub fn set_shader_value_matrix(&self, shader: &mut Shader, uniform_loc: i32, mat: Matrix) {
//         unsafe {
//             ffi::SetShaderValueMatrix(shader.0, uniform_loc, mat.into());
//         }
//     }

//     /// Sets a custom projection matrix (replaces internal projection matrix).
//     #[inline]
//     pub fn set_matrix_projection(&self, proj: Matrix) {
//         unsafe {
//             ffi::SetMatrixProjection(proj.into());
//         }
//     }

//     /// Sets a custom modelview matrix (replaces internal modelview matrix).
//     #[inline]
//     pub fn set_matrix_modelview(&self, view: Matrix) {
//         unsafe {
//             ffi::SetMatrixModelview(view.into());
//         }
//     }

//     /// Gets internal modelview matrix.
//     #[inline]
//     pub fn get_matrix_modelview(&self) -> Matrix {
//         unsafe { ffi::GetMatrixModelview().into() }
//     }

//     /// Generates cubemap texture from HDR texture.
//     #[inline]
//     pub fn gen_texture_cubemap(
//         &self,
//         shader: &Shader,
//         sky_hdr: &Texture2D,
//         size: i32,
//     ) -> Texture2D {
//         unsafe { Texture2D(ffi::GenTextureCubemap(shader.0, sky_hdr.0, size)) }
//     }

//     /// Generates irradiance texture using cubemap data.
//     #[inline]
//     pub fn gen_texture_irradiance(
//         &self,
//         shader: &Shader,
//         cubemap: &Texture2D,
//         size: i32,
//     ) -> Texture2D {
//         unsafe { Texture2D(ffi::GenTextureIrradiance(shader.0, cubemap.0, size)) }
//     }

//     /// Generates prefilter texture using cubemap data.
//     #[inline]
//     pub fn gen_texture_prefilter(
//         &self,
//         shader: &Shader,
//         cubemap: &Texture2D,
//         size: i32,
//     ) -> Texture2D {
//         unsafe { Texture2D(ffi::GenTexturePrefilter(shader.0, cubemap.0, size)) }
//     }

//     /// Generates BRDF texture using cubemap data.
//     #[inline]
//     pub fn gen_texture_brdf(&self, shader: &Shader, size: i32) -> Texture2D {
//         unsafe { Texture2D(ffi::GenTextureBRDF(shader.0, size)) }
//     }

//     /// Begins custom shader drawing.
//     #[inline]
//     pub fn begin_shader_mode(&self, shader: &Shader) {
//         unsafe {
//             ffi::BeginShaderMode(shader.0);
//         }
//     }

//     /// Ends custom shader drawing (and switches to default shader).
//     #[inline]
//     pub fn end_shader_mode(&self) {
//         unsafe {
//             ffi::EndShaderMode();
//         }
//     }

//     /// Begins blending mode (alpha, additive, multiplied).
//     #[inline]
//     pub fn begin_blend_mode(&self, mode: crate::ffi::BlendMode) {
//         unsafe {
//             ffi::BeginBlendMode((mode as u32) as i32);
//         }
//     }

//     /// Ends blending mode (reset to default: alpha blending).
//     #[inline]
//     pub fn end_blend_mode(&self) {
//         unsafe {
//             ffi::EndBlendMode();
//         }
//     }

//     /// Initializes VR simulator for selected device parameters.
//     #[inline]
//     pub fn init_vr_simulator(&self) {
//         unsafe {
//             ffi::InitVrSimulator();
//         }
//     }

//     /// Closes VR simulator for current device.
//     #[inline]
//     pub fn close_vr_simulator(&self) {
//         unsafe {
//             ffi::CloseVrSimulator();
//         }
//     }

//     /// Detects if VR simulator is ready.
//     #[inline]
//     pub fn is_vr_simulator_ready(&self) -> bool {
//         unsafe { ffi::IsVrSimulatorReady() }
//     }

//     /// Updates VR tracking (position and orientation) and camera.
//     #[inline]
//     pub fn update_vr_tracking(&self, camera: &mut Camera3D) {
//         unsafe {
//             let mut fficam: ffi::Camera3D = (*camera).into();
//             ffi::UpdateVrTracking(&mut fficam);
//             *camera = fficam.into();
//         }
//     }

//     /// Enables or disables VR experience.
//     #[inline]
//     pub fn toggle_vr_mode(&self) {
//         unsafe {
//             ffi::ToggleVrMode();
//         }
//     }

//     /// Begins VR simulator stereo rendering.
//     #[inline]
//     pub fn begin_vr_drawing(&self) {
//         unsafe {
//             ffi::BeginVrDrawing();
//         }
//     }

//     /// Ends VR simulator stereo rendering.
//     #[inline]
//     pub fn end_vr_drawing(&self) {
//         unsafe {
//             ffi::EndVrDrawing();
//         }
//     }

//     /// Initializes audio device and context.
//     #[inline]
//     pub fn init_audio_device(&self) {
//         unsafe {
//             ffi::InitAudioDevice();
//         }
//     }

//     /// Closes the audio device and context (and music stream).
//     #[inline]
//     pub fn close_audio_device(&self) {
//         unsafe {
//             ffi::CloseAudioDevice();
//         }
//     }

//     /// Checks if audio device is ready.
//     #[inline]
//     pub fn is_audio_device_ready(&self) -> bool {
//         unsafe { ffi::IsAudioDeviceReady() }
//     }

//     /// Sets master volume (listener).
//     #[inline]
//     pub fn set_master_volume(&self, volume: f32) {
//         unsafe {
//             ffi::SetMasterVolume(volume);
//         }
//     }

//     /// Loads wave data from file into RAM.
//     #[inline]
//     pub fn load_wave(&self, filename: &str) -> Wave {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe { Wave(ffi::LoadWave(c_filename.as_ptr())) }
//     }

//     /// Loads wave data from raw array data.
//     #[inline]
//     pub fn load_wave_ex(
//         &self,
//         data: &[u8],
//         sample_count: i32,
//         sample_rate: i32,
//         sample_size: i32,
//         channels: i32,
//     ) -> Wave {
//         unsafe {
//             Wave(ffi::LoadWaveEx(
//                 data.as_ptr() as *mut std::os::raw::c_void,
//                 sample_count,
//                 sample_rate,
//                 sample_size,
//                 channels,
//             ))
//         }
//     }

//     /// Loads sound from file.
//     #[inline]
//     pub fn load_sound(&self, filename: &str) -> Sound {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe { Sound(ffi::LoadSound(c_filename.as_ptr())) }
//     }

//     /// Loads sound from wave data.
//     #[inline]
//     pub fn load_sound_from_wave(&self, wave: &Wave) -> Sound {
//         unsafe { Sound(ffi::LoadSoundFromWave(wave.0)) }
//     }

//     /// Updates sound buffer with new data.
//     #[inline]
//     pub fn update_sound(&self, sound: &mut Sound, data: &[impl AudioSample]) {
//         unsafe {
//             ffi::UpdateSound(
//                 sound.0,
//                 data.as_ptr() as *const std::os::raw::c_void,
//                 data.len() as i32,
//             );
//         }
//     }

//     /// Plays a sound.
//     #[inline]
//     pub fn play_sound(&self, sound: &Sound) {
//         unsafe {
//             ffi::PlaySound(sound.0);
//         }
//     }

//     /// Pauses a sound.
//     #[inline]
//     pub fn pause_sound(&self, sound: &Sound) {
//         unsafe {
//             ffi::PauseSound(sound.0);
//         }
//     }

//     /// Resumes a paused sound.
//     #[inline]
//     pub fn resume_sound(&self, sound: &Sound) {
//         unsafe {
//             ffi::ResumeSound(sound.0);
//         }
//     }

//     /// Stops playing a sound.
//     #[inline]
//     pub fn stop_sound(&self, sound: &Sound) {
//         unsafe {
//             ffi::StopSound(sound.0);
//         }
//     }

//     /// Checks if a sound is currently playing.
//     #[inline]
//     pub fn is_sound_playing(&self, sound: &Sound) -> bool {
//         unsafe { ffi::IsSoundPlaying(sound.0) }
//     }

//     /// Sets volume for a sound (`1.0` is max level).
//     #[inline]
//     pub fn set_sound_volume(&self, sound: &Sound, volume: f32) {
//         unsafe {
//             ffi::SetSoundVolume(sound.0, volume);
//         }
//     }

//     /// Sets pitch for a sound (`1.0` is base level).
//     #[inline]
//     pub fn set_sound_pitch(&self, sound: &Sound, pitch: f32) {
//         unsafe {
//             ffi::SetSoundPitch(sound.0, pitch);
//         }
//     }

//     /// Converts wave data to desired format.
//     #[inline]
//     pub fn wave_format(&self, wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
//         unsafe {
//             ffi::WaveFormat(&mut wave.0, sample_rate, sample_size, channels);
//         }
//     }

//     /// Copies a wave to a new wave.
//     #[inline]
//     pub fn wave_copy(&self, wave: &Wave) -> Wave {
//         unsafe { Wave(ffi::WaveCopy(wave.0)) }
//     }

//     /// Crops a wave to defined sample range.
//     #[inline]
//     pub fn wave_crop(&self, wave: &mut Wave, init_sample: i32, final_sample: i32) {
//         unsafe {
//             ffi::WaveCrop(&mut wave.0, init_sample, final_sample);
//         }
//     }

//     /// Gets sample data from wave as an `f32` array.
//     #[inline]
//     pub fn get_wave_data(&self, wave: &Wave) -> Vec<f32> {
//         unsafe {
//             let data = ffi::GetWaveData(wave.0);
//             let data_size = (wave.sampleCount * wave.channels) as usize;
//             let mut samples = Vec::with_capacity(data_size);
//             samples.set_len(data_size);
//             std::ptr::copy(data, samples.as_mut_ptr(), data_size);
//             libc::free(data as *mut libc::c_void);
//             samples
//         }
//     }

//     /// Loads music stream from file.
//     #[inline]
//     pub fn load_music_stream(&self, filename: &str) -> Music {
//         let c_filename = CString::new(filename).unwrap();
//         unsafe { Music(ffi::LoadMusicStream(c_filename.as_ptr())) }
//     }

//     /// Starts music playing.
//     #[inline]
//     pub fn play_music_stream(&self, music: &mut Music) {
//         unsafe {
//             ffi::PlayMusicStream(music.0);
//         }
//     }

//     /// Updates buffers for music streaming.
//     #[inline]
//     pub fn update_music_stream(&self, music: &mut Music) {
//         unsafe {
//             ffi::UpdateMusicStream(music.0);
//         }
//     }

//     /// Stops music playing.
//     #[inline]
//     pub fn stop_music_stream(&self, music: &mut Music) {
//         unsafe {
//             ffi::StopMusicStream(music.0);
//         }
//     }

//     /// Pauses music playing.
//     #[inline]
//     pub fn pause_music_stream(&self, music: &mut Music) {
//         unsafe {
//             ffi::PauseMusicStream(music.0);
//         }
//     }

//     /// Resumes playing paused music.
//     #[inline]
//     pub fn resume_music_stream(&self, music: &mut Music) {
//         unsafe {
//             ffi::ResumeMusicStream(music.0);
//         }
//     }

//     /// Checks if music is playing.
//     #[inline]
//     pub fn is_music_playing(&self, music: &Music) -> bool {
//         unsafe { ffi::IsMusicPlaying(music.0) }
//     }

//     /// Sets volume for music (`1.0` is max level).
//     #[inline]
//     pub fn set_music_volume(&self, music: &mut Music, volume: f32) {
//         unsafe {
//             ffi::SetMusicVolume(music.0, volume);
//         }
//     }

//     /// Sets pitch for music (`1.0` is base level).
//     #[inline]
//     pub fn set_music_pitch(&self, music: &mut Music, pitch: f32) {
//         unsafe {
//             ffi::SetMusicPitch(music.0, pitch);
//         }
//     }

//     /// Sets music loop count (loop repeats).
//     #[inline]
//     pub fn set_music_loop_count(&self, music: &mut Music, count: i32) {
//         unsafe {
//             ffi::SetMusicLoopCount(music.0, count);
//         }
//     }

//     /// Gets music time length in seconds.
//     #[inline]
//     pub fn get_music_time_length(&self, music: &Music) -> f32 {
//         unsafe { ffi::GetMusicTimeLength(music.0) }
//     }

//     /// Gets current music time played in seconds.
//     #[inline]
//     pub fn get_music_time_played(&self, music: &Music) -> f32 {
//         unsafe { ffi::GetMusicTimePlayed(music.0) }
//     }

//     /// Initializes audio stream (to stream raw PCM data).
//     #[inline]
//     pub fn init_audio_stream(
//         &self,
//         sample_rate: u32,
//         sample_size: u32,
//         channels: u32,
//     ) -> AudioStream {
//         unsafe { AudioStream(ffi::InitAudioStream(sample_rate, sample_size, channels)) }
//     }

//     /// Updates audio stream buffers with data.
//     #[inline]
//     pub fn update_audio_stream(&self, stream: &mut AudioStream, data: &[impl AudioSample]) {
//         unsafe {
//             ffi::UpdateAudioStream(
//                 stream.0,
//                 data.as_ptr() as *const std::os::raw::c_void,
//                 data.len() as i32,
//             );
//         }
//     }

//     /// Checks if any audio stream buffers requires refill.
//     #[inline]
//     pub fn is_audio_buffer_processed(&self, stream: &AudioStream) -> bool {
//         unsafe { ffi::IsAudioBufferProcessed(stream.0) }
//     }

//     /// Plays audio stream.
//     #[inline]
//     pub fn play_audio_stream(&self, stream: &mut AudioStream) {
//         unsafe {
//             ffi::PlayAudioStream(stream.0);
//         }
//     }

//     /// Pauses audio stream.
//     #[inline]
//     pub fn pause_audio_stream(&self, stream: &mut AudioStream) {
//         unsafe {
//             ffi::PauseAudioStream(stream.0);
//         }
//     }

//     /// Resumes audio stream.
//     #[inline]
//     pub fn resume_audio_stream(&self, stream: &mut AudioStream) {
//         unsafe {
//             ffi::ResumeAudioStream(stream.0);
//         }
//     }

//     /// Checks if audio stream is currently playing.
//     #[inline]
//     pub fn is_audio_stream_playing(&self, stream: &AudioStream) -> bool {
//         unsafe { ffi::IsAudioStreamPlaying(stream.0) }
//     }

//     /// Stops audio stream.
//     #[inline]
//     pub fn stop_audio_stream(&self, stream: &mut AudioStream) {
//         unsafe {
//             ffi::StopAudioStream(stream.0);
//         }
//     }

//     /// Sets volume for audio stream (`1.0` is max level).
//     #[inline]
//     pub fn set_audio_stream_volume(&self, stream: &mut AudioStream, volume: f32) {
//         unsafe {
//             ffi::SetAudioStreamVolume(stream.0, volume);
//         }
//     }

//     /// Sets pitch for audio stream (`1.0` is base level).
//     #[inline]
//     pub fn set_audio_stream_pitch(&self, stream: &mut AudioStream, pitch: f32) {
//         unsafe {
//             ffi::SetAudioStreamPitch(stream.0, pitch);
//         }
//     }
// }
