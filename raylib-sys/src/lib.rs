#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/logging_wrapper_bindings.rs"));

#[cfg(target_os = "macos")]
pub const MAX_MATERIAL_MAPS: u32 = 12;
