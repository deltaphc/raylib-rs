use raylib::prelude::*;
pub fn main() {
    let (mut rl, thread) = raylib::init().width(800).height(450).build();
    let im_red = Image::gen_image_color(800, 450, Color::new(255, 0, 0, 255));
    let tex_red = rl.load_texture_from_image(&thread, &im_red).unwrap();

    let im_blue = Image::gen_image_color(800, 450, Color::new(0, 0, 255, 255));
    let tex_blue = rl.load_texture_from_image(&thread, &im_blue).unwrap();

    let mut shader = rl.load_shader(&thread, None, Some("static/shader/color_mix.fs"));

    // Get an additional sampler2D location to be enabled on drawing
    let tex_blue_loc = shader.get_shader_location("texture1");

    // Get shader uniform for divider
    let divider_loc = shader.get_shader_location("divider");
    let mut divider_value = 0.5;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            divider_value += 0.01;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            divider_value -= 0.01;
        }

        if (divider_value < 0.0) {
            divider_value = 0.0;
        } else if (divider_value > 1.0) {
            divider_value = 1.0;
        };
        rl.start_drawing(&thread, |mut d| {
            d.start_shader_mode(&mut shader, |mut d, shader| {
                shader.set_shader_value(divider_loc, divider_value);
                shader.set_shader_value_texture(tex_blue_loc, &tex_blue);

                d.clear_background(Color::WHITE);
                d.draw_texture(&tex_red, 0, 0, Color::WHITE);
            });

            d.draw_text(
                "Use KEY_LEFT/KEY_RIGHT to move texture mixing in shader!",
                80,
                d.get_screen_height() - 40,
                20,
                Color::RAYWHITE,
            );
        })
    }
}
