//! Contains code related to drawing. Types that can be set as a surface to draw will implement the RaylibDraw trait
use crate::core::*;
use crate::ffi;
use std::ops::{Deref, DerefMut};

/// Seems like all draw commands must be issued from the main thread
impl RaylibHandle {
    /// Setup canvas (framebuffer) to start drawing
    pub fn begin_drawing(self, _: &RaylibThread) -> RaylibDrawHandle {
        unsafe {
            ffi::BeginDrawing();
        }
        RaylibDrawHandle { inner: self }
    }
    /// Pass a function for drawing
    pub fn with_draw<F: Fn(&mut RaylibDrawHandle)>(&mut self, thread: &RaylibThread, drawfn: F) {
        // I'm 99% sure this is safe. If anyone thinks it's not open an issue.
        let clone = RaylibHandle(());
        let mut d = clone.begin_drawing(thread);
        drawfn(&mut d);
        let clone = d.end_drawing();
        std::mem::forget(clone);
    }
}

pub struct RaylibDrawHandle {
    inner: RaylibHandle,
}

impl RaylibDrawHandle {
    pub fn end_drawing(self) -> RaylibHandle {
        unsafe {
            ffi::EndDrawing();
        }
        self.inner
    }
    #[allow(non_snake_case)]
    fn begin_mode_2D(
        &mut self,
        camera: impl Into<ffi::Camera2D>,
    ) -> RaylibMode2D<RaylibDrawHandle> {
        unsafe {
            ffi::BeginMode2D(camera.into());
        }
        RaylibMode2D { inner: self }
    }
}

impl Deref for RaylibDrawHandle {
    type Target = RaylibHandle;

    fn deref(&self) -> &RaylibHandle {
        &self.inner
    }
}

impl DerefMut for RaylibDrawHandle {
    fn deref_mut(&mut self) -> &mut RaylibHandle {
        &mut self.inner
    }
}

pub struct RaylibMode2D<'a, T> {
    inner: &'a mut T,
}

impl<'a, T> Drop for RaylibMode2D<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::EndMode2D();
        }
    }
}

impl<'a, T> RaylibDraw for RaylibMode2D<'a, T> {}
impl RaylibDraw for RaylibDrawHandle {}

/// TODO figure out if you can draw 2D things in 3D mode and vice versa
pub trait RaylibDraw {
    /// Sets background color (framebuffer clear color).
    #[inline]
    fn clear_background(&mut self, color: impl Into<Color>) {
        unsafe {
            ffi::ClearBackground(color.into().into());
        }
    }

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
    fn draw_pixel_v(&self, position: impl Into<ffi::Vector2>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawPixelV(position.into(), color.into());
        }
    }

    /// Draws a line.
    #[inline]
    fn draw_line(
        &self,
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
        &self,
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
        &self,
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
        &self,
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawLineBezier(start_pos.into(), end_pos.into(), thick, color.into());
        }
    }

    /// Draws a color-filled circle.
    #[inline]
    fn draw_circle(&self, center_x: i32, center_y: i32, radius: f32, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawCircle(center_x, center_y, radius, color.into());
        }
    }
    /// Draw a piece of a circle
    #[inline]
    fn draw_circle_sector(
        &self,
        center: impl Into<ffi::Vector2>,
        radius: f32,
        start_angle: i32,
        end_angle: i32,
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
        &self,
        center: impl Into<ffi::Vector2>,
        radius: f32,
        start_angle: i32,
        end_angle: i32,
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
        &self,
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
        &self,
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
        &self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawCircleLines(center_x, center_y, radius, color.into());
        }
    }

    /// Draw ring
    #[inline]
    fn draw_ring(
        &self,
        center: impl Into<ffi::Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: i32,
        end_angle: i32,
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
        &self,
        center: impl Into<ffi::Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: i32,
        end_angle: i32,
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
        &self,
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
        &self,
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
    fn draw_rectangle_rec(&self, rec: impl Into<ffi::Rectangle>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::DrawRectangleRec(rec.into(), color.into());
        }
    }

    /// Draws a color-filled rectangle with pro parameters.
    #[inline]
    fn draw_rectangle_pro(
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
        rec: impl Into<ffi::Rectangle>,
        line_thick: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawRectangleLinesEx(rec.into(), line_thick, color.into());
        }
    }
    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_rounded(
        &self,
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
        &self,
        rec: impl Into<ffi::Rectangle>,
        roundness: f32,
        segments: i32,
        line_thickness: i32,
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
        &self,
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
        &self,
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::DrawTriangleLines(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    fn draw_poly(
        &self,
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

    // TODO update library with new poly funcs
    // /// Draw a closed polygon defined by points
    // #[inline]
    // fn draw_poly_ex(
    //     &self,
    //     points: impl AsRef<[ffi::Vector2]>,
    //     color: impl Into<ffi::Color>,
    // ) {
    //     unsafe {
    //         ffi::DrawPolyEx(points.as_ref().as_ptr(), points.as_ref().len(), color.into());
    //     }
    // }

    // /// Draw polygon lines
    // #[inline]
    // fn draw_poly_ex_lines(
    //     &self,
    //     points: impl AsRef<[ffi::Vector2]>,
    //     color: impl Into<ffi::Color>,
    // ) {
    //     unsafe {
    //         ffi::DrawPolyExLines(points.as_ref().as_ptr(), points.as_ref().len(), color.into());
    //     }
    // }

    /// Draws text (using default font).
    #[inline]
    fn draw_text(&self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<Color>) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::DrawText(c_text.as_ptr(), x, y, font_size, color.into().into());
        }
    }
}

#[cfg(test)]
mod draw_test {
    use crate::core::*;
    use crate::tests::*;
    ray_draw_test!(test_pixel);
    fn test_pixel(d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);
        d.draw_pixel(10, 10, Color::RED);
        d.draw_pixel_v(Vector2::new(20.0, 20.0), Color::RED);
    }
    ray_draw_test!(test_line);
    fn test_line(d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);
        d.draw_line(0, 5, 100, 5, Color::RED);
        d.draw_line_v(
            Vector2::new(0.0, 100.0),
            Vector2::new(100.0, 100.0),
            Color::BLUE,
        );
        d.draw_line_ex(
            Vector2::new(0.0, 200.0),
            Vector2::new(100.0, 200.0),
            10.0,
            Color::GREEN,
        );
        d.draw_line_bezier(
            Vector2::new(0.0, 300.0),
            Vector2::new(100.0, 400.0),
            10.0,
            Color::ORANGE,
        );
    }
    ray_draw_test!(test_circle);
    fn test_circle(d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);
        d.draw_circle(20, 20, 10.0, Color::RED);
        d.draw_circle_v(Vector2::new(40.0, 20.0), 10.0, Color::RED);
        d.draw_circle_lines(60, 20, 10.0, Color::RED);
        d.draw_circle_sector(Vector2::new(80.0, 20.0), 10.0, 0, 90, 5, Color::RED);
        d.draw_circle_sector_lines(Vector2::new(100.0, 20.0), 10.0, 0, 90, 5, Color::RED);
        d.draw_circle_gradient(1200, 20, 10.0, Color::RED, Color::GREEN);
        d.draw_ring(Vector2::new(40.0, 80.0), 10.0, 20.0, 0, 180, 5, Color::RED);
        d.draw_ring_lines(Vector2::new(80.0, 80.0), 10.0, 20.0, 0, 180, 5, Color::RED);
    }

    ray_draw_test!(test_rectangle);
    fn test_rectangle(d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);
        d.draw_rectangle(10, 10, 10, 10, Color::RED);
        d.draw_rectangle_v(
            Vector2::new(20.0, 10.0),
            Vector2::new(10.0, 10.0),
            Color::GREEN,
        );
        d.draw_rectangle_rec(Rectangle::new(30.0, 10.0, 10.0, 10.0), Color::BLUE);
        d.draw_rectangle_pro(
            Rectangle::new(40.0, 10.0, 10.0, 10.0),
            Vector2::new(5.0, 5.0),
            45.0,
            Color::ORANGE,
        );
        d.draw_rectangle_gradient_v(60, 10, 10, 10, Color::RED, Color::GREEN);
        d.draw_rectangle_gradient_h(70, 10, 10, 10, Color::RED, Color::GREEN);
        d.draw_rectangle_gradient_ex(
            Rectangle::new(80.0, 10.0, 10.0, 10.0),
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::WHITE,
        );
        d.draw_rectangle_lines(90, 10, 10, 10, Color::RED);
        d.draw_rectangle_lines_ex(Rectangle::new(100.0, 10.0, 10.0, 10.0), 3, Color::GREEN);
        d.draw_rectangle_rounded(
            Rectangle::new(110.0, 30.0, 100.0, 100.0),
            0.1,
            5,
            Color::BLUE,
        );
        d.draw_rectangle_rounded_lines(
            Rectangle::new(220.0, 30.0, 100.0, 100.0),
            0.10,
            5,
            3,
            Color::ORANGE,
        );
    }

    ray_draw_test!(test_triangle);
    fn test_triangle(d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);
        d.draw_triangle(
            Vector2::new(0.0, 30.0),
            Vector2::new(15.0, 0.0),
            Vector2::new(30.0, 30.0),
            Color::RED,
        );
        d.draw_triangle_lines(
            Vector2::new(30.0, 30.0),
            Vector2::new(45.0, 0.0),
            Vector2::new(60.0, 30.0),
            Color::GREEN,
        );
    }

    ray_draw_test!(test_poly);
    fn test_poly(d: &mut RaylibDrawHandle) {
        // let pentagon = vec![
        //     Vector2::new(20.0, 0.0),
        //     Vector2::new(30.0, 10.0),
        //     Vector2::new(30.0, 20.0),
        //     Vector2::new(0.0, 20.0),
        //     Vector2::new(0.0, 10.0)
        // ];

        // let pentagon2 = vec![
        //     Vector2::new(20.0, 100.0),
        //     Vector2::new(30.0, 110.0),
        //     Vector2::new(30.0, 120.0),
        //     Vector2::new(0.0, 120.0),
        //     Vector2::new(0.0, 110.0)
        // ];
        d.clear_background(Color::WHITE);
        d.draw_poly(Vector2::new(100.0, 100.0), 12, 20.0, 45.0, Color::RED);
        // d.draw_poly_ex(&pentagon, pentagon.len(), Color::GREEN);
        // d.draw_poly_ex_lines(&pentagon2, pentagon.len(), Color::GREEN);
    }
}
