#[cfg(test)]
mod text_test {
    use crate::tests::*;
    use raylib::prelude::*;
    ray_test!(test_font_load);
    fn test_font_load(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let _f = rl
            .load_font(thread, "resources/alagard.png")
            .expect("couldn't load font");
    }

    ray_test!(test_font_load_ex);
    fn test_font_load_ex(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let _f = rl
            .load_font_ex(thread, "resources/pixeloid.ttf", 32, None)
            .expect("couldn't load font");
    }

    ray_test!(test_font_export);
    fn test_font_export(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let f = rl
            .load_font(thread, "resources/alagard.png")
            .expect("couldn't load font");
        f.export_as_code("test_out/font.h");
    }

    ray_draw_test!(test_default_font);
    fn test_default_font(d: &mut RaylibDrawHandle, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_fps(0, 0);
        d.draw_text("Hello World", 100, 100, 32, Color::RED);
    }

    ray_draw_test!(test_custom_font);
    fn test_custom_font(d: &mut RaylibDrawHandle, assets: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_fps(0, 0);
        d.draw_text_ex(
            &assets.font,
            "Hello World",
            Vector2::new(100.0, 100.0),
            32.0,
            5.0,
            Color::RED,
        );
    }

    ray_draw_test!(test_custom_font_ex);
    fn test_custom_font_ex(d: &mut RaylibDrawHandle, assets: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_fps(0, 0);
        d.draw_text_ex(
            &assets.font_ex,
            "Hello World",
            Vector2::new(100.0, 100.0),
            32.0,
            5.0,
            Color::RED,
        );
    }
}
