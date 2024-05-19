#[cfg(test)]
mod draw_test {
    use crate::tests::*;
    use raylib::prelude::*;
    ray_draw_test!(test_pixel);
    fn test_pixel(d: &mut RaylibDrawHandle, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_pixel(10, 10, Color::RED);
        d.draw_pixel_v(Vector2::new(20.0, 20.0), Color::RED);
    }
    ray_draw_test!(test_line);
    fn test_line(d: &mut RaylibDrawHandle, _: &TestAssets) {
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
    fn test_circle(d: &mut RaylibDrawHandle, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_circle(20, 20, 10.0, Color::RED);
        d.draw_circle_v(Vector2::new(40.0, 20.0), 10.0, Color::RED);
        d.draw_circle_lines(60, 20, 10.0, Color::RED);
        d.draw_circle_sector(Vector2::new(80.0, 20.0), 10.0, 0.0, 90.0, 5, Color::RED);
        d.draw_circle_sector_lines(Vector2::new(100.0, 20.0), 10.0, 0.0, 90.0, 5, Color::RED);
        d.draw_circle_gradient(1200, 20, 10.0, Color::RED, Color::GREEN);
        d.draw_ring(
            Vector2::new(40.0, 80.0),
            10.0,
            20.0,
            0.0,
            180.0,
            5,
            Color::RED,
        );
        d.draw_ring_lines(
            Vector2::new(80.0, 80.0),
            10.0,
            20.0,
            0.0,
            180.0,
            5,
            Color::RED,
        );
    }

    ray_draw_test!(test_rectangle);
    fn test_rectangle(d: &mut RaylibDrawHandle, _: &TestAssets) {
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
        d.draw_rectangle_lines_ex(Rectangle::new(100.0, 10.0, 10.0, 10.0), 3.0, Color::GREEN);
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
            3.0,
            Color::ORANGE,
        );
    }

    ray_draw_test!(test_triangle);
    fn test_triangle(d: &mut RaylibDrawHandle, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_triangle(
            Vector2::new(30.0, 30.0),
            Vector2::new(0.0, 30.0),
            Vector2::new(15.0, 0.0),
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
    fn test_poly(d: &mut RaylibDrawHandle, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_poly(Vector2::new(100.0, 100.0), 12, 20.0, 45.0, Color::RED);
    }

    ray_draw_test!(test_spline);
    fn test_spline(d: &mut RaylibDrawHandle, _: &TestAssets) {
        d.draw_spline_linear(
            &vec![
                Vector2::new(10.0, 10.0),
                Vector2::new(20.0, 50.0),
                Vector2::new(30.0, 30.0),
            ],
            2.0,
            Color::BLACK,
        );
        d.draw_spline_basis(
            &vec![
                Vector2::new(10.0, 10.0),
                Vector2::new(20.0, 50.0),
                Vector2::new(30.0, 30.0),
            ],
            2.0,
            Color::BLACK,
        );
        d.draw_spline_catmull_rom(
            &vec![
                Vector2::new(10.0, 10.0),
                Vector2::new(20.0, 50.0),
                Vector2::new(30.0, 30.0),
            ],
            2.0,
            Color::BLACK,
        );
        d.draw_spline_bezier_quadratic(
            &vec![
                Vector2::new(10.0, 10.0),
                Vector2::new(20.0, 50.0),
                Vector2::new(30.0, 30.0),
            ],
            2.0,
            Color::BLACK,
        );
        d.draw_spline_bezier_cubic(
            &vec![
                Vector2::new(10.0, 10.0),
                Vector2::new(20.0, 50.0),
                Vector2::new(30.0, 30.0),
            ],
            2.0,
            Color::BLACK,
        );
        d.draw_spline_segment_linear(
            Vector2::new(10.0, 10.0),
            Vector2::new(20.0, 50.0),
            2.0,
            Color::BLACK,
        );
        d.draw_spline_segment_basis(
            Vector2::new(10.0, 10.0),
            Vector2::new(20.0, 50.0),
            Vector2::new(30.0, 30.0),
            Vector2::new(30.0, 30.0),
            2.0,
            Color::BLACK,
        );
        d.draw_spline_segment_catmull_rom(
            Vector2::new(10.0, 10.0),
            Vector2::new(20.0, 50.0),
            Vector2::new(30.0, 30.0),
            Vector2::new(30.0, 30.0),
            2.0,
            Color::BLACK,
        );
        d.draw_spline_segment_bezier_quadratic(
            Vector2::new(10.0, 10.0),
            Vector2::new(20.0, 50.0),
            Vector2::new(30.0, 30.0),
            2.0,
            Color::BLACK,
        );
        d.draw_spline_segment_bezier_cubic(
            Vector2::new(10.0, 10.0),
            Vector2::new(20.0, 50.0),
            Vector2::new(30.0, 30.0),
            Vector2::new(10.0, 10.0),
            2.0,
            Color::BLACK,
        );
    }

    ray_3d_draw_test!(test_draw_mesh);
    fn test_draw_mesh(
        d: &mut RaylibMode3D<RaylibDrawHandle>,
        thread: &RaylibThread,
        _: &TestAssets,
    ) {
        let mesh = Mesh::gen_mesh_sphere(&thread, 25.0, 5, 5);
        let material = d.load_material_default(&thread);

        d.draw_mesh(mesh, material, Matrix::translate(0.0, 0.0, 0.0));
    }
}
