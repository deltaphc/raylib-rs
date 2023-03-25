#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use mint;

mod color;
mod camera;
mod math;

pub use color::*;
pub use math::*;
pub use camera::*;