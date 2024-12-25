/* raylib-rs
   ease.rs - Easings/interpolation helpers

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

//! The raylib-rs prelude.
//!
//! This prelude module is for bringing many commonly-used types, functions, and constants into scope all at once.
//!
//! # Example
//!
//! ```
//! use raylib::prelude::*;
//! ```

pub use crate::callbacks::*;
pub use crate::consts::*;
pub use crate::core::audio::*;
pub use crate::core::automation::*;
pub use crate::core::camera::*;
pub use crate::core::collision::*;
pub use crate::core::color::*;
pub use crate::core::data::*;
pub use crate::core::drawing::*;
pub use crate::core::file::*;
pub use crate::core::input::*;
pub use crate::core::logging::*;
pub use crate::core::math::*;
pub use crate::core::misc::*;
pub use crate::core::models::*;
pub use crate::core::shaders::*;
pub use crate::core::text::*;
pub use crate::core::texture::*;
pub use crate::core::vr::*;
pub use crate::core::window::*;
pub use crate::core::*;
#[cfg(feature = "imgui")]
pub use crate::imgui::*;
pub use crate::rgui::*;
pub use crate::*;
