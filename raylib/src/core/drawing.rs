//! Contains code related to drawing. Types that can be set as a surface to draw will implement the [`RaylibDraw`] trait
use crate::core::camera::Camera3D;
use crate::core::math::Ray;
use crate::core::math::{Vector2, Vector3};

use crate::core::texture::Texture2D;
use crate::core::vr::VrStereoConfig;
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::convert::AsRef;
use std::ffi::CString;

/// Seems like all draw commands must be issued from the main thread
impl RaylibHandle {
    /// Setup canvas (framebuffer) to start drawing
    #[must_use]
    pub fn begin_drawing(&mut self, _: &RaylibThread) -> RaylibDrawHandle {
        unsafe {
            ffi::BeginDrawing();
        };
        let d = RaylibDrawHandle(self);
        d
    }
}

pub struct RaylibDrawHandle<'a>(&'a mut RaylibHandle);

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

impl<'a> RaylibDraw for RaylibDrawHandle<'a> {}

// Texture2D Stuff

pub struct RaylibTextureMode<'a, T>(&'a T, &'a mut ffi::RenderTexture2D);
impl<'a, T> Drop for RaylibTextureMode<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndTextureMode() }
    }
}
impl<'a, T> std::ops::Deref for RaylibTextureMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibTextureModeExt
where
    Self: Sized,
{
    #[must_use]
    fn begin_texture_mode<'a>(
        &'a mut self,
        _: &RaylibThread,
        framebuffer: &'a mut ffi::RenderTexture2D,
    ) -> RaylibTextureMode<Self> {
        unsafe { ffi::BeginTextureMode(*framebuffer) }
        RaylibTextureMode(self, framebuffer)
    }
}

// Only the DrawHandle and the RaylibHandle can start a texture
impl<'a> RaylibTextureModeExt for RaylibDrawHandle<'a> {}
impl RaylibTextureModeExt for &mut RaylibHandle {}
impl<'a, T> RaylibDraw for RaylibTextureMode<'a, T> {}

// VR Stuff

pub struct RaylibVRMode<'a, T>(&'a T, &'a mut VrStereoConfig);
impl<'a, T> Drop for RaylibVRMode<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndVrStereoMode() }
    }
}
impl<'a, T> std::ops::Deref for RaylibVRMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibVRModeExt
where
    Self: Sized,
{
    #[must_use]
    fn begin_vr_stereo_mode<'a>(
        &'a mut self,
        vr_config: &'a mut VrStereoConfig,
    ) -> RaylibVRMode<Self> {
        unsafe { ffi::BeginVrStereoMode(*vr_config.as_ref()) }
        RaylibVRMode(self, vr_config)
    }
}

impl<D: RaylibDraw> RaylibVRModeExt for D {}
impl<'a, T> RaylibDraw for RaylibVRMode<'a, T> {}

// 2D Mode

pub struct RaylibMode2D<'a, T>(&'a mut T);
impl<'a, T> Drop for RaylibMode2D<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndMode2D() }
    }
}
impl<'a, T> std::ops::Deref for RaylibMode2D<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibMode2DExt
where
    Self: Sized,
{
    #[allow(non_snake_case)]
    #[must_use]
    fn begin_mode2D(&mut self, camera: impl Into<ffi::Camera2D>) -> RaylibMode2D<Self> {
        unsafe {
            ffi::BeginMode2D(camera.into());
        }
        RaylibMode2D(self)
    }
}

impl<D: RaylibDraw> RaylibMode2DExt for D {}
impl<'a, T> RaylibDraw for RaylibMode2D<'a, T> {}

// 3D Mode

pub struct RaylibMode3D<'a, T>(&'a mut T);
impl<'a, T> Drop for RaylibMode3D<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndMode3D() }
    }
}
impl<'a, T> std::ops::Deref for RaylibMode3D<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibMode3DExt
where
    Self: Sized,
{
    #[allow(non_snake_case)]
    #[must_use]
    fn begin_mode3D(&mut self, camera: impl Into<ffi::Camera3D>) -> RaylibMode3D<Self> {
        unsafe {
            ffi::BeginMode3D(camera.into());
        }
        RaylibMode3D(self)
    }
}

impl<D: RaylibDraw> RaylibMode3DExt for D {}
impl<'a, T> RaylibDraw for RaylibMode3D<'a, T> {}
impl<'a, T> RaylibDraw3D for RaylibMode3D<'a, T> {}

// shader Mode

pub struct RaylibShaderMode<'a, T>(&'a mut T, &'a ffi::Shader);
impl<'a, T> Drop for RaylibShaderMode<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndShaderMode() }
    }
}
impl<'a, T> std::ops::Deref for RaylibShaderMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibShaderModeExt
where
    Self: Sized,
{
    #[must_use]
    fn begin_shader_mode<'a>(&'a mut self, shader: &'a ffi::Shader) -> RaylibShaderMode<Self> {
        unsafe { ffi::BeginShaderMode(*shader) }
        RaylibShaderMode(self, shader)
    }
}

impl<D: RaylibDraw> RaylibShaderModeExt for D {}
impl<'a, T> RaylibDraw for RaylibShaderMode<'a, T> {}
impl<'a, T> RaylibDraw3D for RaylibShaderMode<'a, T> {}

// Blend Mode

pub struct RaylibBlendMode<'a, T>(&'a mut T);
impl<'a, T> Drop for RaylibBlendMode<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndBlendMode() }
    }
}
impl<'a, T> std::ops::Deref for RaylibBlendMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibBlendModeExt
where
    Self: Sized,
{
    #[must_use]
    fn begin_blend_mode(&mut self, blend_mode: crate::consts::BlendMode) -> RaylibBlendMode<Self> {
        unsafe { ffi::BeginBlendMode((blend_mode as u32) as i32) }
        RaylibBlendMode(self)
    }
}

impl<D: RaylibDraw> RaylibBlendModeExt for D {}
impl<'a, T> RaylibDraw for RaylibBlendMode<'a, T> {}
impl<'a, T> RaylibDraw3D for RaylibBlendMode<'a, T> {}

// Scissor Mode stuff

pub struct RaylibScissorMode<'a, T>(&'a mut T);
impl<'a, T> Drop for RaylibScissorMode<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::EndScissorMode() }
    }
}
impl<'a, T> std::ops::Deref for RaylibScissorMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait RaylibScissorModeExt
where
    Self: Sized,
{
    #[must_use]
    fn begin_scissor_mode(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> RaylibScissorMode<Self> {
        unsafe { ffi::BeginScissorMode(x, y, width, height) }
        RaylibScissorMode(self)
    }
}

impl<D: RaylibDraw> RaylibScissorModeExt for D {}
impl<'a, T> RaylibDraw for RaylibScissorMode<'a, T> {}
impl<'a, T: RaylibDraw3D> RaylibDraw3D for RaylibScissorMode<'a, T> {}

// Actual drawing functions

pub trait RaylibDraw {
    /// Sets background color (framebuffer clear color).
    #[inline]
    fn clear_background(&mut self, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ClearBackground(color.into());
        }
    }

    /// Define default texture used to draw shapes
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
    fn draw_pixel_v(&mut self, position: impl Into<ffi::Vector2>, color: impl Into<ffi::Color>) {
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
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
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
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
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
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLineBezier(start_pos.into(), end_pos.into(), thick, color.into());
        }
    }
    /// Draw line using quadratic bezier curves with a control point
    #[inline]
    fn draw_line_bezier_quad(
        &mut self,
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
        control_pos: impl Into<ffi::Vector2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLineBezierQuad(
                start_pos.into(),
                end_pos.into(),
                control_pos.into(),
                thick,
                color.into(),
            );
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
        center: impl Into<ffi::Vector2>,
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
        center: impl Into<ffi::Vector2>,
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
        center: impl Into<ffi::Vector2>,
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
        center: impl Into<ffi::Vector2>,
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
        center: impl Into<ffi::Vector2>,
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
        position: impl Into<ffi::Vector2>,
        size: impl Into<ffi::Vector2>,
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
        origin: impl Into<ffi::Vector2>,
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
    /// Draws rectangle outline with extended parameters.
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

    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_rounded_lines(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        roundness: f32,
        segments: i32,
        line_thickness: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleRoundedLines(
                rec.into(),
                roundness,
                segments,
                line_thickness,
                color.into(),
            );
        }
    }

    /// Draws a triangle.
    #[inline]
    fn draw_triangle(
        &mut self,
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
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
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
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
        center: impl Into<ffi::Vector2>,
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
        center: impl Into<ffi::Vector2>,
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
        position: impl Into<ffi::Vector2>,
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
        position: impl Into<ffi::Vector2>,
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
        position: impl Into<ffi::Vector2>,
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

    /// Draw texture quad with tiling and offset parameters
    #[inline]
    fn draw_texture_quad(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        tiling: impl Into<ffi::Vector2>,
        offset: impl Into<ffi::Vector2>,
        quad: impl Into<ffi::Rectangle>,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTextureQuad(
                *texture.as_ref(),
                tiling.into(),
                offset.into(),
                quad.into(),
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
        origin: impl Into<ffi::Vector2>,
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

    /// Draw part of a texture (defined by a rectangle) with rotation and scale tiled into dest.
    #[inline]
    fn draw_texture_tiled(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        source_rec: impl Into<ffi::Rectangle>,
        dest_rec: impl Into<ffi::Rectangle>,
        origin: impl Into<ffi::Vector2>,
        rotation: f32,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTextureTiled(
                *texture.as_ref(),
                source_rec.into(),
                dest_rec.into(),
                origin.into(),
                rotation,
                scale,
                tint.into(),
            )
        }
    }

    ///Draws a texture (or part of it) that stretches or shrinks nicely
    #[inline]
    fn draw_texture_n_patch(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        n_patch_info: impl Into<ffi::NPatchInfo>,
        dest_rec: impl Into<ffi::Rectangle>,
        origin: impl Into<ffi::Vector2>,
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

    ///Draws a texture (or part of it) that stretches or shrinks nicely
    #[inline]
    fn draw_texture_poly(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        center: impl Into<ffi::Vector2>,
        points: &[Vector2],
        texcoords: &[Vector2],
        tint: impl Into<ffi::Color>,
    ) {
        assert!(points.len() == texcoords.len());
        unsafe {
            ffi::DrawTexturePoly(
                *texture.as_ref(),
                center.into(),
                points.as_ptr() as *mut _,
                texcoords.as_ptr() as *mut _,
                points.len() as _,
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

    /// Draws text using `font` and additional parameters.
    #[inline]
    fn draw_text_ex(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<ffi::Vector2>,
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

    /// Draw one character (codepoint)
    #[inline]
    fn draw_text_codepoint(
        &mut self,
        font: impl AsRef<ffi::Font>,
        codepoint: i32,
        position: impl Into<ffi::Vector2>,
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
}

pub trait RaylibDraw3D {
    /// Draw a point in 3D space, actually a small line
    #[allow(non_snake_case)]
    #[inline]
    fn draw_point3D(&mut self, position: impl Into<ffi::Vector3>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawPoint3D(position.into(), color.into());
        }
    }

    ///// Draw a color-filled triangle (vertex in counter-clockwise order!)
    #[allow(non_snake_case)]
    #[inline]
    fn draw_triangle3D(
        &mut self,
        v1: impl Into<ffi::Vector3>,
        v2: impl Into<ffi::Vector3>,
        v3: impl Into<ffi::Vector3>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTriangle3D(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// // Draw a triangle strip defined by points
    #[allow(non_snake_case)]
    #[inline]
    fn draw_triangle_strip3D(&mut self, points: &[Vector3], color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawTriangleStrip3D(points.as_ptr() as *mut _, points.len() as i32, color.into());
        }
    }

    /// Draws a line in 3D world space.
    #[inline]
    #[allow(non_snake_case)]
    fn draw_line_3D(
        &mut self,
        start_pos: impl Into<ffi::Vector3>,
        end_pos: impl Into<ffi::Vector3>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLine3D(start_pos.into(), end_pos.into(), color.into());
        }
    }

    /// Draws a circle in 3D world space.
    #[inline]
    #[allow(non_snake_case)]
    fn draw_circle_3D(
        &mut self,
        center: impl Into<ffi::Vector3>,
        radius: f32,
        rotation_axis: impl Into<ffi::Vector3>,
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
        position: impl Into<ffi::Vector3>,
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
        position: impl Into<ffi::Vector3>,
        size: impl Into<ffi::Vector3>,
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
        position: impl Into<ffi::Vector3>,
        width: f32,
        height: f32,
        length: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCubeWires(position.into(), width, height, length, color.into());
        }
    }

    /// Draws a textured cube.
    #[inline]
    fn draw_cube_texture(
        &mut self,
        texture: &Texture2D,
        position: impl Into<ffi::Vector3>,
        width: f32,
        height: f32,
        length: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCubeTexture(
                texture.0,
                position.into(),
                width,
                height,
                length,
                color.into(),
            );
        }
    }

    /// Draws a sphere.
    #[inline]
    fn draw_sphere(
        &mut self,
        center_pos: impl Into<ffi::Vector3>,
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
        center_pos: impl Into<ffi::Vector3>,
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
        center_pos: impl Into<ffi::Vector3>,
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
        position: impl Into<ffi::Vector3>,
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

    /// Draws a cylinder in wireframe.
    #[inline]
    fn draw_cylinder_wires(
        &mut self,
        position: impl Into<ffi::Vector3>,
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

    /// Draws an X/Z plane.
    #[inline]
    fn draw_plane(
        &mut self,
        center_pos: impl Into<ffi::Vector3>,
        size: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawPlane(center_pos.into(), size.into(), color.into());
        }
    }

    /// Draws a ray line.
    #[inline]
    fn draw_ray(&mut self, ray: Ray, color: impl Into<ffi::Color>) {
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
        position: impl Into<ffi::Vector3>,
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
        position: impl Into<ffi::Vector3>,
        rotation_axis: impl Into<ffi::Vector3>,
        rotation_angle: f32,
        scale: impl Into<ffi::Vector3>,
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
        position: impl Into<ffi::Vector3>,
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
        position: impl Into<ffi::Vector3>,
        rotation_axis: impl Into<ffi::Vector3>,
        rotation_angle: f32,
        scale: impl Into<ffi::Vector3>,
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
        center: impl Into<ffi::Vector3>,
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
        camera: Camera3D,
        texture: &Texture2D,
        source_rec: impl Into<ffi::Rectangle>,
        center: impl Into<ffi::Vector3>,
        size: impl Into<ffi::Vector2>,
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
}
