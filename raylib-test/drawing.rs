
#[cfg(test)]
mod draw_test {
    use raylib::prelude::*;
    use crate::tests::*;
    ray_draw_test!(test_pixel);
    fn test_pixel(d: &mut RaylibDrawHandle<RaylibHandle>, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_pixel(10, 10, Color::RED);
        d.draw_pixel_v(Vector2::new(20.0, 20.0), Color::RED);
    }
    ray_draw_test!(test_line);
    fn test_line(d: &mut RaylibDrawHandle<RaylibHandle>, _: &TestAssets) {
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
    fn test_circle(d: &mut RaylibDrawHandle<RaylibHandle>, _: &TestAssets) {
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
    fn test_rectangle(d: &mut RaylibDrawHandle<RaylibHandle>, _: &TestAssets) {
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
    fn test_triangle(d: &mut RaylibDrawHandle<RaylibHandle>, _: &TestAssets) {
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
    fn test_poly(d: &mut RaylibDrawHandle<RaylibHandle>, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_poly(Vector2::new(100.0, 100.0), 12, 20.0, 45.0, Color::RED);
    }
}
