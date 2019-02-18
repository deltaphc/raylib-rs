/* raylib-rs
   safe_types.rs - Safe versions of raylib structs

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

use std::ops::{Deref, DerefMut};
use raylib_sys::ffi;
use crate::raymath::*;

#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub proj: CameraType,
}

impl From<Camera3D> for ffi::Camera3D {
    #[inline]
    fn from(c: Camera3D) -> ffi::Camera3D {
        ffi::Camera3D {
            position: c.position.into(),
            target: c.target.into(),
            up: c.up.into(),
            fovy: c.fovy,
            type_: c.proj as i32,
        }
    }
}

impl From<ffi::Camera3D> for Camera3D {
    #[inline]
    fn from(c: ffi::Camera3D) -> Camera3D {
        Camera3D {
            position: c.position.into(),
            target: c.target.into(),
            up: c.up.into(),
            fovy: c.fovy,
            proj: c.type_.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

impl From<Camera2D> for ffi::Camera2D {
    #[inline]
    fn from(c: Camera2D) -> ffi::Camera2D {
        ffi::Camera2D {
            offset: c.offset.into(),
            target: c.target.into(),
            rotation: c.rotation,
            zoom: c.zoom,
        }
    }
}

impl From<ffi::Camera2D> for Camera2D {
    #[inline]
    fn from(c: ffi::Camera2D) -> Camera2D {
        Camera2D {
            offset: c.offset.into(),
            target: c.target.into(),
            rotation: c.rotation,
            zoom: c.zoom,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl From<BoundingBox> for ffi::BoundingBox {
    #[inline]
    fn from(b: BoundingBox) -> ffi::BoundingBox {
        ffi::BoundingBox {
            min: b.min.into(),
            max: b.max.into(),
        }
    }
}

impl From<ffi::BoundingBox> for BoundingBox {
    #[inline]
    fn from(b: ffi::BoundingBox) -> BoundingBox {
        BoundingBox {
            min: b.min.into(),
            max: b.max.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}

impl From<Ray> for ffi::Ray {
    #[inline]
    fn from(r: Ray) -> ffi::Ray {
        ffi::Ray {
            position: r.position.into(),
            direction: r.direction.into(),
        }
    }
}

impl From<ffi::Ray> for Ray {
    #[inline]
    fn from(r: ffi::Ray) -> Ray {
        Ray {
            position: r.position.into(),
            direction: r.direction.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RayHitInfo {
    pub hit: bool,
    pub distance: f32,
    pub position: Vector3,
    pub normal: Vector3,
}

impl From<ffi::RayHitInfo> for RayHitInfo {
    #[inline]
    fn from(r: ffi::RayHitInfo) -> RayHitInfo {
        RayHitInfo {
            hit: r.hit,
            distance: r.distance,
            position: r.position.into(),
            normal: r.normal.into(),
        }
    }
}

macro_rules! bitflag_type {
    ($vis:vis struct $name:ident($inner_vis:vis $t:ty);) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        $vis struct $name($inner_vis $t);

        impl std::ops::BitOr<$name> for $name {
            type Output = Self;
            #[inline]
            fn bitor(self, other: Self) -> Self {
                $name(self.0 | other.0)
            }
        }
        impl std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                self.0 |= rhs.0;
            }
        }
        impl std::ops::BitAnd<$name> for $name {
            type Output = Self;
            #[inline]
            fn bitand(self, other: Self) -> Self {
                $name(self.0 & other.0)
            }
        }
        impl std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                self.0 &= rhs.0;
            }
        }
    };
}

macro_rules! enum_from_i32 {
    ($vis:vis enum $name:ident { $($variant:ident = $value:path, )* }) => {
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        $vis enum $name {
            $($variant = $value,)*
        }

        impl From<i32> for $name {
            #[inline]
            fn from(format: i32) -> $name {
                match format as u32 {
                    $($value => $name::$variant,)*
                    _ => panic!("Invalid integer {} passed to {}::from(i32)", format, stringify!($name)),
                }
            }
        }
    }
}

macro_rules! impl_bidirectional_from {
    ($t1:path, $t2:path, $($field:ident),*) => {
        impl From<$t1> for $t2 {
            #[inline]
            fn from(v: $t1) -> $t2 {
                $t2 {
                    $($field: v.$field,)*
                }
            }
        }

        impl From<$t2> for $t1 {
            #[inline]
            fn from(v: $t2) -> $t1 {
                $t1 {
                    $($field: v.$field,)*
                }
            }
        }
    };
}

bitflag_type! { pub struct Log(pub u32); }
impl Log {
    pub const INFO: Log = Log(ffi::LOG_INFO);
    pub const WARNING: Log = Log(ffi::LOG_WARNING);
    pub const ERROR: Log = Log(ffi::LOG_ERROR);
    pub const DEBUG: Log = Log(ffi::LOG_DEBUG);
    pub const OTHER: Log = Log(ffi::LOG_OTHER);
}

bitflag_type! { pub struct Gesture(pub u32); }
impl Gesture {
    pub const NONE: Gesture = Gesture(ffi::GESTURE_NONE);
    pub const TAP: Gesture = Gesture(ffi::GESTURE_TAP);
    pub const DOUBLETAP: Gesture = Gesture(ffi::GESTURE_DOUBLETAP);
    pub const HOLD: Gesture = Gesture(ffi::GESTURE_HOLD);
    pub const DRAG: Gesture = Gesture(ffi::GESTURE_DRAG);
    pub const SWIPE_RIGHT: Gesture = Gesture(ffi::GESTURE_SWIPE_RIGHT);
    pub const SWIPE_LEFT: Gesture = Gesture(ffi::GESTURE_SWIPE_LEFT);
    pub const SWIPE_UP: Gesture = Gesture(ffi::GESTURE_SWIPE_UP);
    pub const SWIPE_DOWN: Gesture = Gesture(ffi::GESTURE_SWIPE_DOWN);
    pub const PINCH_IN: Gesture = Gesture(ffi::GESTURE_PINCH_IN);
    pub const PINCH_OUT: Gesture = Gesture(ffi::GESTURE_PINCH_OUT);
}

enum_from_i32! {
    pub enum ShaderLoc {
        VertexPosition = ffi::LOC_VERTEX_POSITION,
        VertexTexCoord01 = ffi::LOC_VERTEX_TEXCOORD01,
        VertexTexCoord02 = ffi::LOC_VERTEX_TEXCOORD02,
        VertexNormal = ffi::LOC_VERTEX_NORMAL,
        VertexTangent = ffi::LOC_VERTEX_TANGENT,
        VertexColor = ffi::LOC_VERTEX_COLOR,
        MatrixMVP = ffi::LOC_MATRIX_MVP,
        MatrixModel = ffi::LOC_MATRIX_MODEL,
        MatrixView = ffi::LOC_MATRIX_VIEW,
        MatrixProjection = ffi::LOC_MATRIX_PROJECTION,
        VectorView = ffi::LOC_VECTOR_VIEW,
        ColorDiffuse = ffi::LOC_COLOR_DIFFUSE,
        ColorSpecular = ffi::LOC_COLOR_SPECULAR,
        ColorAmbient = ffi::LOC_COLOR_AMBIENT,
        MapAlbedo = ffi::LOC_MAP_ALBEDO,
        MapMetalness = ffi::LOC_MAP_METALNESS,
        MapNormal = ffi::LOC_MAP_NORMAL,
        MapRoughness = ffi::LOC_MAP_ROUGHNESS,
        MapOcclusion = ffi::LOC_MAP_OCCLUSION,
        MapEmission = ffi::LOC_MAP_EMISSION,
        MapHeight = ffi::LOC_MAP_HEIGHT,
        MapCubeMap = ffi::LOC_MAP_CUBEMAP,
        MapIrradiance = ffi::LOC_MAP_IRRADIANCE,
        MapPrefilter = ffi::LOC_MAP_PREFILTER,
        MapBRDF = ffi::LOC_MAP_BRDF,
    }
}

enum_from_i32! {
    pub enum Texmap {
        Albedo = ffi::MAP_ALBEDO,
        Metalness = ffi::MAP_METALNESS,
        Normal = ffi::MAP_NORMAL,
        Roughness = ffi::MAP_ROUGHNESS,
        Occlusion = ffi::MAP_OCCLUSION,
        Emission = ffi::MAP_EMISSION,
        Height = ffi::MAP_HEIGHT,
        CubeMap = ffi::MAP_CUBEMAP,
        Irradiance = ffi::MAP_IRRADIANCE,
        Prefilter = ffi::MAP_PREFILTER,
        BRDF = ffi::MAP_BRDF,
    }
}

enum_from_i32! {
    pub enum PixelFormat {
        UncompressedGrayscale = ffi::UNCOMPRESSED_GRAYSCALE,
        UncompressedGrayAlpha = ffi::UNCOMPRESSED_GRAY_ALPHA,
        UncompressedR5G6B5 = ffi::UNCOMPRESSED_R5G6B5,
        UncompressedR8G8B8 = ffi::UNCOMPRESSED_R8G8B8,
        UncompressedR5G5B5A1 = ffi::UNCOMPRESSED_R5G5B5A1,
        UncompressedR4G4B4A4 = ffi::UNCOMPRESSED_R4G4B4A4,
        UncompressedR8G8B8A8 = ffi::UNCOMPRESSED_R8G8B8A8,
        UncompressedR32 = ffi::UNCOMPRESSED_R32,
        UncompressedR32G32B32 = ffi::UNCOMPRESSED_R32G32B32,
        UncompressedR32G32B32A32 = ffi::UNCOMPRESSED_R32G32B32A32,
        CompressedDXT1RGB = ffi::COMPRESSED_DXT1_RGB,
        CompressedDXT1RGBA = ffi::COMPRESSED_DXT1_RGBA,
        CompressedDXT3RGBA = ffi::COMPRESSED_DXT3_RGBA,
        CompressedDXT5RGBA = ffi::COMPRESSED_DXT5_RGBA,
        CompressedETC1RGB = ffi::COMPRESSED_ETC1_RGB,
        CompressedETC2RGB = ffi::COMPRESSED_ETC2_RGB,
        CompressedETC2EACRGBA = ffi::COMPRESSED_ETC2_EAC_RGBA,
        CompressedPVRTRGB = ffi::COMPRESSED_PVRT_RGB,
        CompressedPVRTRGBA = ffi::COMPRESSED_PVRT_RGBA,
        CompressedASTC4x4RGBA = ffi::COMPRESSED_ASTC_4x4_RGBA,
        CompressedASTC8x8RGBA = ffi::COMPRESSED_ASTC_8x8_RGBA,
    }
}

enum_from_i32! {
    pub enum TextureFilter {
        Point = ffi::FILTER_POINT,
        Bilinear = ffi::FILTER_BILINEAR,
        Trilinear = ffi::FILTER_TRILINEAR,
        Anisotropic4x = ffi::FILTER_ANISOTROPIC_4X,
        Anisotropic8x = ffi::FILTER_ANISOTROPIC_8X,
        Anisotropic16x = ffi::FILTER_ANISOTROPIC_16X,
    }
}

enum_from_i32! {
    pub enum TextureWrap {
        Repeat = ffi::WRAP_REPEAT,
        Clamp = ffi::WRAP_CLAMP,
        Mirror = ffi::WRAP_MIRROR,
    }
}

enum_from_i32! {
    pub enum BlendMode {
        Alpha = ffi::BLEND_ALPHA,
        Additive = ffi::BLEND_ADDITIVE,
        Multiplied = ffi::BLEND_MULTIPLIED,
    }
}

enum_from_i32! {
    pub enum CameraMode {
        Custom = ffi::CAMERA_CUSTOM,
        Free = ffi::CAMERA_FREE,
        Orbital = ffi::CAMERA_ORBITAL,
        FirstPerson = ffi::CAMERA_FIRST_PERSON,
        ThirdPerson = ffi::CAMERA_THIRD_PERSON,
    }
}

enum_from_i32! {
    pub enum CameraType {
        Perspective = ffi::CAMERA_PERSPECTIVE,
        Orthographic = ffi::CAMERA_ORTHOGRAPHIC,
    }
}

enum_from_i32! {
    pub enum VrDevice {
        Default = ffi::HMD_DEFAULT_DEVICE,
        OculusRiftDK2 = ffi::HMD_OCULUS_RIFT_DK2,
        OculusRiftCV1 = ffi::HMD_OCULUS_RIFT_CV1,
        OculusGo = ffi::HMD_OCULUS_GO,
        ValveHTCVive = ffi::HMD_VALVE_HTC_VIVE,
        SonyPSVR = ffi::HMD_SONY_PSVR,
    }
}

impl_bidirectional_from!(Vector2, ffi::Vector2, x, y);
impl_bidirectional_from!(Vector3, ffi::Vector3, x, y, z);
impl_bidirectional_from!(Vector4, ffi::Vector4, x, y, z, w);
impl_bidirectional_from!(Matrix, ffi::Matrix,
    m0, m4, m8, m12,
    m1, m5, m9, m13,
    m2, m6, m10, m14,
    m3, m7, m11, m15);

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const LIGHTGRAY  : Color = Color { r: 200, g: 200, b: 200, a: 255 };
    pub const GRAY       : Color = Color { r: 130, g: 130, b: 130, a: 255 };
    pub const DARKGRAY   : Color = Color { r: 80,  g: 80,  b: 80,  a: 255 };
    pub const YELLOW     : Color = Color { r: 253, g: 249, b: 0,   a: 255 };
    pub const GOLD       : Color = Color { r: 255, g: 203, b: 0,   a: 255 };
    pub const ORANGE     : Color = Color { r: 255, g: 161, b: 0,   a: 255 };
    pub const PINK       : Color = Color { r: 255, g: 109, b: 194, a: 255 };
    pub const RED        : Color = Color { r: 230, g: 41,  b: 55,  a: 255 };
    pub const MAROON     : Color = Color { r: 190, g: 33,  b: 55,  a: 255 };
    pub const GREEN      : Color = Color { r: 0,   g: 228, b: 48,  a: 255 };
    pub const LIME       : Color = Color { r: 0,   g: 158, b: 47,  a: 255 };
    pub const DARKGREEN  : Color = Color { r: 0,   g: 117, b: 44,  a: 255 };
    pub const SKYBLUE    : Color = Color { r: 102, g: 191, b: 255, a: 255 };
    pub const BLUE       : Color = Color { r: 0,   g: 121, b: 241, a: 255 };
    pub const DARKBLUE   : Color = Color { r: 0,   g: 82,  b: 172, a: 255 };
    pub const PURPLE     : Color = Color { r: 200, g: 122, b: 255, a: 255 };
    pub const VIOLET     : Color = Color { r: 135, g: 60,  b: 190, a: 255 };
    pub const DARKPURPLE : Color = Color { r: 112, g: 31,  b: 126, a: 255 };
    pub const BEIGE      : Color = Color { r: 211, g: 176, b: 131, a: 255 };
    pub const BROWN      : Color = Color { r: 127, g: 106, b: 79,  a: 255 };
    pub const DARKBROWN  : Color = Color { r: 76,  g: 63,  b: 47,  a: 255 };
    pub const WHITE      : Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK      : Color = Color { r: 0,   g: 0,   b: 0,   a: 255 };
    pub const BLANK      : Color = Color { r: 0,   g: 0,   b: 0,   a: 0   };
    pub const MAGENTA    : Color = Color { r: 255, g: 0,   b: 255, a: 255 };
    pub const RAYWHITE   : Color = Color { r: 245, g: 245, b: 245, a: 255 };

    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

impl_bidirectional_from!(Color, ffi::Color, r, g, b, a);

impl From<(u8, u8, u8)> for Color {
    #[inline]
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color { r, g, b, a: 255 }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    #[inline]
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color { r, g, b, a }
    }
}

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

/// An extension trait allowing for safe manipulation of `Font` structs.
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

/// An extension trait allowing for safe manipulation of `MaterialMap` structs.
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

/// An extension trait allowing for safe manipulation of `Material` structs.
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

/// An extension trait allowing for safe manipulation of `Model` structs.
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
