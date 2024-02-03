#[cfg(test)]
mod image_test {
    use crate::tests::*;
    use raylib::prelude::*;
    fn run_image_test<A>(thread: &RaylibThread, name: &str, func: &mut A)
    where
        A: FnMut(&mut Image) -> (),
    {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let mut img = Image::load_image("./resources/billboard.png").unwrap();
        func(&mut img);

        let tex = rl.load_texture_from_image(&thread, &img).unwrap();
        {
            let mut d = rl.begin_drawing(&thread);

            d.draw_texture(&tex, 0, 0, Color::WHITE);
        }
        rl.take_screenshot(&thread, &format!("test_image_{}.png", name));
        {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
        }
    }

    ray_test!(image_blur);
    fn image_blur(thread: &RaylibThread) {
        run_image_test(&thread, "gaussian", &mut |img| {
            img.blur_gaussian(10);
        });
    }

    ray_test!(image_rotate);
    fn image_rotate(thread: &RaylibThread) {
        run_image_test(&thread, "rotate", &mut |img| {
            img.rotate(10);
        });
    }

    ray_test!(image_draw_circle_lines);
    fn image_draw_circle_lines(thread: &RaylibThread) {
        run_image_test(&thread, "draw_circle_lines", &mut |img| {
            img.draw_circle_lines(10, 10, 10, Color::RED);
        });
    }

    ray_test!(image_draw_circle_lines_v);
    fn image_draw_circle_lines_v(thread: &RaylibThread) {
        run_image_test(&thread, "draw_circle_lines_v", &mut |img| {
            img.draw_circle_lines_v(Vector2::new(10.0, 10.0), 10, Color::RED);
        });
    }
}
