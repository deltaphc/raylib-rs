use crate::consts::LightType;
use crate::core::color::Color;
use crate::core::math::Vector3;
use crate::ffi;

pub use ffi::LIGHT_DISTANCE;
pub use ffi::LIGHT_HEIGHT;
pub use ffi::MAX_LIGHTS;

pub use crate::consts::LightType;

#[derive(Debug, Clone)]
struct Light {
    enabled: bool,
    light_type: LightType,
    position: Vector3,
    target: Vector3,
    color: Color,
    enabled_loc: i32,
    type_loc: i32,
    pos_loc: i32,
    target_loc: i32,
    color_loc: i32,
}

impl From<ffi::Light> for Light {
    fn from(light: ffi::Light) -> Self {
        unsafe { std::mem::transmute(light) }
    }
}

impl Into<ffi::Light> for Light {
    fn into(self) -> ffi::Light {
        unsafe { std::mem::transmute(self) }
    }
}

// Defines a light and get locations from PBR shader
pub fn create_light(
    light_type: LightType,
    pos: impl Into<ffi::Vector3>,
    targ: impl Into<ffi::Vector3>,
    color: impl Into<ffi::Color>,
    shader: impl AsRef<ffi::Shader>,
) {
    unsafe {
        ffi::CreateLight(
            light_type as i32,
            pos.into(),
            targ.into(),
            color.into(),
            *shader.as_ref(),
        )
    }
}

pub fn update_light_values(shader: impl AsRef<ffi::Shader>, light: impl Into<ffi::Light>) {
    unsafe {
        ffi::UpdateLightValues(*shader.as_ref(), light.into());
    }
}
