/* raylib-rs
   raiiwrap.rs - RAII versions of raylib structs

Copyright (c) 2018 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

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

// Workarounds for lazy_static
unsafe impl Sync for Font {}
unsafe impl Sync for Material {}
