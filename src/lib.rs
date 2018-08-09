/* raylib-rs
   lib.rs - Main library code (the safe layer)

Copyright (c) 2018 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

extern crate libc;

use std::ffi::{CString, CStr};

mod raylib;
pub use raylib::{
    CBool, Image, Color,
    Camera2D, Camera3D, Camera,
    RenderTexture2D, Texture2D,
    Texture, RenderTexture,
    Vector2, Vector3, Vector4, Quaternion,
    Ray, Matrix, Rectangle, RayHitInfo,
    Font, SpriteFont, CharInfo, Shader,
    Model, Mesh, BoundingBox, Material,
    VrDeviceInfo,
    Sound, Wave, Music, AudioStream
};

pub mod ease;

pub const PI: f64 = std::f64::consts::PI;
pub const DEG2RAD: f64 = (PI / 180.0);
pub const RAD2DEG: f64 = (180.0 / PI);
pub const FLAG_SHOW_LOGO: u8 = 1;
pub const FLAG_FULLSCREEN_MODE: u8 = 2;
pub const FLAG_WINDOW_RESIZABLE: u8 = 4;
pub const FLAG_WINDOW_UNDECORATED: u8 = 8;
pub const FLAG_WINDOW_TRANSPARENT: u8 = 16;
pub const FLAG_MSAA_4X_HINT: u8 = 32;
pub const FLAG_VSYNC_HINT: u8 = 64;
pub const KEY_SPACE: i32 = 32;
pub const KEY_ESCAPE: i32 = 256;
pub const KEY_ENTER: i32 = 257;
pub const KEY_TAB: i32 = 258;
pub const KEY_BACKSPACE: i32 = 259;
pub const KEY_INSERT: i32 = 260;
pub const KEY_DELETE: i32 = 261;
pub const KEY_RIGHT: i32 = 262;
pub const KEY_LEFT: i32 = 263;
pub const KEY_DOWN: i32 = 264;
pub const KEY_UP: i32 = 265;
pub const KEY_PAGE_UP: i32 = 266;
pub const KEY_PAGE_DOWN: i32 = 267;
pub const KEY_HOME: i32 = 268;
pub const KEY_END: i32 = 269;
pub const KEY_CAPS_LOCK: i32 = 280;
pub const KEY_SCROLL_LOCK: i32 = 281;
pub const KEY_NUM_LOCK: i32 = 282;
pub const KEY_PRINT_SCREEN: i32 = 283;
pub const KEY_PAUSE: i32 = 284;
pub const KEY_F1: i32 = 290;
pub const KEY_F2: i32 = 291;
pub const KEY_F3: i32 = 292;
pub const KEY_F4: i32 = 293;
pub const KEY_F5: i32 = 294;
pub const KEY_F6: i32 = 295;
pub const KEY_F7: i32 = 296;
pub const KEY_F8: i32 = 297;
pub const KEY_F9: i32 = 298;
pub const KEY_F10: i32 = 299;
pub const KEY_F11: i32 = 300;
pub const KEY_F12: i32 = 301;
pub const KEY_LEFT_SHIFT: i32 = 340;
pub const KEY_LEFT_CONTROL: i32 = 341;
pub const KEY_LEFT_ALT: i32 = 342;
pub const KEY_RIGHT_SHIFT: i32 = 344;
pub const KEY_RIGHT_CONTROL: i32 = 345;
pub const KEY_RIGHT_ALT: i32 = 346;
pub const KEY_GRAVE: i32 = 96;
pub const KEY_SLASH: i32 = 47;
pub const KEY_BACKSLASH: i32 = 92;
pub const KEY_ZERO: i32 = 48;
pub const KEY_ONE: i32 = 49;
pub const KEY_TWO: i32 = 50;
pub const KEY_THREE: i32 = 51;
pub const KEY_FOUR: i32 = 52;
pub const KEY_FIVE: i32 = 53;
pub const KEY_SIX: i32 = 54;
pub const KEY_SEVEN: i32 = 55;
pub const KEY_EIGHT: i32 = 56;
pub const KEY_NINE: i32 = 57;
pub const KEY_A: i32 = 65;
pub const KEY_B: i32 = 66;
pub const KEY_C: i32 = 67;
pub const KEY_D: i32 = 68;
pub const KEY_E: i32 = 69;
pub const KEY_F: i32 = 70;
pub const KEY_G: i32 = 71;
pub const KEY_H: i32 = 72;
pub const KEY_I: i32 = 73;
pub const KEY_J: i32 = 74;
pub const KEY_K: i32 = 75;
pub const KEY_L: i32 = 76;
pub const KEY_M: i32 = 77;
pub const KEY_N: i32 = 78;
pub const KEY_O: i32 = 79;
pub const KEY_P: i32 = 80;
pub const KEY_Q: i32 = 81;
pub const KEY_R: i32 = 82;
pub const KEY_S: i32 = 83;
pub const KEY_T: i32 = 84;
pub const KEY_U: i32 = 85;
pub const KEY_V: i32 = 86;
pub const KEY_W: i32 = 87;
pub const KEY_X: i32 = 88;
pub const KEY_Y: i32 = 89;
pub const KEY_Z: i32 = 90;
pub const KEY_BACK: i32 = 4;
pub const KEY_MENU: i32 = 82;
pub const KEY_VOLUME_UP: i32 = 24;
pub const KEY_VOLUME_DOWN: i32 = 25;
pub const MOUSE_LEFT_BUTTON: i32 = 0;
pub const MOUSE_RIGHT_BUTTON: i32 = 1;
pub const MOUSE_MIDDLE_BUTTON: i32 = 2;
pub const MAX_TOUCH_POINTS: i32 = 2;
pub const GAMEPAD_PLAYER1: i32 = 0;
pub const GAMEPAD_PLAYER2: i32 = 1;
pub const GAMEPAD_PLAYER3: i32 = 2;
pub const GAMEPAD_PLAYER4: i32 = 3;
pub const GAMEPAD_PS3_BUTTON_TRIANGLE: i32 = 0;
pub const GAMEPAD_PS3_BUTTON_CIRCLE: i32 = 1;
pub const GAMEPAD_PS3_BUTTON_CROSS: i32 = 2;
pub const GAMEPAD_PS3_BUTTON_SQUARE: i32 = 3;
pub const GAMEPAD_PS3_BUTTON_L1: i32 = 6;
pub const GAMEPAD_PS3_BUTTON_R1: i32 = 7;
pub const GAMEPAD_PS3_BUTTON_L2: i32 = 4;
pub const GAMEPAD_PS3_BUTTON_R2: i32 = 5;
pub const GAMEPAD_PS3_BUTTON_START: i32 = 8;
pub const GAMEPAD_PS3_BUTTON_SELECT: i32 = 9;
pub const GAMEPAD_PS3_BUTTON_UP: i32 = 24;
pub const GAMEPAD_PS3_BUTTON_RIGHT: i32 = 25;
pub const GAMEPAD_PS3_BUTTON_DOWN: i32 = 26;
pub const GAMEPAD_PS3_BUTTON_LEFT: i32 = 27;
pub const GAMEPAD_PS3_BUTTON_PS: i32 = 12;
pub const GAMEPAD_PS3_AXIS_LEFT_X: i32 = 0;
pub const GAMEPAD_PS3_AXIS_LEFT_Y: i32 = 1;
pub const GAMEPAD_PS3_AXIS_RIGHT_X: i32 = 2;
pub const GAMEPAD_PS3_AXIS_RIGHT_Y: i32 = 5;
pub const GAMEPAD_PS3_AXIS_L2: i32 = 3;
pub const GAMEPAD_PS3_AXIS_R2: i32 = 4;
pub const GAMEPAD_XBOX_BUTTON_A: i32 = 0;
pub const GAMEPAD_XBOX_BUTTON_B: i32 = 1;
pub const GAMEPAD_XBOX_BUTTON_X: i32 = 2;
pub const GAMEPAD_XBOX_BUTTON_Y: i32 = 3;
pub const GAMEPAD_XBOX_BUTTON_LB: i32 = 4;
pub const GAMEPAD_XBOX_BUTTON_RB: i32 = 5;
pub const GAMEPAD_XBOX_BUTTON_SELECT: i32 = 6;
pub const GAMEPAD_XBOX_BUTTON_START: i32 = 7;
pub const GAMEPAD_XBOX_BUTTON_UP: i32 = 10;
pub const GAMEPAD_XBOX_BUTTON_RIGHT: i32 = 11;
pub const GAMEPAD_XBOX_BUTTON_DOWN: i32 = 12;
pub const GAMEPAD_XBOX_BUTTON_LEFT: i32 = 13;
pub const GAMEPAD_XBOX_BUTTON_HOME: i32 = 8;
pub const GAMEPAD_ANDROID_DPAD_UP: i32 = 19;
pub const GAMEPAD_ANDROID_DPAD_DOWN: i32 = 20;
pub const GAMEPAD_ANDROID_DPAD_LEFT: i32 = 21;
pub const GAMEPAD_ANDROID_DPAD_RIGHT: i32 = 22;
pub const GAMEPAD_ANDROID_DPAD_CENTER: i32 = 23;
pub const GAMEPAD_ANDROID_BUTTON_A: i32 = 96;
pub const GAMEPAD_ANDROID_BUTTON_B: i32 = 97;
pub const GAMEPAD_ANDROID_BUTTON_C: i32 = 98;
pub const GAMEPAD_ANDROID_BUTTON_X: i32 = 99;
pub const GAMEPAD_ANDROID_BUTTON_Y: i32 = 100;
pub const GAMEPAD_ANDROID_BUTTON_Z: i32 = 101;
pub const GAMEPAD_ANDROID_BUTTON_L1: i32 = 102;
pub const GAMEPAD_ANDROID_BUTTON_R1: i32 = 103;
pub const GAMEPAD_ANDROID_BUTTON_L2: i32 = 104;
pub const GAMEPAD_ANDROID_BUTTON_R2: i32 = 105;
pub const GAMEPAD_XBOX_AXIS_LEFT_X: i32 = 0;
pub const GAMEPAD_XBOX_AXIS_LEFT_Y: i32 = 1;
pub const GAMEPAD_XBOX_AXIS_RIGHT_X: i32 = 2;
pub const GAMEPAD_XBOX_AXIS_RIGHT_Y: i32 = 3;
pub const GAMEPAD_XBOX_AXIS_LT: i32 = 4;
pub const GAMEPAD_XBOX_AXIS_RT: i32 = 5;
pub const MAX_SHADER_LOCATIONS: u32 = 32;
pub const MAX_MATERIAL_MAPS: u32 = 12;

pub const LIGHTGRAY  : Color = Color { r: 200, g: 200, b: 200, a: 255 };   // Light Gray
pub const GRAY       : Color = Color { r: 130, g: 130, b: 130, a: 255 };   // Gray
pub const DARKGRAY   : Color = Color { r: 80, g: 80, b: 80, a: 255 };      // Dark Gray
pub const YELLOW     : Color = Color { r: 253, g: 249, b: 0, a: 255 };     // Yellow
pub const GOLD       : Color = Color { r: 255, g: 203, b: 0, a: 255 };     // Gold
pub const ORANGE     : Color = Color { r: 255, g: 161, b: 0, a: 255 };     // Orange
pub const PINK       : Color = Color { r: 255, g: 109, b: 194, a: 255 };   // Pink
pub const RED        : Color = Color { r: 230, g: 41, b: 55, a: 255 };     // Red
pub const MAROON     : Color = Color { r: 190, g: 33, b: 55, a: 255 };     // Maroon
pub const GREEN      : Color = Color { r: 0, g: 228, b: 48, a: 255 };      // Green
pub const LIME       : Color = Color { r: 0, g: 158, b: 47, a: 255 };      // Lime
pub const DARKGREEN  : Color = Color { r: 0, g: 117, b: 44, a: 255 };      // Dark Green
pub const SKYBLUE    : Color = Color { r: 102, g: 191, b: 255, a: 255 };   // Sky Blue
pub const BLUE       : Color = Color { r: 0, g: 121, b: 241, a: 255 };     // Blue
pub const DARKBLUE   : Color = Color { r: 0, g: 82, b: 172, a: 255 };      // Dark Blue
pub const PURPLE     : Color = Color { r: 200, g: 122, b: 255, a: 255 };   // Purple
pub const VIOLET     : Color = Color { r: 135, g: 60, b: 190, a: 255 };    // Violet
pub const DARKPURPLE : Color = Color { r: 112, g: 31, b: 126, a: 255 };    // Dark Purple
pub const BEIGE      : Color = Color { r: 211, g: 176, b: 131, a: 255 };   // Beige
pub const BROWN      : Color = Color { r: 127, g: 106, b: 79, a: 255 };    // Brown
pub const DARKBROWN  : Color = Color { r: 76, g: 63, b: 47, a: 255 };      // Dark Brown

pub const WHITE      : Color = Color { r: 255, g: 255, b: 255, a: 255 };   // White
pub const BLACK      : Color = Color { r: 0, g: 0, b: 0, a: 255 };         // Black
pub const BLANK      : Color = Color { r: 0, g: 0, b: 0, a: 0 };           // Blank (Transparent)
pub const MAGENTA    : Color = Color { r: 255, g: 0, b: 255, a: 255 };     // Magenta
pub const RAYWHITE   : Color = Color { r: 245, g: 245, b: 245, a: 255 };   // My own White (raylib logo)

pub const LOG_INFO: LogType = 1;
pub const LOG_WARNING: LogType = 2;
pub const LOG_ERROR: LogType = 4;
pub const LOG_DEBUG: LogType = 8;
pub const LOG_OTHER: LogType = 16;
pub type LogType = u8;

pub const LOC_VERTEX_POSITION: ShaderLocationIndex = 0;
pub const LOC_VERTEX_TEXCOORD01: ShaderLocationIndex = 1;
pub const LOC_VERTEX_TEXCOORD02: ShaderLocationIndex = 2;
pub const LOC_VERTEX_NORMAL: ShaderLocationIndex = 3;
pub const LOC_VERTEX_TANGENT: ShaderLocationIndex = 4;
pub const LOC_VERTEX_COLOR: ShaderLocationIndex = 5;
pub const LOC_MATRIX_MVP: ShaderLocationIndex = 6;
pub const LOC_MATRIX_MODEL: ShaderLocationIndex = 7;
pub const LOC_MATRIX_VIEW: ShaderLocationIndex = 8;
pub const LOC_MATRIX_PROJECTION: ShaderLocationIndex = 9;
pub const LOC_VECTOR_VIEW: ShaderLocationIndex = 10;
pub const LOC_COLOR_DIFFUSE: ShaderLocationIndex = 11;
pub const LOC_COLOR_SPECULAR: ShaderLocationIndex = 12;
pub const LOC_COLOR_AMBIENT: ShaderLocationIndex = 13;
pub const LOC_MAP_ALBEDO: ShaderLocationIndex = 14;
pub const LOC_MAP_METALNESS: ShaderLocationIndex = 15;
pub const LOC_MAP_NORMAL: ShaderLocationIndex = 16;
pub const LOC_MAP_ROUGHNESS: ShaderLocationIndex = 17;
pub const LOC_MAP_OCCLUSION: ShaderLocationIndex = 18;
pub const LOC_MAP_EMISSION: ShaderLocationIndex = 19;
pub const LOC_MAP_HEIGHT: ShaderLocationIndex = 20;
pub const LOC_MAP_CUBEMAP: ShaderLocationIndex = 21;
pub const LOC_MAP_IRRADIANCE: ShaderLocationIndex = 22;
pub const LOC_MAP_PREFILTER: ShaderLocationIndex = 23;
pub const LOC_MAP_BRDF: ShaderLocationIndex = 24;
pub type ShaderLocationIndex = i32;

pub const MAP_ALBEDO: TexmapIndex = 0;
pub const MAP_METALNESS: TexmapIndex = 1;
pub const MAP_NORMAL: TexmapIndex = 2;
pub const MAP_ROUGHNESS: TexmapIndex = 3;
pub const MAP_OCCLUSION: TexmapIndex = 4;
pub const MAP_EMISSION: TexmapIndex = 5;
pub const MAP_HEIGHT: TexmapIndex = 6;
pub const MAP_CUBEMAP: TexmapIndex = 7;
pub const MAP_IRRADIANCE: TexmapIndex = 8;
pub const MAP_PREFILTER: TexmapIndex = 9;
pub const MAP_BRDF: TexmapIndex = 10;
pub type TexmapIndex = i32;

pub const UNCOMPRESSED_GRAYSCALE: PixelFormat = 1;
pub const UNCOMPRESSED_GRAY_ALPHA: PixelFormat = 2;
pub const UNCOMPRESSED_R5G6B5: PixelFormat = 3;
pub const UNCOMPRESSED_R8G8B8: PixelFormat = 4;
pub const UNCOMPRESSED_R5G5B5A1: PixelFormat = 5;
pub const UNCOMPRESSED_R4G4B4A4: PixelFormat = 6;
pub const UNCOMPRESSED_R8G8B8A8: PixelFormat = 7;
pub const UNCOMPRESSED_R32: PixelFormat = 8;
pub const UNCOMPRESSED_R32G32B32: PixelFormat = 9;
pub const UNCOMPRESSED_R32G32B32A32: PixelFormat = 10;
pub const COMPRESSED_DXT1_RGB: PixelFormat = 11;
pub const COMPRESSED_DXT1_RGBA: PixelFormat = 12;
pub const COMPRESSED_DXT3_RGBA: PixelFormat = 13;
pub const COMPRESSED_DXT5_RGBA: PixelFormat = 14;
pub const COMPRESSED_ETC1_RGB: PixelFormat = 15;
pub const COMPRESSED_ETC2_RGB: PixelFormat = 16;
pub const COMPRESSED_ETC2_EAC_RGBA: PixelFormat = 17;
pub const COMPRESSED_PVRT_RGB: PixelFormat = 18;
pub const COMPRESSED_PVRT_RGBA: PixelFormat = 19;
pub const COMPRESSED_ASTC_4X4_RGBA: PixelFormat = 20;
pub const COMPRESSED_ASTC_8X8_RGBA: PixelFormat = 21;
pub type PixelFormat = i32;

pub const FILTER_POINT: TextureFilterMode = 0;
pub const FILTER_BILINEAR: TextureFilterMode = 1;
pub const FILTER_TRILINEAR: TextureFilterMode = 2;
pub const FILTER_ANISOTROPIC_4X: TextureFilterMode = 3;
pub const FILTER_ANISOTROPIC_8X: TextureFilterMode = 4;
pub const FILTER_ANISOTROPIC_16X: TextureFilterMode = 5;
pub type TextureFilterMode = i32;

pub const WRAP_REPEAT: TextureWrapMode = 0;
pub const WRAP_CLAMP: TextureWrapMode = 1;
pub const WRAP_MIRROR: TextureWrapMode = 2;
pub type TextureWrapMode = i32;

pub const BLEND_ALPHA: BlendMode = 0;
pub const BLEND_ADDITIVE: BlendMode = 1;
pub const BLEND_MULTIPLIED: BlendMode = 2;
pub type BlendMode = i32;

pub const GESTURE_NONE: Gestures = 0;
pub const GESTURE_TAP: Gestures = 1;
pub const GESTURE_DOUBLETAP: Gestures = 2;
pub const GESTURE_HOLD: Gestures = 4;
pub const GESTURE_DRAG: Gestures = 8;
pub const GESTURE_SWIPE_RIGHT: Gestures = 16;
pub const GESTURE_SWIPE_LEFT: Gestures = 32;
pub const GESTURE_SWIPE_UP: Gestures = 64;
pub const GESTURE_SWIPE_DOWN: Gestures = 128;
pub const GESTURE_PINCH_IN: Gestures = 256;
pub const GESTURE_PINCH_OUT: Gestures = 512;
pub type Gestures = u32;

pub const CAMERA_CUSTOM: CameraMode = 0;
pub const CAMERA_FREE: CameraMode = 1;
pub const CAMERA_ORBITAL: CameraMode = 2;
pub const CAMERA_FIRST_PERSON: CameraMode = 3;
pub const CAMERA_THIRD_PERSON: CameraMode = 4;
pub type CameraMode = i32;

pub const CAMERA_PERSPECTIVE: CameraType = 0;
pub const CAMERA_ORTHOGRAPHIC: CameraType = 1;
pub type CameraType = i32;

pub const HMD_DEFAULT_DEVICE: VrDeviceType = 0;
pub const HMD_OCULUS_RIFT_DK2: VrDeviceType = 1;
pub const HMD_OCULUS_RIFT_CV1: VrDeviceType = 2;
pub const HMD_OCULUS_GO: VrDeviceType = 3;
pub const HMD_VALVE_HTC_VIVE: VrDeviceType = 4;
pub const HMD_SONY_PSVR: VrDeviceType = 5;
pub type VrDeviceType = i32;

/// A marker trait specifying an audio sample (`u8`, `i16`, or `f32`).
pub trait AudioSample { }
impl AudioSample for u8 { }
impl AudioSample for i16 { }
impl AudioSample for f32 { }

/// Initializes window and OpenGL context.
pub fn init_window(width: i32, height: i32, title: &str) {
    let c_title = CString::new(title).unwrap();
    unsafe {
        raylib::InitWindow(width, height, c_title.as_ptr());
    }
}

/// Closes window and unloads OpenGL context.
pub fn close_window() {
    unsafe {
        raylib::CloseWindow();
    }
}

/// Checks if window has been initialized successfully.
pub fn is_window_ready() -> bool {
    unsafe {
        raylib::IsWindowReady().is_true()
    }
}

/// Checks if KEY_ESCAPE or Close icon was pressed.
pub fn window_should_close() -> bool {
    unsafe {
        raylib::WindowShouldClose().is_true()
    }
}

/// Checks if window has been minimized (or lost focus).
pub fn is_window_minimized() -> bool {
    unsafe {
        raylib::IsWindowMinimized().is_true()
    }
}

/// Toggles fullscreen mode (only on desktop platforms).
pub fn toggle_fullscreen() {
    unsafe {
        raylib::ToggleFullscreen();
    }
}

/// Sets icon for window (only on desktop platforms).
pub fn set_window_icon(image: Image) {
    unsafe {
        raylib::SetWindowIcon(image);
    }
}

/// Sets title for window (only on desktop platforms).
pub fn set_window_title(title: &str) {
    let c_title = CString::new(title).unwrap();
    unsafe {
        raylib::SetWindowTitle(c_title.as_ptr());
    }
}

/// Sets window position on screen (only on desktop platforms).
pub fn set_window_position(x: i32, y: i32) {
    unsafe {
        raylib::SetWindowPosition(x, y);
    }
}

/// Sets monitor for the current window (fullscreen mode).
pub fn set_window_monitor(monitor: i32) {
    unsafe {
        raylib::SetWindowMonitor(monitor);
    }
}

/// Sets minimum window dimensions (for `FLAG_WINDOW_RESIZABLE`).
pub fn set_window_min_size(width: i32, height: i32) {
    unsafe {
        raylib::SetWindowMinSize(width, height);
    }
}

/// Sets window dimensions.
pub fn set_window_size(width: i32, height: i32) {
    unsafe {
        raylib::SetWindowSize(width, height);
    }
}

/// Gets current screen width.
pub fn get_screen_width() -> i32 {
    unsafe {
        raylib::GetScreenWidth()
    }
}

/// Gets current screen height.
pub fn get_screen_height() -> i32 {
    unsafe {
        raylib::GetScreenHeight()
    }
}

/// Shows mouse cursor.
pub fn show_cursor() {
    unsafe {
        raylib::ShowCursor();
    }
}

/// Hides mouse cursor.
pub fn hide_cursor() {
    unsafe {
        raylib::HideCursor();
    }
}

/// Checks if mouse cursor is not visible.
pub fn is_cursor_hidden() -> bool {
    unsafe {
        raylib::IsCursorHidden().is_true()
    }
}

/// Enables mouse cursor (unlock cursor).
pub fn enable_cursor() {
    unsafe {
        raylib::EnableCursor();
    }
}

/// Disables mouse cursor (lock cursor).
pub fn disable_cursor() {
    unsafe {
        raylib::DisableCursor();
    }
}

/// Sets background color (framebuffer clear color).
pub fn clear_background(color: Color) {
    unsafe {
        raylib::ClearBackground(color);
    }
}

/// Sets up canvas (framebuffer) to start drawing.
pub fn begin_drawing() {
    unsafe {
        raylib::BeginDrawing();
    }
}

/// Ends canvas drawing and swaps buffers (double buffering).
pub fn end_drawing() {
    unsafe {
        raylib::EndDrawing();
    }
}

/// Initializes 2D mode with custom camera (2D).
pub fn begin_mode_2d(camera: Camera2D) {
    unsafe {
        raylib::BeginMode2D(camera);
    }
}

/// Ends 2D mode with custom camera.
pub fn end_mode_2d() {
    unsafe {
        raylib::EndMode2D();
    }
}

/// Initializes 3D mode with custom camera (3D).
pub fn begin_mode_3d(camera: Camera3D) {
    unsafe {
        raylib::BeginMode3D(camera);
    }
}

/// Ends 3D mode and returns to default 2D orthographic mode.
pub fn end_mode_3d() {
    unsafe {
        raylib::EndMode3D();
    }
}

/// Initializes render texture for drawing.
pub fn begin_texture_mode(target: RenderTexture2D) {
    unsafe {
        raylib::BeginTextureMode(target);
    }
}

/// Ends drawing to render texture.
pub fn end_texture_mode() {
    unsafe {
        raylib::EndTextureMode();
    }
}

/// Returns a ray trace from mouse position.
pub fn get_mouse_ray(mouse_position: Vector2, camera: Camera3D) -> Ray {
    unsafe {
        raylib::GetMouseRay(mouse_position, camera)
    }
}

/// Returns the screen space position for a 3D world space position.
pub fn get_world_to_screen(position: Vector3, camera: Camera3D) -> Vector2 {
    unsafe {
        raylib::GetWorldToScreen(position, camera)
    }
}

/// Returns camera transform matrix (view matrix).
pub fn get_camera_matrix(camera: Camera3D) -> Matrix {
    unsafe {
        raylib::GetCameraMatrix(camera)
    }
}

/// Sets target FPS (maximum).
pub fn set_target_fps(fps: i32) {
    unsafe {
        raylib::SetTargetFPS(fps);
    }
}

/// Returns current FPS.
pub fn get_fps() -> i32 {
    unsafe {
        raylib::GetFPS()
    }
}

/// Returns time in seconds for last frame drawn.
pub fn get_frame_time() -> f32 {
    unsafe {
        raylib::GetFrameTime()
    }
}

/// Returns elapsed time in seconds since `init_window()` was called.
pub fn get_time() -> f64 {
    unsafe {
        raylib::GetTime()
    }
}

/// Returns hexadecimal value for a Color.
pub fn color_to_int(color: Color) -> i32 {
    unsafe {
        raylib::ColorToInt(color)
    }
}

/// Returns color normalized as `f32` [0..1].
pub fn color_normalize(color: Color) -> Vector4 {
    unsafe {
        raylib::ColorNormalize(color)
    }
}

/// Returns HSV values for a Color.
pub fn color_to_hsv(color: Color) -> Vector3 {
    unsafe {
        raylib::ColorToHSV(color)
    }
}

/// Returns a Color struct from hexadecimal value.
pub fn get_color(hex_value: i32) -> Color {
    unsafe {
        raylib::GetColor(hex_value)
    }
}

/// Color fade-in or fade-out, `alpha` goes from `0.0` to `1.0`.
pub fn fade(color: Color, alpha: f32) -> Color {
    unsafe {
        raylib::Fade(color, alpha)
    }
}

/// Activates raylib logo at startup (can be done with flags).
pub fn show_logo() {
    unsafe {
        raylib::ShowLogo();
    }
}

/// Sets window configuration flags (using `FLAG` consts).
pub fn set_config_flags(flags: u8) {
    unsafe {
        raylib::SetConfigFlags(flags);
    }
}

#[allow(non_upper_case_globals)]
static mut log_type_flags: LogType = LOG_INFO | LOG_WARNING | LOG_ERROR;

/// Enables trace log message types (bit flags based).
pub fn set_trace_log(types: LogType) {
    unsafe {
        log_type_flags = types;
        raylib::SetTraceLog(types);
    }
}

/// Writes a trace log message (`LOG_INFO`, `LOG_WARNING`, `LOG_ERROR`, `LOG_DEBUG`).
pub fn trace_log(msg_type: LogType, text: &str) {
    unsafe {
        if (log_type_flags & msg_type) == 0 {
            return;
        }
    }
    
    let mut output = String::new();
    output += match msg_type {
        LOG_INFO => "INFO: ",
        LOG_ERROR => "ERROR: ",
        LOG_WARNING => "WARNING: ",
        LOG_DEBUG => "DEBUG: ",
        _ => ""
    };
    
    output += text;
    println!("{}", output);

    if msg_type == LOG_ERROR {
        std::process::exit(1);
    }
}

/// Takes a screenshot of current screen (in PNG format).
pub fn take_screenshot(filename: &str) {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::TakeScreenshot(c_filename.as_ptr());
    }
}

/// Returns a random value between min and max (both included).
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe {
        raylib::GetRandomValue(min, max)
    }
}

/// Checks if `filename` has an `ext` extension.
pub fn is_file_extension(filename: &str, ext: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    let c_ext = CString::new(ext).unwrap();
    unsafe {
        raylib::IsFileExtension(c_filename.as_ptr(), c_ext.as_ptr()).is_true()
    }
}

/// Gets the extension for a `filename` string.
pub fn get_extension(filename: &str) -> String {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let ext = raylib::GetExtension(c_filename.as_ptr());
        CStr::from_ptr(ext).to_str().unwrap().to_owned()
    }
}

/// Gets the filename for a path string.
pub fn get_file_name(file_path: &str) -> String {
    let c_file_path = CString::new(file_path).unwrap();
    unsafe {
        let filename = raylib::GetFileName(c_file_path.as_ptr());
        CStr::from_ptr(filename).to_str().unwrap().to_owned()
    }
}

/// Gets full path for a given `filename`.
pub fn get_directory_path(filename: &str) -> String {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let dirpath = raylib::GetDirectoryPath(c_filename.as_ptr());
        CStr::from_ptr(dirpath).to_str().unwrap().to_owned()
    }
}

/// Gets current working directory.
pub fn get_working_directory() -> String {
    unsafe {
        let workdir = raylib::GetWorkingDirectory();
        CStr::from_ptr(workdir).to_str().unwrap().to_owned()
    }
}

/// Changes working directory, returns true on success.
pub fn change_directory(dir: &str) -> bool {
    let c_dir = CString::new(dir).unwrap();
    unsafe {
        raylib::ChangeDirectory(c_dir.as_ptr()).is_true()
    }
}

/// Checks if a file has been dropped into the window.
pub fn is_file_dropped() -> bool {
    unsafe {
        raylib::IsFileDropped().is_true()
    }
}

/// Gets dropped filenames.
pub fn get_dropped_files() -> Vec<String> {
    let mut v = Vec::new();
    unsafe {
        let mut count: i32 = 0;
        let dropfiles = raylib::GetDroppedFiles(&mut count);
        for i in 0..count {
            let filestr = CStr::from_ptr(*dropfiles.offset(i as isize)).to_str().unwrap();
            let file = String::from(filestr);
            v.push(file);
        }
    }
    v
}

/// Clears dropped files paths buffer.
pub fn clear_dropped_files() {
    unsafe {
        raylib::ClearDroppedFiles();
    }
}

/// Saves integer value to storage file (to defined `position`).
pub fn storage_save_value(position: i32, value: i32) {
    unsafe {
        raylib::StorageSaveValue(position, value);
    }
}

/// Loads integer value from storage file (from defined `position`).
pub fn storage_load_value(position: i32) -> i32 {
    unsafe {
        raylib::StorageLoadValue(position)
    }
}

/// Detect if a key has been pressed once.
pub fn is_key_pressed(key: i32) -> bool {
    unsafe {
        raylib::IsKeyPressed(key).is_true()
    }
}

/// Detect if a key is being pressed.
pub fn is_key_down(key: i32) -> bool {
    unsafe {
        raylib::IsKeyDown(key).is_true()
    }
}

/// Detect if a key has been released once.
pub fn is_key_released(key: i32) -> bool {
    unsafe {
        raylib::IsKeyReleased(key).is_true()
    }
}

/// Detect if a key is NOT being pressed.
pub fn is_key_up(key: i32) -> bool {
    unsafe {
        raylib::IsKeyUp(key).is_true()
    }
}

/// Gets latest key pressed.
pub fn get_key_pressed() -> i32 {
    unsafe {
        raylib::GetKeyPressed()
    }
}

/// Sets a custom key to exit program (default is ESC).
pub fn set_exit_key(key: i32) {
    unsafe {
        raylib::SetExitKey(key);
    }
}

/// Detect if a gamepad is available.
pub fn is_gamepad_available(gamepad: i32) -> bool {
    unsafe {
        raylib::IsGamepadAvailable(gamepad).is_true()
    }
}

/// Checks gamepad name (if available).
pub fn is_gamepad_name(gamepad: i32, name: &str) -> bool {
    let c_name = CString::new(name).unwrap();
    unsafe {
        raylib::IsGamepadName(gamepad, c_name.as_ptr()).is_true()
    }
}

/// Returns gamepad internal name id.
pub fn get_gamepad_name(gamepad: i32) -> Option<String> {
    unsafe {
        let name = raylib::GetGamepadName(gamepad);
        match name.is_null() {
            false => Some(CStr::from_ptr(name).to_str().unwrap().to_owned()),
            true => None
        }
    }
}

/// Detect if a gamepad button has been pressed once.
pub fn is_gamepad_button_pressed(gamepad: i32, button: i32) -> bool {
    unsafe {
        raylib::IsGamepadButtonPressed(gamepad, button).is_true()
    }
}

/// Detect if a gamepad button is being pressed.
pub fn is_gamepad_button_down(gamepad: i32, button: i32) -> bool {
    unsafe {
        raylib::IsGamepadButtonDown(gamepad, button).is_true()
    }
}

/// Detect if a gamepad button has been released once.
pub fn is_gamepad_button_released(gamepad: i32, button: i32) -> bool {
    unsafe {
        raylib::IsGamepadButtonReleased(gamepad, button).is_true()
    }
}

/// Detect if a gamepad button is NOT being pressed.
pub fn is_gamepad_button_up(gamepad: i32, button: i32) -> bool {
    unsafe {
        raylib::IsGamepadButtonUp(gamepad, button).is_true()
    }
}

/// Gets the last gamepad button pressed.
pub fn get_gamepad_button_pressed() -> i32 {
    unsafe {
        raylib::GetGamepadButtonPressed()
    }
}

/// Returns gamepad axis count for a gamepad.
pub fn get_gamepad_axis_count(gamepad: i32) -> i32 {
    unsafe {
        raylib::GetGamepadAxisCount(gamepad)
    }
}

/// Returns axis movement value for a gamepad axis.
pub fn get_gamepad_axis_movement(gamepad: i32, axis: i32) -> f32 {
    unsafe {
        raylib::GetGamepadAxisMovement(gamepad, axis)
    }
}

/// Detect if a mouse button has been pressed once.
pub fn is_mouse_button_pressed(button: i32) -> bool {
    unsafe {
        raylib::IsMouseButtonPressed(button).is_true()
    }
}

/// Detect if a mouse button is being pressed.
pub fn is_mouse_button_down(button: i32) -> bool {
    unsafe {
        raylib::IsMouseButtonDown(button).is_true()
    }
}

/// Detect if a mouse button has been released once.
pub fn is_mouse_button_released(button: i32) -> bool {
    unsafe {
        raylib::IsMouseButtonReleased(button).is_true()
    }
}

/// Detect if a mouse button is NOT being pressed.
pub fn is_mouse_button_up(button: i32) -> bool {
    unsafe {
        raylib::IsMouseButtonUp(button).is_true()
    }
}

/// Returns mouse position X.
pub fn get_mouse_x() -> i32 {
    unsafe {
        raylib::GetMouseX()
    }
}

/// Returns mouse position Y.
pub fn get_mouse_y() -> i32 {
    unsafe {
        raylib::GetMouseY()
    }
}

/// Returns mouse position.
pub fn get_mouse_position() -> Vector2 {
    unsafe {
        raylib::GetMousePosition()
    }
}

/// Sets mouse position.
pub fn set_mouse_position(position: Vector2) {
    unsafe {
        raylib::SetMousePosition(position);
    }
}

/// Sets mouse scaling.
pub fn set_mouse_scale(scale: f32) {
    unsafe {
        raylib::SetMouseScale(scale);
    }
}

/// Returns mouse wheel movement Y.
pub fn get_mouse_wheel_move() -> i32 {
    unsafe {
        raylib::GetMouseWheelMove()
    }
}

/// Returns touch position X for touch point 0 (relative to screen size).
pub fn get_touch_x() -> i32 {
    unsafe {
        raylib::GetTouchX()
    }
}

/// Returns touch position Y for touch point 0 (relative to screen size).
pub fn get_touch_y() -> i32 {
    unsafe {
        raylib::GetTouchY()
    }
}

/// Returns touch position XY for a touch point index (relative to screen size).
pub fn get_touch_position(index: i32) -> Vector2 {
    unsafe {
        raylib::GetTouchPosition(index)
    }
}

/// Enables a set of gestures using flags.
pub fn set_gestures_enabled(gesture_flags: Gestures) {
    unsafe {
        raylib::SetGesturesEnabled(gesture_flags);
    }
}

/// Checks if a gesture have been detected.
pub fn is_gesture_detected(gesture: Gestures) -> bool {
    unsafe {
        raylib::IsGestureDetected(gesture as i32).is_true()
    }
}

/// Gets latest detected gesture.
pub fn get_gesture_detected() -> i32 {
    unsafe {
        raylib::GetGestureDetected()
    }
}

/// Gets touch points count.
pub fn get_touch_points_count() -> i32 {
    unsafe {
        raylib::GetTouchPointsCount()
    }
}

/// Gets gesture hold time in milliseconds.
pub fn get_gesture_hold_duration() -> f32 {
    unsafe {
        raylib::GetGestureHoldDuration()
    }
}

/// Gets gesture drag vector.
pub fn get_gesture_drag_vector() -> Vector2 {
    unsafe {
        raylib::GetGestureDragVector()
    }
}

/// Gets gesture drag angle.
pub fn get_gesture_drag_angle() -> f32 {
    unsafe {
        raylib::GetGestureDragAngle()
    }
}

/// Gets gesture pinch delta.
pub fn get_gesture_pinch_vector() -> Vector2 {
    unsafe {
        raylib::GetGesturePinchVector()
    }
}

/// Gets gesture pinch angle.
pub fn get_gesture_pinch_angle() -> f32 {
    unsafe {
        raylib::GetGesturePinchAngle()
    }
}

/// Sets camera mode.
pub fn set_camera_mode(camera: Camera3D, mode: CameraMode) {
    unsafe {
        raylib::SetCameraMode(camera, mode);
    }
}

/// Updates camera position for selected mode.
pub fn update_camera(camera: &mut Camera3D) {
    unsafe {
        raylib::UpdateCamera(camera);
    }
}

/// Sets camera pan key to combine with mouse movement (free camera).
pub fn set_camera_pan_control(pan_key: i32) {
    unsafe {
        raylib::SetCameraPanControl(pan_key);
    }
}

/// Sets camera alt key to combine with mouse movement (free camera).
pub fn set_camera_alt_control(alt_key: i32) {
    unsafe {
        raylib::SetCameraAltControl(alt_key);
    }
}

/// Sets camera smooth zoom key to combine with mouse (free camera).
pub fn set_camera_smooth_zoom_control(sz_key: i32) {
    unsafe {
        raylib::SetCameraSmoothZoomControl(sz_key);
    }
}

/// Sets camera move controls (1st person and 3rd person cameras).
pub fn set_camera_move_controls(
    front_key: i32,
    back_key: i32,
    right_key: i32,
    left_key: i32,
    up_key: i32,
    down_key: i32) {
    unsafe {
        raylib::SetCameraMoveControls(front_key, back_key, right_key, left_key, up_key, down_key);
    }
}

/// Draws a pixel.
pub fn draw_pixel(x: i32, y: i32, color: Color) {
    unsafe {
        raylib::DrawPixel(x, y, color);
    }
}

/// Draws a pixel (Vector version).
pub fn draw_pixel_v(position: Vector2, color: Color) {
    unsafe {
        raylib::DrawPixelV(position, color);
    }
}

/// Draws a line.
pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
    unsafe {
        raylib::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color);
    }
}

/// Draws a line (Vector version).
pub fn draw_line_v(start_pos: Vector2, end_pos: Vector2, color: Color) {
    unsafe {
        raylib::DrawLineV(start_pos, end_pos, color);
    }
}

/// Draws a line with thickness.
pub fn draw_line_ex(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe {
        raylib::DrawLineEx(start_pos, end_pos, thick, color);
    }
}

/// Draws a line using cubic-bezier curves in-out.
pub fn draw_line_bezier(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe {
        raylib::DrawLineBezier(start_pos, end_pos, thick, color);
    }
}

/// Draws a color-filled circle.
pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe {
        raylib::DrawCircle(center_x, center_y, radius, color);
    }
}

/// Draws a gradient-filled circle.
pub fn draw_circle_gradient(center_x: i32, center_y: i32, radius: f32, color1: Color, color2: Color) {
    unsafe {
        raylib::DrawCircleGradient(center_x, center_y, radius, color1, color2);
    }
}

/// Draws a color-filled circle (Vector version).
pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    unsafe {
        raylib::DrawCircleV(center, radius, color);
    }
}

/// Draws circle outline.
pub fn draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe {
        raylib::DrawCircleLines(center_x, center_y, radius, color);
    }
}

/// Draws a color-filled rectangle.
pub fn draw_rectangle(x: i32, y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        raylib::DrawRectangle(x, y, width, height, color);
    }
}

/// Draws a color-filled rectangle (Vector version).
pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
    unsafe {
        raylib::DrawRectangleV(position, size, color);
    }
}

/// Draws a color-filled rectangle from `rec`.
pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    unsafe {
        raylib::DrawRectangleRec(rec, color);
    }
}

/// Draws a color-filled rectangle with pro parameters.
pub fn draw_rectangle_pro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
    unsafe {
        raylib::DrawRectanglePro(rec, origin, rotation, color);
    }
}

/// Draws a vertical-gradient-filled rectangle.
/// 
/// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
pub fn draw_rectangle_gradient_v(x: i32, y: i32, width: i32, height: i32, color1: Color, color2: Color) {
    unsafe {
        raylib::DrawRectangleGradientV(x, y, width, height, color1, color2);
    }
}

/// Draws a horizontal-gradient-filled rectangle.
/// 
/// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
pub fn draw_rectangle_gradient_h(x: i32, y: i32, width: i32, height: i32, color1: Color, color2: Color) {
    unsafe {
        raylib::DrawRectangleGradientH(x, y, width, height, color1, color2);
    }
}

/// Draws a gradient-filled rectangle with custom vertex colors.
/// 
/// **NOTE**: Colors refer to corners, starting at top-left corner and going counter-clockwise.
pub fn draw_rectangle_gradient_ex(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color) {
    unsafe {
        raylib::DrawRectangleGradientEx(rec, col1, col2, col3, col4);
    }
}

/// Draws rectangle outline.
pub fn draw_rectangle_lines(x: i32, y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        raylib::DrawRectangleLines(x, y, width, height, color);
    }
}

/// Draws rectangle outline with extended parameters.
pub fn draw_rectangle_lines_ex(rec: Rectangle, line_thick: i32, color: Color) {
    unsafe {
        raylib::DrawRectangleLinesEx(rec, line_thick, color);
    }
}

/// Draws a triangle.
pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe {
        raylib::DrawTriangle(v1, v2, v3, color);
    }
}

/// Draws a triangle using lines.
pub fn draw_triangle_lines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe {
        raylib::DrawTriangleLines(v1, v2, v3, color);
    }
}

/// Draws a regular polygon of n sides (Vector version).
pub fn draw_poly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
    unsafe {
        raylib::DrawPoly(center, sides, radius, rotation, color);
    }
}

/// Draws a closed polygon defined by points.
pub fn draw_poly_ex(points: &mut [Vector2], color: Color) {
    unsafe {
        raylib::DrawPolyEx(points.as_mut_ptr(), points.len() as i32, color);
    }
}

/// Draws a polygon using lines.
pub fn draw_poly_ex_lines(points: &mut [Vector2], color: Color) {
    unsafe {
        raylib::DrawPolyExLines(points.as_mut_ptr(), points.len() as i32, color);
    }
}

/// Checks collision between two rectangles.
pub fn check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
    unsafe {
        raylib::CheckCollisionRecs(rec1, rec2).is_true()
    }
}

/// Checks collision between two circles.
pub fn check_collision_circles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32) -> bool {
    unsafe {
        raylib::CheckCollisionCircles(center1, radius1, center2, radius2).is_true()
    }
}

/// Checks collision between circle and rectangle.
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    unsafe {
        raylib::CheckCollisionCircleRec(center, radius, rec).is_true()
    }
}

/// Gets the overlap between two colliding rectangles.
pub fn get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
    unsafe {
        raylib::GetCollisionRec(rec1, rec2)
    }
}

/// Checks if point is inside rectangle.
pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
    unsafe {
        raylib::CheckCollisionPointRec(point, rec).is_true()
    }
}

/// Checks if point is inside circle.
pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    unsafe {
        raylib::CheckCollisionPointCircle(point, center, radius).is_true()
    }
}

/// Checks if point is inside a triangle.
pub fn check_collision_point_triangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2) -> bool {
    unsafe {
        raylib::CheckCollisionPointTriangle(point, p1, p2, p3).is_true()
    }
}

/// Loads image from file into CPU memory (RAM).
pub fn load_image(filename: &str) -> Image {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadImage(c_filename.as_ptr())
    }
}

/// Loads image from Color array data (RGBA - 32bit).
pub fn load_image_ex(pixels: &mut [Color], width: i32, height: i32) -> Image {
    let expected_len = (width * height) as usize;
    if pixels.len() != expected_len {
        panic!("load_image_ex: Data is wrong size. Expected {}, got {}", expected_len, pixels.len());
    }
    unsafe {
        raylib::LoadImageEx(pixels.as_mut_ptr(), width, height)
    }
}

/// Loads image from raw data with parameters.
pub fn load_image_pro(data: &mut [u8], width: i32, height: i32, format: PixelFormat) -> Image {
    let expected_len = get_pixel_data_size(width, height, format) as usize;
    if data.len() != expected_len {
        panic!("load_image_pro: Data is wrong size. Expected {}, got {}", expected_len, data.len());
    }
    unsafe {
        raylib::LoadImagePro(data.as_mut_ptr() as *mut std::os::raw::c_void, width, height, format)
    }
}

/// Loads image from RAW file data.
pub fn load_image_raw(filename: &str, width: i32, height: i32, format: i32, header_size: i32) -> Image {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadImageRaw(c_filename.as_ptr(), width, height, format, header_size)
    }
}

/// Exports image as a PNG file.
pub fn export_image(filename: &str, image: Image) {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::ExportImage(c_filename.as_ptr(), image);
    }
}

/// Loads texture from file into GPU memory (VRAM).
pub fn load_texture(filename: &str) -> Texture2D {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadTexture(c_filename.as_ptr())
    }
}

/// Loads texture from image data.
pub fn load_texture_from_image(image: Image) -> Texture2D {
    unsafe {
        raylib::LoadTextureFromImage(image)
    }
}

/// Loads texture for rendering (framebuffer).
pub fn load_render_texture(width: i32, height: i32) -> RenderTexture2D {
    unsafe {
        raylib::LoadRenderTexture(width, height)
    }
}

/// Unloads image from CPU memory (RAM).
pub fn unload_image(image: Image) {
    unsafe {
        raylib::UnloadImage(image);
    }
}

/// Unloads texture from GPU memory (VRAM).
pub fn unload_texture(texture: Texture2D) {
    unsafe {
        raylib::UnloadTexture(texture);
    }
}

/// Unloads render texture from GPU memory (VRAM).
pub fn unload_render_texture(target: RenderTexture2D) {
    unsafe {
        raylib::UnloadRenderTexture(target);
    }
}

/// Gets pixel data from `image` as a Vec of Color structs.
pub fn get_image_data(image: Image) -> Vec<Color> {
    unsafe {
        let image_data = raylib::GetImageData(image);
        let image_data_len = (image.width * image.height) as usize;
        let mut safe_image_data = Vec::with_capacity(image_data_len);
        safe_image_data.set_len(image_data_len);
        std::ptr::copy(image_data, safe_image_data.as_mut_ptr(), image_data_len);
        libc::free(image_data as *mut libc::c_void);
        safe_image_data
    }
}

/// Gets normalized (`0.0` to `1.0`) pixel data from `image` as a Vec of Vector4 structs.
pub fn get_image_data_normalized(image: Image) -> Vec<Vector4> {
    unsafe {
        let image_data = raylib::GetImageDataNormalized(image);
        let image_data_len = (image.width * image.height) as usize;
        let mut safe_image_data = Vec::with_capacity(image_data_len);
        safe_image_data.set_len(image_data_len);
        std::ptr::copy(image_data, safe_image_data.as_mut_ptr(), image_data_len);
        libc::free(image_data as *mut libc::c_void);
        safe_image_data
    }
}

/// Gets pixel data size in bytes (image or texture).
pub fn get_pixel_data_size(width: i32, height: i32, format: PixelFormat) -> i32 {
    unsafe {
        raylib::GetPixelDataSize(width, height, format)
    }
}

/// Gets pixel data from GPU texture and returns an `Image`.
pub fn get_texture_data(texture: Texture2D) -> Image {
    unsafe {
        raylib::GetTextureData(texture)
    }
}

/// Updates GPU texture with new data.
pub fn update_texture(texture: Texture2D, pixels: &[u8]) {
    let expected_len = get_pixel_data_size(texture.width, texture.height, texture.format) as usize;
    if pixels.len() != expected_len {
        panic!("update_texture: Data is wrong size. Expected {}, got {}", expected_len, pixels.len());
    }
    unsafe {
        raylib::UpdateTexture(texture, pixels.as_ptr() as *const std::os::raw::c_void);
    }
}

/// Creates an image duplicate (useful for transformations).
pub fn image_copy(image: Image) -> Image {
    unsafe {
        raylib::ImageCopy(image)
    }
}

/// Converts `image` to POT (power-of-two).
pub fn image_to_pot(image: &mut Image, fill_color: Color) {
    unsafe {
        raylib::ImageToPOT(image, fill_color);
    }
}

/// Converts `image` data to desired pixel format.
pub fn image_format(image: &mut Image, new_format: PixelFormat) {
    unsafe {
        raylib::ImageFormat(image, new_format);
    }
}

/// Applies alpha mask to `image`.
pub fn image_alpha_mask(image: &mut Image, alpha_mask: Image) {
    unsafe {
        raylib::ImageAlphaMask(image, alpha_mask);
    }
}

/// Clears alpha channel on `image` to desired color.
pub fn image_alpha_clear(image: &mut Image, color: Color, threshold: f32) {
    unsafe {
        raylib::ImageAlphaClear(image, color, threshold);
    }
}

/// Crops `image` depending on alpha value.
pub fn image_alpha_crop(image: &mut Image, threshold: f32) {
    unsafe {
        raylib::ImageAlphaCrop(image, threshold);
    }
}

/// Premultiplies alpha channel on `image`.
pub fn image_alpha_premultiply(image: &mut Image) {
    unsafe {
        raylib::ImageAlphaPremultiply(image);
    }
}

/// Crops `image` to a defined rectangle.
pub fn image_crop(image: &mut Image, crop: Rectangle) {
    unsafe {
        raylib::ImageCrop(image, crop);
    }
}

/// Resizes `image` (bilinear filtering).
pub fn image_resize(image: &mut Image, new_width: i32, new_height: i32) {
    unsafe {
        raylib::ImageResize(image, new_width, new_height);
    }
}

/// Resizes `image` (nearest-neighbor scaling).
pub fn image_resize_nn(image: &mut Image, new_width: i32, new_height: i32) {
    unsafe {
        raylib::ImageResizeNN(image, new_width, new_height);
    }
}

/// Resizes `image` canvas and fills with `color`.
pub fn image_resize_canvas(image: &mut Image, new_width: i32, new_height: i32, offset_x: i32, offset_y: i32, color: Color) {
    unsafe {
        raylib::ImageResizeCanvas(image, new_width, new_height, offset_x, offset_y, color);
    }
}

/// Generates all mipmap levels for a provided `image`.
pub fn image_mipmaps(image: &mut Image) {
    unsafe {
        raylib::ImageMipmaps(image);
    }
}

/// Dithers `image` data to 16bpp or lower (Floyd-Steinberg dithering).
pub fn image_dither(image: &mut Image, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
    unsafe {
        raylib::ImageDither(image, r_bpp, g_bpp, b_bpp, a_bpp);
    }
}

/// Creates an image from `text` (default font).
pub fn image_text(text: &str, font_size: i32, color: Color) -> Image {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::ImageText(c_text.as_ptr(), font_size, color)
    }
}

/// Creates an image from `text` (custom font).
pub fn image_text_ex(font: Font, text: &str, font_size: f32, spacing: f32, tint: Color) -> Image {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::ImageTextEx(font, c_text.as_ptr(), font_size, spacing, tint)
    }
}

/// Draws a source image within a destination image.
pub fn image_draw(dst: &mut Image, src: Image, src_rec: Rectangle, dst_rec: Rectangle) {
    unsafe {
        raylib::ImageDraw(dst, src, src_rec, dst_rec);
    }
}

/// Draws a rectangle within an image.
pub fn image_draw_rectangle(dst: &mut Image, position: Vector2, rec: Rectangle, color: Color) {
    unsafe {
        raylib::ImageDrawRectangle(dst, position, rec, color);
    }
}

/// Draws text (default font) within an image (destination).
pub fn image_draw_text(dst: &mut Image, position: Vector2, text: &str, font_size: i32, color: Color) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::ImageDrawText(dst, position, c_text.as_ptr(), font_size, color);
    }
}

/// Draws text (custom font) within an image (destination).
pub fn image_draw_text_ex(dst: &mut Image, position: Vector2, font: Font, text: &str, font_size: f32, spacing: f32, color: Color) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::ImageDrawTextEx(dst, position, font, c_text.as_ptr(), font_size, spacing, color);
    }
}

/// Flips `image` vertically.
pub fn image_flip_vertical(image: &mut Image) {
    unsafe {
        raylib::ImageFlipVertical(image);
    }
}

/// Flips `image` horizontally.
pub fn image_flip_horizontal(image: &mut Image) {
    unsafe {
        raylib::ImageFlipHorizontal(image);
    }
}

/// Rotates `image` clockwise by 90 degrees (PI/2 radians).
pub fn image_rotate_cw(image: &mut Image) {
    unsafe {
        raylib::ImageRotateCW(image);
    }
}

/// Rotates `image` counterclockwise by 90 degrees (PI/2 radians).
pub fn image_rotate_ccw(image: &mut Image) {
    unsafe {
        raylib::ImageRotateCCW(image);
    }
}

/// Tints colors in `image` using specified `color`.
pub fn image_color_tint(image: &mut Image, color: Color) {
    unsafe {
        raylib::ImageColorTint(image, color);
    }
}

/// Inverts the colors in `image`.
pub fn image_color_invert(image: &mut Image) {
    unsafe {
        raylib::ImageColorInvert(image);
    }
}

/// Converts `image color to grayscale.
pub fn image_color_grayscale(image: &mut Image) {
    unsafe {
        raylib::ImageColorGrayscale(image);
    }
}

/// Adjusts the contrast of `image`.
pub fn image_color_contrast(image: &mut Image, contrast: f32) {
    unsafe {
        raylib::ImageColorContrast(image, contrast);
    }
}

/// Adjusts the brightness of `image`.
pub fn image_color_brightness(image: &mut Image, brightness: i32) {
    unsafe {
        raylib::ImageColorBrightness(image, brightness);
    }
}

/// Searches `image` for all occurences of `color` and replaces them with `replace` color.
pub fn image_color_replace(image: &mut Image, color: Color, replace: Color) {
    unsafe {
        raylib::ImageColorReplace(image, color, replace);
    }
}

/// Generates a plain `color` Image.
pub fn gen_image_color(width: i32, height: i32, color: Color) -> Image {
    unsafe {
        raylib::GenImageColor(width, height, color)
    }
}

/// Generates an Image containing a vertical gradient.
pub fn gen_image_gradient_v(width: i32, height: i32, top: Color, bottom: Color) -> Image {
    unsafe {
        raylib::GenImageGradientV(width, height, top, bottom)
    }
}

/// Generates an Image containing a horizonal gradient.
pub fn gen_image_gradient_h(width: i32, height: i32, left: Color, right: Color) -> Image {
    unsafe {
        raylib::GenImageGradientH(width, height, left, right)
    }
}

/// Generates an Image containing a radial gradient.
pub fn gen_image_gradient_radial(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Image {
    unsafe {
        raylib::GenImageGradientRadial(width, height, density, inner, outer)
    }
}

/// Generates an Image containing a checkerboard pattern.
pub fn gen_image_checked(width: i32, height: i32, checks_x: i32, checks_y: i32, col1: Color, col2: Color) -> Image {
    unsafe {
        raylib::GenImageChecked(width, height, checks_x, checks_y, col1, col2)
    }
}

/// Generates an Image containing white noise.
pub fn gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
    unsafe {
        raylib::GenImageWhiteNoise(width, height, factor)
    }
}

/// Generates an Image containing perlin noise.
pub fn gen_image_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Image {
    unsafe {
        raylib::GenImagePerlinNoise(width, height, offset_x, offset_y, scale)
    }
}

/// Generates an Image using a cellular algorithm. Bigger `tile_size` means bigger cells.
pub fn gen_image_cellular(width: i32, height: i32, tile_size: i32) -> Image {
    unsafe {
        raylib::GenImageCellular(width, height, tile_size)
    }
}

/// Generates GPU mipmaps for a `texture`.
pub fn gen_texture_mipmaps(texture: &mut Texture2D) {
    unsafe {
        raylib::GenTextureMipmaps(texture);
    }
}

/// Sets `texture` scaling filter mode.
pub fn set_texture_filter(texture: Texture2D, filter_mode: TextureFilterMode) {
    unsafe {
        raylib::SetTextureFilter(texture, filter_mode);
    }
}

/// Sets texture wrapping mode.
pub fn set_texture_wrap(texture: Texture2D, wrap_mode: TextureWrapMode) {
    unsafe {
        raylib::SetTextureWrap(texture, wrap_mode);
    }
}

/// Draws a `texture` using specified position and `tint` color.
pub fn draw_texture(texture: Texture2D, x: i32, y: i32, tint: Color) {
    unsafe {
        raylib::DrawTexture(texture, x, y, tint);
    }
}

/// Draws a `texture` using specified `position` vector and `tint` color.
pub fn draw_texture_v(texture: Texture2D, position: Vector2, tint: Color) {
    unsafe {
        raylib::DrawTextureV(texture, position, tint);
    }
}

/// Draws a `texture` with extended parameters.
pub fn draw_texture_ex(texture: Texture2D, position: Vector2, rotation: f32, scale: f32, tint: Color) {
    unsafe {
        raylib::DrawTextureEx(texture, position, rotation, scale, tint);
    }
}

/// Draws from a region of `texture` defined by the `source_rec` rectangle.
pub fn draw_texture_rec(texture: Texture2D, source_rec: Rectangle, position: Vector2, tint: Color) {
    unsafe {
        raylib::DrawTextureRec(texture, source_rec, position, tint);
    }
}

/// Draw from a region of `texture` defined by the `source_rec` rectangle with pro parameters.
pub fn draw_texture_pro(texture: Texture2D, source_rec: Rectangle, dest_rec: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
    unsafe {
        raylib::DrawTexturePro(texture, source_rec, dest_rec, origin, rotation, tint);
    }
}

/// Gets the default font.
pub fn get_font_default() -> Font {
    unsafe {
        raylib::GetFontDefault()
    }
}

/// Loads font from file into GPU memory (VRAM).
pub fn load_font(filename: &str) -> Font {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadFont(c_filename.as_ptr())
    }
}

/// Loads font from file with extended parameters.
pub fn load_font_ex(filename: &str, font_size: i32, chars: Option<&mut [i32]>) -> Font {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        match chars {
            Some(c) => raylib::LoadFontEx(c_filename.as_ptr(), font_size, c.len() as i32, c.as_mut_ptr()),
            None => raylib::LoadFontEx(c_filename.as_ptr(), font_size, 0, std::ptr::null_mut())
        }
    }
}

/// Loads font data for further use (see also `Font::from_data`).
pub fn load_font_data(filename: &str, font_size: i32, mut chars: Option<&mut [i32]>, sdf: bool) -> Vec<CharInfo> {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let ci_arr_ptr = match chars {
            Some(ref mut c) => {
                raylib::LoadFontData(c_filename.as_ptr(), font_size, c.as_mut_ptr(), c.len() as i32, CBool::from(sdf))
            }
            None => {
                raylib::LoadFontData(c_filename.as_ptr(), font_size, std::ptr::null_mut(), 0, CBool::from(sdf))
            }
        };
        let ci_size = if let Some(ref mut c) = chars { c.len() } else { 95 }; // raylib assumes 95 if none given
        let mut ci_vec = Vec::with_capacity(ci_size);
        for i in 0..ci_size {
            ci_vec.push(*ci_arr_ptr.offset(i as isize));
        }
        libc::free(ci_arr_ptr as *mut libc::c_void);
        ci_vec
    }
}

/// Generates image font atlas using `chars` info.
pub fn gen_image_font_atlas(chars: &mut [CharInfo], font_size: i32, padding: i32, pack_method: i32) -> Image {
    unsafe {
        raylib::GenImageFontAtlas(chars.as_mut_ptr(), font_size, chars.len() as i32, padding, pack_method)
    }
}

/// Unloads Font from GPU memory (VRAM).
pub fn unload_font(font: Font) {
    unsafe {
        raylib::UnloadFont(font);
    }
}

/// Shows current FPS.
pub fn draw_fps(x: i32, y: i32) {
    unsafe {
        raylib::DrawFPS(x, y);
    }
}

/// Draws text (using default font).
pub fn draw_text(text: &str, x: i32, y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::DrawText(c_text.as_ptr(), x, y, font_size, color);
    }
}

/// Draws text using `font` and additional parameters.
pub fn draw_text_ex(font: Font, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::DrawTextEx(font, c_text.as_ptr(), position, font_size, spacing, tint);
    }
}

/// Measures string width in pixels for default font.
pub fn measure_text(text: &str, font_size: i32) -> i32 {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::MeasureText(c_text.as_ptr(), font_size)
    }
}

/// Measures string width in pixels for `font`.
pub fn measure_text_ex(font: Font, text: &str, font_size: f32, spacing: f32) -> Vector2 {
    let c_text = CString::new(text).unwrap();
    unsafe {
        raylib::MeasureTextEx(font, c_text.as_ptr(), font_size, spacing)
    }
}

/// Gets index position for a unicode character on `font`.
pub fn get_glyph_index(font: Font, character: i32) -> i32 {
    unsafe {
        raylib::GetGlyphIndex(font, character)
    }
}

/// Draws a line in 3D world space.
pub fn draw_line_3d(start_pos: Vector3, end_pos: Vector3, color: Color) {
    unsafe {
        raylib::DrawLine3D(start_pos, end_pos, color);
    }
}

/// Draws a circle in 3D world space.
pub fn draw_circle_3d(center: Vector3, radius: f32, rotation_axis: Vector3, rotation_angle: f32, color: Color) {
    unsafe {
        raylib::DrawCircle3D(center, radius, rotation_axis, rotation_angle, color);
    }
}

/// Draws a cube.
pub fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe {
        raylib::DrawCube(position, width, height, length, color);
    }
}

/// Draws a cube (Vector version).
pub fn draw_cube_v(position: Vector3, size: Vector3, color: Color) {
    unsafe {
        raylib::DrawCubeV(position, size, color);
    }
}

/// Draws a cube in wireframe.
pub fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe {
        raylib::DrawCubeWires(position, width, height, length, color);
    }
}

/// Draws a textured cube.
pub fn draw_cube_texture(texture: Texture2D, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe {
        raylib::DrawCubeTexture(texture, position, width, height, length, color);
    }
}

/// Draws a sphere.
pub fn draw_sphere(center_pos: Vector3, radius: f32, color: Color) {
    unsafe {
        raylib::DrawSphere(center_pos, radius, color);
    }
}

/// Draws a sphere with extended parameters.
pub fn draw_sphere_ex(center_pos: Vector3, radius: f32, rings: i32, slices: i32, color: Color) {
    unsafe {
        raylib::DrawSphereEx(center_pos, radius, rings, slices, color);
    }
}

/// Draws a sphere in wireframe.
pub fn draw_sphere_wires(center_pos: Vector3, radius: f32, rings: i32, slices: i32, color: Color) {
    unsafe {
        raylib::DrawSphereWires(center_pos, radius, rings, slices, color);
    }
}

/// Draws a cylinder.
pub fn draw_cylinder(position: Vector3, radius_top: f32, radius_bottom: f32, height: f32, slices: i32, color: Color) {
    unsafe {
        raylib::DrawCylinder(position, radius_top, radius_bottom, height, slices, color);
    }
}

/// Draws a cylinder in wireframe.
pub fn draw_cylinder_wires(position: Vector3, radius_top: f32, radius_bottom: f32, height: f32, slices: i32, color: Color) {
    unsafe {
        raylib::DrawCylinderWires(position, radius_top, radius_bottom, height, slices, color);
    }
}

/// Draws an X/Z plane.
pub fn draw_plane(center_pos: Vector3, size: Vector2, color: Color) {
    unsafe {
        raylib::DrawPlane(center_pos, size, color);
    }
}

/// Draws a ray line.
pub fn draw_ray(ray: Ray, color: Color) {
    unsafe {
        raylib::DrawRay(ray, color);
    }
}

/// Draws a grid (centered at (0, 0, 0)).
pub fn draw_grid(slices: i32, spacing: f32) {
    unsafe {
        raylib::DrawGrid(slices, spacing);
    }
}

/// Draws a simple gizmo.
pub fn draw_gizmo(position: Vector3) {
    unsafe {
        raylib::DrawGizmo(position);
    }
}

/// Loads model from files (mesh and material).
pub fn load_model(filename: &str) -> Model {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadModel(c_filename.as_ptr())
    }
}

/// Loads model from generated mesh.
pub fn load_model_from_mesh(mesh: Mesh) -> Model {
    unsafe {
        raylib::LoadModelFromMesh(mesh)
    }
}

/// Unloads model from memory (RAM and/or VRAM).
pub fn unload_model(model: Model) {
    unsafe {
        raylib::UnloadModel(model);
    }
}

/// Loads mesh from file.
pub fn load_mesh(filename: &str) -> Mesh {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadMesh(c_filename.as_ptr())
    }
}

/// Unloads mesh from memory (RAM and/or VRAM).
pub fn unload_mesh(mesh: &mut Mesh) {
    unsafe {
        raylib::UnloadMesh(mesh);
    }
}

/// Exports mesh as an OBJ file.
pub fn export_mesh(filename: &str, mesh: Mesh) {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::ExportMesh(c_filename.as_ptr(), mesh);
    }
}

/// Computes mesh bounding box limits.
pub fn mesh_bounding_box(mesh: Mesh) -> BoundingBox {
    unsafe {
        raylib::MeshBoundingBox(mesh)
    }
}

/// Computes mesh tangents.
pub fn mesh_tangents(mesh: &mut Mesh) {
    unsafe {
        raylib::MeshTangents(mesh);
    }
}

/// Computes mesh binormals.
pub fn mesh_binormals(mesh: &mut Mesh) {
    unsafe {
        raylib::MeshBinormals(mesh);
    }
}

/// Generates plane mesh (with subdivisions).
pub fn gen_mesh_plane(width: f32, length: f32, res_x: i32, res_z: i32) -> Mesh {
    unsafe {
        raylib::GenMeshPlane(width, length, res_x, res_z)
    }
}

/// Generates cuboid mesh.
pub fn gen_mesh_cube(width: f32, height: f32, length: f32) -> Mesh {
    unsafe {
        raylib::GenMeshCube(width, height, length)
    }
}

/// Generates sphere mesh (standard sphere).
pub fn gen_mesh_sphere(radius: f32, rings: i32, slices: i32) -> Mesh {
    unsafe {
        raylib::GenMeshSphere(radius, rings, slices)
    }
}

/// Generates half-sphere mesh (no bottom cap).
pub fn gen_mesh_hemisphere(radius: f32, rings: i32, slices: i32) -> Mesh {
    unsafe {
        raylib::GenMeshHemiSphere(radius, rings, slices)
    }
}

/// Generates cylinder mesh.
pub fn gen_mesh_cylinder(radius: f32, height: f32, slices: i32) -> Mesh {
    unsafe {
        raylib::GenMeshCylinder(radius, height, slices)
    }
}

/// Generates torus mesh.
pub fn gen_mesh_torus(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
    unsafe {
        raylib::GenMeshTorus(radius, size, rad_seg, sides)
    }
}

/// Generates trefoil knot mesh.
pub fn gen_mesh_knot(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
    unsafe {
        raylib::GenMeshKnot(radius, size, rad_seg, sides)
    }
}

/// Generates heightmap mesh from image data.
pub fn gen_mesh_heightmap(heightmap: Image, size: Vector3) -> Mesh {
    unsafe {
        raylib::GenMeshHeightmap(heightmap, size)
    }
}

/// Generates cubes-based map mesh from image data.
pub fn gen_mesh_cubicmap(cubicmap: Image, cube_size: Vector3) -> Mesh {
    unsafe {
        raylib::GenMeshCubicmap(cubicmap, cube_size)
    }
}

/// Loads material from file.
pub fn load_material(filename: &str) -> Material {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadMaterial(c_filename.as_ptr())
    }
}

/// Loads default material (supports `DIFFUSE`, `SPECULAR`, and `NORMAL` maps).
pub fn load_material_default() -> Material {
    unsafe {
        raylib::LoadMaterialDefault()
    }
}

/// Unloads material from GPU memory (VRAM).
pub fn unload_material(material: Material) {
    unsafe {
        raylib::UnloadMaterial(material);
    }
}

/// Draws a model (with texture if set).
pub fn draw_model(model: Model, position: Vector3, scale: f32, tint: Color) {
    unsafe {
        raylib::DrawModel(model, position, scale, tint);
    }
}

/// Draws a model with extended parameters.
pub fn draw_model_ex(model: Model, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color) {
    unsafe {
        raylib::DrawModelEx(model, position, rotation_axis, rotation_angle, scale, tint);
    }
}

/// Draws a model with wires (with texture if set).
pub fn draw_model_wires(model: Model, position: Vector3, scale: f32, tint: Color) {
    unsafe {
        raylib::DrawModelWires(model, position, scale, tint);
    }
}

/// Draws a model with wires.
pub fn draw_model_wires_ex(model: Model, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color) {
    unsafe {
        raylib::DrawModelWiresEx(model, position, rotation_axis, rotation_angle, scale, tint);
    }
}

/// Draws a bounding box (wires).
pub fn draw_bounding_box(bbox: BoundingBox, color: Color) {
    unsafe {
        raylib::DrawBoundingBox(bbox, color);
    }
}

/// Draws a billboard texture.
pub fn draw_billboard(camera: Camera3D, texture: Texture2D, center: Vector3, size: f32, tint: Color) {
    unsafe {
        raylib::DrawBillboard(camera, texture, center, size, tint);
    }
}

/// Draws a billboard texture defined by `source_rec`.
pub fn draw_billboard_rec(camera: Camera3D, texture: Texture2D, source_rec: Rectangle, center: Vector3, size: f32, tint: Color) {
    unsafe {
        raylib::DrawBillboardRec(camera, texture, source_rec, center, size, tint);
    }
}

/// Detects collision between two spheres.
pub fn check_collision_spheres(center_a: Vector3, radius_a: f32, center_b: Vector3, radius_b: f32) -> bool {
    unsafe {
        raylib::CheckCollisionSpheres(center_a, radius_a, center_b, radius_b).is_true()
    }
}

/// Detects collision between two boxes.
pub fn check_collision_boxes(box1: BoundingBox, box2: BoundingBox) -> bool {
    unsafe {
        raylib::CheckCollisionBoxes(box1, box2).is_true()
    }
}

/// Detects collision between box and sphere.
pub fn check_collision_box_sphere(bbox: BoundingBox, center_sphere: Vector3, radius_sphere: f32) -> bool {
    unsafe {
        raylib::CheckCollisionBoxSphere(bbox, center_sphere, radius_sphere).is_true()
    }
}

/// Detects collision between ray and sphere.
pub fn check_collision_ray_sphere(ray: Ray, sphere_position: Vector3, sphere_radius: f32) -> bool {
    unsafe {
        raylib::CheckCollisionRaySphere(ray, sphere_position, sphere_radius).is_true()
    }
}

/// Detects collision between ray and sphere, and returns the collision point.
pub fn check_collision_ray_sphere_ex(ray: Ray, sphere_position: Vector3, sphere_radius: f32, collision_point: &mut Vector3) -> bool {
    unsafe {
        raylib::CheckCollisionRaySphereEx(ray, sphere_position, sphere_radius, collision_point).is_true()
    }
}

/// Detects collision between ray and box.
pub fn check_collision_ray_box(ray: Ray, bbox: BoundingBox) -> bool {
    unsafe {
        raylib::CheckCollisionRayBox(ray, bbox).is_true()
    }
}

/// Gets collision info between ray and model.
pub fn get_collision_ray_model(ray: Ray, model: &mut Model) -> RayHitInfo {
    unsafe {
        raylib::GetCollisionRayModel(ray, model)
    }
}

/// Gets collision info between ray and triangle.
pub fn get_collision_ray_triangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3) -> RayHitInfo {
    unsafe {
        raylib::GetCollisionRayTriangle(ray, p1, p2, p3)
    }
}

/// Gets collision info between ray and ground plane (Y-normal plane).
pub fn get_collision_ray_ground(ray: Ray, ground_height: f32) -> RayHitInfo {
    unsafe {
        raylib::GetCollisionRayGround(ray, ground_height)
    }
}

/// Loads a text file and returns its contents in a string.
pub fn load_text(filename: &str) -> String {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let text = raylib::LoadText(c_filename.as_ptr());
        let safe_text = CStr::from_ptr(text).to_str().unwrap().to_owned();
        libc::free(text as *mut libc::c_void);
        safe_text
    }
}

/// Loads a custom shader and binds default locations.
pub fn load_shader(vs_filename: &str, fs_filename: &str) -> Shader {
    let c_vs_filename = CString::new(vs_filename).unwrap();
    let c_fs_filename = CString::new(fs_filename).unwrap();
    unsafe {
        raylib::LoadShader(c_vs_filename.as_ptr(), c_fs_filename.as_ptr())
    }
}

/// Loads shader from code strings and binds default locations.
pub fn load_shader_code(vs_code: &str, fs_code: &str) -> Shader {
    let c_vs_code = CString::new(vs_code).unwrap();
    let c_fs_code = CString::new(fs_code).unwrap();
    unsafe {
        raylib::LoadShaderCode(c_vs_code.as_ptr() as *mut i8, c_fs_code.as_ptr() as *mut i8)
    }
}

/// Unloads a custom shader from memory.
pub fn unload_shader(shader: Shader) {
    unsafe {
        raylib::UnloadShader(shader);
    }
}

/// Gets default shader.
pub fn get_shader_default() -> Shader {
    unsafe {
        raylib::GetShaderDefault()
    }
}

/// Gets default texture.
pub fn get_texture_default() -> Texture2D {
    unsafe {
        raylib::GetTextureDefault()
    }
}

/// Gets shader uniform location by name.
pub fn get_shader_location(shader: Shader, uniform_name: &str) -> i32 {
    let c_uniform_name = CString::new(uniform_name).unwrap();
    unsafe {
        raylib::GetShaderLocation(shader, c_uniform_name.as_ptr())
    }
}

/// Sets shader uniform value (`f32`).
pub fn set_shader_value(shader: Shader, uniform_loc: i32, value: &[f32]) {
    unsafe {
        raylib::SetShaderValue(shader, uniform_loc, value.as_ptr(), value.len() as i32);
    }
}

/// Sets shader uniform value (`i32`).
pub fn set_shader_value_i(shader: Shader, uniform_loc: i32, value: &[i32]) {
    unsafe {
        raylib::SetShaderValuei(shader, uniform_loc, value.as_ptr(), value.len() as i32);
    }
}

/// Sets shader uniform value (matrix 4x4).
pub fn set_shader_value_matrix(shader: Shader, uniform_loc: i32, mat: Matrix) {
    unsafe {
        raylib::SetShaderValueMatrix(shader, uniform_loc, mat);
    }
}

/// Sets a custom projection matrix (replaces internal projection matrix).
pub fn set_matrix_projection(proj: Matrix) {
    unsafe {
        raylib::SetMatrixProjection(proj);
    }
}

/// Sets a custom modelview matrix (replaces internal modelview matrix).
pub fn set_matrix_modelview(view: Matrix) {
    unsafe {
        raylib::SetMatrixModelview(view);
    }
}

/// Gets internal modelview matrix.
pub fn get_matrix_modelview() -> Matrix {
    unsafe {
        raylib::GetMatrixModelview()
    }
}

/// Generates cubemap texture from HDR texture.
pub fn gen_texture_cubemap(shader: Shader, sky_hdr: Texture2D, size: i32) -> Texture2D {
    unsafe {
        raylib::GenTextureCubemap(shader, sky_hdr, size)
    }
}

/// Generates irradiance texture using cubemap data.
pub fn gen_texture_irradiance(shader: Shader, cubemap: Texture2D, size: i32) -> Texture2D {
    unsafe {
        raylib::GenTextureIrradiance(shader, cubemap, size)
    }
}

/// Generates prefilter texture using cubemap data.
pub fn gen_texture_prefilter(shader: Shader, cubemap: Texture2D, size: i32) -> Texture2D {
    unsafe {
        raylib::GenTexturePrefilter(shader, cubemap, size)
    }
}

/// Generates BRDF texture using cubemap data.
pub fn gen_texture_brdf(shader: Shader, cubemap: Texture2D, size: i32) -> Texture2D {
    unsafe {
        raylib::GenTextureBRDF(shader, cubemap, size)
    }
}

/// Begins custom shader drawing.
pub fn begin_shader_mode(shader: Shader) {
    unsafe {
        raylib::BeginShaderMode(shader);
    }
}

/// Ends custom shader drawing (and switches to default shader).
pub fn end_shader_mode() {
    unsafe {
        raylib::EndShaderMode();
    }
}

/// Begins blending mode (alpha, additive, multiplied).
pub fn begin_blend_mode(mode: BlendMode) {
    unsafe {
        raylib::BeginBlendMode(mode);
    }
}

/// Ends blending mode (reset to default: alpha blending).
pub fn end_blend_mode() {
    unsafe {
        raylib::EndBlendMode();
    }
}

/// Gets VR device information for some standard devices.
pub fn get_vr_device_info(vr_device_type: VrDeviceType) -> VrDeviceInfo {
    unsafe {
        raylib::GetVrDeviceInfo(vr_device_type)
    }
}

/// Initializes VR simulator for selected device parameters.
pub fn init_vr_simulator(info: VrDeviceInfo) {
    unsafe {
        raylib::InitVrSimulator(info);
    }
}

/// Closes VR simulator for current device.
pub fn close_vr_simulator() {
    unsafe {
        raylib::CloseVrSimulator();
    }
}

/// Detects if VR simulator is ready.
pub fn is_vr_simulator_ready() -> bool {
    unsafe {
        raylib::IsVrSimulatorReady().is_true()
    }
}

/// Sets VR distortion shader for stereoscopic rendering.
pub fn set_vr_distortion_shader(shader: Shader) {
    unsafe {
        raylib::SetVrDistortionShader(shader);
    }
}

/// Updates VR tracking (position and orientation) and camera.
pub fn update_vr_tracking(camera: &mut Camera3D) {
    unsafe {
        raylib::UpdateVrTracking(camera);
    }
}

/// Enables or disables VR experience.
pub fn toggle_vr_mode() {
    unsafe {
        raylib::ToggleVrMode();
    }
}

/// Begins VR simulator stereo rendering.
pub fn begin_vr_drawing() {
    unsafe {
        raylib::BeginVrDrawing();
    }
}

/// Ends VR simulator stereo rendering.
pub fn end_vr_drawing() {
    unsafe {
        raylib::EndVrDrawing();
    }
}

/// Initializes audio device and context.
pub fn init_audio_device() {
    unsafe {
        raylib::InitAudioDevice();
    }
}

/// Closes the audio device and context (and music stream).
pub fn close_audio_device() {
    unsafe {
        raylib::CloseAudioDevice();
    }
}

/// Checks if audio device is ready.
pub fn is_audio_device_ready() -> bool {
    unsafe {
        raylib::IsAudioDeviceReady().is_true()
    }
}

/// Sets master volume (listener).
pub fn set_master_volume(volume: f32) {
    unsafe {
        raylib::SetMasterVolume(volume);
    }
}

/// Loads wave data from file into RAM.
pub fn load_wave(filename: &str) -> Wave {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadWave(c_filename.as_ptr())
    }
}

/// Loads wave data from raw array data.
pub fn load_wave_ex(data: &mut [u8], sample_count: i32, sample_rate: i32, sample_size: i32, channels: i32) -> Wave {
    unsafe {
        raylib::LoadWaveEx(data.as_mut_ptr() as *mut std::os::raw::c_void, sample_count, sample_rate, sample_size, channels)
    }
}

/// Loads sound from file.
pub fn load_sound(filename: &str) -> Sound {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadSound(c_filename.as_ptr())
    }
}

/// Loads sound from wave data.
pub fn load_sound_from_wave(wave: Wave) -> Sound {
    unsafe {
        raylib::LoadSoundFromWave(wave)
    }
}

/// Updates sound buffer with new data.
pub fn update_sound(sound: Sound, data: &[u8], samples_count: i32) {
    unsafe {
        raylib::UpdateSound(sound, data.as_ptr() as *const std::os::raw::c_void, samples_count);
    }
}

/// Unloads wave data.
pub fn unload_wave(wave: Wave) {
    unsafe {
        raylib::UnloadWave(wave);
    }
}

/// Unloads sound.
pub fn unload_sound(sound: Sound) {
    unsafe {
        raylib::UnloadSound(sound);
    }
}

/// Plays a sound.
pub fn play_sound(sound: Sound) {
    unsafe {
        raylib::PlaySound(sound);
    }
}

/// Pauses a sound.
pub fn pause_sound(sound: Sound) {
    unsafe {
        raylib::PauseSound(sound);
    }
}

/// Resumes a paused sound.
pub fn resume_sound(sound: Sound) {
    unsafe {
        raylib::ResumeSound(sound);
    }
}

/// Stops playing a sound.
pub fn stop_sound(sound: Sound) {
    unsafe {
        raylib::StopSound(sound);
    }
}

/// Checks if a sound is currently playing.
pub fn is_sound_playing(sound: Sound) -> bool {
    unsafe {
        raylib::IsSoundPlaying(sound).is_true()
    }
}

/// Sets volume for a sound (`1.0` is max level).
pub fn set_sound_volume(sound: Sound, volume: f32) {
    unsafe {
        raylib::SetSoundVolume(sound, volume);
    }
}

/// Sets pitch for a sound (`1.0` is base level).
pub fn set_sound_pitch(sound: Sound, pitch: f32) {
    unsafe {
        raylib::SetSoundPitch(sound, pitch);
    }
}

/// Converts wave data to desired format.
pub fn wave_format(wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
    unsafe {
        raylib::WaveFormat(wave, sample_rate, sample_size, channels);
    }
}

/// Copies a wave to a new wave.
pub fn wave_copy(wave: Wave) -> Wave {
    unsafe {
        raylib::WaveCopy(wave)
    }
}

/// Crops a wave to defined sample range.
pub fn wave_crop(wave: &mut Wave, init_sample: i32, final_sample: i32) {
    unsafe {
        raylib::WaveCrop(wave, init_sample, final_sample);
    }
}

/// Gets sample data from wave as an `f32` array.
pub fn get_wave_data(wave: Wave) -> Vec<f32> {
    unsafe {
        let data = raylib::GetWaveData(wave);
        let data_size = (wave.sample_count * wave.channels) as usize;
        let mut samples = Vec::with_capacity(data_size);
        samples.set_len(data_size);
        std::ptr::copy(data, samples.as_mut_ptr(), data_size);
        libc::free(data as *mut libc::c_void);
        samples
    }
}

/// Loads music stream from file.
pub fn load_music_stream(filename: &str) -> Music {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        raylib::LoadMusicStream(c_filename.as_ptr())
    }
}

/// Unloads music stream.
pub fn unload_music_stream(music: Music) {
    unsafe {
        raylib::UnloadMusicStream(music);
    }
}

/// Starts music playing.
pub fn play_music_stream(music: Music) {
    unsafe {
        raylib::PlayMusicStream(music);
    }
}

/// Updates buffers for music streaming.
pub fn update_music_stream(music: Music) {
    unsafe {
        raylib::UpdateMusicStream(music);
    }
}

/// Stops music playing.
pub fn stop_music_stream(music: Music) {
    unsafe {
        raylib::StopMusicStream(music);
    }
}

/// Pauses music playing.
pub fn pause_music_stream(music: Music) {
    unsafe {
        raylib::PauseMusicStream(music);
    }
}

/// Resumes playing paused music.
pub fn resume_music_stream(music: Music) {
    unsafe {
        raylib::ResumeMusicStream(music);
    }
}

/// Checks if music is playing.
pub fn is_music_playing(music: Music) -> bool {
    unsafe {
        raylib::IsMusicPlaying(music).is_true()
    }
}

/// Sets volume for music (`1.0` is max level).
pub fn set_music_volume(music: Music, volume: f32) {
    unsafe {
        raylib::SetMusicVolume(music, volume);
    }
}

/// Sets pitch for music (`1.0` is base level).
pub fn set_music_pitch(music: Music, pitch: f32) {
    unsafe {
        raylib::SetMusicPitch(music, pitch);
    }
}

/// Sets music loop count (loop repeats).
pub fn set_music_loop_count(music: Music, count: i32) {
    unsafe {
        raylib::SetMusicLoopCount(music, count);
    }
}

/// Gets music time length in seconds.
pub fn get_music_time_length(music: Music) -> f32 {
    unsafe {
        raylib::GetMusicTimeLength(music)
    }
}

/// Gets current music time played in seconds.
pub fn get_music_time_played(music: Music) -> f32 {
    unsafe {
        raylib::GetMusicTimePlayed(music)
    }
}

/// Initializes audio stream (to stream raw PCM data).
pub fn init_audio_stream(sample_rate: u32, sample_size: u32, channels: u32) -> AudioStream {
    unsafe {
        raylib::InitAudioStream(sample_rate, sample_size, channels)
    }
}

/// Updates audio stream buffers with data.
pub fn update_audio_stream(stream: AudioStream, data: &[impl AudioSample]) {
    unsafe {
        raylib::UpdateAudioStream(stream, data.as_ptr() as *const std::os::raw::c_void, data.len() as i32);
    }
}

/// Closes audio stream and frees memory.
pub fn close_audio_stream(stream: AudioStream) {
    unsafe {
        raylib::CloseAudioStream(stream);
    }
}

/// Checks if any audio stream buffers requires refill.
pub fn is_audio_buffer_processed(stream: AudioStream) -> bool {
    unsafe {
        raylib::IsAudioBufferProcessed(stream).is_true()
    }
}

/// Plays audio stream.
pub fn play_audio_stream(stream: AudioStream) {
    unsafe {
        raylib::PlayAudioStream(stream);
    }
}

/// Pauses audio stream.
pub fn pause_audio_stream(stream: AudioStream) {
    unsafe {
        raylib::PauseAudioStream(stream);
    }
}

/// Resumes audio stream.
pub fn resume_audio_stream(stream: AudioStream) {
    unsafe {
        raylib::ResumeAudioStream(stream);
    }
}

/// Checks if audio stream is currently playing.
pub fn is_audio_stream_playing(stream: AudioStream) -> bool {
    unsafe {
        raylib::IsAudioStreamPlaying(stream).is_true()
    }
}

/// Stops audio stream.
pub fn stop_audio_stream(stream: AudioStream) {
    unsafe {
        raylib::StopAudioStream(stream);
    }
}

/// Sets volume for audio stream (`1.0` is max level).
pub fn set_audio_stream_volume(stream: AudioStream, volume: f32) {
    unsafe {
        raylib::SetAudioStreamVolume(stream, volume);
    }
}

/// Sets pitch for audio stream (`1.0` is base level).
pub fn set_audio_stream_pitch(stream: AudioStream, pitch: f32) {
    unsafe {
        raylib::SetAudioStreamPitch(stream, pitch);
    }
}
