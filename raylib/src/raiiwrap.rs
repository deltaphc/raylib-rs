/* raylib-rs
   raiiwrap.rs - RAII versions of raylib structs

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

use std::ops::{Deref, DerefMut};

macro_rules! impl_wrapper {
    ($name:ident, $t:ty, $dropfunc:expr, $rawfield:tt) => {
        impl Drop for $name {
            #[allow(unused_unsafe)]
            fn drop(&mut self) {
                unsafe {
                    ($dropfunc)(self.$rawfield);
                }
            }
        }

        impl Deref for $name {
            type Target = $t;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$rawfield
            }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$rawfield
            }
        }
    };
}

macro_rules! make_thin_wrapper {
    ($name:ident, $t:ty, $dropfunc:expr) => {
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct $name(pub(crate) $t);

        impl_wrapper!($name, $t, $dropfunc, 0);
    };
}

make_thin_wrapper!(Image, rl::Image, rl::UnloadImage);
make_thin_wrapper!(Texture2D, rl::Texture2D, rl::UnloadTexture);
make_thin_wrapper!(
    RenderTexture2D,
    rl::RenderTexture2D,
    rl::UnloadRenderTexture
);
make_thin_wrapper!(Font, rl::Font, rl::UnloadFont);
make_thin_wrapper!(Mesh, rl::Mesh, |mut mesh| rl::UnloadMesh(&mut mesh));
make_thin_wrapper!(Shader, rl::Shader, rl::UnloadShader);
make_thin_wrapper!(Material, rl::Material, rl::UnloadMaterial);
make_thin_wrapper!(Model, rl::Model, rl::UnloadModel);
make_thin_wrapper!(Wave, rl::Wave, rl::UnloadWave);
make_thin_wrapper!(Sound, rl::Sound, rl::UnloadSound);
make_thin_wrapper!(Music, rl::Music, rl::UnloadMusicStream);
make_thin_wrapper!(AudioStream, rl::AudioStream, rl::CloseAudioStream);

/// An extension trait allowing for safe manipulation of `Font` structs.
pub trait FontExt {
    fn from_data(chars: &[rl::CharInfo], base_size: i32, padding: i32, pack_method: i32) -> Font;
    fn set_chars(&mut self, chars: &[rl::CharInfo]);
    fn set_texture(&mut self, tex: Texture2D);
}

impl FontExt for rl::Font {
    /// Returns a new `Font` using provided `CharInfo` data and parameters.
    fn from_data(chars: &[rl::CharInfo], base_size: i32, padding: i32, pack_method: i32) -> Font {
        unsafe {
            let mut f = std::mem::zeroed::<rl::Font>();
            f.baseSize = base_size;
            f.set_chars(chars);

            let atlas =
                rl::GenImageFontAtlas(f.chars, f.baseSize, f.charsCount, padding, pack_method);
            f.texture = rl::LoadTextureFromImage(atlas);
            rl::UnloadImage(atlas);
            Font(f)
        }
    }

    /// Sets the character data on the current Font.
    fn set_chars(&mut self, chars: &[rl::CharInfo]) {
        unsafe {
            self.charsCount = chars.len() as i32;
            let data_size = self.charsCount as usize * std::mem::size_of::<rl::CharInfo>();
            let ci_arr_ptr = libc::malloc(data_size); // raylib frees this data in UnloadFont
            std::ptr::copy(chars.as_ptr(), ci_arr_ptr as *mut rl::CharInfo, chars.len());
            self.chars = ci_arr_ptr as *mut rl::CharInfo;
        }
    }

    /// Sets the texture on the current Font, and takes ownership of `tex`.
    fn set_texture(&mut self, tex: Texture2D) {
        self.texture = tex.0;
        std::mem::forget(tex); // UnloadFont will also unload the texture
    }
}

/// An extension trait allowing for safe manipulation of `MaterialMap` structs.
pub trait MaterialMapExt {
    fn set_texture(&mut self, tex: Texture2D);
}

impl MaterialMapExt for rl::MaterialMap {
    /// Sets the texture on the current MaterialMap, and takes ownership of `tex`.
    fn set_texture(&mut self, tex: Texture2D) {
        self.texture = tex.0;
        std::mem::forget(tex); // Since MaterialMaps are only used inside Materials, they will be dropped by Material
    }
}

/// An extension trait allowing for safe manipulation of `Material` structs.
pub trait MaterialExt {
    fn set_shader(&mut self, shader: Shader);
}

impl MaterialExt for rl::Material {
    /// Sets the shader on the current Material, and takes ownership of `shader`.
    fn set_shader(&mut self, shader: Shader) {
        self.shader = shader.0;
        std::mem::forget(shader); // UnloadMaterial will also unload the shader
    }
}

/// An extension trait allowing for safe manipulation of `Model` structs.
pub trait ModelExt {
    fn set_material(&mut self, material: Material);
}

impl ModelExt for rl::Model {
    /// Sets the material on the current Model and takes ownership of `material`.
    fn set_material(&mut self, material: Material) {
        self.material = material.0;
        std::mem::forget(material); // UnloadModel will also unload the material
    }
}

// Workarounds for lazy_static
unsafe impl Sync for Font {}
unsafe impl Sync for Material {}
