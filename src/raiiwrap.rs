/* raylib-rs
   raiiwrap.rs - RAII versions of raylib structs

Copyright (c) 2018 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

extern crate libc;
use std::ops::{Deref, DerefMut};
use raylib;

macro_rules! make_raii_wrapper {
    ($name:ident, $t:ty, $dropfunc:expr) => (
        #[repr(transparent)]
        #[derive(Debug, PartialEq)]
        pub struct $name(pub(crate) $t);

        impl Drop for $name {
            #[allow(unused_unsafe)]
            fn drop(&mut self) {
                unsafe { ($dropfunc)(self.0); }
            }
        }

        impl Deref for $name {
            type Target = $t;
            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    )
}

make_raii_wrapper!(Image, raylib::Image, raylib::UnloadImage);
make_raii_wrapper!(Texture2D, raylib::Texture2D, raylib::UnloadTexture);
make_raii_wrapper!(RenderTexture2D, raylib::RenderTexture2D, raylib::UnloadRenderTexture);
make_raii_wrapper!(Font, raylib::Font, raylib::UnloadFont);
make_raii_wrapper!(Mesh, raylib::Mesh, |mut mesh| raylib::UnloadMesh(&mut mesh));
make_raii_wrapper!(Shader, raylib::Shader, raylib::UnloadShader);
make_raii_wrapper!(Material, raylib::Material, raylib::UnloadMaterial);
make_raii_wrapper!(Model, raylib::Model, raylib::UnloadModel);
make_raii_wrapper!(Wave, raylib::Wave, raylib::UnloadWave);
make_raii_wrapper!(Sound, raylib::Sound, raylib::UnloadSound);
make_raii_wrapper!(Music, raylib::Music, raylib::UnloadMusicStream);
make_raii_wrapper!(AudioStream, raylib::AudioStream, raylib::CloseAudioStream);

impl raylib::Font {
    /// Returns a new `Font` using provided `CharInfo` data and parameters.
    pub fn from_data(chars: &Vec<raylib::CharInfo>, base_size: i32, padding: i32, pack_method: i32) -> Font {
        unsafe {
            let mut f = ::std::mem::zeroed::<raylib::Font>();
            f.base_size = base_size;
            f.chars_count = chars.len() as i32;
            
            let data_size = f.chars_count as usize * ::std::mem::size_of::<raylib::CharInfo>();
            let ci_arr_ptr = libc::malloc(data_size); // raylib frees this data in UnloadFont
            ::std::ptr::copy(chars.as_ptr(), ci_arr_ptr as *mut raylib::CharInfo, chars.len());
            f.chars = ci_arr_ptr as *mut raylib::CharInfo;

            let atlas = raylib::GenImageFontAtlas(f.chars, f.base_size, f.chars_count, padding, pack_method);
            f.texture = raylib::LoadTextureFromImage(atlas);
            raylib::UnloadImage(atlas);
            Font(f)
        }
    }
}

impl raylib::MaterialMap {
    /// Sets the texture on the current MaterialMap, and takes ownership of `tex`.
    pub fn set_texture(&mut self, tex: Texture2D) {
        self.texture = tex.0;
        ::std::mem::forget(tex); // Since MaterialMaps are only used inside Materials, they will be dropped by Material
    }
}

impl raylib::Material {
    /// Sets the shader on the current Material, and takes ownership of `shader`.
    pub fn set_shader(&mut self, shader: Shader) {
        self.shader = shader.0;
        ::std::mem::forget(shader); // UnloadMaterial will also unload the shader
    }
}

impl raylib::Model {
    /// Sets the material on the current Model and takes ownership of `material`.
    pub fn set_material(&mut self, material: Material) {
        self.material = material.0;
        ::std::mem::forget(material); // UnloadModel will also unload the material
    }
}

// Workarounds for lazy_static
unsafe impl Sync for Font {}
unsafe impl Sync for Material {}
