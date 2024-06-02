/* raylib-rs
   lib.rs - Main library code (the safe layer)

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

//! # raylib-test
//!
//! Test crate for raylib functions requires nightly
//! ```
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![allow(dead_code)]
#![doc(
    html_logo_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust_256x256.png",
    html_favicon_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust.ico"
)]
#![feature(test)]
extern crate test;

#[cfg(test)]
#[macro_use]
pub mod tests;

#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod audio;
#[cfg(not(target_os = "windows"))]
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod callbacks;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod data;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod drawing;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod image;
#[cfg(feature = "custom_frame_control")]
mod manual;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod misc;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod models;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod random;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod text;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod texture;
#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod window;

#[cfg(not(feature = "custom_frame_control"))]
#[cfg(not(feature = "automation_event_test"))]
mod logging;

mod automation;
