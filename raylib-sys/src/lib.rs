#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;

pub use bindings::*;

pub const MAX_SHADER_LOCATIONS: u32 = bindings::RL_MAX_SHADER_LOCATIONS;
