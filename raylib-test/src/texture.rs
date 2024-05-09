#[cfg(test)]
mod texture_test {
    use crate::tests::*;
    use raylib::prelude::*;
    #[test]
    fn test_image_loading() {
        let i = Image::load_image("resources/billboard.png").expect("image not found");
        Image::load_image("resources/doesnt_exist.png").expect_err("image found?");
        i.export_image("test_out/billboard.png");
        i.export_image_as_code("test_out/billboard_code.h");
    }

    ray_test!(test_texture_load);
    fn test_texture_load(thread: &RaylibThread) {
        let i =
            Image::load_image("resources/billboard.png").expect("could not load image billboard");
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let _ = rl
            .load_texture(thread, "resources/billboard.png")
            .expect("could not load texture billboard");
        let t = rl
            .load_texture_from_image(thread, &i)
            .expect("could not load texture from image");
        let _ = t
            .load_image()
            .expect("can't get an image from a texture created from an image...");
        i.export_image("test_out/billboard_texture.png");
    }

    ray_test!(test_render_texture);
    fn test_render_texture(t: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.load_render_texture(t, 256, 256)
            .expect("render texture created");
    }

    #[test]
    fn test_image_manipulations() {
        // Just checking that nothing segfaults. Not ensuring they work as expected.
        let mut col = Vec::new();
        let mut alpha = Vec::new();
        let mut blank = Vec::new();
        for i in 0..32 {
            for j in 0..32 {
                col.push(Color::RED);
                blank.push(Color::new(0, 0, 0, 0));
                if (i / 8) % 2 == (j / 8) % 2 {
                    alpha.push(Color::new(255, 255, 255, 255))
                } else {
                    alpha.push(Color::new(0, 0, 0, 0))
                }
            }
        }

        let mut i = Image::gen_image_color(32, 32, Color::RED);
        let mut canvas = Image::gen_image_color(32, 32, Color::BLANK);
        // let mask = Image::load_image_ex(&alpha, 32, 32).expect("failed to load alpha image");
        let mask = Image::gen_image_checked(32, 32, 8, 8, Color::WHITE, Color::BLANK);

        let mut c = i.clone();

        c.alpha_mask(&mask);
        c.alpha_clear(Color::BLUE, 0.5);
        // shouldn't do anything
        c.alpha_crop(0.5);
        // shouldn't do anything
        c.alpha_premultiply();
        let mut blurry = c.clone();
        blurry.resize(256, 256);
        blurry.export_image("test_out/chessboard_blurry.png");
        c.resize_nn(256, 256);
        i.resize_canvas(256, 256, 10, 10, Color::BLUE);
        i.export_image("test_out/resized.png");
        c.export_image("test_out/chessboard.png");
        c.mipmaps();
        blurry.dither(128, 128, 128, 128);
        let colors = c.extract_palette(100);
        assert_eq!(colors.len(), 2, "color palette extraction failed");
        canvas.draw(
            &i,
            Rectangle::new(0.0, 0.0, 20.0, 20.0),
            Rectangle::new(0.0, 0.0, 20.0, 20.0),
            Color::WHITE,
        );
        canvas.draw_rectangle_lines(Rectangle::new(20.0, 0.0, 20.0, 20.0), 4, Color::GREEN);
        let rec = Rectangle::new(40.0, 0.0, 20.0, 20.0);
        canvas.draw_rectangle(
            rec.x as i32,
            rec.y as i32,
            rec.width as i32,
            rec.height as i32,
            Color::ORANGE,
        );
        canvas.flip_vertical();
        canvas.flip_horizontal();
        canvas.rotate_cw();
        canvas.rotate_ccw();
        canvas.color_tint(Color::PINK);
        canvas.color_invert();
        canvas.color_contrast(0.5);
        canvas.color_brightness(128);
        canvas.color_replace(Color::GREEN, Color::RED);
        canvas.export_image("test_out/canvas.png");

        // Test generation functions
        /*let g = Image::gen_image_color(64, 64, Color::BLUE);
        g.export_image("test_out/generated_color.png");
        let g = Image::gen_image_gradient_v(64, 64, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_gradient_v.png");
        let g = Image::gen_image_gradient_h(64, 64, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_gradient_h.png");
        let g = Image::gen_image_gradient_radial(64, 64, 0.5, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_gradient_radial.png");
        let g = Image::gen_image_checked(64, 64, 8, 8, Color::RED, Color::BLUE);
        g.export_image("test_out/generated_checked.png");
        let g = Image::gen_image_white_noise(64, 64, 0.7);
        g.export_image("test_out/generated_white.png");
        let g = Image::gen_image_perlin_noise(64, 64, 0, 0, 0.7);
        g.export_image("test_out/generated_perlin.png");
        let g = Image::gen_image_cellular(64, 64, 4);
        g.export_image("test_out/generated_cellular.png");*/
    }
}
