//! Contains code related to drawing. Types that can be set as a surface to draw will implement the [`RaylibDraw`] trait

use raylib_sys::Rectangle;

use crate::core::texture::Texture2D;
use crate::core::vr::VrStereoConfig;
use crate::core::{RaylibHandle, RaylibThread};
use crate::math::Matrix;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::models::WeakMaterial;
use crate::{MintMatrix, MintVec2};
use crate::{MintVec3, ffi};
use std::ffi::CString;
use std::{convert::AsRef, marker::PhantomData};

use super::shaders::Shader;

/// Seems like all draw commands must be issued from the main thread
impl RaylibHandle {
    #[inline]
    #[must_use]
    /// Setup canvas (framebuffer) to start drawing.
    /// Prefer using the closure version, [RaylibHandle::draw]. This version returns a handle that calls [raylib_sys::EndDrawing] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    pub fn begin_drawing<'a>(&'a mut self, _: &RaylibThread) -> RaylibDrawHandle<'a> {
        unsafe {
            ffi::BeginDrawing();
        };

        let d = RaylibDrawHandle(self);
        d
    }
    /// Setup canvas (framebuffer) to start drawing.
    // Every FnMut is a FnOnce, but not every FnOnce is a FnMut. The closure may possibly execute multiple times throughout the program, but not multiple times in a single call to this method.
    // Taking a FnOnce instead of a FnMut when the function only needs to be called once in this method makes the method slightly more versatile/less needlessly restrictive for no actual cost.
    pub fn draw<'a>(&'a mut self, _: &RaylibThread, func: impl FnOnce(RaylibDrawHandle<'a>)) {
        unsafe {
            ffi::BeginDrawing();
        };
        func(RaylibDrawHandle(self));
        // Uncomment the following if RaylibDrawHandle has been changed to no longer call EndDrawing() in its drop implementation:
        // unsafe {
        //     ffi::EndDrawing();
        // }
    }
}

pub struct RaylibDrawHandle<'a>(&'a mut RaylibHandle);

impl<'a> RaylibDrawHandle<'a> {
    #[deprecated = "Calling begin_drawing within RaylibDrawHandle will result in a runtime error."]
    #[doc(hidden)]
    pub fn begin_drawing(&mut self, _: &RaylibThread) -> RaylibDrawHandle {
        panic!("Nested begin_drawing call")
    }
    #[deprecated = "Calling draw within RaylibDrawHandle will result in a runtime error."]
    #[doc(hidden)]
    pub fn draw(&mut self, _: &RaylibThread, mut _func: impl FnMut(RaylibDrawHandle)) {
        panic!("Nested draw call")
    }
}

impl<'a> Drop for RaylibDrawHandle<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::EndDrawing();
        }
    }
}

impl<'a> std::ops::Deref for RaylibDrawHandle<'a> {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for RaylibDrawHandle<'a> {
    fn deref_mut(&mut self) -> &mut RaylibHandle {
        self.0
    }
}
impl<'a> RaylibDraw for RaylibDrawHandle<'a> {}

// Texture2D Stuff

// The texture does not need to be held exclusively for the duration of the *previous* draw mode, nor does the *previous* draw mode need to outlive the texture reference.
// The texture exclusivity and previous draw mode only need to outlive the *current* draw mode. So they can (and should) have separate lifetimes.
//
// Additionally: the texture is not actually *used* by this wrapper after construction, only the previous draw mode.
// Some space can be saved by using a phantom instead of copying the actual reference.
// The PhantomData will ensure that the borrow checker still analyzes as though the mutable texture reference was held, without physically storing it in the runtime memory.
pub struct RaylibTextureMode<'a, 'b, T: 'a>(&'a mut T, PhantomData<&'b mut ffi::RenderTexture2D>);

impl<'a, 'b, T: 'a> Drop for RaylibTextureMode<'a, 'b, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndTextureMode() }
    }
}
impl<'a, 'b, T: 'a> std::ops::Deref for RaylibTextureMode<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, 'b, T: 'a> std::ops::DerefMut for RaylibTextureMode<'a, 'b, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}
// framebuffer: &'a mut ffi::RenderTexture2D,

pub trait RaylibTextureModeExt
where
    Self: Sized,
{
    /// Begin drawing to render texture.
    /// Prefer using the closure version, [RaylibTextureModeExt::draw_texture_mode] . This version returns a handle that calls [raylib_sys::EndTextureMode] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    #[inline]
    #[must_use]
    fn begin_texture_mode<'a, 'b>(
        &'a mut self,
        _: &RaylibThread,
        framebuffer: &'b mut ffi::RenderTexture2D,
    ) -> RaylibTextureMode<'a, 'b, Self> {
        unsafe { ffi::BeginTextureMode(*framebuffer) }
        RaylibTextureMode(self, PhantomData)
    }

    /// Begin drawing to render texture.
    fn draw_texture_mode<'a, 'b>(
        &'a mut self,
        _: &RaylibThread,
        framebuffer: &'b mut ffi::RenderTexture2D,
        func: impl FnOnce(RaylibTextureMode<'a, 'b, Self>),
    ) {
        unsafe { ffi::BeginTextureMode(*framebuffer) }
        func(RaylibTextureMode(self, PhantomData));
        // Uncomment the following if RaylibTextureMode has been changed to no longer call EndTextureMode() in its drop implementation:
        // unsafe { ffi::EndTextureMode(); }
    }
}

// Only the DrawHandle and the RaylibHandle can start a texture
impl<'a> RaylibTextureModeExt for RaylibDrawHandle<'a> {}
impl RaylibTextureModeExt for RaylibHandle {}
impl<'a, 'b, T: 'a> RaylibDraw for RaylibTextureMode<'a, 'b, T> {}

// VR Stuff

// Lifetime 'a is duplicatively stored in a PhantomData so that the borrow checker knows T is being held *exclusively* for the lifetime of the mode, without giving the false impression that it can actually be mutated by the library.
pub struct RaylibVRMode<'a, 'b, T: 'a>(
    &'a T,
    PhantomData<&'a mut T>,
    PhantomData<&'b mut VrStereoConfig>,
);
impl<'a, 'b, T: 'a> Drop for RaylibVRMode<'a, 'b, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndVrStereoMode() }
    }
}
impl<'a, 'b, T: 'a> std::ops::Deref for RaylibVRMode<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibVRModeExt
where
    Self: Sized,
{
    /// Begin stereo rendering (requires VR simulator).
    /// Prefer using the closure version, [RaylibVRModeExt::draw_vr_stereo_mode] . This version returns a handle that calls [raylib_sys::EndVrStereoMode] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    #[inline]
    #[must_use]
    fn begin_vr_stereo_mode<'a, 'b>(
        &'a mut self,
        _: &RaylibThread,
        vr_config: &'b mut VrStereoConfig,
    ) -> RaylibVRMode<'a, 'b, Self> {
        unsafe { ffi::BeginVrStereoMode(*vr_config.as_ref()) }
        RaylibVRMode(self, PhantomData, PhantomData)
    }

    /// Begin stereo rendering (requires VR simulator).
    fn draw_vr_stereo_mode<'a, 'b>(
        &'a mut self,
        vr_config: &'b mut VrStereoConfig,
        func: impl FnOnce(RaylibVRMode<'a, 'b, Self>),
    ) {
        unsafe { ffi::BeginVrStereoMode(*vr_config.as_ref()) }
        func(RaylibVRMode(self, PhantomData, PhantomData));
        // Uncomment the following if RaylibVRMode has been changed to no longer call EndTextureMode() in its drop implementation:
        // unsafe { ffi::EndVrStereoMode(); }
    }
}

impl<D: RaylibDraw> RaylibVRModeExt for D {}
impl<'a, 'b, T: 'a> RaylibDraw for RaylibVRMode<'a, 'b, T> {}

// 2D Mode

pub struct RaylibMode2D<'a, T: 'a>(&'a mut T);
impl<'a, T: 'a> Drop for RaylibMode2D<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndMode2D() }
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibMode2D<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibMode2D<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibMode2DExt
where
    Self: Sized,
{
    /// Begin 2D mode with custom camera (2D).
    /// Prefer using the closure version, [RaylibMode2DExt::draw_mode2D]. This version returns a handle that calls [raylib_sys::EndMode2D] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    fn begin_mode2D(&mut self, camera: impl Into<ffi::Camera2D>) -> RaylibMode2D<'_, Self> {
        unsafe {
            ffi::BeginMode2D(camera.into());
        }
        RaylibMode2D(self)
    }

    /// Begin 2D mode with custom camera (2D).
    #[allow(non_snake_case)]
    fn draw_mode2D<'a>(
        &'a mut self,
        camera: impl Into<ffi::Camera2D>,
        func: impl FnOnce(RaylibMode2D<'a, Self>),
    ) {
        unsafe {
            ffi::BeginMode2D(camera.into());
        }
        func(RaylibMode2D(self));
        // Uncomment the following if RaylibMode2D has been changed to no longer call EndMode2D() in its drop implementation:
        // unsafe {
        //     ffi::EndMode2D();
        // }
    }
}

impl<D: RaylibDraw> RaylibMode2DExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibMode2D<'a, T> {}

// 3D Mode

pub struct RaylibMode3D<'a, T: 'a>(&'a mut T);
impl<'a, T: 'a> Drop for RaylibMode3D<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndMode3D() }
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibMode3D<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibMode3D<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibMode3DExt
where
    Self: Sized,
{
    /// Begin 3D mode with custom camera (3D).
    /// Prefer using the closure version, [RaylibMode3DExt::draw_mode3D]. This version returns a handle that calls [raylib_sys::EndMode3D] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    #[allow(non_snake_case)]
    #[must_use]
    #[inline]
    fn begin_mode3D(&mut self, camera: impl Into<ffi::Camera3D>) -> RaylibMode3D<'_, Self> {
        unsafe {
            ffi::BeginMode3D(camera.into());
        }
        RaylibMode3D(self)
    }

    /// Begin 3D mode with custom camera (3D).
    #[allow(non_snake_case)]
    fn draw_mode3D<'a>(
        &'a mut self,
        camera: impl Into<ffi::Camera3D>,
        func: impl FnOnce(RaylibMode3D<'a, Self>),
    ) {
        unsafe {
            ffi::BeginMode3D(camera.into());
        }
        func(RaylibMode3D(self));
        // Uncomment the following if RaylibMode3D has been changed to no longer call EndMode3D() in its drop implementation:
        // unsafe {
        //     ffi::EndMode3D();
        // }
    }
}

impl<D: RaylibDraw> RaylibMode3DExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibMode3D<'a, T> {}
impl<'a, T: 'a> RaylibDraw3D for RaylibMode3D<'a, T> {}

// shader Mode

pub struct RaylibShaderMode<'a, 'b, T: 'a>(&'a mut T, PhantomData<&'b mut Shader>);

impl<'a, 'b, T: 'a> Drop for RaylibShaderMode<'a, 'b, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndShaderMode() }
    }
}
impl<'a, 'b, T: 'a> std::ops::Deref for RaylibShaderMode<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, 'b, T: 'a> std::ops::DerefMut for RaylibShaderMode<'a, 'b, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibShaderModeExt
where
    Self: Sized,
{
    /// Begin custom shader drawing.
    /// Prefer using the closure version, [RaylibShaderModeExt::draw_shader_mode]. This version returns a handle that calls [raylib_sys::EndShaderMode] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    #[must_use]
    #[inline]
    fn begin_shader_mode<'a, 'b>(
        &'a mut self,
        shader: &'b mut Shader,
    ) -> RaylibShaderMode<'a, 'b, Self> {
        unsafe { ffi::BeginShaderMode(*shader.as_ref()) }
        RaylibShaderMode(self, PhantomData)
    }

    /// Begin custom shader drawing.
    fn draw_shader_mode<'a, 'b>(
        &'a mut self,
        shader: &'b mut Shader,
        func: impl FnOnce(RaylibShaderMode<'a, 'b, Self>),
    ) {
        unsafe { ffi::BeginShaderMode(*shader.as_ref()) }
        func(RaylibShaderMode(self, PhantomData));
        // Uncomment the following if RaylibShaderMode has been changed to no longer call EndShaderMode() in its drop implementation:
        // unsafe { ffi::EndShaderMode(); }
    }
}

impl<D: RaylibDraw> RaylibShaderModeExt for D {}
impl<'a, 'b, T: 'a> RaylibDraw for RaylibShaderMode<'a, 'b, T> {}
impl<'a, 'b, T: 'a> RaylibDraw3D for RaylibShaderMode<'a, 'b, T> {}

// Blend Mode

pub struct RaylibBlendMode<'a, T: 'a>(&'a mut T);
impl<'a, T: 'a> Drop for RaylibBlendMode<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndBlendMode() }
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibBlendMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibBlendMode<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibBlendModeExt
where
    Self: Sized,
{
    /// Begin blending mode (alpha, additive, multiplied, subtract, custom).
    /// Prefer using the closure version, [RaylibBlendModeExt::draw_blend_mode]. This version returns a handle that calls [raylib_sys::EndBlendMode] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    #[inline]
    #[must_use]
    fn begin_blend_mode(
        &mut self,
        blend_mode: crate::consts::BlendMode,
    ) -> RaylibBlendMode<'_, Self> {
        unsafe { ffi::BeginBlendMode((blend_mode as u32) as i32) }
        RaylibBlendMode(self)
    }

    /// Begin blending mode (alpha, additive, multiplied, subtract, custom).
    fn draw_blend_mode<'a>(
        &'a mut self,
        blend_mode: crate::consts::BlendMode,
        func: impl FnOnce(RaylibBlendMode<'a, Self>),
    ) {
        unsafe { ffi::BeginBlendMode((blend_mode as u32) as i32) }
        func(RaylibBlendMode(self));
        // Uncomment the following if RaylibBlendMode has been changed to no longer call EndBlendMode() in its drop implementation:
        // unsafe { ffi::EndBlendMode(); }
    }
}

impl<D: RaylibDraw> RaylibBlendModeExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibBlendMode<'a, T> {}
impl<'a, T: 'a> RaylibDraw3D for RaylibBlendMode<'a, T> {}

// Scissor Mode stuff

pub struct RaylibScissorMode<'a, T: 'a>(&'a mut T);
impl<'a, T: 'a> Drop for RaylibScissorMode<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndScissorMode() }
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibScissorMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibScissorMode<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibScissorModeExt
where
    Self: Sized,
{
    /// Begin scissor mode (define screen area for following drawing).
    /// Prefer using the closure version, [RaylibScissorModeExt::draw_scissor_mode]. This version returns a handle that calls [raylib_sys::EndScissorMode] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    #[must_use]
    #[inline]
    fn begin_scissor_mode(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> RaylibScissorMode<'_, Self> {
        unsafe { ffi::BeginScissorMode(x, y, width, height) }
        RaylibScissorMode(self)
    }

    /// Begin scissor mode (define screen area for following drawing).
    fn draw_scissor_mode<'a>(
        &'a mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        func: impl FnOnce(RaylibScissorMode<'a, Self>),
    ) {
        unsafe { ffi::BeginScissorMode(x, y, width, height) }
        func(RaylibScissorMode(self));
        // Uncomment the following if RaylibScissorMode has been changed to no longer call EndScissorMode() in its drop implementation:
        // unsafe { ffi::EndScissorMode(); }
    }
}

impl<D: RaylibDraw> RaylibScissorModeExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibScissorMode<'a, T> {}
impl<'a, T: 'a + RaylibDraw3D> RaylibDraw3D for RaylibScissorMode<'a, T> {}

// Actual drawing functions

pub trait RaylibDraw {
    /// Sets background color (framebuffer clear color.into()).
    #[inline]
    fn clear_background(&mut self, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ClearBackground(color.into());
        }
    }

    /// Get texture that is used for shapes drawing
    #[inline]
    #[must_use]
    fn get_shapes_texture(&self) -> Texture2D {
        Texture2D(unsafe { ffi::GetShapesTexture() })
    }

    /// Get texture source rectangle that is used for shapes drawing
    #[inline]
    #[must_use]
    fn get_shapes_texture_rectangle(&self) -> Rectangle {
        unsafe { ffi::GetShapesTextureRectangle() }
    }

    /// Define default texture used to draw shapes
    #[inline]
    fn set_shapes_texture(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        source: impl Into<ffi::Rectangle>,
    ) {
        unsafe { ffi::SetShapesTexture(*texture.as_ref(), source.into()) }
    }

    // // Draw gui widget
    // fn draw_gui<G: crate::rgui::GuiDraw>(&mut self, widget: G) -> crate::rgui::DrawResult {
    //     widget.draw()
    // }

    // SHAPES
    /// Draws a pixel.
    #[inline]
    fn draw_pixel(&mut self, x: i32, y: i32, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawPixel(x, y, color.into());
        }
    }

    /// Draws a pixel (Vector version).
    #[inline]
    fn draw_pixel_v(&mut self, position: impl Into<MintVec2>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawPixelV(position.into(), color.into());
        }
    }

    /// Draws a line.
    #[inline]
    fn draw_line(
        &mut self,
        start_pos_x: i32,
        start_pos_y: i32,
        end_pos_x: i32,
        end_pos_y: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color.into());
        }
    }

    /// Draws a line (Vector version).
    #[inline]
    fn draw_line_v(
        &mut self,
        start_pos: impl Into<MintVec2>,
        end_pos: impl Into<MintVec2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLineV(start_pos.into(), end_pos.into(), color.into());
        }
    }

    /// Draws a line with thickness.
    #[inline]
    fn draw_line_ex(
        &mut self,
        start_pos: impl Into<MintVec2>,
        end_pos: impl Into<MintVec2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLineEx(start_pos.into(), end_pos.into(), thick, color.into());
        }
    }

    /// Draws a line using cubic-bezier curves in-out.
    #[inline]
    fn draw_line_bezier(
        &mut self,
        start_pos: impl Into<MintVec2>,
        end_pos: impl Into<MintVec2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLineBezier(start_pos.into(), end_pos.into(), thick, color.into());
        }
    }

    /// Draw lines sequence
    #[inline]
    fn draw_line_strip(&mut self, points: &[Vector2], color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawLineStrip(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into(),
            );
        }
    }

    /// Draws a color-filled circle.
    #[inline]
    fn draw_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircle(center_x, center_y, radius, color.into());
        }
    }
    /// Draw a piece of a circle
    #[inline]
    fn draw_circle_sector(
        &mut self,
        center: impl Into<MintVec2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircleSector(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments,
                color.into(),
            );
        }
    }

    /// Draw circle sector outline
    #[inline]
    fn draw_circle_sector_lines(
        &mut self,
        center: impl Into<MintVec2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircleSectorLines(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments,
                color.into(),
            );
        }
    }

    /// Draws a gradient-filled circle.
    #[inline]
    fn draw_circle_gradient(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color1: impl Into<ffi::Color>,
        color2: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircleGradient(center_x, center_y, radius, color1.into(), color2.into());
        }
    }

    /// Draws a color-filled circle (Vector version).
    #[inline]
    fn draw_circle_v(
        &mut self,
        center: impl Into<MintVec2>,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircleV(center.into(), radius, color.into());
        }
    }

    /// Draws circle outline.
    #[inline]
    fn draw_circle_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircleLines(center_x, center_y, radius, color.into());
        }
    }

    /// Draws circle outline. (Vector Version)
    #[inline]
    fn draw_circle_lines_v(
        &mut self,
        center: impl Into<MintVec2>,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircleLinesV(center.into(), radius, color.into());
        }
    }

    /// Draws ellipse.
    #[inline]
    fn draw_ellipse(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawEllipse(center_x, center_y, radius_h, radius_v, color.into());
        }
    }

    /// Draws ellipse.
    #[inline]
    fn draw_ellipse_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawEllipseLines(center_x, center_y, radius_h, radius_v, color.into());
        }
    }

    /// Draw ring
    #[inline]
    fn draw_ring(
        &mut self,
        center: impl Into<MintVec2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRing(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color.into(),
            );
        }
    }

    /// Draw ring lines
    #[inline]
    fn draw_ring_lines(
        &mut self,
        center: impl Into<MintVec2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRingLines(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color.into(),
            );
        }
    }

    /// Draws a color-filled rectangle.
    #[inline]
    fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangle(x, y, width, height, color.into());
        }
    }

    /// Draws a color-filled rectangle (Vector version).
    #[inline]
    fn draw_rectangle_v(
        &mut self,
        position: impl Into<MintVec2>,
        size: impl Into<MintVec2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleV(position.into(), size.into(), color.into());
        }
    }

    /// Draws a color-filled rectangle from `rec`.
    #[inline]
    fn draw_rectangle_rec(&mut self, rec: impl Into<ffi::Rectangle>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawRectangleRec(rec.into(), color.into());
        }
    }

    /// Draws a color-filled rectangle with pro parameters.
    #[inline]
    fn draw_rectangle_pro(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        origin: impl Into<MintVec2>,
        rotation: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectanglePro(rec.into(), origin.into(), rotation, color.into());
        }
    }

    /// Draws a vertical-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    fn draw_rectangle_gradient_v(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: impl Into<ffi::Color>,
        color2: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleGradientV(x, y, width, height, color1.into(), color2.into());
        }
    }

    /// Draws a horizontal-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    fn draw_rectangle_gradient_h(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: impl Into<ffi::Color>,
        color2: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleGradientH(x, y, width, height, color1.into(), color2.into());
        }
    }

    /// Draws a gradient-filled rectangle with custom vertex colors.
    ///
    /// **NOTE**: Colors refer to corners, starting at top-left corner and going counter-clockwise.
    #[inline]
    fn draw_rectangle_gradient_ex(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        col1: impl Into<ffi::Color>,
        col2: impl Into<ffi::Color>,
        col3: impl Into<ffi::Color>,
        col4: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleGradientEx(
                rec.into(),
                col1.into(),
                col2.into(),
                col3.into(),
                col4.into(),
            );
        }
    }

    /// Draws rectangle outline.
    #[inline]
    fn draw_rectangle_lines(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleLines(x, y, width, height, color.into());
        }
    }

    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_lines_ex(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        line_thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleLinesEx(rec.into(), line_thick, color.into());
        }
    }
    /// Draws rectangle with rounded edges.
    #[inline]
    fn draw_rectangle_rounded(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        roundness: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleRounded(rec.into(), roundness, segments, color.into());
        }
    }

    /// Draws rectangle outline with rounded edges included.
    #[inline]
    fn draw_rectangle_rounded_lines(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        roundness: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleRoundedLines(rec.into(), roundness, segments, color.into());
        }
    }

    /// Draw rectangle with rounded edges outline
    #[inline]
    fn draw_rectangle_rounded_lines_ex(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        roundness: f32,
        segments: i32,
        line_thickness: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleRoundedLinesEx(
                rec.into(),
                roundness,
                segments,
                line_thickness,
                color.into(),
            )
        };
    }
    /// Draws a triangle.
    #[inline]
    fn draw_triangle(
        &mut self,
        v1: impl Into<MintVec2>,
        v2: impl Into<MintVec2>,
        v3: impl Into<MintVec2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTriangle(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draws a triangle using lines.
    #[inline]
    fn draw_triangle_lines(
        &mut self,
        v1: impl Into<MintVec2>,
        v2: impl Into<MintVec2>,
        v3: impl Into<MintVec2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTriangleLines(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draw a triangle fan defined by points.
    #[inline]
    fn draw_triangle_fan(&mut self, points: &[Vector2], color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawTriangleFan(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into(),
            );
        }
    }

    /// Draw a triangle strip defined by points
    #[inline]
    fn draw_triangle_strip(&mut self, points: &[Vector2], color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawTriangleStrip(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into(),
            );
        }
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    fn draw_poly(
        &mut self,
        center: impl Into<MintVec2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawPoly(center.into(), sides, radius, rotation, color.into());
        }
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    fn draw_poly_lines(
        &mut self,
        center: impl Into<MintVec2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawPolyLines(center.into(), sides, radius, rotation, color.into());
        }
    }

    /// Draws a `texture` using specified position and `tint` color.
    #[inline]
    fn draw_texture(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        x: i32,
        y: i32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTexture(*texture.as_ref(), x, y, tint.into());
        }
    }

    /// Draws a `texture` using specified `position` vector and `tint` color.
    #[inline]
    fn draw_texture_v(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        position: impl Into<MintVec2>,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTextureV(*texture.as_ref(), position.into(), tint.into());
        }
    }

    /// Draws a `texture` with extended parameters.
    #[inline]
    fn draw_texture_ex(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        position: impl Into<MintVec2>,
        rotation: f32,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTextureEx(
                *texture.as_ref(),
                position.into(),
                rotation,
                scale,
                tint.into(),
            );
        }
    }

    /// Draws from a region of `texture` defined by the `source_rec` rectangle.
    #[inline]
    fn draw_texture_rec(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        source_rec: impl Into<ffi::Rectangle>,
        position: impl Into<MintVec2>,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTextureRec(
                *texture.as_ref(),
                source_rec.into(),
                position.into(),
                tint.into(),
            );
        }
    }

    /// Draw from a region of `texture` defined by the `source_rec` rectangle with pro parameters.
    #[inline]
    fn draw_texture_pro(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        source_rec: impl Into<ffi::Rectangle>,
        dest_rec: impl Into<ffi::Rectangle>,
        origin: impl Into<MintVec2>,
        rotation: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTexturePro(
                *texture.as_ref(),
                source_rec.into(),
                dest_rec.into(),
                origin.into(),
                rotation,
                tint.into(),
            );
        }
    }

    /// Draws a texture (or part of it) that stretches or shrinks nicely
    #[inline]
    fn draw_texture_n_patch(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        n_patch_info: impl Into<ffi::NPatchInfo>,
        dest_rec: impl Into<ffi::Rectangle>,
        origin: impl Into<MintVec2>,
        rotation: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTextureNPatch(
                *texture.as_ref(),
                n_patch_info.into(),
                dest_rec.into(),
                origin.into(),
                rotation,
                tint.into(),
            );
        }
    }

    /// Shows current FPS.
    #[inline]
    fn draw_fps(&mut self, x: i32, y: i32) {
        unsafe {
            ffi::DrawFPS(x, y);
        }
    }

    /// Draws text (using default font).
    /// This does not support UTF-8. Use `[RaylibDrawHandle::draw_text_codepoints]` for that.
    #[inline]
    fn draw_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: i32,
        color: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();

        unsafe {
            ffi::DrawText(c_text.as_ptr(), x, y, font_size, color.into());
        }
    }

    /// Draws text (using default font) with support for UTF-8.
    /// If you do not need UTF-8, use `[RaylibDrawHandle::draw_text]`.
    fn draw_text_codepoints(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<MintVec2>,
        font_size: f32,
        spacing: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            let c_text = CString::new(text).unwrap();

            let mut len = 0;
            let u = ffi::LoadCodepoints(c_text.as_ptr(), &mut len);

            ffi::DrawTextCodepoints(
                *font.as_ref(),
                u,
                text.len() as i32,
                position.into(),
                font_size,
                spacing,
                tint.into(),
            )
        }
    }
    /// Draws text using `font` and additional parameters.
    #[inline]
    fn draw_text_ex(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<MintVec2>,
        font_size: f32,
        spacing: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::DrawTextEx(
                *font.as_ref(),
                c_text.as_ptr(),
                position.into(),
                font_size,
                spacing,
                tint.into(),
            );
        }
    }

    /// Draw text using Font and pro parameters (rotation)
    #[inline]
    fn draw_text_pro(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<MintVec2>,
        origin: impl Into<MintVec2>,
        rotation: f32,
        font_size: f32,
        spacing: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::DrawTextPro(
                *font.as_ref(),
                c_text.as_ptr(),
                position.into(),
                origin.into(),
                rotation,
                font_size,
                spacing,
                tint.into(),
            );
        }
    }

    /// Draw one character (codepoint)
    #[inline]
    fn draw_text_codepoint(
        &mut self,
        font: impl AsRef<ffi::Font>,
        codepoint: i32,
        position: impl Into<MintVec2>,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTextCodepoint(
                *font.as_ref(),
                codepoint,
                position.into(),
                scale,
                tint.into(),
            );
        }
    }

    /// Enable waiting for events when the handle is dropped, no automatic event polling
    #[inline]
    fn enable_event_waiting(&self) {
        unsafe { ffi::EnableEventWaiting() }
    }

    /// Disable waiting for events when the handle is dropped, no automatic event polling
    #[inline]
    fn disable_event_waiting(&self) {
        unsafe { ffi::DisableEventWaiting() }
    }

    /// Draw a polygon outline of n sides with extended parameters
    #[inline]
    fn draw_poly_lines_ex(
        &mut self,
        center: impl Into<MintVec2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        line_thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawPolyLinesEx(
                center.into(),
                sides,
                radius,
                rotation,
                line_thick,
                color.into(),
            );
        }
    }
    /// Draw spline: Linear, minimum 2 points
    #[inline]
    fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawSplineLinear(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                thick,
                color.into(),
            )
        }
    }
    /// Draw spline: B-Spline, minimum 4 points
    #[inline]
    fn draw_spline_basis(&mut self, points: &[Vector2], thick: f32, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawSplineBasis(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                thick,
                color.into(),
            )
        }
    }
    /// Draw spline: Catmull-Rom, minimum 4 points
    #[inline]
    fn draw_spline_catmull_rom(
        &mut self,
        points: &[Vector2],
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSplineCatmullRom(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                thick,
                color.into(),
            )
        }
    }

    /// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
    #[inline]
    fn draw_spline_bezier_quadratic(
        &mut self,
        points: &[Vector2],
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSplineBezierQuadratic(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                thick,
                color.into(),
            )
        }
    }

    /// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
    #[inline]
    fn draw_spline_bezier_cubic(
        &mut self,
        points: &[Vector2],
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSplineBezierCubic(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                thick,
                color.into(),
            )
        }
    }

    /// Draw spline segment: Linear, 2 points
    #[inline]
    fn draw_spline_segment_linear(
        &mut self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe { ffi::DrawSplineSegmentLinear(p1.into(), p2.into(), thick, color.into()) }
    }

    /// Draw spline segment: B-Spline, 4 points
    #[inline]
    fn draw_spline_segment_basis(
        &mut self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSplineSegmentBasis(
                p1.into(),
                p2.into(),
                p3.into(),
                p4.into(),
                thick,
                color.into(),
            )
        }
    }

    /// Draw spline segment: Catmull-Rom, 4 points
    #[inline]
    fn draw_spline_segment_catmull_rom(
        &mut self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSplineSegmentCatmullRom(
                p1.into(),
                p2.into(),
                p3.into(),
                p4.into(),
                thick,
                color.into(),
            )
        }
    }

    /// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
    #[inline]
    fn draw_spline_segment_bezier_quadratic(
        &mut self,
        p1: impl Into<MintVec2>,
        c2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSplineSegmentBezierQuadratic(
                p1.into(),
                c2.into(),
                p3.into(),
                thick,
                color.into(),
            )
        }
    }

    /// Draw spline segment: Cubic Bezier, 2 points, 2 control points
    #[inline]
    fn draw_spline_segment_bezier_cubic(
        &mut self,
        p1: impl Into<MintVec2>,
        c2: impl Into<MintVec2>,
        c3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSplineSegmentBezierCubic(
                p1.into(),
                c2.into(),
                c3.into(),
                p4.into(),
                thick,
                color.into(),
            )
        }
    }

    /// Get (evaluate) spline point: Linear
    #[inline]
    #[must_use]
    fn get_spline_point_linear(
        &mut self,
        start_pos: impl Into<MintVec2>,
        end_pos: impl Into<MintVec2>,
        t: f32,
    ) -> Vector2 {
        unsafe { ffi::GetSplinePointLinear(start_pos.into(), end_pos.into(), t).into() }
    }

    /// Get (evaluate) spline point: B-Spline
    #[inline]
    #[must_use]
    fn get_spline_point_basis(
        &mut self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
        t: f32,
    ) -> Vector2 {
        unsafe { ffi::GetSplinePointBasis(p1.into(), p2.into(), p3.into(), p4.into(), t).into() }
    }

    /// Get (evaluate) spline point: Catmull-Rom
    #[inline]
    #[must_use]
    fn get_spline_point_catmull_rom(
        &mut self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
        t: f32,
    ) -> Vector2 {
        unsafe {
            ffi::GetSplinePointCatmullRom(p1.into(), p2.into(), p3.into(), p4.into(), t).into()
        }
    }

    /// Get (evaluate) spline point: Quadratic Bezier
    #[inline]
    #[must_use]
    fn get_spline_point_bezier_quad(
        &mut self,
        p1: impl Into<MintVec2>,
        c2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        t: f32,
    ) -> Vector2 {
        unsafe { ffi::GetSplinePointBezierQuad(p1.into(), c2.into(), p3.into(), t).into() }
    }

    /// Get (evaluate) spline point: Cubic Bezier
    #[inline]
    #[must_use]
    fn get_spline_point_bezier_cubic(
        &mut self,
        p1: impl Into<MintVec2>,
        c2: impl Into<MintVec2>,
        c3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
        t: f32,
    ) -> Vector2 {
        unsafe {
            ffi::GetSplinePointBezierCubic(p1.into(), c2.into(), c3.into(), p4.into(), t).into()
        }
    }
}

pub trait RaylibDraw3D {
    /// Draw a point in 3D space, actually a small line
    #[allow(non_snake_case)]
    #[inline]
    fn draw_point3D(&mut self, position: impl Into<MintVec3>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawPoint3D(position.into(), color.into());
        }
    }

    /// Draw a color-filled triangle (vertex in counter-clockwise order!)
    #[allow(non_snake_case)]
    #[inline]
    fn draw_triangle3D(
        &mut self,
        v1: impl Into<MintVec3>,
        v2: impl Into<MintVec3>,
        v3: impl Into<MintVec3>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTriangle3D(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draw a triangle strip defined by points
    #[allow(non_snake_case)]
    #[inline]
    fn draw_triangle_strip3D(&mut self, points: &[Vector3], color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawTriangleStrip3D(points.as_ptr() as *mut _, points.len() as i32, color.into());
        }
    }

    /// Draws a line in 3D world space.
    #[allow(non_snake_case)]
    #[inline]
    fn draw_line3D(
        &mut self,
        start_pos: impl Into<MintVec3>,
        end_pos: impl Into<MintVec3>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLine3D(start_pos.into(), end_pos.into(), color.into());
        }
    }

    /// Draws a circle in 3D world space.
    #[allow(non_snake_case)]
    #[inline]
    fn draw_circle3D(
        &mut self,
        center: impl Into<MintVec3>,
        radius: f32,
        rotation_axis: impl Into<MintVec3>,
        rotation_angle: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircle3D(
                center.into(),
                radius,
                rotation_axis.into(),
                rotation_angle,
                color.into(),
            );
        }
    }

    /// Draws a cube.
    #[inline]
    fn draw_cube(
        &mut self,
        position: impl Into<MintVec3>,
        width: f32,
        height: f32,
        length: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCube(position.into(), width, height, length, color.into());
        }
    }

    /// Draws a cube (Vector version).
    #[inline]
    fn draw_cube_v(
        &mut self,
        position: impl Into<MintVec3>,
        size: impl Into<MintVec3>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCubeV(position.into(), size.into(), color.into());
        }
    }

    /// Draws a cube in wireframe.
    #[inline]
    fn draw_cube_wires(
        &mut self,
        position: impl Into<MintVec3>,
        width: f32,
        height: f32,
        length: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCubeWires(position.into(), width, height, length, color.into());
        }
    }

    /// Draws a cube in wireframe. (Vector Version)
    #[inline]
    fn draw_cube_wires_v(
        &mut self,
        position: impl Into<MintVec3>,
        size: impl Into<MintVec3>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCubeWiresV(position.into(), size.into(), color.into());
        }
    }

    /// Draw a 3d mesh with material and transform
    #[inline]
    fn draw_mesh(
        &mut self,
        mesh: impl AsRef<ffi::Mesh>,
        material: WeakMaterial,
        transform: impl Into<ffi::Matrix>,
    ) {
        unsafe { ffi::DrawMesh(*mesh.as_ref(), material.0, transform.into()) }
    }

    /// Draw multiple mesh instances with material and different transforms
    #[inline]
    fn draw_mesh_instanced(
        &mut self,
        mesh: impl AsRef<ffi::Mesh>,
        material: WeakMaterial,
        transforms: &[Matrix],
    ) {
        unsafe {
            ffi::DrawMeshInstanced(
                *mesh.as_ref(),
                material.0,
                transforms.as_ptr() as *const MintMatrix,
                transforms.len() as i32,
            )
        }
    }

    /// Draws a sphere.
    #[inline]
    fn draw_sphere(
        &mut self,
        center_pos: impl Into<MintVec3>,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSphere(center_pos.into(), radius, color.into());
        }
    }

    /// Draws a sphere with extended parameters.
    #[inline]
    fn draw_sphere_ex(
        &mut self,
        center_pos: impl Into<MintVec3>,
        radius: f32,
        rings: i32,
        slices: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSphereEx(center_pos.into(), radius, rings, slices, color.into());
        }
    }

    /// Draws a sphere in wireframe.
    #[inline]
    fn draw_sphere_wires(
        &mut self,
        center_pos: impl Into<MintVec3>,
        radius: f32,
        rings: i32,
        slices: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawSphereWires(center_pos.into(), radius, rings, slices, color.into());
        }
    }

    /// Draws a cylinder.
    #[inline]
    fn draw_cylinder(
        &mut self,
        position: impl Into<MintVec3>,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCylinder(
                position.into(),
                radius_top,
                radius_bottom,
                height,
                slices,
                color.into(),
            );
        }
    }

    /// Draws a cylinder with extended parameters.
    #[inline]
    fn draw_cylinder_ex(
        &mut self,
        start_position: impl Into<MintVec3>,
        end_position: impl Into<MintVec3>,
        radius_start: f32,
        radius_end: f32,
        slices: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCylinderEx(
                start_position.into(),
                end_position.into(),
                radius_start,
                radius_end,
                slices,
                color.into(),
            );
        }
    }

    /// Draws a cylinder in wireframe.
    #[inline]
    fn draw_cylinder_wires(
        &mut self,
        position: impl Into<MintVec3>,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCylinderWires(
                position.into(),
                radius_top,
                radius_bottom,
                height,
                slices,
                color.into(),
            );
        }
    }

    /// Draws a cylinder in wireframe with extended parameters.
    #[inline]
    fn draw_cylinder_wires_ex(
        &mut self,
        start_position: impl Into<MintVec3>,
        end_position: impl Into<MintVec3>,
        radius_start: f32,
        radius_end: f32,
        slices: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCylinderWiresEx(
                start_position.into(),
                end_position.into(),
                radius_start,
                radius_end,
                slices,
                color.into(),
            );
        }
    }

    /// Draw capsule with the center of its sphere caps at startPos and endPos
    #[inline]
    fn draw_capsule(
        &mut self,
        start_pos: impl Into<MintVec3>,
        end_pos: impl Into<MintVec3>,
        radius: f32,
        slices: i32,
        rings: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCapsule(
                start_pos.into(),
                end_pos.into(),
                radius,
                slices,
                rings,
                color.into(),
            )
        }
    }

    ///Draw capsule wireframe with the center of its sphere caps at startPos and endPos
    #[inline]
    fn draw_capsule_wires(
        &mut self,
        start_pos: impl Into<MintVec3>,
        end_pos: impl Into<MintVec3>,
        radius: f32,
        slices: i32,
        rings: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCapsuleWires(
                start_pos.into(),
                end_pos.into(),
                radius,
                slices,
                rings,
                color.into(),
            )
        }
    }

    /// Draws an X/Z plane.
    #[inline]
    fn draw_plane(
        &mut self,
        center_pos: impl Into<MintVec3>,
        size: impl Into<MintVec2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawPlane(center_pos.into(), size.into(), color.into());
        }
    }

    /// Draws a ray line.
    #[inline]
    fn draw_ray(&mut self, ray: impl Into<ffi::Ray>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawRay(ray.into(), color.into());
        }
    }

    /// Draws a grid (centered at (0, 0, 0)).
    #[inline]
    fn draw_grid(&mut self, slices: i32, spacing: f32) {
        unsafe {
            ffi::DrawGrid(slices, spacing);
        }
    }

    /// Draws a model (with texture if set).
    #[inline]
    fn draw_model(
        &mut self,
        model: impl AsRef<ffi::Model>,
        position: impl Into<MintVec3>,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawModel(*model.as_ref(), position.into(), scale, tint.into());
        }
    }

    /// Draws a model with extended parameters.
    #[inline]
    fn draw_model_ex(
        &mut self,
        model: impl AsRef<ffi::Model>,
        position: impl Into<MintVec3>,
        rotation_axis: impl Into<MintVec3>,
        rotation_angle: f32,
        scale: impl Into<MintVec3>,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawModelEx(
                *model.as_ref(),
                position.into(),
                rotation_axis.into(),
                rotation_angle,
                scale.into(),
                tint.into(),
            );
        }
    }

    /// Draws a model with wires (with texture if set).
    #[inline]
    fn draw_model_wires(
        &mut self,
        model: impl AsRef<ffi::Model>,
        position: impl Into<MintVec3>,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawModelWires(*model.as_ref(), position.into(), scale, tint.into());
        }
    }

    /// Draws a model with wires.
    #[inline]
    fn draw_model_wires_ex(
        &mut self,
        model: impl AsRef<ffi::Model>,
        position: impl Into<MintVec3>,
        rotation_axis: impl Into<MintVec3>,
        rotation_angle: f32,
        scale: impl Into<MintVec3>,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawModelWiresEx(
                *model.as_ref(),
                position.into(),
                rotation_axis.into(),
                rotation_angle,
                scale.into(),
                tint.into(),
            );
        }
    }

    /// Draws a bounding box (wires).
    #[inline]
    fn draw_bounding_box(
        &mut self,
        bbox: impl Into<ffi::BoundingBox>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawBoundingBox(bbox.into(), color.into());
        }
    }

    /// Draws a billboard texture.
    #[inline]
    fn draw_billboard(
        &mut self,
        camera: impl Into<ffi::Camera3D>,
        texture: &Texture2D,
        center: impl Into<MintVec3>,
        size: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawBillboard(camera.into(), texture.0, center.into(), size, tint.into());
        }
    }

    /// Draws a billboard texture defined by `source_rec`.
    #[inline]
    fn draw_billboard_rec(
        &mut self,
        camera: impl Into<ffi::Camera3D>,
        texture: &Texture2D,
        source_rec: impl Into<ffi::Rectangle>,
        center: impl Into<MintVec3>,
        size: impl Into<MintVec2>,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawBillboardRec(
                camera.into(),
                texture.0,
                source_rec.into(),
                center.into(),
                size.into(),
                tint.into(),
            );
        }
    }

    /// Draw a billboard texture defined by source and rotation
    #[inline]
    fn draw_billboard_pro(
        &mut self,
        camera: impl Into<ffi::Camera>,
        texture: impl Into<ffi::Texture2D>,
        source: impl Into<ffi::Rectangle>,
        position: impl Into<MintVec3>,
        up: impl Into<MintVec3>,
        size: impl Into<MintVec2>,
        origin: impl Into<MintVec2>,
        rotation: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawBillboardPro(
                camera.into(),
                texture.into(),
                source.into(),
                position.into(),
                up.into(),
                size.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        }
    }

    /// Draw a model as points
    #[inline]
    fn draw_model_points(
        &mut self,
        model: impl Into<ffi::Model>,
        position: impl Into<MintVec3>,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawModelPoints(model.into(), position.into(), scale, tint.into());
        }
    }

    /// Draw a model as points with extended parameters
    #[inline]
    fn draw_model_points_ex(
        &mut self,
        model: impl Into<ffi::Model>,
        position: impl Into<MintVec3>,
        rotation_axis: impl Into<MintVec3>,
        angle: f32,
        scale: impl Into<MintVec3>,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawModelPointsEx(
                model.into(),
                position.into(),
                rotation_axis.into(),
                angle,
                scale.into(),
                tint.into(),
            );
        }
    }
}
