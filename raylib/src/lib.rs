/* raylib-rs
   lib.rs - Main library code (the safe layer)

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

#![doc(
    html_logo_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust_256x256.png",
    html_favicon_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust.ico"
)]

use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::ffi::{CString, CStr};
use lazy_static::lazy_static;

mod raiiwrap;
mod raymath;
mod raylib;
pub mod ease;

// TODO: Replace usage of `raylib` module with `ffi`
use raylib_sys as ffi;

pub use crate::raiiwrap::*;
pub use crate::raylib::{
    Color,
    Camera2D, Camera3D, Camera,
    Vector2, Vector3, Vector4, Quaternion,
    Ray, Matrix, Rectangle, RayHitInfo,
    CharInfo,
    BoundingBox,
    VrDeviceInfo,
};

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
pub type ShaderLocationIndex = usize;

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
pub type TexmapIndex = usize;

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

static IS_INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

lazy_static! {
    static ref FONT_DEFAULT: Font = {
        unsafe { Font(raylib::GetFontDefault()) }
    };
}

lazy_static! {
    static ref MATERIAL_DEFAULT: Material = {
        unsafe { Material(raylib::LoadMaterialDefault()) }
    };
}

lazy_static! {
    static ref SHADER_DEFAULT: Shader = {
        unsafe { Shader(raylib::GetShaderDefault()) }
    };
}

lazy_static! {
    static ref TEXTURE_DEFAULT: Texture2D = {
        unsafe {
            Texture2D(raylib::GetTextureDefault())
        }
    };
}

#[allow(non_upper_case_globals)]
static mut log_type_flags: LogType = LOG_INFO | LOG_WARNING | LOG_ERROR;

#[derive(Debug, Default)]
pub struct RaylibBuilder {
    show_logo: bool,
    fullscreen_mode: bool,
    window_resizable: bool,
    window_undecorated: bool,
    window_transparent: bool,
    msaa_4x_hint: bool,
    vsync_hint: bool,
    width: i32,
    height: i32,
    title: String,
}

impl RaylibBuilder {
    pub fn with_logo(&mut self) -> &mut Self {
        self.show_logo = true;
        self
    }

    pub fn fullscreen(&mut self) -> &mut Self {
        self.fullscreen_mode = true;
        self
    }

    pub fn resizable(&mut self) -> &mut Self {
        self.window_resizable = true;
        self
    }

    pub fn undecorated(&mut self) -> &mut Self {
        self.window_undecorated = true;
        self
    }

    pub fn transparent(&mut self) -> &mut Self {
        self.window_transparent = true;
        self
    }

    pub fn msaa_4x(&mut self) -> &mut Self {
        self.msaa_4x_hint = true;
        self
    }

    pub fn vsync(&mut self) -> &mut Self {
        self.vsync_hint = true;
        self
    }

    pub fn width(&mut self, w: i32) -> &mut Self {
        self.width = w;
        self
    }

    pub fn height(&mut self, h: i32) -> &mut Self {
        self.height = h;
        self
    }

    pub fn size(&mut self, w: i32, h: i32) -> &mut Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = text.to_string();
        self
    }

    /// Builds and initializes a Raylib window. Panics if raylib is already initialized.
    pub fn build(&self) -> RaylibHandle {
        let mut flags = 0u8;
        if self.show_logo { flags |= FLAG_SHOW_LOGO; }
        if self.fullscreen_mode { flags |= FLAG_FULLSCREEN_MODE; }
        if self.window_resizable { flags |= FLAG_WINDOW_RESIZABLE; }
        if self.window_undecorated { flags |= FLAG_WINDOW_UNDECORATED; }
        if self.window_transparent { flags |= FLAG_WINDOW_TRANSPARENT; }
        if self.msaa_4x_hint { flags |= FLAG_MSAA_4X_HINT; }
        if self.vsync_hint { flags |= FLAG_VSYNC_HINT; }

        unsafe { raylib::SetConfigFlags(flags); }
        init_window(self.width, self.height, &self.title)
    }
}

/// Enables trace log message types (bit flags based).
#[inline]
pub fn set_trace_log(types: LogType) {
    unsafe {
        log_type_flags = types;
        raylib::SetTraceLog(types);
    }
}

/// Writes a trace log message (`LOG_INFO`, `LOG_WARNING`, `LOG_ERROR`, `LOG_DEBUG`).
#[inline]
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

pub struct RaylibHandle;

/// Creates a `RaylibBuilder` for choosing window options before initialization.
pub fn init() -> RaylibBuilder {
    RaylibBuilder {
        width: 640,
        height: 480,
        title: "raylib-rs".to_string(),
        ..Default::default()
    }
}

/// Initializes window and OpenGL context. Panics if raylib is already initialized.
pub fn init_window(width: i32, height: i32, title: &str) -> RaylibHandle {
    if IS_INITIALIZED.load(Ordering::Relaxed) {
        panic!("Attempted to initialize raylib-rs more than once");
    }
    else {
        unsafe {
            let c_title = CString::new(title).unwrap();
            raylib::InitWindow(width, height, c_title.as_ptr());
        }
        IS_INITIALIZED.store(true, Ordering::Relaxed);
        RaylibHandle
    }
}

impl Drop for RaylibHandle {
    fn drop(&mut self) {
        if IS_INITIALIZED.load(Ordering::Relaxed) {
            unsafe { raylib::CloseWindow(); }
            IS_INITIALIZED.store(false, Ordering::Relaxed);
        }
    }
}

impl RaylibHandle {
    /// Checks if window has been initialized successfully.
    #[inline]
    pub fn is_window_ready(&self) -> bool {
        unsafe {
            raylib::IsWindowReady()
        }
    }

    /// Checks if KEY_ESCAPE or Close icon was pressed.
    #[inline]
    pub fn window_should_close(&self) -> bool {
        unsafe {
            raylib::WindowShouldClose()
        }
    }

    /// Checks if window has been minimized (or lost focus).
    #[inline]
    pub fn is_window_minimized(&self) -> bool {
        unsafe {
            raylib::IsWindowMinimized()
        }
    }

    /// Toggles fullscreen mode (only on desktop platforms).
    #[inline]
    pub fn toggle_fullscreen(&self) {
        unsafe {
            raylib::ToggleFullscreen();
        }
    }

    /// Sets icon for window (only on desktop platforms).
    #[inline]
    pub fn set_window_icon(&self, image: &Image) {
        unsafe {
            raylib::SetWindowIcon(image.0);
        }
    }

    /// Sets title for window (only on desktop platforms).
    #[inline]
    pub fn set_window_title(&self, title: &str) {
        let c_title = CString::new(title).unwrap();
        unsafe {
            raylib::SetWindowTitle(c_title.as_ptr());
        }
    }

    /// Sets window position on screen (only on desktop platforms).
    #[inline]
    pub fn set_window_position(&self, x: i32, y: i32) {
        unsafe {
            raylib::SetWindowPosition(x, y);
        }
    }

    /// Sets monitor for the current window (fullscreen mode).
    #[inline]
    pub fn set_window_monitor(&self, monitor: i32) {
        unsafe {
            raylib::SetWindowMonitor(monitor);
        }
    }

    /// Sets minimum window dimensions (for `FLAG_WINDOW_RESIZABLE`).
    #[inline]
    pub fn set_window_min_size(&self, width: i32, height: i32) {
        unsafe {
            raylib::SetWindowMinSize(width, height);
        }
    }

    /// Sets window dimensions.
    #[inline]
    pub fn set_window_size(&self, width: i32, height: i32) {
        unsafe {
            raylib::SetWindowSize(width, height);
        }
    }

    /// Gets current screen width.
    #[inline]
    pub fn get_screen_width(&self) -> i32 {
        unsafe {
            raylib::GetScreenWidth()
        }
    }

    /// Gets current screen height.
    #[inline]
    pub fn get_screen_height(&self) -> i32 {
        unsafe {
            raylib::GetScreenHeight()
        }
    }

    /// Shows mouse cursor.
    #[inline]
    pub fn show_cursor(&self) {
        unsafe {
            raylib::ShowCursor();
        }
    }

    /// Hides mouse cursor.
    #[inline]
    pub fn hide_cursor(&self) {
        unsafe {
            raylib::HideCursor();
        }
    }

    /// Checks if mouse cursor is not visible.
    #[inline]
    pub fn is_cursor_hidden(&self) -> bool {
        unsafe {
            raylib::IsCursorHidden()
        }
    }

    /// Enables mouse cursor (unlock cursor).
    #[inline]
    pub fn enable_cursor(&self) {
        unsafe {
            raylib::EnableCursor();
        }
    }

    /// Disables mouse cursor (lock cursor).
    #[inline]
    pub fn disable_cursor(&self) {
        unsafe {
            raylib::DisableCursor();
        }
    }

    /// Sets background color (framebuffer clear color).
    #[inline]
    pub fn clear_background(&self, color: impl Into<Color>) {
        unsafe {
            raylib::ClearBackground(color.into());
        }
    }

    /// Sets up canvas (framebuffer) to start drawing.
    #[inline]
    pub fn begin_drawing(&self) {
        unsafe {
            raylib::BeginDrawing();
        }
    }

    /// Ends canvas drawing and swaps buffers (double buffering).
    #[inline]
    pub fn end_drawing(&self) {
        unsafe {
            raylib::EndDrawing();
        }
    }

    /// Initializes 2D mode with custom camera (2D).
    #[inline]
    pub fn begin_mode_2d(&self, camera: Camera2D) {
        unsafe {
            raylib::BeginMode2D(camera);
        }
    }

    /// Ends 2D mode with custom camera.
    #[inline]
    pub fn end_mode_2d(&self) {
        unsafe {
            raylib::EndMode2D();
        }
    }

    /// Initializes 3D mode with custom camera (3D).
    #[inline]
    pub fn begin_mode_3d(&self, camera: Camera3D) {
        unsafe {
            raylib::BeginMode3D(camera);
        }
    }

    /// Ends 3D mode and returns to default 2D orthographic mode.
    #[inline]
    pub fn end_mode_3d(&self) {
        unsafe {
            raylib::EndMode3D();
        }
    }

    /// Initializes render texture for drawing.
    #[inline]
    pub fn begin_texture_mode(&self, target: &RenderTexture2D) {
        unsafe {
            raylib::BeginTextureMode(target.0);
        }
    }

    /// Ends drawing to render texture.
    #[inline]
    pub fn end_texture_mode(&self) {
        unsafe {
            raylib::EndTextureMode();
        }
    }

    /// Returns a ray trace from mouse position.
    #[inline]
    pub fn get_mouse_ray(&self, mouse_position: impl Into<Vector2>, camera: Camera3D) -> Ray {
        unsafe {
            raylib::GetMouseRay(mouse_position.into(), camera)
        }
    }

    /// Returns the screen space position for a 3D world space position.
    #[inline]
    pub fn get_world_to_screen(&self, position: impl Into<Vector3>, camera: Camera3D) -> Vector2 {
        unsafe {
            raylib::GetWorldToScreen(position.into(), camera)
        }
    }

    /// Returns camera transform matrix (view matrix).
    #[inline]
    pub fn get_camera_matrix(&self, camera: Camera3D) -> Matrix {
        unsafe {
            raylib::GetCameraMatrix(camera)
        }
    }

    /// Sets target FPS (maximum).
    #[inline]
    pub fn set_target_fps(&self, fps: i32) {
        unsafe {
            raylib::SetTargetFPS(fps);
        }
    }

    /// Returns current FPS.
    #[inline]
    pub fn get_fps(&self) -> i32 {
        unsafe {
            raylib::GetFPS()
        }
    }

    /// Returns time in seconds for last frame drawn.
    #[inline]
    pub fn get_frame_time(&self) -> f32 {
        unsafe {
            raylib::GetFrameTime()
        }
    }

    /// Returns elapsed time in seconds since `init_window` was called.
    #[inline]
    pub fn get_time(&self) -> f64 {
        unsafe {
            raylib::GetTime()
        }
    }

    /// Returns hexadecimal value for a Color.
    #[inline]
    pub fn color_to_int(&self, color: impl Into<Color>) -> i32 {
        unsafe {
            raylib::ColorToInt(color.into())
        }
    }

    /// Returns color normalized as `f32` [0..1].
    #[inline]
    pub fn color_normalize(&self, color: impl Into<Color>) -> Vector4 {
        unsafe {
            raylib::ColorNormalize(color.into())
        }
    }

    /// Returns HSV values for a Color.
    #[inline]
    pub fn color_to_hsv(&self, color: impl Into<Color>) -> Vector3 {
        unsafe {
            raylib::ColorToHSV(color.into())
        }
    }

    /// Returns a Color struct from hexadecimal value.
    #[inline]
    pub fn get_color(&self, hex_value: i32) -> Color {
        unsafe {
            raylib::GetColor(hex_value)
        }
    }

    /// Color fade-in or fade-out, `alpha` goes from `0.0` to `1.0`.
    #[inline]
    pub fn fade(&self, color: impl Into<Color>, alpha: f32) -> Color {
        unsafe {
            raylib::Fade(color.into(), alpha)
        }
    }

    /// Takes a screenshot of current screen (in PNG format).
    #[inline]
    pub fn take_screenshot(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            raylib::TakeScreenshot(c_filename.as_ptr());
        }
    }

    /// Returns a random value between min and max (both included).
    #[inline]
    pub fn get_random_value(&self, min: i32, max: i32) -> i32 {
        unsafe {
            raylib::GetRandomValue(min, max)
        }
    }

    /// Checks if `filename` has an `ext` extension.
    #[inline]
    pub fn is_file_extension(&self, filename: &str, ext: &str) -> bool {
        let c_filename = CString::new(filename).unwrap();
        let c_ext = CString::new(ext).unwrap();
        unsafe {
            raylib::IsFileExtension(c_filename.as_ptr(), c_ext.as_ptr())
        }
    }

    /// Gets the extension for a `filename` string.
    #[inline]
    pub fn get_extension(&self, filename: &str) -> String {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            let ext = raylib::GetExtension(c_filename.as_ptr());
            CStr::from_ptr(ext).to_str().unwrap().to_owned()
        }
    }

    /// Gets the filename for a path string.
    #[inline]
    pub fn get_file_name(&self, file_path: &str) -> String {
        let c_file_path = CString::new(file_path).unwrap();
        unsafe {
            let filename = raylib::GetFileName(c_file_path.as_ptr());
            CStr::from_ptr(filename).to_str().unwrap().to_owned()
        }
    }

    /// Gets full path for a given `filename`.
    #[inline]
    pub fn get_directory_path(&self, filename: &str) -> String {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            let dirpath = raylib::GetDirectoryPath(c_filename.as_ptr());
            CStr::from_ptr(dirpath).to_str().unwrap().to_owned()
        }
    }

    /// Gets current working directory.
    #[inline]
    pub fn get_working_directory(&self) -> String {
        unsafe {
            let workdir = raylib::GetWorkingDirectory();
            CStr::from_ptr(workdir).to_str().unwrap().to_owned()
        }
    }

    /// Changes working directory, returns true on success.
    #[inline]
    pub fn change_directory(&self, dir: &str) -> bool {
        let c_dir = CString::new(dir).unwrap();
        unsafe {
            raylib::ChangeDirectory(c_dir.as_ptr())
        }
    }

    /// Checks if a file has been dropped into the window.
    #[inline]
    pub fn is_file_dropped(&self) -> bool {
        unsafe {
            raylib::IsFileDropped()
        }
    }

    /// Gets dropped filenames.
    #[inline]
    pub fn get_dropped_files(&self) -> Vec<String> {
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
    #[inline]
    pub fn clear_dropped_files(&self) {
        unsafe {
            raylib::ClearDroppedFiles();
        }
    }

    /// Saves integer value to storage file (to defined `position`).
    #[inline]
    pub fn storage_save_value(&self, position: i32, value: i32) {
        unsafe {
            raylib::StorageSaveValue(position, value);
        }
    }

    /// Loads integer value from storage file (from defined `position`).
    #[inline]
    pub fn storage_load_value(&self, position: i32) -> i32 {
        unsafe {
            raylib::StorageLoadValue(position)
        }
    }

    /// Detect if a key has been pressed once.
    #[inline]
    pub fn is_key_pressed(&self, key: i32) -> bool {
        unsafe {
            raylib::IsKeyPressed(key)
        }
    }

    /// Detect if a key is being pressed.
    #[inline]
    pub fn is_key_down(&self, key: i32) -> bool {
        unsafe {
            raylib::IsKeyDown(key)
        }
    }

    /// Detect if a key has been released once.
    #[inline]
    pub fn is_key_released(&self, key: i32) -> bool {
        unsafe {
            raylib::IsKeyReleased(key)
        }
    }

    /// Detect if a key is NOT being pressed.
    #[inline]
    pub fn is_key_up(&self, key: i32) -> bool {
        unsafe {
            raylib::IsKeyUp(key)
        }
    }

    /// Gets latest key pressed.
    #[inline]
    pub fn get_key_pressed(&self) -> i32 {
        unsafe {
            raylib::GetKeyPressed()
        }
    }

    /// Sets a custom key to exit program (default is ESC).
    #[inline]
    pub fn set_exit_key(&self, key: i32) {
        unsafe {
            raylib::SetExitKey(key);
        }
    }

    /// Detect if a gamepad is available.
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: i32) -> bool {
        unsafe {
            raylib::IsGamepadAvailable(gamepad)
        }
    }

    /// Checks gamepad name (if available).
    #[inline]
    pub fn is_gamepad_name(&self, gamepad: i32, name: &str) -> bool {
        let c_name = CString::new(name).unwrap();
        unsafe {
            raylib::IsGamepadName(gamepad, c_name.as_ptr())
        }
    }

    /// Returns gamepad internal name id.
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: i32) -> Option<String> {
        unsafe {
            let name = raylib::GetGamepadName(gamepad);
            match name.is_null() {
                false => Some(CStr::from_ptr(name).to_str().unwrap().to_owned()),
                true => None
            }
        }
    }

    /// Detect if a gamepad button has been pressed once.
    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: i32, button: i32) -> bool {
        unsafe {
            raylib::IsGamepadButtonPressed(gamepad, button)
        }
    }

    /// Detect if a gamepad button is being pressed.
    #[inline]
    pub fn is_gamepad_button_down(&self, gamepad: i32, button: i32) -> bool {
        unsafe {
            raylib::IsGamepadButtonDown(gamepad, button)
        }
    }

    /// Detect if a gamepad button has been released once.
    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: i32, button: i32) -> bool {
        unsafe {
            raylib::IsGamepadButtonReleased(gamepad, button)
        }
    }

    /// Detect if a gamepad button is NOT being pressed.
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: i32, button: i32) -> bool {
        unsafe {
            raylib::IsGamepadButtonUp(gamepad, button)
        }
    }

    /// Gets the last gamepad button pressed.
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> i32 {
        unsafe {
            raylib::GetGamepadButtonPressed()
        }
    }

    /// Returns gamepad axis count for a gamepad.
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: i32) -> i32 {
        unsafe {
            raylib::GetGamepadAxisCount(gamepad)
        }
    }

    /// Returns axis movement value for a gamepad axis.
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: i32, axis: i32) -> f32 {
        unsafe {
            raylib::GetGamepadAxisMovement(gamepad, axis)
        }
    }

    /// Detect if a mouse button has been pressed once.
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: i32) -> bool {
        unsafe {
            raylib::IsMouseButtonPressed(button)
        }
    }

    /// Detect if a mouse button is being pressed.
    #[inline]
    pub fn is_mouse_button_down(&self, button: i32) -> bool {
        unsafe {
            raylib::IsMouseButtonDown(button)
        }
    }

    /// Detect if a mouse button has been released once.
    #[inline]
    pub fn is_mouse_button_released(&self, button: i32) -> bool {
        unsafe {
            raylib::IsMouseButtonReleased(button)
        }
    }

    /// Detect if a mouse button is NOT being pressed.
    #[inline]
    pub fn is_mouse_button_up(&self, button: i32) -> bool {
        unsafe {
            raylib::IsMouseButtonUp(button)
        }
    }

    /// Returns mouse position X.
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        unsafe {
            raylib::GetMouseX()
        }
    }

    /// Returns mouse position Y.
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        unsafe {
            raylib::GetMouseY()
        }
    }

    /// Returns mouse position.
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe {
            raylib::GetMousePosition()
        }
    }

    /// Sets mouse position.
    #[inline]
    pub fn set_mouse_position(&self, position: impl Into<Vector2>) {
        unsafe {
            raylib::SetMousePosition(position.into());
        }
    }

    /// Sets mouse scaling.
    #[inline]
    pub fn set_mouse_scale(&self, scale: f32) {
        unsafe {
            raylib::SetMouseScale(scale);
        }
    }

    /// Returns mouse wheel movement Y.
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> i32 {
        unsafe {
            raylib::GetMouseWheelMove()
        }
    }

    /// Returns touch position X for touch point 0 (relative to screen size).
    #[inline]
    pub fn get_touch_x(&self) -> i32 {
        unsafe {
            raylib::GetTouchX()
        }
    }

    /// Returns touch position Y for touch point 0 (relative to screen size).
    #[inline]
    pub fn get_touch_y(&self) -> i32 {
        unsafe {
            raylib::GetTouchY()
        }
    }

    /// Returns touch position XY for a touch point index (relative to screen size).
    #[inline]
    pub fn get_touch_position(&self, index: i32) -> Vector2 {
        unsafe {
            raylib::GetTouchPosition(index)
        }
    }

    /// Enables a set of gestures using flags.
    #[inline]
    pub fn set_gestures_enabled(&self, gesture_flags: Gestures) {
        unsafe {
            raylib::SetGesturesEnabled(gesture_flags);
        }
    }

    /// Checks if a gesture have been detected.
    #[inline]
    pub fn is_gesture_detected(&self, gesture: Gestures) -> bool {
        unsafe {
            raylib::IsGestureDetected(gesture as i32)
        }
    }

    /// Gets latest detected gesture.
    #[inline]
    pub fn get_gesture_detected(&self) -> i32 {
        unsafe {
            raylib::GetGestureDetected()
        }
    }

    /// Gets touch points count.
    #[inline]
    pub fn get_touch_points_count(&self) -> i32 {
        unsafe {
            raylib::GetTouchPointsCount()
        }
    }

    /// Gets gesture hold time in milliseconds.
    #[inline]
    pub fn get_gesture_hold_duration(&self) -> f32 {
        unsafe {
            raylib::GetGestureHoldDuration()
        }
    }

    /// Gets gesture drag vector.
    #[inline]
    pub fn get_gesture_drag_vector(&self) -> Vector2 {
        unsafe {
            raylib::GetGestureDragVector()
        }
    }

    /// Gets gesture drag angle.
    #[inline]
    pub fn get_gesture_drag_angle(&self) -> f32 {
        unsafe {
            raylib::GetGestureDragAngle()
        }
    }

    /// Gets gesture pinch delta.
    #[inline]
    pub fn get_gesture_pinch_vector(&self) -> Vector2 {
        unsafe {
            raylib::GetGesturePinchVector()
        }
    }

    /// Gets gesture pinch angle.
    #[inline]
    pub fn get_gesture_pinch_angle(&self) -> f32 {
        unsafe {
            raylib::GetGesturePinchAngle()
        }
    }

    /// Sets camera mode.
    #[inline]
    pub fn set_camera_mode(&self, camera: Camera3D, mode: CameraMode) {
        unsafe {
            raylib::SetCameraMode(camera, mode);
        }
    }

    /// Updates camera position for selected mode.
    #[inline]
    pub fn update_camera(&self, camera: &mut Camera3D) {
        unsafe {
            raylib::UpdateCamera(camera);
        }
    }

    /// Sets camera pan key to combine with mouse movement (free camera).
    #[inline]
    pub fn set_camera_pan_control(&self, pan_key: i32) {
        unsafe {
            raylib::SetCameraPanControl(pan_key);
        }
    }

    /// Sets camera alt key to combine with mouse movement (free camera).
    #[inline]
    pub fn set_camera_alt_control(&self, alt_key: i32) {
        unsafe {
            raylib::SetCameraAltControl(alt_key);
        }
    }

    /// Sets camera smooth zoom key to combine with mouse (free camera).
    #[inline]
    pub fn set_camera_smooth_zoom_control(&self, sz_key: i32) {
        unsafe {
            raylib::SetCameraSmoothZoomControl(sz_key);
        }
    }

    /// Sets camera move controls (1st person and 3rd person cameras).
    #[inline]
    pub fn set_camera_move_controls(&self,
        front_key: i32,
        back_key: i32,
        right_key: i32,
        left_key: i32,
        up_key: i32,
        down_key: i32)
    {
        unsafe {
            raylib::SetCameraMoveControls(front_key, back_key, right_key, left_key, up_key, down_key);
        }
    }

    /// Draws a pixel.
    #[inline]
    pub fn draw_pixel(&self, x: i32, y: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawPixel(x, y, color.into());
        }
    }

    /// Draws a pixel (Vector version).
    #[inline]
    pub fn draw_pixel_v(&self, position: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawPixelV(position.into(), color.into());
        }
    }

    /// Draws a line.
    #[inline]
    pub fn draw_line(&self, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color.into());
        }
    }

    /// Draws a line (Vector version).
    #[inline]
    pub fn draw_line_v(&self, start_pos: impl Into<Vector2>, end_pos: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawLineV(start_pos.into(), end_pos.into(), color.into());
        }
    }

    /// Draws a line with thickness.
    #[inline]
    pub fn draw_line_ex(&self, start_pos: impl Into<Vector2>, end_pos: impl Into<Vector2>, thick: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawLineEx(start_pos.into(), end_pos.into(), thick, color.into());
        }
    }

    /// Draws a line using cubic-bezier curves in-out.
    #[inline]
    pub fn draw_line_bezier(&self, start_pos: impl Into<Vector2>, end_pos: impl Into<Vector2>, thick: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawLineBezier(start_pos.into(), end_pos.into(), thick, color.into());
        }
    }

    /// Draws a color-filled circle.
    #[inline]
    pub fn draw_circle(&self, center_x: i32, center_y: i32, radius: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCircle(center_x, center_y, radius, color.into());
        }
    }

    /// Draws a gradient-filled circle.
    #[inline]
    pub fn draw_circle_gradient(&self, center_x: i32, center_y: i32, radius: f32, color1: impl Into<Color>, color2: impl Into<Color>) {
        unsafe {
            raylib::DrawCircleGradient(center_x, center_y, radius, color1.into(), color2.into());
        }
    }

    /// Draws a color-filled circle (Vector version).
    #[inline]
    pub fn draw_circle_v(&self, center: impl Into<Vector2>, radius: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCircleV(center.into(), radius, color.into());
        }
    }

    /// Draws circle outline.
    #[inline]
    pub fn draw_circle_lines(&self, center_x: i32, center_y: i32, radius: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCircleLines(center_x, center_y, radius, color.into());
        }
    }

    /// Draws a color-filled rectangle.
    #[inline]
    pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangle(x, y, width, height, color.into());
        }
    }

    /// Draws a color-filled rectangle (Vector version).
    #[inline]
    pub fn draw_rectangle_v(&self, position: impl Into<Vector2>, size: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangleV(position.into(), size.into(), color.into());
        }
    }

    /// Draws a color-filled rectangle from `rec`.
    #[inline]
    pub fn draw_rectangle_rec(&self, rec: Rectangle, color: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangleRec(rec, color.into());
        }
    }

    /// Draws a color-filled rectangle with pro parameters.
    #[inline]
    pub fn draw_rectangle_pro(&self, rec: Rectangle, origin: impl Into<Vector2>, rotation: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawRectanglePro(rec, origin.into(), rotation, color.into());
        }
    }

    /// Draws a vertical-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    pub fn draw_rectangle_gradient_v(&self, x: i32, y: i32, width: i32, height: i32, color1: impl Into<Color>, color2: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangleGradientV(x, y, width, height, color1.into(), color2.into());
        }
    }

    /// Draws a horizontal-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    pub fn draw_rectangle_gradient_h(&self, x: i32, y: i32, width: i32, height: i32, color1: impl Into<Color>, color2: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangleGradientH(x, y, width, height, color1.into(), color2.into());
        }
    }

    /// Draws a gradient-filled rectangle with custom vertex colors.
    ///
    /// **NOTE**: Colors refer to corners, starting at top-left corner and going counter-clockwise.
    #[inline]
    pub fn draw_rectangle_gradient_ex(&self, rec: Rectangle, col1: impl Into<Color>, col2: impl Into<Color>, col3: impl Into<Color>, col4: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangleGradientEx(rec, col1.into(), col2.into(), col3.into(), col4.into());
        }
    }

    /// Draws rectangle outline.
    #[inline]
    pub fn draw_rectangle_lines(&self, x: i32, y: i32, width: i32, height: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangleLines(x, y, width, height, color.into());
        }
    }

    /// Draws rectangle outline with extended parameters.
    #[inline]
    pub fn draw_rectangle_lines_ex(&self, rec: Rectangle, line_thick: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawRectangleLinesEx(rec, line_thick, color.into());
        }
    }

    /// Draws a triangle.
    #[inline]
    pub fn draw_triangle(&self, v1: impl Into<Vector2>, v2: impl Into<Vector2>, v3: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawTriangle(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draws a triangle using lines.
    #[inline]
    pub fn draw_triangle_lines(&self, v1: impl Into<Vector2>, v2: impl Into<Vector2>, v3: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawTriangleLines(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    pub fn draw_poly(&self, center: impl Into<Vector2>, sides: i32, radius: f32, rotation: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawPoly(center.into(), sides, radius, rotation, color.into());
        }
    }

    /// Draws a closed polygon defined by points.
    #[inline]
    pub fn draw_poly_ex(&self, points: &mut [Vector2], color: impl Into<Color>) {
        unsafe {
            raylib::DrawPolyEx(points.as_mut_ptr(), points.len() as i32, color.into());
        }
    }

    /// Draws a polygon using lines.
    #[inline]
    pub fn draw_poly_ex_lines(&self, points: &mut [Vector2], color: impl Into<Color>) {
        unsafe {
            raylib::DrawPolyExLines(points.as_mut_ptr(), points.len() as i32, color.into());
        }
    }

    /// Checks collision between two rectangles.
    #[inline]
    pub fn check_collision_recs(&self, rec1: Rectangle, rec2: Rectangle) -> bool {
        unsafe {
            raylib::CheckCollisionRecs(rec1, rec2)
        }
    }

    /// Checks collision between two circles.
    #[inline]
    pub fn check_collision_circles(&self, center1: impl Into<Vector2>, radius1: f32, center2: impl Into<Vector2>, radius2: f32) -> bool {
        unsafe {
            raylib::CheckCollisionCircles(center1.into(), radius1, center2.into(), radius2)
        }
    }

    /// Checks collision between circle and rectangle.
    #[inline]
    pub fn check_collision_circle_rec(&self, center: impl Into<Vector2>, radius: f32, rec: Rectangle) -> bool {
        unsafe {
            raylib::CheckCollisionCircleRec(center.into(), radius, rec)
        }
    }

    /// Gets the overlap between two colliding rectangles.
    #[inline]
    pub fn get_collision_rec(&self, rec1: Rectangle, rec2: Rectangle) -> Rectangle {
        unsafe {
            raylib::GetCollisionRec(rec1, rec2)
        }
    }

    /// Checks if point is inside rectangle.
    #[inline]
    pub fn check_collision_point_rec(&self, point: impl Into<Vector2>, rec: Rectangle) -> bool {
        unsafe {
            raylib::CheckCollisionPointRec(point.into(), rec)
        }
    }

    /// Checks if point is inside circle.
    #[inline]
    pub fn check_collision_point_circle(&self, point: impl Into<Vector2>, center: impl Into<Vector2>, radius: f32) -> bool {
        unsafe {
            raylib::CheckCollisionPointCircle(point.into(), center.into(), radius)
        }
    }

    /// Checks if point is inside a triangle.
    #[inline]
    pub fn check_collision_point_triangle(&self, point: impl Into<Vector2>, p1: impl Into<Vector2>, p2: impl Into<Vector2>, p3: impl Into<Vector2>) -> bool {
        unsafe {
            raylib::CheckCollisionPointTriangle(point.into(), p1.into(), p2.into(), p3.into())
        }
    }

    /// Loads image from file into CPU memory (RAM).
    #[inline]
    pub fn load_image(&self, filename: &str) -> Image {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Image(raylib::LoadImage(c_filename.as_ptr()))
        }
    }

    /// Loads image from Color array data (RGBA - 32bit).
    #[inline]
    pub fn load_image_ex(&self, pixels: &mut [Color], width: i32, height: i32) -> Image {
        let expected_len = (width * height) as usize;
        if pixels.len() != expected_len {
            panic!("load_image_ex: Data is wrong size. Expected {}, got {}", expected_len, pixels.len());
        }
        unsafe {
            Image(raylib::LoadImageEx(pixels.as_mut_ptr(), width, height))
        }
    }

    /// Loads image from raw data with parameters.
    #[inline]
    pub fn load_image_pro(&self, data: &[u8], width: i32, height: i32, format: PixelFormat) -> Image {
        let expected_len = self.get_pixel_data_size(width, height, format) as usize;
        if data.len() != expected_len {
            panic!("load_image_pro: Data is wrong size. Expected {}, got {}", expected_len, data.len());
        }
        unsafe {
            Image(raylib::LoadImagePro(data.as_ptr() as *mut std::os::raw::c_void, width, height, format))
        }
    }

    /// Loads image from RAW file data.
    #[inline]
    pub fn load_image_raw(&self, filename: &str, width: i32, height: i32, format: i32, header_size: i32) -> Image {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Image(raylib::LoadImageRaw(c_filename.as_ptr(), width, height, format, header_size))
        }
    }

    /// Exports image as a PNG file.
    #[inline]
    pub fn export_image(&self, filename: &str, image: &Image) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            raylib::ExportImage(c_filename.as_ptr(), image.0);
        }
    }

    /// Loads texture from file into GPU memory (VRAM).
    #[inline]
    pub fn load_texture(&self, filename: &str) -> Texture2D {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Texture2D(raylib::LoadTexture(c_filename.as_ptr()))
        }
    }

    /// Loads texture from image data.
    #[inline]
    pub fn load_texture_from_image(&self, image: &Image) -> Texture2D {
        unsafe {
            Texture2D(raylib::LoadTextureFromImage(image.0))
        }
    }

    /// Loads texture for rendering (framebuffer).
    #[inline]
    pub fn load_render_texture(&self, width: i32, height: i32) -> RenderTexture2D {
        unsafe {
            RenderTexture2D(raylib::LoadRenderTexture(width, height))
        }
    }

    /// Gets pixel data from `image` as a Vec of Color structs.
    #[inline]
    pub fn get_image_data(&self, image: &Image) -> Vec<Color> {
        unsafe {
            let image_data = raylib::GetImageData(image.0);
            let image_data_len = (image.width * image.height) as usize;
            let mut safe_image_data = Vec::with_capacity(image_data_len);
            safe_image_data.set_len(image_data_len);
            std::ptr::copy(image_data, safe_image_data.as_mut_ptr(), image_data_len);
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    /// Gets normalized (`0.0` to `1.0`) pixel data from `image` as a Vec of Vector4 structs.
    #[inline]
    pub fn get_image_data_normalized(&self, image: &Image) -> Vec<Vector4> {
        unsafe {
            let image_data = raylib::GetImageDataNormalized(image.0);
            let image_data_len = (image.width * image.height) as usize;
            let mut safe_image_data = Vec::with_capacity(image_data_len);
            safe_image_data.set_len(image_data_len);
            std::ptr::copy(image_data, safe_image_data.as_mut_ptr(), image_data_len);
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    /// Gets pixel data size in bytes (image or texture).
    #[inline]
    pub fn get_pixel_data_size(&self, width: i32, height: i32, format: PixelFormat) -> i32 {
        unsafe {
            raylib::GetPixelDataSize(width, height, format)
        }
    }

    /// Gets pixel data from GPU texture and returns an `Image`.
    #[inline]
    pub fn get_texture_data(&self, texture: &Texture2D) -> Image {
        unsafe {
            Image(raylib::GetTextureData(texture.0))
        }
    }

    /// Updates GPU texture with new data.
    #[inline]
    pub fn update_texture(&self, texture: &mut Texture2D, pixels: &[u8]) {
        let expected_len = self.get_pixel_data_size(texture.width, texture.height, texture.format) as usize;
        if pixels.len() != expected_len {
            panic!("update_texture: Data is wrong size. Expected {}, got {}", expected_len, pixels.len());
        }
        unsafe {
            raylib::UpdateTexture(texture.0, pixels.as_ptr() as *const std::os::raw::c_void);
        }
    }

    /// Creates an image duplicate (useful for transformations).
    #[inline]
    pub fn image_copy(&self, image: &Image) -> Image {
        unsafe {
            Image(raylib::ImageCopy(image.0))
        }
    }

    /// Converts `image` to POT (power-of-two).
    #[inline]
    pub fn image_to_pot(&self, image: &mut Image, fill_color: impl Into<Color>) {
        unsafe {
            raylib::ImageToPOT(&mut image.0, fill_color.into());
        }
    }

    /// Converts `image` data to desired pixel format.
    #[inline]
    pub fn image_format(&self, image: &mut Image, new_format: PixelFormat) {
        unsafe {
            raylib::ImageFormat(&mut image.0, new_format);
        }
    }

    /// Applies alpha mask to `image`.
    #[inline]
    pub fn image_alpha_mask(&self, image: &mut Image, alpha_mask: &Image) {
        unsafe {
            raylib::ImageAlphaMask(&mut image.0, alpha_mask.0);
        }
    }

    /// Clears alpha channel on `image` to desired color.
    #[inline]
    pub fn image_alpha_clear(&self, image: &mut Image, color: impl Into<Color>, threshold: f32) {
        unsafe {
            raylib::ImageAlphaClear(&mut image.0, color.into(), threshold);
        }
    }

    /// Crops `image` depending on alpha value.
    #[inline]
    pub fn image_alpha_crop(&self, image: &mut Image, threshold: f32) {
        unsafe {
            raylib::ImageAlphaCrop(&mut image.0, threshold);
        }
    }

    /// Premultiplies alpha channel on `image`.
    #[inline]
    pub fn image_alpha_premultiply(&self, image: &mut Image) {
        unsafe {
            raylib::ImageAlphaPremultiply(&mut image.0);
        }
    }

    /// Crops `image` to a defined rectangle.
    #[inline]
    pub fn image_crop(&self, image: &mut Image, crop: Rectangle) {
        unsafe {
            raylib::ImageCrop(&mut image.0, crop);
        }
    }

    /// Resizes `image` (bilinear filtering).
    #[inline]
    pub fn image_resize(&self, image: &mut Image, new_width: i32, new_height: i32) {
        unsafe {
            raylib::ImageResize(&mut image.0, new_width, new_height);
        }
    }

    /// Resizes `image` (nearest-neighbor scaling).
    #[inline]
    pub fn image_resize_nn(&self, image: &mut Image, new_width: i32, new_height: i32) {
        unsafe {
            raylib::ImageResizeNN(&mut image.0, new_width, new_height);
        }
    }

    /// Resizes `image` canvas and fills with `color`.
    #[inline]
    pub fn image_resize_canvas(&self, image: &mut Image, new_width: i32, new_height: i32, offset_x: i32, offset_y: i32, color: impl Into<Color>) {
        unsafe {
            raylib::ImageResizeCanvas(&mut image.0, new_width, new_height, offset_x, offset_y, color.into());
        }
    }

    /// Generates all mipmap levels for a provided `image`.
    #[inline]
    pub fn image_mipmaps(&self, image: &mut Image) {
        unsafe {
            raylib::ImageMipmaps(&mut image.0);
        }
    }

    /// Dithers `image` data to 16bpp or lower (Floyd-Steinberg dithering).
    #[inline]
    pub fn image_dither(&self, image: &mut Image, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
        unsafe {
            raylib::ImageDither(&mut image.0, r_bpp, g_bpp, b_bpp, a_bpp);
        }
    }

    /// Creates an image from `text` (default font).
    #[inline]
    pub fn image_text(&self, text: &str, font_size: i32, color: impl Into<Color>) -> Image {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Image(raylib::ImageText(c_text.as_ptr(), font_size, color.into()))
        }
    }

    /// Creates an image from `text` (custom font).
    #[inline]
    pub fn image_text_ex(&self, font: &Font, text: &str, font_size: f32, spacing: f32, tint: impl Into<Color>) -> Image {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Image(raylib::ImageTextEx(font.0, c_text.as_ptr(), font_size, spacing, tint.into()))
        }
    }

    /// Draws a source image within a destination image.
    #[inline]
    pub fn image_draw(&self, dst: &mut Image, src: &Image, src_rec: Rectangle, dst_rec: Rectangle) {
        unsafe {
            raylib::ImageDraw(&mut dst.0, src.0, src_rec, dst_rec);
        }
    }

    /// Draws a rectangle within an image.
    #[inline]
    pub fn image_draw_rectangle(&self, dst: &mut Image, position: impl Into<Vector2>, rec: Rectangle, color: impl Into<Color>) {
        unsafe {
            raylib::ImageDrawRectangle(&mut dst.0, position.into(), rec, color.into());
        }
    }

    /// Draws text (default font) within an image (destination).
    #[inline]
    pub fn image_draw_text(&self, dst: &mut Image, position: impl Into<Vector2>, text: &str, font_size: i32, color: impl Into<Color>) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            raylib::ImageDrawText(&mut dst.0, position.into(), c_text.as_ptr(), font_size, color.into());
        }
    }

    /// Draws text (custom font) within an image (destination).
    #[inline]
    pub fn image_draw_text_ex(&self, dst: &mut Image, position: impl Into<Vector2>, font: &Font, text: &str, font_size: f32, spacing: f32, color: impl Into<Color>) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            raylib::ImageDrawTextEx(&mut dst.0, position.into(), font.0, c_text.as_ptr(), font_size, spacing, color.into());
        }
    }

    /// Flips `image` vertically.
    #[inline]
    pub fn image_flip_vertical(&self, image: &mut Image) {
        unsafe {
            raylib::ImageFlipVertical(&mut image.0);
        }
    }

    /// Flips `image` horizontally.
    #[inline]
    pub fn image_flip_horizontal(&self, image: &mut Image) {
        unsafe {
            raylib::ImageFlipHorizontal(&mut image.0);
        }
    }

    /// Rotates `image` clockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn image_rotate_cw(&self, image: &mut Image) {
        unsafe {
            raylib::ImageRotateCW(&mut image.0);
        }
    }

    /// Rotates `image` counterclockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn image_rotate_ccw(&self, image: &mut Image) {
        unsafe {
            raylib::ImageRotateCCW(&mut image.0);
        }
    }

    /// Tints colors in `image` using specified `color`.
    #[inline]
    pub fn image_color_tint(&self, image: &mut Image, color: impl Into<Color>) {
        unsafe {
            raylib::ImageColorTint(&mut image.0, color.into());
        }
    }

    /// Inverts the colors in `image`.
    #[inline]
    pub fn image_color_invert(&self, image: &mut Image) {
        unsafe {
            raylib::ImageColorInvert(&mut image.0);
        }
    }

    /// Converts `image color to grayscale.
    #[inline]
    pub fn image_color_grayscale(&self, image: &mut Image) {
        unsafe {
            raylib::ImageColorGrayscale(&mut image.0);
        }
    }

    /// Adjusts the contrast of `image`.
    #[inline]
    pub fn image_color_contrast(&self, image: &mut Image, contrast: f32) {
        unsafe {
            raylib::ImageColorContrast(&mut image.0, contrast);
        }
    }

    /// Adjusts the brightness of `image`.
    #[inline]
    pub fn image_color_brightness(&self, image: &mut Image, brightness: i32) {
        unsafe {
            raylib::ImageColorBrightness(&mut image.0, brightness);
        }
    }

    /// Searches `image` for all occurences of `color` and replaces them with `replace` color.
    #[inline]
    pub fn image_color_replace(&self, image: &mut Image, color: impl Into<Color>, replace: impl Into<Color>) {
        unsafe {
            raylib::ImageColorReplace(&mut image.0, color.into(), replace.into());
        }
    }

    /// Generates a plain `color` Image.
    #[inline]
    pub fn gen_image_color(&self, width: i32, height: i32, color: impl Into<Color>) -> Image {
        unsafe {
            Image(raylib::GenImageColor(width, height, color.into()))
        }
    }

    /// Generates an Image containing a vertical gradient.
    #[inline]
    pub fn gen_image_gradient_v(&self, width: i32, height: i32, top: impl Into<Color>, bottom: impl Into<Color>) -> Image {
        unsafe {
            Image(raylib::GenImageGradientV(width, height, top.into(), bottom.into()))
        }
    }

    /// Generates an Image containing a horizonal gradient.
    #[inline]
    pub fn gen_image_gradient_h(&self, width: i32, height: i32, left: impl Into<Color>, right: impl Into<Color>) -> Image {
        unsafe {
            Image(raylib::GenImageGradientH(width, height, left.into(), right.into()))
        }
    }

    /// Generates an Image containing a radial gradient.
    #[inline]
    pub fn gen_image_gradient_radial(&self, width: i32, height: i32, density: f32, inner: impl Into<Color>, outer: impl Into<Color>) -> Image {
        unsafe {
            Image(raylib::GenImageGradientRadial(width, height, density, inner.into(), outer.into()))
        }
    }

    /// Generates an Image containing a checkerboard pattern.
    #[inline]
    pub fn gen_image_checked(&self, width: i32, height: i32, checks_x: i32, checks_y: i32, col1: impl Into<Color>, col2: impl Into<Color>) -> Image {
        unsafe {
            Image(raylib::GenImageChecked(width, height, checks_x, checks_y, col1.into(), col2.into()))
        }
    }

    /// Generates an Image containing white noise.
    #[inline]
    pub fn gen_image_white_noise(&self, width: i32, height: i32, factor: f32) -> Image {
        unsafe {
            Image(raylib::GenImageWhiteNoise(width, height, factor))
        }
    }

    /// Generates an Image containing perlin noise.
    #[inline]
    pub fn gen_image_perlin_noise(&self, width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Image {
        unsafe {
            Image(raylib::GenImagePerlinNoise(width, height, offset_x, offset_y, scale))
        }
    }

    /// Generates an Image using a cellular algorithm. Bigger `tile_size` means bigger cells.
    #[inline]
    pub fn gen_image_cellular(&self, width: i32, height: i32, tile_size: i32) -> Image {
        unsafe {
            Image(raylib::GenImageCellular(width, height, tile_size))
        }
    }

    /// Generates GPU mipmaps for a `texture`.
    #[inline]
    pub fn gen_texture_mipmaps(&self, texture: &mut Texture2D) {
        unsafe {
            raylib::GenTextureMipmaps(&mut texture.0);
        }
    }

    /// Sets `texture` scaling filter mode.
    #[inline]
    pub fn set_texture_filter(&self, texture: &mut Texture2D, filter_mode: TextureFilterMode) {
        unsafe {
            raylib::SetTextureFilter(texture.0, filter_mode);
        }
    }

    /// Sets texture wrapping mode.
    #[inline]
    pub fn set_texture_wrap(&self, texture: &mut Texture2D, wrap_mode: TextureWrapMode) {
        unsafe {
            raylib::SetTextureWrap(texture.0, wrap_mode);
        }
    }

    /// Draws a `texture` using specified position and `tint` color.
    #[inline]
    pub fn draw_texture(&self, texture: &Texture2D, x: i32, y: i32, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawTexture(texture.0, x, y, tint.into());
        }
    }

    /// Draws a `texture` using specified `position` vector and `tint` color.
    #[inline]
    pub fn draw_texture_v(&self, texture: &Texture2D, position: impl Into<Vector2>, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawTextureV(texture.0, position.into(), tint.into());
        }
    }

    /// Draws a `texture` with extended parameters.
    #[inline]
    pub fn draw_texture_ex(&self, texture: &Texture2D, position: impl Into<Vector2>, rotation: f32, scale: f32, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawTextureEx(texture.0, position.into(), rotation, scale, tint.into());
        }
    }

    /// Draws from a region of `texture` defined by the `source_rec` rectangle.
    #[inline]
    pub fn draw_texture_rec(&self, texture: &Texture2D, source_rec: Rectangle, position: impl Into<Vector2>, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawTextureRec(texture.0, source_rec, position.into(), tint.into());
        }
    }

    /// Draw from a region of `texture` defined by the `source_rec` rectangle with pro parameters.
    #[inline]
    pub fn draw_texture_pro(&self, texture: &Texture2D, source_rec: Rectangle, dest_rec: Rectangle, origin: impl Into<Vector2>, rotation: f32, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawTexturePro(texture.0, source_rec, dest_rec, origin.into(), rotation, tint.into());
        }
    }

    /// Gets the default font.
    #[inline]
    pub fn get_font_default(&self) -> &'static Font {
        &FONT_DEFAULT
    }

    /// Loads font from file into GPU memory (VRAM).
    #[inline]
    pub fn load_font(&self, filename: &str) -> Font {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Font(raylib::LoadFont(c_filename.as_ptr()))
        }
    }

    /// Loads font from file with extended parameters.
    #[inline]
    pub fn load_font_ex(&self, filename: &str, font_size: i32, chars: Option<&[i32]>) -> Font {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            match chars {
                Some(c) => Font(raylib::LoadFontEx(c_filename.as_ptr(), font_size, c.len() as i32, c.as_ptr() as *mut i32)),
                None => Font(raylib::LoadFontEx(c_filename.as_ptr(), font_size, 0, std::ptr::null_mut()))
            }
        }
    }

    /// Loads font data for further use (see also `Font::from_data`).
    #[inline]
    pub fn load_font_data(&self, filename: &str, font_size: i32, chars: Option<&[i32]>, sdf: bool) -> Vec<CharInfo> {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            let ci_arr_ptr = match chars {
                Some(c) => {
                    raylib::LoadFontData(c_filename.as_ptr(), font_size, c.as_ptr() as *mut i32, c.len() as i32, sdf)
                }
                None => {
                    raylib::LoadFontData(c_filename.as_ptr(), font_size, std::ptr::null_mut(), 0, sdf)
                }
            };
            let ci_size = if let Some(c) = chars { c.len() } else { 95 }; // raylib assumes 95 if none given
            let mut ci_vec = Vec::with_capacity(ci_size);
            for i in 0..ci_size {
                ci_vec.push(*ci_arr_ptr.offset(i as isize));
            }
            libc::free(ci_arr_ptr as *mut libc::c_void);
            ci_vec
        }
    }

    /// Generates image font atlas using `chars` info.
    #[inline]
    pub fn gen_image_font_atlas(&self, chars: &mut [CharInfo], font_size: i32, padding: i32, pack_method: i32) -> Image {
        unsafe {
            Image(raylib::GenImageFontAtlas(chars.as_mut_ptr(), font_size, chars.len() as i32, padding, pack_method))
        }
    }

    /// Shows current FPS.
    #[inline]
    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            raylib::DrawFPS(x, y);
        }
    }

    /// Draws text (using default font).
    #[inline]
    pub fn draw_text(&self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<Color>) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            raylib::DrawText(c_text.as_ptr(), x, y, font_size, color.into());
        }
    }

    /// Draws text using `font` and additional parameters.
    #[inline]
    pub fn draw_text_ex(&self, font: &Font, text: &str, position: impl Into<Vector2>, font_size: f32, spacing: f32, tint: impl Into<Color>) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            raylib::DrawTextEx(font.0, c_text.as_ptr(), position.into(), font_size, spacing, tint.into());
        }
    }

    /// Measures string width in pixels for default font.
    #[inline]
    pub fn measure_text(&self, text: &str, font_size: i32) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe {
            raylib::MeasureText(c_text.as_ptr(), font_size)
        }
    }

    /// Measures string width in pixels for `font`.
    #[inline]
    pub fn measure_text_ex(&self, font: &Font, text: &str, font_size: f32, spacing: f32) -> Vector2 {
        let c_text = CString::new(text).unwrap();
        unsafe {
            raylib::MeasureTextEx(font.0, c_text.as_ptr(), font_size, spacing)
        }
    }

    /// Gets index position for a unicode character on `font`.
    #[inline]
    pub fn get_glyph_index(&self, font: &Font, character: i32) -> i32 {
        unsafe {
            raylib::GetGlyphIndex(font.0, character)
        }
    }

    /// Draws a line in 3D world space.
    #[inline]
    pub fn draw_line_3d(&self, start_pos: impl Into<Vector3>, end_pos: impl Into<Vector3>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawLine3D(start_pos.into(), end_pos.into(), color.into());
        }
    }

    /// Draws a circle in 3D world space.
    #[inline]
    pub fn draw_circle_3d(&self, center: impl Into<Vector3>, radius: f32, rotation_axis: impl Into<Vector3>, rotation_angle: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCircle3D(center.into(), radius, rotation_axis.into(), rotation_angle, color.into());
        }
    }

    /// Draws a cube.
    #[inline]
    pub fn draw_cube(&self, position: impl Into<Vector3>, width: f32, height: f32, length: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCube(position.into(), width, height, length, color.into());
        }
    }

    /// Draws a cube (Vector version).
    #[inline]
    pub fn draw_cube_v(&self, position: impl Into<Vector3>, size: impl Into<Vector3>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCubeV(position.into(), size.into(), color.into());
        }
    }

    /// Draws a cube in wireframe.
    #[inline]
    pub fn draw_cube_wires(&self, position: impl Into<Vector3>, width: f32, height: f32, length: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCubeWires(position.into(), width, height, length, color.into());
        }
    }

    /// Draws a textured cube.
    #[inline]
    pub fn draw_cube_texture(&self, texture: &Texture2D, position: impl Into<Vector3>, width: f32, height: f32, length: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCubeTexture(texture.0, position.into(), width, height, length, color.into());
        }
    }

    /// Draws a sphere.
    #[inline]
    pub fn draw_sphere(&self, center_pos: impl Into<Vector3>, radius: f32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawSphere(center_pos.into(), radius, color.into());
        }
    }

    /// Draws a sphere with extended parameters.
    #[inline]
    pub fn draw_sphere_ex(&self, center_pos: impl Into<Vector3>, radius: f32, rings: i32, slices: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawSphereEx(center_pos.into(), radius, rings, slices, color.into());
        }
    }

    /// Draws a sphere in wireframe.
    #[inline]
    pub fn draw_sphere_wires(&self, center_pos: impl Into<Vector3>, radius: f32, rings: i32, slices: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawSphereWires(center_pos.into(), radius, rings, slices, color.into());
        }
    }

    /// Draws a cylinder.
    #[inline]
    pub fn draw_cylinder(&self, position: impl Into<Vector3>, radius_top: f32, radius_bottom: f32, height: f32, slices: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCylinder(position.into(), radius_top, radius_bottom, height, slices, color.into());
        }
    }

    /// Draws a cylinder in wireframe.
    #[inline]
    pub fn draw_cylinder_wires(&self, position: impl Into<Vector3>, radius_top: f32, radius_bottom: f32, height: f32, slices: i32, color: impl Into<Color>) {
        unsafe {
            raylib::DrawCylinderWires(position.into(), radius_top, radius_bottom, height, slices, color.into());
        }
    }

    /// Draws an X/Z plane.
    #[inline]
    pub fn draw_plane(&self, center_pos: impl Into<Vector3>, size: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            raylib::DrawPlane(center_pos.into(), size.into(), color.into());
        }
    }

    /// Draws a ray line.
    #[inline]
    pub fn draw_ray(&self, ray: Ray, color: impl Into<Color>) {
        unsafe {
            raylib::DrawRay(ray, color.into());
        }
    }

    /// Draws a grid (centered at (0, 0, 0)).
    #[inline]
    pub fn draw_grid(&self, slices: i32, spacing: f32) {
        unsafe {
            raylib::DrawGrid(slices, spacing);
        }
    }

    /// Draws a simple gizmo.
    #[inline]
    pub fn draw_gizmo(&self, position: impl Into<Vector3>) {
        unsafe {
            raylib::DrawGizmo(position.into());
        }
    }

    /// Loads model from files (mesh and material).
    #[inline]
    pub fn load_model(&self, filename: &str) -> Model {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Model(raylib::LoadModel(c_filename.as_ptr()))
        }
    }

    /// Loads model from generated mesh. Returned Model takes ownership of `mesh`.
    #[inline]
    pub fn load_model_from_mesh(&self, mesh: Mesh) -> Model {
        unsafe {
            let m = mesh.0;
            std::mem::forget(mesh);
            Model(raylib::LoadModelFromMesh(m))
        }
    }

    /// Loads mesh from file.
    #[inline]
    pub fn load_mesh(&self, filename: &str) -> Mesh {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Mesh(raylib::LoadMesh(c_filename.as_ptr()))
        }
    }

    /// Exports mesh as an OBJ file.
    #[inline]
    pub fn export_mesh(&self, filename: &str, mesh: &Mesh) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            raylib::ExportMesh(c_filename.as_ptr(), mesh.0);
        }
    }

    /// Computes mesh bounding box limits.
    #[inline]
    pub fn mesh_bounding_box(&self, mesh: &Mesh) -> BoundingBox {
        unsafe {
            raylib::MeshBoundingBox(mesh.0)
        }
    }

    /// Computes mesh tangents.
    #[inline]
    pub fn mesh_tangents(&self, mesh: &mut Mesh) {
        unsafe {
            raylib::MeshTangents(&mut mesh.0);
        }
    }

    /// Computes mesh binormals.
    #[inline]
    pub fn mesh_binormals(&self, mesh: &mut Mesh) {
        unsafe {
            raylib::MeshBinormals(&mut mesh.0);
        }
    }

    /// Generates plane mesh (with subdivisions).
    #[inline]
    pub fn gen_mesh_plane(&self, width: f32, length: f32, res_x: i32, res_z: i32) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshPlane(width, length, res_x, res_z))
        }
    }

    /// Generates cuboid mesh.
    #[inline]
    pub fn gen_mesh_cube(&self, width: f32, height: f32, length: f32) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshCube(width, height, length))
        }
    }

    /// Generates sphere mesh (standard sphere).
    #[inline]
    pub fn gen_mesh_sphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshSphere(radius, rings, slices))
        }
    }

    /// Generates half-sphere mesh (no bottom cap).
    #[inline]
    pub fn gen_mesh_hemisphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshHemiSphere(radius, rings, slices))
        }
    }

    /// Generates cylinder mesh.
    #[inline]
    pub fn gen_mesh_cylinder(&self, radius: f32, height: f32, slices: i32) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshCylinder(radius, height, slices))
        }
    }

    /// Generates torus mesh.
    #[inline]
    pub fn gen_mesh_torus(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshTorus(radius, size, rad_seg, sides))
        }
    }

    /// Generates trefoil knot mesh.
    #[inline]
    pub fn gen_mesh_knot(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshKnot(radius, size, rad_seg, sides))
        }
    }

    /// Generates heightmap mesh from image data.
    #[inline]
    pub fn gen_mesh_heightmap(&self, heightmap: &Image, size: impl Into<Vector3>) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshHeightmap(heightmap.0, size.into()))
        }
    }

    /// Generates cubes-based map mesh from image data.
    #[inline]
    pub fn gen_mesh_cubicmap(&self, cubicmap: &Image, cube_size: impl Into<Vector3>) -> Mesh {
        unsafe {
            Mesh(raylib::GenMeshCubicmap(cubicmap.0, cube_size.into()))
        }
    }

    /// Loads material from file.
    #[inline]
    pub fn load_material(&self, filename: &str) -> Material {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Material(raylib::LoadMaterial(c_filename.as_ptr()))
        }
    }

    /// Loads default material (supports `DIFFUSE`, `SPECULAR`, and `NORMAL` maps).
    #[inline]
    pub fn load_material_default(&self) -> &'static Material {
        &MATERIAL_DEFAULT
    }

    /// Draws a model (with texture if set).
    #[inline]
    pub fn draw_model(&self, model: &Model, position: impl Into<Vector3>, scale: f32, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawModel(model.0, position.into(), scale, tint.into());
        }
    }

    /// Draws a model with extended parameters.
    #[inline]
    pub fn draw_model_ex(&self, model: &Model, position: impl Into<Vector3>, rotation_axis: impl Into<Vector3>, rotation_angle: f32, scale: impl Into<Vector3>, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawModelEx(model.0, position.into(), rotation_axis.into(), rotation_angle, scale.into(), tint.into());
        }
    }

    /// Draws a model with wires (with texture if set).
    #[inline]
    pub fn draw_model_wires(&self, model: &Model, position: impl Into<Vector3>, scale: f32, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawModelWires(model.0, position.into(), scale, tint.into());
        }
    }

    /// Draws a model with wires.
    #[inline]
    pub fn draw_model_wires_ex(&self, model: &Model, position: impl Into<Vector3>, rotation_axis: impl Into<Vector3>, rotation_angle: f32, scale: impl Into<Vector3>, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawModelWiresEx(model.0, position.into(), rotation_axis.into(), rotation_angle, scale.into(), tint.into());
        }
    }

    /// Draws a bounding box (wires).
    #[inline]
    pub fn draw_bounding_box(&self, bbox: BoundingBox, color: impl Into<Color>) {
        unsafe {
            raylib::DrawBoundingBox(bbox, color.into());
        }
    }

    /// Draws a billboard texture.
    #[inline]
    pub fn draw_billboard(&self, camera: Camera3D, texture: &Texture2D, center: impl Into<Vector3>, size: f32, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawBillboard(camera, texture.0, center.into(), size, tint.into());
        }
    }

    /// Draws a billboard texture defined by `source_rec`.
    #[inline]
    pub fn draw_billboard_rec(&self, camera: Camera3D, texture: &Texture2D, source_rec: Rectangle, center: impl Into<Vector3>, size: f32, tint: impl Into<Color>) {
        unsafe {
            raylib::DrawBillboardRec(camera, texture.0, source_rec, center.into(), size, tint.into());
        }
    }

    /// Detects collision between two spheres.
    #[inline]
    pub fn check_collision_spheres(&self, center_a: impl Into<Vector3>, radius_a: f32, center_b: impl Into<Vector3>, radius_b: f32) -> bool {
        unsafe {
            raylib::CheckCollisionSpheres(center_a.into(), radius_a, center_b.into(), radius_b)
        }
    }

    /// Detects collision between two boxes.
    #[inline]
    pub fn check_collision_boxes(&self, box1: BoundingBox, box2: BoundingBox) -> bool {
        unsafe {
            raylib::CheckCollisionBoxes(box1, box2)
        }
    }

    /// Detects collision between box and sphere.
    #[inline]
    pub fn check_collision_box_sphere(&self, bbox: BoundingBox, center_sphere: impl Into<Vector3>, radius_sphere: f32) -> bool {
        unsafe {
            raylib::CheckCollisionBoxSphere(bbox, center_sphere.into(), radius_sphere)
        }
    }

    /// Detects collision between ray and sphere.
    #[inline]
    pub fn check_collision_ray_sphere(&self, ray: Ray, sphere_position: impl Into<Vector3>, sphere_radius: f32) -> bool {
        unsafe {
            raylib::CheckCollisionRaySphere(ray, sphere_position.into(), sphere_radius)
        }
    }

    /// Detects collision between ray and sphere, and returns the collision point.
    #[inline]
    pub fn check_collision_ray_sphere_ex(&self, ray: Ray, sphere_position: impl Into<Vector3>, sphere_radius: f32, collision_point: &mut Vector3) -> bool {
        unsafe {
            raylib::CheckCollisionRaySphereEx(ray, sphere_position.into(), sphere_radius, collision_point)
        }
    }

    /// Detects collision between ray and box.
    #[inline]
    pub fn check_collision_ray_box(&self, ray: Ray, bbox: BoundingBox) -> bool {
        unsafe {
            raylib::CheckCollisionRayBox(ray, bbox)
        }
    }

    /// Gets collision info between ray and model.
    #[inline]
    pub fn get_collision_ray_model(&self, ray: Ray, model: &Model) -> RayHitInfo {
        unsafe {
            let mut model = model.0;
            raylib::GetCollisionRayModel(ray, &mut model)
        }
    }

    /// Gets collision info between ray and triangle.
    #[inline]
    pub fn get_collision_ray_triangle(&self, ray: Ray, p1: impl Into<Vector3>, p2: impl Into<Vector3>, p3: impl Into<Vector3>) -> RayHitInfo {
        unsafe {
            raylib::GetCollisionRayTriangle(ray, p1.into(), p2.into(), p3.into())
        }
    }

    /// Gets collision info between ray and ground plane (Y-normal plane).
    #[inline]
    pub fn get_collision_ray_ground(&self, ray: Ray, ground_height: f32) -> RayHitInfo {
        unsafe {
            raylib::GetCollisionRayGround(ray, ground_height)
        }
    }

    /// Loads a text file and returns its contents in a string.
    #[inline]
    pub fn load_text(&self, filename: &str) -> String {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            let text = raylib::LoadText(c_filename.as_ptr());
            let safe_text = CStr::from_ptr(text).to_str().unwrap().to_owned();
            libc::free(text as *mut libc::c_void);
            safe_text
        }
    }

    /// Loads a custom shader and binds default locations.
    #[inline]
    pub fn load_shader(&self, vs_filename: &str, fs_filename: &str) -> Shader {
        let c_vs_filename = CString::new(vs_filename).unwrap();
        let c_fs_filename = CString::new(fs_filename).unwrap();
        unsafe {
            Shader(raylib::LoadShader(c_vs_filename.as_ptr(), c_fs_filename.as_ptr()))
        }
    }

    /// Loads shader from code strings and binds default locations.
    #[inline]
    pub fn load_shader_code(&self, vs_code: &str, fs_code: &str) -> Shader {
        let c_vs_code = CString::new(vs_code).unwrap();
        let c_fs_code = CString::new(fs_code).unwrap();
        unsafe {
            Shader(raylib::LoadShaderCode(c_vs_code.as_ptr() as *mut i8, c_fs_code.as_ptr() as *mut i8))
        }
    }

    /// Gets default shader.
    #[inline]
    pub fn get_shader_default(&self) -> &'static Shader {
        &SHADER_DEFAULT
    }

    /// Gets default texture.
    #[inline]
    pub fn get_texture_default(&self) -> &'static Texture2D {
        &TEXTURE_DEFAULT
    }

    /// Gets shader uniform location by name.
    #[inline]
    pub fn get_shader_location(&self, shader: &Shader, uniform_name: &str) -> i32 {
        let c_uniform_name = CString::new(uniform_name).unwrap();
        unsafe {
            raylib::GetShaderLocation(shader.0, c_uniform_name.as_ptr())
        }
    }

    /// Sets shader uniform value (`f32`).
    #[inline]
    pub fn set_shader_value(&self, shader: &mut Shader, uniform_loc: i32, value: &[f32]) {
        unsafe {
            raylib::SetShaderValue(shader.0, uniform_loc, value.as_ptr(), value.len() as i32);
        }
    }

    /// Sets shader uniform value (`i32`).
    #[inline]
    pub fn set_shader_value_i(&self, shader: &mut Shader, uniform_loc: i32, value: &[i32]) {
        unsafe {
            raylib::SetShaderValuei(shader.0, uniform_loc, value.as_ptr(), value.len() as i32);
        }
    }

    /// Sets shader uniform value (matrix 4x4).
    #[inline]
    pub fn set_shader_value_matrix(&self, shader: &mut Shader, uniform_loc: i32, mat: Matrix) {
        unsafe {
            raylib::SetShaderValueMatrix(shader.0, uniform_loc, mat);
        }
    }

    /// Sets a custom projection matrix (replaces internal projection matrix).
    #[inline]
    pub fn set_matrix_projection(&self, proj: Matrix) {
        unsafe {
            raylib::SetMatrixProjection(proj);
        }
    }

    /// Sets a custom modelview matrix (replaces internal modelview matrix).
    #[inline]
    pub fn set_matrix_modelview(&self, view: Matrix) {
        unsafe {
            raylib::SetMatrixModelview(view);
        }
    }

    /// Gets internal modelview matrix.
    #[inline]
    pub fn get_matrix_modelview(&self) -> Matrix {
        unsafe {
            raylib::GetMatrixModelview()
        }
    }

    /// Generates cubemap texture from HDR texture.
    #[inline]
    pub fn gen_texture_cubemap(&self, shader: &Shader, sky_hdr: &Texture2D, size: i32) -> Texture2D {
        unsafe {
            Texture2D(raylib::GenTextureCubemap(shader.0, sky_hdr.0, size))
        }
    }

    /// Generates irradiance texture using cubemap data.
    #[inline]
    pub fn gen_texture_irradiance(&self, shader: &Shader, cubemap: &Texture2D, size: i32) -> Texture2D {
        unsafe {
            Texture2D(raylib::GenTextureIrradiance(shader.0, cubemap.0, size))
        }
    }

    /// Generates prefilter texture using cubemap data.
    #[inline]
    pub fn gen_texture_prefilter(&self, shader: &Shader, cubemap: &Texture2D, size: i32) -> Texture2D {
        unsafe {
            Texture2D(raylib::GenTexturePrefilter(shader.0, cubemap.0, size))
        }
    }

    /// Generates BRDF texture using cubemap data.
    #[inline]
    pub fn gen_texture_brdf(&self, shader: &Shader, cubemap: &Texture2D, size: i32) -> Texture2D {
        unsafe {
            Texture2D(raylib::GenTextureBRDF(shader.0, cubemap.0, size))
        }
    }

    /// Begins custom shader drawing.
    #[inline]
    pub fn begin_shader_mode(&self, shader: &Shader) {
        unsafe {
            raylib::BeginShaderMode(shader.0);
        }
    }

    /// Ends custom shader drawing (and switches to default shader).
    #[inline]
    pub fn end_shader_mode(&self) {
        unsafe {
            raylib::EndShaderMode();
        }
    }

    /// Begins blending mode (alpha, additive, multiplied).
    #[inline]
    pub fn begin_blend_mode(&self, mode: BlendMode) {
        unsafe {
            raylib::BeginBlendMode(mode);
        }
    }

    /// Ends blending mode (reset to default: alpha blending).
    #[inline]
    pub fn end_blend_mode(&self) {
        unsafe {
            raylib::EndBlendMode();
        }
    }

    /// Gets VR device information for some standard devices.
    #[inline]
    pub fn get_vr_device_info(&self, vr_device_type: VrDeviceType) -> VrDeviceInfo {
        unsafe {
            raylib::GetVrDeviceInfo(vr_device_type)
        }
    }

    /// Initializes VR simulator for selected device parameters.
    #[inline]
    pub fn init_vr_simulator(&self, info: VrDeviceInfo) {
        unsafe {
            raylib::InitVrSimulator(info);
        }
    }

    /// Closes VR simulator for current device.
    #[inline]
    pub fn close_vr_simulator(&self) {
        unsafe {
            raylib::CloseVrSimulator();
        }
    }

    /// Detects if VR simulator is ready.
    #[inline]
    pub fn is_vr_simulator_ready(&self) -> bool {
        unsafe {
            raylib::IsVrSimulatorReady()
        }
    }

    /// Sets VR distortion shader for stereoscopic rendering.
    #[inline]
    pub fn set_vr_distortion_shader(&self, shader: &Shader) {
        unsafe {
            raylib::SetVrDistortionShader(shader.0);
        }
    }

    /// Updates VR tracking (position and orientation) and camera.
    #[inline]
    pub fn update_vr_tracking(&self, camera: &mut Camera3D) {
        unsafe {
            raylib::UpdateVrTracking(camera);
        }
    }

    /// Enables or disables VR experience.
    #[inline]
    pub fn toggle_vr_mode(&self) {
        unsafe {
            raylib::ToggleVrMode();
        }
    }

    /// Begins VR simulator stereo rendering.
    #[inline]
    pub fn begin_vr_drawing(&self) {
        unsafe {
            raylib::BeginVrDrawing();
        }
    }

    /// Ends VR simulator stereo rendering.
    #[inline]
    pub fn end_vr_drawing(&self) {
        unsafe {
            raylib::EndVrDrawing();
        }
    }

    /// Initializes audio device and context.
    #[inline]
    pub fn init_audio_device(&self) {
        unsafe {
            raylib::InitAudioDevice();
        }
    }

    /// Closes the audio device and context (and music stream).
    #[inline]
    pub fn close_audio_device(&self) {
        unsafe {
            raylib::CloseAudioDevice();
        }
    }

    /// Checks if audio device is ready.
    #[inline]
    pub fn is_audio_device_ready(&self) -> bool {
        unsafe {
            raylib::IsAudioDeviceReady()
        }
    }

    /// Sets master volume (listener).
    #[inline]
    pub fn set_master_volume(&self, volume: f32) {
        unsafe {
            raylib::SetMasterVolume(volume);
        }
    }

    /// Loads wave data from file into RAM.
    #[inline]
    pub fn load_wave(&self, filename: &str) -> Wave {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Wave(raylib::LoadWave(c_filename.as_ptr()))
        }
    }

    /// Loads wave data from raw array data.
    #[inline]
    pub fn load_wave_ex(&self, data: &[u8], sample_count: i32, sample_rate: i32, sample_size: i32, channels: i32) -> Wave {
        unsafe {
            Wave(raylib::LoadWaveEx(data.as_ptr() as *mut std::os::raw::c_void, sample_count, sample_rate, sample_size, channels))
        }
    }

    /// Loads sound from file.
    #[inline]
    pub fn load_sound(&self, filename: &str) -> Sound {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Sound(raylib::LoadSound(c_filename.as_ptr()))
        }
    }

    /// Loads sound from wave data.
    #[inline]
    pub fn load_sound_from_wave(&self, wave: &Wave) -> Sound {
        unsafe {
            Sound(raylib::LoadSoundFromWave(wave.0))
        }
    }

    /// Updates sound buffer with new data.
    #[inline]
    pub fn update_sound(&self, sound: &mut Sound, data: &[impl AudioSample]) {
        unsafe {
            raylib::UpdateSound(sound.0, data.as_ptr() as *const std::os::raw::c_void, data.len() as i32);
        }
    }

    /// Plays a sound.
    #[inline]
    pub fn play_sound(&self, sound: &Sound) {
        unsafe {
            raylib::PlaySound(sound.0);
        }
    }

    /// Pauses a sound.
    #[inline]
    pub fn pause_sound(&self, sound: &Sound) {
        unsafe {
            raylib::PauseSound(sound.0);
        }
    }

    /// Resumes a paused sound.
    #[inline]
    pub fn resume_sound(&self, sound: &Sound) {
        unsafe {
            raylib::ResumeSound(sound.0);
        }
    }

    /// Stops playing a sound.
    #[inline]
    pub fn stop_sound(&self, sound: &Sound) {
        unsafe {
            raylib::StopSound(sound.0);
        }
    }

    /// Checks if a sound is currently playing.
    #[inline]
    pub fn is_sound_playing(&self, sound: &Sound) -> bool {
        unsafe {
            raylib::IsSoundPlaying(sound.0)
        }
    }

    /// Sets volume for a sound (`1.0` is max level).
    #[inline]
    pub fn set_sound_volume(&self, sound: &Sound, volume: f32) {
        unsafe {
            raylib::SetSoundVolume(sound.0, volume);
        }
    }

    /// Sets pitch for a sound (`1.0` is base level).
    #[inline]
    pub fn set_sound_pitch(&self, sound: &Sound, pitch: f32) {
        unsafe {
            raylib::SetSoundPitch(sound.0, pitch);
        }
    }

    /// Converts wave data to desired format.
    #[inline]
    pub fn wave_format(&self, wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
        unsafe {
            raylib::WaveFormat(&mut wave.0, sample_rate, sample_size, channels);
        }
    }

    /// Copies a wave to a new wave.
    #[inline]
    pub fn wave_copy(&self, wave: &Wave) -> Wave {
        unsafe {
            Wave(raylib::WaveCopy(wave.0))
        }
    }

    /// Crops a wave to defined sample range.
    #[inline]
    pub fn wave_crop(&self, wave: &mut Wave, init_sample: i32, final_sample: i32) {
        unsafe {
            raylib::WaveCrop(&mut wave.0, init_sample, final_sample);
        }
    }

    /// Gets sample data from wave as an `f32` array.
    #[inline]
    pub fn get_wave_data(&self, wave: &Wave) -> Vec<f32> {
        unsafe {
            let data = raylib::GetWaveData(wave.0);
            let data_size = (wave.sample_count * wave.channels) as usize;
            let mut samples = Vec::with_capacity(data_size);
            samples.set_len(data_size);
            std::ptr::copy(data, samples.as_mut_ptr(), data_size);
            libc::free(data as *mut libc::c_void);
            samples
        }
    }

    /// Loads music stream from file.
    #[inline]
    pub fn load_music_stream(&self, filename: &str) -> Music {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            Music(raylib::LoadMusicStream(c_filename.as_ptr()))
        }
    }

    /// Starts music playing.
    #[inline]
    pub fn play_music_stream(&self, music: &mut Music) {
        unsafe {
            raylib::PlayMusicStream(music.0);
        }
    }

    /// Updates buffers for music streaming.
    #[inline]
    pub fn update_music_stream(&self, music: &mut Music) {
        unsafe {
            raylib::UpdateMusicStream(music.0);
        }
    }

    /// Stops music playing.
    #[inline]
    pub fn stop_music_stream(&self, music: &mut Music) {
        unsafe {
            raylib::StopMusicStream(music.0);
        }
    }

    /// Pauses music playing.
    #[inline]
    pub fn pause_music_stream(&self, music: &mut Music) {
        unsafe {
            raylib::PauseMusicStream(music.0);
        }
    }

    /// Resumes playing paused music.
    #[inline]
    pub fn resume_music_stream(&self, music: &mut Music) {
        unsafe {
            raylib::ResumeMusicStream(music.0);
        }
    }

    /// Checks if music is playing.
    #[inline]
    pub fn is_music_playing(&self, music: &Music) -> bool {
        unsafe {
            raylib::IsMusicPlaying(music.0)
        }
    }

    /// Sets volume for music (`1.0` is max level).
    #[inline]
    pub fn set_music_volume(&self, music: &mut Music, volume: f32) {
        unsafe {
            raylib::SetMusicVolume(music.0, volume);
        }
    }

    /// Sets pitch for music (`1.0` is base level).
    #[inline]
    pub fn set_music_pitch(&self, music: &mut Music, pitch: f32) {
        unsafe {
            raylib::SetMusicPitch(music.0, pitch);
        }
    }

    /// Sets music loop count (loop repeats).
    #[inline]
    pub fn set_music_loop_count(&self, music: &mut Music, count: i32) {
        unsafe {
            raylib::SetMusicLoopCount(music.0, count);
        }
    }

    /// Gets music time length in seconds.
    #[inline]
    pub fn get_music_time_length(&self, music: &Music) -> f32 {
        unsafe {
            raylib::GetMusicTimeLength(music.0)
        }
    }

    /// Gets current music time played in seconds.
    #[inline]
    pub fn get_music_time_played(&self, music: &Music) -> f32 {
        unsafe {
            raylib::GetMusicTimePlayed(music.0)
        }
    }

    /// Initializes audio stream (to stream raw PCM data).
    #[inline]
    pub fn init_audio_stream(&self, sample_rate: u32, sample_size: u32, channels: u32) -> AudioStream {
        unsafe {
            AudioStream(raylib::InitAudioStream(sample_rate, sample_size, channels))
        }
    }

    /// Updates audio stream buffers with data.
    #[inline]
    pub fn update_audio_stream(&self, stream: &mut AudioStream, data: &[impl AudioSample]) {
        unsafe {
            raylib::UpdateAudioStream(stream.0, data.as_ptr() as *const std::os::raw::c_void, data.len() as i32);
        }
    }

    /// Checks if any audio stream buffers requires refill.
    #[inline]
    pub fn is_audio_buffer_processed(&self, stream: &AudioStream) -> bool {
        unsafe {
            raylib::IsAudioBufferProcessed(stream.0)
        }
    }

    /// Plays audio stream.
    #[inline]
    pub fn play_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            raylib::PlayAudioStream(stream.0);
        }
    }

    /// Pauses audio stream.
    #[inline]
    pub fn pause_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            raylib::PauseAudioStream(stream.0);
        }
    }

    /// Resumes audio stream.
    #[inline]
    pub fn resume_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            raylib::ResumeAudioStream(stream.0);
        }
    }

    /// Checks if audio stream is currently playing.
    #[inline]
    pub fn is_audio_stream_playing(&self, stream: &AudioStream) -> bool {
        unsafe {
            raylib::IsAudioStreamPlaying(stream.0)
        }
    }

    /// Stops audio stream.
    #[inline]
    pub fn stop_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            raylib::StopAudioStream(stream.0);
        }
    }

    /// Sets volume for audio stream (`1.0` is max level).
    #[inline]
    pub fn set_audio_stream_volume(&self, stream: &mut AudioStream, volume: f32) {
        unsafe {
            raylib::SetAudioStreamVolume(stream.0, volume);
        }
    }

    /// Sets pitch for audio stream (`1.0` is base level).
    #[inline]
    pub fn set_audio_stream_pitch(&self, stream: &mut AudioStream, pitch: f32) {
        unsafe {
            raylib::SetAudioStreamPitch(stream.0, pitch);
        }
    }
}
