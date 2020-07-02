//! Various constant enums to use with raylib
pub use crate::ffi;

pub use ffi::BlendMode;
pub use ffi::CameraMode;
pub use ffi::CameraType;
pub use ffi::ConfigFlag;
pub use ffi::GamepadButton;
pub use ffi::GamepadNumber;
pub use ffi::GestureType;
pub use ffi::KeyboardKey;
pub use ffi::MaterialMapType;
pub use ffi::MouseButton;
pub use ffi::NPatchType;
pub use ffi::PixelFormat;
pub use ffi::ShaderLocationIndex;
pub use ffi::ShaderUniformDataType;
pub use ffi::TextureFilterMode;
pub use ffi::TextureWrapMode;
pub use ffi::TraceLogType;
pub use ffi::DEG2RAD;
// TODO Fix when rlgl bindings are in
pub const MAX_MATERIAL_MAPS: u32 = 12;
pub const MAX_SHADER_LOCATIONS: u32 = 32;
pub use ffi::MAX_TOUCH_POINTS;
pub use ffi::PI;
pub use ffi::RAD2DEG;
