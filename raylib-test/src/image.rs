#[cfg(test)]
mod image_test {
    use crate::tests::*;
    use raylib::prelude::*;
    fn default_image() -> Image {
        Image::load_image("./resources/billboard.png").unwrap()
    }
    fn run_image_test<A>(thread: &RaylibThread, name: &str, func: &mut A)
    where
        A: FnMut(&mut Image) -> (),
    {
        run_image_test_w_create(&thread, name, &mut default_image, func)
    }
    fn run_image_test_w_create<A, B>(
        thread: &RaylibThread,
        name: &str,
        create_func: &mut B,
        func: &mut A,
    ) where
        A: FnMut(&mut Image) -> (),
        B: FnMut() -> Image,
    {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        let mut img = create_func();
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

    ray_test!(gen_image_gradient_linear);
    fn gen_image_gradient_linear(thread: &RaylibThread) {
        run_image_test_w_create(
            &thread,
            "gen_image_gradient_linear",
            &mut || {
                Image::gen_image_gradient_linear(
                    TEST_WIDTH,
                    TEST_HEIGHT,
                    45,
                    Color::RED,
                    Color::BLUE,
                )
            },
            &mut |_img| {},
        );
    }
    ray_test!(gen_image_gradient_square);
    fn gen_image_gradient_square(thread: &RaylibThread) {
        run_image_test_w_create(
            &thread,
            "gen_image_gradient_square",
            &mut || {
                Image::gen_image_gradient_square(
                    TEST_WIDTH,
                    TEST_HEIGHT,
                    0.1,
                    Color::RED,
                    Color::BLUE,
                )
            },
            &mut |_img| {},
        );
    }
    ray_test!(gen_image_text);
    fn gen_image_text(thread: &RaylibThread) {
        run_image_test_w_create(
            &thread,
            "gen_image_text",
            &mut || Image::gen_image_text(TEST_WIDTH / 5, TEST_HEIGHT / 5, "text image"),
            &mut |_img| {},
        );
    }
}
