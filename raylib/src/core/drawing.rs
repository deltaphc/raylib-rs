//! Contains code related to drawing. Types that can be set as a surface to draw will implement the [`RaylibDraw`] trait
use raylib_sys::Camera;

use super::{
    shaders::Shader,
    texture::{RenderTexture2D, Texture2D},
    vr::VrStereoConfig,
    RaylibHandle,
};
use crate::ffi::{
    self, BlendMode, BoundingBox, Camera2D, Camera3D, Color, NPatchInfo, Rectangle, Vector2,
    Vector3,
};

use std::{cell::Cell, convert::AsRef, ffi::CString, marker::PhantomData};

/// Holds the state of "special" Begin/End modes.
#[derive(Clone, Default)]
pub(crate) struct RaylibDrawState {
    texture: bool,
    vr: bool,
    camera2d: bool,
    camera3d: bool,
    shader: bool,
    blend: bool,
    scissor: bool,
}

pub struct RaylibDrawHandle<'bind>(
    pub(crate) PhantomData<&'bind RaylibHandle<'bind>>,
    pub(crate) Cell<RaylibDrawState>,
);

impl RaylibDrawHandle<'_> {
    pub fn begin_texture<F: FnOnce()>(&self, target: &mut RenderTexture2D, func: F) {
        let mut state = self.1.take();

        if state.texture {
            panic!("Nested begin_texture occured !");
        }

        state.texture = true;
        self.1.set(state.clone());

        unsafe { ffi::BeginTextureMode(target.0) };
        func();
        unsafe { ffi::EndTextureMode() };

        state.texture = false;
        self.1.set(state);
    }

    pub fn begin_vr<F: FnOnce()>(&self, config: &VrStereoConfig, func: F) {
        let mut state = self.1.take();

        if state.vr {
            panic!("Nested begin_vr occured !");
        }

        state.vr = true;
        self.1.set(state.clone());

        unsafe { ffi::BeginVrStereoMode(config.0) };
        func();
        unsafe { ffi::EndVrStereoMode() };

        state.vr = false;
        self.1.set(state);
    }

    pub fn begin_camera_2d<F: FnOnce()>(&self, camera: &Camera2D, func: F) {
        let mut state = self.1.take();

        if state.camera2d {
            panic!("Nested begin_camera_2d occured !");
        }

        state.camera2d = true;
        self.1.set(state.clone());

        unsafe { ffi::BeginMode2D(*camera) };
        func();
        unsafe { ffi::EndMode2D() };

        state.camera2d = false;
        self.1.set(state);
    }

    pub fn begin_camera_3d<F: FnOnce()>(&self, camera: &Camera, func: F) {
        let mut state = self.1.take();

        if state.camera3d {
            panic!("Nested begin_camera_3d occured !");
        }

        state.camera3d = true;
        self.1.set(state.clone());

        unsafe { ffi::BeginMode3D(*camera) };
        func();
        unsafe { ffi::EndMode3D() };

        state.camera3d = false;
        self.1.set(state);
    }

    pub fn begin_shader<F: FnOnce()>(&self, shader: &Shader, func: F) {
        let mut state = self.1.take();

        if state.shader {
            panic!("Nested begin_shader occured !");
        }

        state.shader = true;
        self.1.set(state.clone());

        unsafe { ffi::BeginShaderMode(shader.0) };
        func();
        unsafe { ffi::EndShaderMode() };

        state.shader = false;
        self.1.set(state);
    }

    pub fn begin_blend<F: FnOnce()>(&self, blend_mode: BlendMode, func: F) {
        let mut state = self.1.take();

        if state.blend {
            panic!("Nested begin_blend occured !");
        }

        state.blend = true;
        self.1.set(state.clone());

        unsafe { ffi::BeginBlendMode(blend_mode as _) };
        func();
        unsafe { ffi::EndBlendMode() };

        state.blend = false;
        self.1.set(state);
    }

    pub fn begin_scissors<F: FnOnce()>(&self, x: i32, y: i32, width: i32, height: i32, func: F) {
        let mut state = self.1.take();

        if state.scissor {
            panic!("Nested begin_scissors occured !");
        }

        state.scissor = true;
        self.1.set(state.clone());

        unsafe { ffi::BeginScissorMode(x, y, width, height) };
        func();
        unsafe { ffi::EndScissorMode() };

        state.scissor = false;
        self.1.set(state);
    }
}

impl RaylibDraw for RaylibDrawHandle<'_> {}

// Actual drawing functions
pub trait RaylibDraw {
    /// Sets background color (framebuffer clear color).
    #[inline]
    fn clear_background(&self, color: Color) {
        unsafe { ffi::ClearBackground(color) }
    }

    /// Define default texture used to draw shapes
    fn set_shapes_texture(&self, texture: impl AsRef<ffi::Texture2D>, source: Rectangle) {
        unsafe { ffi::SetShapesTexture(*texture.as_ref(), source) }
    }

    // // Draw gui widget
    // fn draw_gui<G: crate::rgui::GuiDraw>(&self, widget: G) -> crate::rgui::DrawResult {
    //     widget.draw()
    // }

    // SHAPES
    /// Draws a pixel.
    #[inline]
    fn draw_pixel(&self, x: i32, y: i32, color: Color) {
        unsafe { ffi::DrawPixel(x, y, color) }
    }

    /// Draws a pixel (Vector version).
    #[inline]
    fn draw_pixel_v(&self, position: impl Into<Vector2>, color: Color) {
        unsafe { ffi::DrawPixelV(position.into(), color) }
    }

    /// Draws a line.
    #[inline]
    fn draw_line(
        &self,
        start_pos_x: i32,
        start_pos_y: i32,
        end_pos_x: i32,
        end_pos_y: i32,
        color: Color,
    ) {
        unsafe { ffi::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color) }
    }

    /// Draws a line (Vector version).
    #[inline]
    fn draw_line_v(
        &self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        color: Color,
    ) {
        unsafe { ffi::DrawLineV(start_pos.into(), end_pos.into(), color) }
    }

    /// Draws a line with thickness.
    #[inline]
    fn draw_line_ex(
        &self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawLineEx(start_pos.into(), end_pos.into(), thick, color) }
    }

    /// Draws a line using cubic-bezier curves in-out.
    #[inline]
    fn draw_line_bezier(
        &self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawLineBezier(start_pos.into(), end_pos.into(), thick, color) }
    }
    /// Draw line using quadratic bezier curves with a control point
    #[inline]
    fn draw_line_bezier_quad(
        &self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        control_pos: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawLineBezierQuad(
                start_pos.into(),
                end_pos.into(),
                control_pos.into(),
                thick,
                color,
            )
        }
    }

    /// Draw line using cubic bezier curves with 2 control points
    #[inline]
    fn draw_line_bezier_cubic(
        &self,
        start_pos: impl Into<Vector2>,
        end_pos: impl Into<Vector2>,
        start_control_pos: impl Into<Vector2>,
        end_control_pos: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawLineBezierCubic(
                start_pos.into(),
                end_pos.into(),
                start_control_pos.into(),
                end_control_pos.into(),
                thick,
                color,
            )
        }
    }

    /// Draw lines sequence
    #[inline]
    fn draw_line_strip(&self, points: &[Vector2], color: Color) {
        unsafe { ffi::DrawLineStrip(points.as_ptr() as *mut Vector2, points.len() as i32, color) }
    }

    /// Draws a color-filled circle.
    #[inline]
    fn draw_circle(&self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        unsafe { ffi::DrawCircle(center_x, center_y, radius, color) }
    }

    /// Draw a piece of a circle
    #[inline]
    fn draw_circle_sector(
        &self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCircleSector(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments,
                color,
            )
        }
    }

    /// Draw circle sector outline
    #[inline]
    fn draw_circle_sector_lines(
        &self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCircleSectorLines(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments,
                color,
            )
        }
    }

    /// Draws a gradient-filled circle.
    #[inline]
    fn draw_circle_gradient(
        &self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { ffi::DrawCircleGradient(center_x, center_y, radius, color1, color2) }
    }

    /// Draws a color-filled circle (Vector version).
    #[inline]
    fn draw_circle_v(&self, center: impl Into<Vector2>, radius: f32, color: Color) {
        unsafe { ffi::DrawCircleV(center.into(), radius, color) }
    }

    /// Draws circle outline.
    #[inline]
    fn draw_circle_lines(&self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        unsafe { ffi::DrawCircleLines(center_x, center_y, radius, color) }
    }

    /// Draws ellipse.
    #[inline]
    fn draw_ellipse(
        &self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawEllipse(center_x, center_y, radius_h, radius_v, color) }
    }

    /// Draws ellipse.
    #[inline]
    fn draw_ellipse_lines(
        &self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawEllipseLines(center_x, center_y, radius_h, radius_v, color) }
    }

    /// Draw ring
    #[inline]
    fn draw_ring(
        &self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawRing(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color,
            )
        }
    }

    /// Draw ring lines
    #[inline]
    fn draw_ring_lines(
        &self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawRingLines(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color,
            )
        }
    }

    /// Draws a color-filled rectangle.
    #[inline]
    fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe { ffi::DrawRectangle(x, y, width, height, color) }
    }

    /// Draws a color-filled rectangle (Vector version).
    #[inline]
    fn draw_rectangle_v(
        &self,
        position: impl Into<Vector2>,
        size: impl Into<Vector2>,
        color: Color,
    ) {
        unsafe { ffi::DrawRectangleV(position.into(), size.into(), color) }
    }

    /// Draws a color-filled rectangle from `rec`.
    #[inline]
    fn draw_rectangle_rec(&self, rec: Rectangle, color: Color) {
        unsafe { ffi::DrawRectangleRec(rec, color) }
    }

    /// Draws a color-filled rectangle with pro parameters.
    #[inline]
    fn draw_rectangle_pro(
        &self,
        rec: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawRectanglePro(rec, origin.into(), rotation, color) }
    }

    /// Draws a vertical-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    fn draw_rectangle_gradient_v(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { ffi::DrawRectangleGradientV(x, y, width, height, color1, color2) }
    }

    /// Draws a horizontal-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    fn draw_rectangle_gradient_h(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { ffi::DrawRectangleGradientH(x, y, width, height, color1, color2) }
    }

    /// Draws a gradient-filled rectangle with custom vertex colors.
    ///
    /// **NOTE**: Colors refer to corners, starting at top-left corner and going counter-clockwise.
    #[inline]
    fn draw_rectangle_gradient_ex(
        &self,
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    ) {
        unsafe { ffi::DrawRectangleGradientEx(rec, col1, col2, col3, col4) }
    }

    /// Draws rectangle outline.
    #[inline]
    fn draw_rectangle_lines(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe { ffi::DrawRectangleLines(x, y, width, height, color) }
    }

    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_lines_ex(&self, rec: Rectangle, line_thick: f32, color: Color) {
        unsafe { ffi::DrawRectangleLinesEx(rec, line_thick, color) }
    }
    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_rounded(&self, rec: Rectangle, roundness: f32, segments: i32, color: Color) {
        unsafe { ffi::DrawRectangleRounded(rec, roundness, segments, color) }
    }

    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_rounded_lines(
        &self,
        rec: Rectangle,
        roundness: f32,
        segments: i32,
        line_thickness: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawRectangleRoundedLines(rec, roundness, segments, line_thickness, color) }
    }

    /// Draws a triangle.
    #[inline]
    fn draw_triangle(
        &self,
        v1: impl Into<Vector2>,
        v2: impl Into<Vector2>,
        v3: impl Into<Vector2>,
        color: Color,
    ) {
        unsafe { ffi::DrawTriangle(v1.into(), v2.into(), v3.into(), color) }
    }

    /// Draws a triangle using lines.
    #[inline]
    fn draw_triangle_lines(
        &self,
        v1: impl Into<Vector2>,
        v2: impl Into<Vector2>,
        v3: impl Into<Vector2>,
        color: Color,
    ) {
        unsafe { ffi::DrawTriangleLines(v1.into(), v2.into(), v3.into(), color) }
    }

    /// Draw a triangle fan defined by points.
    #[inline]
    fn draw_triangle_fan(&self, points: &[Vector2], color: Color) {
        unsafe { ffi::DrawTriangleFan(points.as_ptr() as *mut Vector2, points.len() as i32, color) }
    }

    /// Draw a triangle strip defined by points
    #[inline]
    fn draw_triangle_strip(&self, points: &[Vector2], color: Color) {
        unsafe {
            ffi::DrawTriangleStrip(points.as_ptr() as *mut Vector2, points.len() as i32, color)
        }
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    fn draw_poly(
        &self,
        center: impl Into<Vector2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawPoly(center.into(), sides, radius, rotation, color) }
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    fn draw_poly_lines(
        &self,
        center: impl Into<Vector2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawPolyLines(center.into(), sides, radius, rotation, color) }
    }

    #[inline]
    fn draw_poly_lines_ex(
        &self,
        center: impl Into<Vector2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        line_thick: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawPolyLinesEx(center.into(), sides, radius, rotation, line_thick, color) }
    }

    /// Draws a `texture` using specified position and `tint` color.
    #[inline]
    fn draw_texture(&self, texture: impl AsRef<ffi::Texture2D>, x: i32, y: i32, tint: Color) {
        unsafe { ffi::DrawTexture(*texture.as_ref(), x, y, tint) }
    }

    /// Draws a `texture` using specified `position` vector and `tint` color.
    #[inline]
    fn draw_texture_v(
        &self,
        texture: impl AsRef<ffi::Texture2D>,
        position: impl Into<Vector2>,
        tint: Color,
    ) {
        unsafe { ffi::DrawTextureV(*texture.as_ref(), position.into(), tint) }
    }

    /// Draws a `texture` with extended parameters.
    #[inline]
    fn draw_texture_ex(
        &self,
        texture: impl AsRef<ffi::Texture2D>,
        position: impl Into<Vector2>,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        unsafe { ffi::DrawTextureEx(*texture.as_ref(), position.into(), rotation, scale, tint) }
    }

    /// Draws from a region of `texture` defined by the `source_rec` rectangle.
    #[inline]
    fn draw_texture_rec(
        &self,
        texture: impl AsRef<ffi::Texture2D>,
        source_rec: Rectangle,
        position: impl Into<Vector2>,
        tint: Color,
    ) {
        unsafe { ffi::DrawTextureRec(*texture.as_ref(), source_rec, position.into(), tint) }
    }

    /// Draw a part of a texture defined by a rectangle with 'pro' parameters.
    #[inline]
    fn draw_texture_pro(
        &self,
        texture: impl AsRef<ffi::Texture2D>,
        source: Rectangle,
        dest: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTexturePro(
                *texture.as_ref(),
                source,
                dest,
                origin.into(),
                rotation,
                tint,
            )
        }
    }

    ///Draws a texture (or part of it) that stretches or shrinks nicely
    #[inline]
    fn draw_texture_n_patch(
        &self,
        texture: impl AsRef<ffi::Texture2D>,
        n_patch_info: NPatchInfo,
        dest_rec: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawTextureNPatch(
                *texture.as_ref(),
                n_patch_info,
                dest_rec,
                origin.into(),
                rotation,
                tint,
            )
        }
    }

    /// Shows current FPS.
    #[inline]
    fn draw_fps(&self, x: i32, y: i32) {
        unsafe { ffi::DrawFPS(x, y) }
    }

    /// Draws text (using default font).
    #[inline]
    fn draw_text(&self, text: &str, x: i32, y: i32, font_size: i32, color: Color) {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::DrawText(c_text.as_ptr(), x, y, font_size, color) }
    }

    /// Draws text using `font` and additional parameters.
    #[inline]
    fn draw_text_ex(
        &self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<Vector2>,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::DrawTextEx(
                *font.as_ref(),
                c_text.as_ptr(),
                position.into(),
                font_size,
                spacing,
                tint,
            )
        }
    }

    /// Draw one character (codepoint)
    #[inline]
    fn draw_text_codepoint(
        &self,
        font: impl AsRef<ffi::Font>,
        codepoint: i32,
        position: impl Into<Vector2>,
        scale: f32,
        tint: Color,
    ) {
        unsafe { ffi::DrawTextCodepoint(*font.as_ref(), codepoint, position.into(), scale, tint) }
    }

    /// Draw a point in 3D space, actually a small line
    #[inline]
    fn draw_point_3d(&self, position: Vector3, color: Color) {
        unsafe { ffi::DrawPoint3D(position, color) }
    }

    ///// Draw a color-filled triangle (vertex in counter-clockwise order!)
    #[inline]
    fn draw_triangle_3d(&self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        unsafe { ffi::DrawTriangle3D(v1, v2, v3, color) }
    }

    /// // Draw a triangle strip defined by points
    #[inline]
    fn draw_triangle_strip_3d(&self, points: &[Vector3], color: Color) {
        unsafe { ffi::DrawTriangleStrip3D(points.as_ptr() as *mut _, points.len() as i32, color) }
    }

    /// Draws a line in 3D world space.
    #[inline]
    fn draw_line_3d(&self, start_pos: Vector3, end_pos: Vector3, color: Color) {
        unsafe { ffi::DrawLine3D(start_pos, end_pos, color) }
    }

    /// Draws a circle in 3D world space.
    #[inline]
    fn draw_circle_3d(
        &self,
        center: Vector3,
        radius: f32,
        rotation_axis: Vector3,
        rotation_angle: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawCircle3D(center, radius, rotation_axis, rotation_angle, color) }
    }

    /// Draws a cube.
    #[inline]
    fn draw_cube(&self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { ffi::DrawCube(position, width, height, length, color) }
    }

    /// Draws a cube (Vector version).
    #[inline]
    fn draw_cube_v(&self, position: Vector3, size: Vector3, color: Color) {
        unsafe { ffi::DrawCubeV(position, size, color) }
    }

    /// Draws a cube in wireframe.
    #[inline]
    fn draw_cube_wires(
        &self,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        unsafe { ffi::DrawCubeWires(position, width, height, length, color) }
    }

    /// Draws a sphere.
    #[inline]
    fn draw_sphere(&self, center_pos: Vector3, radius: f32, color: Color) {
        unsafe { ffi::DrawSphere(center_pos, radius, color) }
    }

    /// Draws a sphere with extended parameters.
    #[inline]
    fn draw_sphere_ex(
        &self,
        center_pos: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        unsafe { ffi::DrawSphereEx(center_pos, radius, rings, slices, color) }
    }

    /// Draws a sphere in wireframe.
    #[inline]
    fn draw_sphere_wires(
        &self,
        center_pos: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        unsafe { ffi::DrawSphereWires(center_pos, radius, rings, slices, color) }
    }

    /// Draws a cylinder.
    #[inline]
    fn draw_cylinder(
        &self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        unsafe { ffi::DrawCylinder(position, radius_top, radius_bottom, height, slices, color) }
    }

    /// Draws a cylinder in wireframe.
    #[inline]
    fn draw_cylinder_wires(
        &self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinderWires(position, radius_top, radius_bottom, height, slices, color)
        }
    }

    /// Draw a cylinder wires with base at startPos and top at endPos
    #[inline]
    fn draw_cylinder_wires_ex(
        &self,
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        sides: i32,
        color: Color,
    ) {
        unsafe {
            ffi::DrawCylinderWiresEx(start_pos, end_pos, start_radius, end_radius, sides, color)
        }
    }

    /// Draws an X/Z plane.
    #[inline]
    fn draw_plane(&self, center_pos: Vector3, size: impl Into<Vector2>, color: Color) {
        unsafe { ffi::DrawPlane(center_pos, size.into(), color) }
    }

    /// Draws a ray line.
    #[inline]
    fn draw_ray(&self, ray: ffi::Ray, color: Color) {
        unsafe { ffi::DrawRay(ray, color) }
    }

    /// Draws a grid (centered at (0, 0, 0)).
    #[inline]
    fn draw_grid(&self, slices: i32, spacing: f32) {
        unsafe { ffi::DrawGrid(slices, spacing) }
    }

    /// Draws a model (with texture if set).
    #[inline]
    fn draw_model(
        &self,
        model: impl AsRef<ffi::Model>,
        position: Vector3,
        scale: f32,
        tint: Color,
    ) {
        unsafe { ffi::DrawModel(*model.as_ref(), position, scale, tint) }
    }

    /// Draws a model with extended parameters.
    #[inline]
    fn draw_model_ex(
        &self,
        model: impl AsRef<ffi::Model>,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawModelEx(
                *model.as_ref(),
                position,
                rotation_axis,
                rotation_angle,
                scale,
                tint,
            )
        }
    }

    /// Draws a model with wires (with texture if set).
    #[inline]
    fn draw_model_wires(
        &self,
        model: impl AsRef<ffi::Model>,
        position: Vector3,
        scale: f32,
        tint: Color,
    ) {
        unsafe { ffi::DrawModelWires(*model.as_ref(), position, scale, tint) }
    }

    /// Draws a model with wires.
    #[inline]
    fn draw_model_wires_ex(
        &self,
        model: impl AsRef<ffi::Model>,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        unsafe {
            ffi::DrawModelWiresEx(
                *model.as_ref(),
                position,
                rotation_axis,
                rotation_angle,
                scale,
                tint,
            )
        }
    }

    /// Draws a bounding box (wires).
    #[inline]
    fn draw_bounding_box(&self, bbox: BoundingBox, color: Color) {
        unsafe { ffi::DrawBoundingBox(bbox, color) }
    }

    /// Draws a billboard texture.
    #[inline]
    fn draw_billboard(
        &self,
        camera: Camera3D,
        texture: &Texture2D,
        center: Vector3,
        size: f32,
        tint: Color,
    ) {
        unsafe { ffi::DrawBillboard(camera, texture.0, center, size, tint) }
    }

    /// Draws a billboard texture defined by `source_rec`.
    #[inline]
    fn draw_billboard_rec(
        &self,
        camera: Camera3D,
        texture: &Texture2D,
        source_rec: Rectangle,
        center: Vector3,
        size: impl Into<Vector2>,
        tint: Color,
    ) {
        unsafe { ffi::DrawBillboardRec(camera, texture.0, source_rec, center, size.into(), tint) }
    }
}
