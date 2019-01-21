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
use raylib_sys::ffi;

macro_rules! impl_wrapper {
    ($name:ident, $t:ty, $dropfunc:expr, $rawfield:tt) => (
        impl Drop for $name {
            #[allow(unused_unsafe)]
            fn drop(&mut self) {
                unsafe { ($dropfunc)(self.$rawfield); }
            }
        }

        impl Deref for $name {
            type Target = $t;
            #[inline]
            fn deref(&self) -> &Self::Target { &self.$rawfield }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.$rawfield }
        }
    )
}

macro_rules! make_thin_wrapper {
    ($name:ident, $t:ty, $dropfunc:expr) => (
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct $name(pub(crate) $t);

        impl_wrapper!($name, $t, $dropfunc, 0);
    )
}

make_thin_wrapper!(Image, ffi::Image, ffi::UnloadImage);
make_thin_wrapper!(Texture2D, ffi::Texture2D, ffi::UnloadTexture);
make_thin_wrapper!(RenderTexture2D, ffi::RenderTexture2D, ffi::UnloadRenderTexture);
make_thin_wrapper!(Font, ffi::Font, ffi::UnloadFont);
make_thin_wrapper!(Mesh, ffi::Mesh, |mut mesh| ffi::UnloadMesh(&mut mesh));
make_thin_wrapper!(Shader, ffi::Shader, ffi::UnloadShader);
make_thin_wrapper!(Material, ffi::Material, ffi::UnloadMaterial);
make_thin_wrapper!(Model, ffi::Model, ffi::UnloadModel);
make_thin_wrapper!(Wave, ffi::Wave, ffi::UnloadWave);
make_thin_wrapper!(Sound, ffi::Sound, ffi::UnloadSound);
make_thin_wrapper!(Music, ffi::Music, ffi::UnloadMusicStream);
make_thin_wrapper!(AudioStream, ffi::AudioStream, ffi::CloseAudioStream);

pub trait FontExt {
    fn from_data(chars: &[ffi::CharInfo], base_size: i32, padding: i32, pack_method: i32) -> Font;
    fn set_chars(&mut self, chars: &[ffi::CharInfo]);
    fn set_texture(&mut self, tex: Texture2D);
}

impl FontExt for ffi::Font {
    /// Returns a new `Font` using provided `CharInfo` data and parameters.
    fn from_data(chars: &[ffi::CharInfo], base_size: i32, padding: i32, pack_method: i32) -> Font {
        unsafe {
            let mut f = std::mem::zeroed::<ffi::Font>();
            f.baseSize = base_size;
            f.set_chars(chars);

            let atlas = ffi::GenImageFontAtlas(f.chars, f.baseSize, f.charsCount, padding, pack_method);
            f.texture = ffi::LoadTextureFromImage(atlas);
            ffi::UnloadImage(atlas);
            Font(f)
        }
    }

    /// Sets the character data on the current Font.
    fn set_chars(&mut self, chars: &[ffi::CharInfo]) {
        unsafe {
            self.charsCount = chars.len() as i32;
            let data_size = self.charsCount as usize * std::mem::size_of::<ffi::CharInfo>();
            let ci_arr_ptr = libc::malloc(data_size); // raylib frees this data in UnloadFont
            std::ptr::copy(chars.as_ptr(), ci_arr_ptr as *mut ffi::CharInfo, chars.len());
            self.chars = ci_arr_ptr as *mut ffi::CharInfo;
        }
    }

    /// Sets the texture on the current Font, and takes ownership of `tex`.
    fn set_texture(&mut self, tex: Texture2D) {
        self.texture = tex.0;
        std::mem::forget(tex); // UnloadFont will also unload the texture
    }
}

pub trait MaterialMapExt {
    fn set_texture(&mut self, tex: Texture2D);
}

impl MaterialMapExt for ffi::MaterialMap {
    /// Sets the texture on the current MaterialMap, and takes ownership of `tex`.
    fn set_texture(&mut self, tex: Texture2D) {
        self.texture = tex.0;
        std::mem::forget(tex); // Since MaterialMaps are only used inside Materials, they will be dropped by Material
    }
}

pub trait MaterialExt {
    fn set_shader(&mut self, shader: Shader);
}

impl MaterialExt for ffi::Material {
    /// Sets the shader on the current Material, and takes ownership of `shader`.
    fn set_shader(&mut self, shader: Shader) {
        self.shader = shader.0;
        std::mem::forget(shader); // UnloadMaterial will also unload the shader
    }
}

pub trait ModelExt {
    fn set_material(&mut self, material: Material);
}

impl ModelExt for ffi::Model {
    /// Sets the material on the current Model and takes ownership of `material`.
    fn set_material(&mut self, material: Material) {
        self.material = material.0;
        std::mem::forget(material); // UnloadModel will also unload the material
    }
}

// Workarounds for lazy_static
unsafe impl Sync for Font {}
unsafe impl Sync for Material {}
