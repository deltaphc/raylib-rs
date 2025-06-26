#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::approx_constant)]

#[cfg(not(feature = "nobindgen"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "nobindgen")]
include!(env!("RAYLIB_BINDGEN_LOCATION"));

#[cfg(target_os = "macos")]
pub const MAX_MATERIAL_MAPS: u32 = 12;

mod color;
mod math;
#[allow(unused_imports)]
pub use color::*;
#[allow(unused_imports)]
pub use math::*;

impl Default for TraceLogLevel {
    fn default() -> Self {
        TraceLogLevel::LOG_INFO
    }
}
