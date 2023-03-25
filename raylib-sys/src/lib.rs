#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use mint;
pub type Vector2 = mint::Vector2<f32>;
pub type Vector3 = mint::Vector3<f32>;
pub type Vector4 = mint::Vector4<f32>;
pub type Matrix = mint::ColumnMatrix4<f32>;
pub type Quaternion = mint::Quaternion<f32>;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const MAX_MATERIAL_MAPS: u32 = 12;

mod color;
mod camera;
mod math;

pub use color::*;
pub use math::*;
pub use camera::*;