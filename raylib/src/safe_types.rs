/* raylib-rs
   safe_types.rs - Safe versions of raylib structs

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

use crate::raymath::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub proj: CameraType,
}

impl From<Camera3D> for rl::Camera3D {
    #[inline]
    fn from(c: Camera3D) -> rl::Camera3D {
        rl::Camera3D {
            position: c.position.into(),
            target: c.target.into(),
            up: c.up.into(),
            fovy: c.fovy,
            type_: c.proj as i32,
        }
    }
}

impl From<rl::Camera3D> for Camera3D {
    #[inline]
    fn from(c: rl::Camera3D) -> Camera3D {
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

impl From<Camera2D> for rl::Camera2D {
    #[inline]
    fn from(c: Camera2D) -> rl::Camera2D {
        rl::Camera2D {
            offset: c.offset.into(),
            target: c.target.into(),
            rotation: c.rotation,
            zoom: c.zoom,
        }
    }
}

impl From<rl::Camera2D> for Camera2D {
    #[inline]
    fn from(c: rl::Camera2D) -> Camera2D {
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

impl From<BoundingBox> for rl::BoundingBox {
    #[inline]
    fn from(b: BoundingBox) -> rl::BoundingBox {
        rl::BoundingBox {
            min: b.min.into(),
            max: b.max.into(),
        }
    }
}

impl From<rl::BoundingBox> for BoundingBox {
    #[inline]
    fn from(b: rl::BoundingBox) -> BoundingBox {
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

impl From<Ray> for rl::Ray {
    #[inline]
    fn from(r: Ray) -> rl::Ray {
        rl::Ray {
            position: r.position.into(),
            direction: r.direction.into(),
        }
    }
}

impl From<rl::Ray> for Ray {
    #[inline]
    fn from(r: rl::Ray) -> Ray {
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

impl From<rl::RayHitInfo> for RayHitInfo {
    #[inline]
    fn from(r: rl::RayHitInfo) -> RayHitInfo {
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
    pub const INFO: Log = Log(rl::TraceLogType::LOG_INFO);
    pub const WARNING: Log = Log(rl::TraceLogType::LOG_WARNING);
    pub const ERROR: Log = Log(rl::TraceLogType::LOG_ERROR);
    pub const DEBUG: Log = Log(rl::TraceLogType::LOG_DEBUG);
}

bitflag_type! { pub struct Gesture(pub u32); }
impl Gesture {
    pub const NONE: Gesture = Gesture(rl::GestureType::GESTURE_NONE);
    pub const TAP: Gesture = Gesture(rl::GestureType::GESTURE_TAP);
    pub const DOUBLETAP: Gesture = Gesture(rl::GestureType::GESTURE_DOUBLETAP);
    pub const HOLD: Gesture = Gesture(rl::GestureType::GESTURE_HOLD);
    pub const DRAG: Gesture = Gesture(rl::GestureType::GESTURE_DRAG);
    pub const SWIPE_RIGHT: Gesture = Gesture(rl::GestureType::GESTURE_SWIPE_RIGHT);
    pub const SWIPE_LEFT: Gesture = Gesture(rl::GestureType::GESTURE_SWIPE_LEFT);
    pub const SWIPE_UP: Gesture = Gesture(rl::GestureType::GESTURE_SWIPE_UP);
    pub const SWIPE_DOWN: Gesture = Gesture(rl::GestureType::GESTURE_SWIPE_DOWN);
    pub const PINCH_IN: Gesture = Gesture(rl::GestureType::GESTURE_PINCH_IN);
    pub const PINCH_OUT: Gesture = Gesture(rl::GestureType::GESTURE_PINCH_OUT);
}

enum_from_i32! {
    pub enum ShaderLoc {
        VertexPosition = rl::ShaderLocationIndex::LOC_VERTEX_POSITION,
        VertexTexCoord01 = rl::ShaderLocationIndex::LOC_VERTEX_TEXCOORD01,
        VertexTexCoord02 = rl::ShaderLocationIndex::LOC_VERTEX_TEXCOORD02,
        VertexNormal = rl::ShaderLocationIndex::LOC_VERTEX_NORMAL,
        VertexTangent = rl::ShaderLocationIndex::LOC_VERTEX_TANGENT,
        VertexColor = rl::ShaderLocationIndex::LOC_VERTEX_COLOR,
        MatrixMVP = rl::ShaderLocationIndex::LOC_MATRIX_MVP,
        MatrixModel = rl::ShaderLocationIndex::LOC_MATRIX_MODEL,
        MatrixView = rl::ShaderLocationIndex::LOC_MATRIX_VIEW,
        MatrixProjection = rl::ShaderLocationIndex::LOC_MATRIX_PROJECTION,
        VectorView = rl::ShaderLocationIndex::LOC_VECTOR_VIEW,
        ColorDiffuse = rl::ShaderLocationIndex::LOC_COLOR_DIFFUSE,
        ColorSpecular = rl::ShaderLocationIndex::LOC_COLOR_SPECULAR,
        ColorAmbient = rl::ShaderLocationIndex::LOC_COLOR_AMBIENT,
        MapAlbedo = rl::ShaderLocationIndex::LOC_MAP_ALBEDO,
        MapMetalness = rl::ShaderLocationIndex::LOC_MAP_METALNESS,
        MapNormal = rl::ShaderLocationIndex::LOC_MAP_NORMAL,
        MapRoughness = rl::ShaderLocationIndex::LOC_MAP_ROUGHNESS,
        MapOcclusion = rl::ShaderLocationIndex::LOC_MAP_OCCLUSION,
        MapEmission = rl::ShaderLocationIndex::LOC_MAP_EMISSION,
        MapHeight = rl::ShaderLocationIndex::LOC_MAP_HEIGHT,
        MapCubeMap = rl::ShaderLocationIndex::LOC_MAP_CUBEMAP,
        MapIrradiance = rl::ShaderLocationIndex::LOC_MAP_IRRADIANCE,
        MapPrefilter = rl::ShaderLocationIndex::LOC_MAP_PREFILTER,
        MapBRDF = rl::ShaderLocationIndex::LOC_MAP_BRDF,
    }
}

enum_from_i32! {
    pub enum Texmap {
        Albedo = rl::TexmapIndex::MAP_ALBEDO,
        Metalness = rl::TexmapIndex::MAP_METALNESS,
        Normal = rl::TexmapIndex::MAP_NORMAL,
        Roughness = rl::TexmapIndex::MAP_ROUGHNESS,
        Occlusion = rl::TexmapIndex::MAP_OCCLUSION,
        Emission = rl::TexmapIndex::MAP_EMISSION,
        Height = rl::TexmapIndex::MAP_HEIGHT,
        CubeMap = rl::TexmapIndex::MAP_CUBEMAP,
        Irradiance = rl::TexmapIndex::MAP_IRRADIANCE,
        Prefilter = rl::TexmapIndex::MAP_PREFILTER,
        BRDF = rl::TexmapIndex::MAP_BRDF,
    }
}

enum_from_i32! {
    pub enum PixelFormat {
        UncompressedGrayscale = rl::PixelFormat::UNCOMPRESSED_GRAYSCALE,
        UncompressedGrayAlpha = rl::PixelFormat::UNCOMPRESSED_GRAY_ALPHA,
        UncompressedR5G6B5 = rl::PixelFormat::UNCOMPRESSED_R5G6B5,
        UncompressedR8G8B8 = rl::PixelFormat::UNCOMPRESSED_R8G8B8,
        UncompressedR5G5B5A1 = rl::PixelFormat::UNCOMPRESSED_R5G5B5A1,
        UncompressedR4G4B4A4 = rl::PixelFormat::UNCOMPRESSED_R4G4B4A4,
        UncompressedR8G8B8A8 = rl::PixelFormat::UNCOMPRESSED_R8G8B8A8,
        UncompressedR32 = rl::PixelFormat::UNCOMPRESSED_R32,
        UncompressedR32G32B32 = rl::PixelFormat::UNCOMPRESSED_R32G32B32,
        UncompressedR32G32B32A32 = rl::PixelFormat::UNCOMPRESSED_R32G32B32A32,
        CompressedDXT1RGB = rl::PixelFormat::COMPRESSED_DXT1_RGB,
        CompressedDXT1RGBA = rl::PixelFormat::COMPRESSED_DXT1_RGBA,
        CompressedDXT3RGBA = rl::PixelFormat::COMPRESSED_DXT3_RGBA,
        CompressedDXT5RGBA = rl::PixelFormat::COMPRESSED_DXT5_RGBA,
        CompressedETC1RGB = rl::PixelFormat::COMPRESSED_ETC1_RGB,
        CompressedETC2RGB = rl::PixelFormat::COMPRESSED_ETC2_RGB,
        CompressedETC2EACRGBA = rl::PixelFormat::COMPRESSED_ETC2_EAC_RGBA,
        CompressedPVRTRGB = rl::PixelFormat::COMPRESSED_PVRT_RGB,
        CompressedPVRTRGBA = rl::PixelFormat::COMPRESSED_PVRT_RGBA,
        CompressedASTC4x4RGBA = rl::PixelFormat::COMPRESSED_ASTC_4x4_RGBA,
        CompressedASTC8x8RGBA = rl::PixelFormat::COMPRESSED_ASTC_8x8_RGBA,
    }
}

enum_from_i32! {
    pub enum TextureFilter {
        Point = rl::TextureFilterMode::FILTER_POINT,
        Bilinear = rl::TextureFilterMode::FILTER_BILINEAR,
        Trilinear = rl::TextureFilterMode::FILTER_TRILINEAR,
        Anisotropic4x = rl::TextureFilterMode::FILTER_ANISOTROPIC_4X,
        Anisotropic8x = rl::TextureFilterMode::FILTER_ANISOTROPIC_8X,
        Anisotropic16x = rl::TextureFilterMode::FILTER_ANISOTROPIC_16X,
    }
}

enum_from_i32! {
    pub enum TextureWrap {
        Repeat = rl::TextureWrapMode::WRAP_REPEAT,
        Clamp = rl::TextureWrapMode::WRAP_CLAMP,
        MirrorRepeat = rl::TextureWrapMode::WRAP_MIRROR_REPEAT,
        MirrorClamp = rl::TextureWrapMode::WRAP_MIRROR_CLAMP,
    }
}

enum_from_i32! {
    pub enum BlendMode {
        Alpha = rl::BlendMode::BLEND_ALPHA,
        Additive = rl::BlendMode::BLEND_ADDITIVE,
        Multiplied = rl::BlendMode::BLEND_MULTIPLIED,
    }
}

enum_from_i32! {
    pub enum CameraMode {
        Custom = rl::CameraMode::CAMERA_CUSTOM,
        Free = rl::CameraMode::CAMERA_FREE,
        Orbital = rl::CameraMode::CAMERA_ORBITAL,
        FirstPerson = rl::CameraMode::CAMERA_FIRST_PERSON,
        ThirdPerson = rl::CameraMode::CAMERA_THIRD_PERSON,
    }
}

enum_from_i32! {
    pub enum CameraType {
        Perspective = rl::CameraType::CAMERA_PERSPECTIVE,
        Orthographic = rl::CameraType::CAMERA_ORTHOGRAPHIC,
    }
}

enum_from_i32! {
    pub enum VrDevice {
        Default = rl::VrDeviceType::HMD_DEFAULT_DEVICE,
        OculusRiftDK2 = rl::VrDeviceType::HMD_OCULUS_RIFT_DK2,
        OculusRiftCV1 = rl::VrDeviceType::HMD_OCULUS_RIFT_CV1,
        OculusGo = rl::VrDeviceType::HMD_OCULUS_GO,
        ValveHTCVive = rl::VrDeviceType::HMD_VALVE_HTC_VIVE,
        SonyPSVR = rl::VrDeviceType::HMD_SONY_PSVR,
    }
}

impl_bidirectional_from!(Vector2, rl::Vector2, x, y);
impl_bidirectional_from!(Vector3, rl::Vector3, x, y, z);
impl_bidirectional_from!(Vector4, rl::Vector4, x, y, z, w);
impl_bidirectional_from!(
    Matrix,
    rl::Matrix,
    m0,
    m4,
    m8,
    m12,
    m1,
    m5,
    m9,
    m13,
    m2,
    m6,
    m10,
    m14,
    m3,
    m7,
    m11,
    m15
);

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const LIGHTGRAY: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };
    pub const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };
    pub const DARKGRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };
    pub const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };
    pub const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };
    pub const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };
    pub const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };
    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };
    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };
    pub const DARKGREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };
    pub const SKYBLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };
    pub const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };
    pub const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };
    pub const DARKPURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };
    pub const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };
    pub const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };
    pub const DARKBROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const BLANK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const RAYWHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };

    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

impl_bidirectional_from!(Color, rl::Color, r, g, b, a);

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
