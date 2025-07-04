//! Various constant enums to use with raylib
pub use crate::ffi;

pub use ffi::BlendMode;
pub use ffi::CameraMode;
pub use ffi::CameraProjection;
pub use ffi::ConfigFlags;
pub use ffi::CubemapLayout;
pub use ffi::GamepadAxis;
pub use ffi::GamepadButton;
pub use ffi::Gesture;
pub use ffi::KeyboardKey;
pub use ffi::MaterialMapIndex;
pub use ffi::MouseButton;
pub use ffi::NPatchLayout;
pub use ffi::PixelFormat;
pub use ffi::ShaderLocationIndex;
pub use ffi::ShaderUniformDataType;
pub use ffi::TextureFilter;
pub use ffi::TextureWrap;
pub use ffi::TraceLogLevel;
pub use ffi::DEG2RAD;
// TODO Fix when rlgl bindings are in
pub const MAX_MATERIAL_MAPS: u32 = 12;
pub const MAX_SHADER_LOCATIONS: u32 = 32;
pub use ffi::GuiCheckBoxProperty;
pub use ffi::GuiColorPickerProperty;
pub use ffi::GuiComboBoxProperty;
pub use ffi::GuiControl;
pub use ffi::GuiControlProperty;
pub use ffi::GuiDefaultProperty;
pub use ffi::GuiDropdownBoxProperty;
pub use ffi::GuiIconName;
pub use ffi::GuiListViewProperty;
pub use ffi::GuiProgressBarProperty;
pub use ffi::GuiScrollBarProperty;
pub use ffi::GuiSliderProperty;
pub use ffi::GuiState;
pub use ffi::GuiTextAlignment;
pub use ffi::GuiTextAlignmentVertical;
pub use ffi::GuiTextBoxProperty;
pub use ffi::GuiTextWrapMode;
pub use ffi::GuiToggleProperty;
pub use ffi::GuiValueBoxProperty;
pub use ffi::MouseCursor;
pub use ffi::PI;
pub use ffi::RAD2DEG;
