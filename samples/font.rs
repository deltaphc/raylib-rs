extern crate raylib;
use raylib::prelude::*;

fn main() {
    let w = 800;
    let h = 450;
    let rust_orange = Color::new(222, 165, 132, 255);
    let ray_white = Color::new(255, 255, 255, 255);
    let (mut rl, thread) = raylib::init().size(w, h).title("Logo").build();
    rl.set_target_fps(60);
    let font = rl
        .load_font(&thread, "static/alagard.png")
        .expect("couldn't load font");
    while !rl.window_should_close() {
        rl.start_drawing(&thread, |mut d| {
            d.clear_background(ray_white);
            d.draw_rectangle(w / 2 - 128, h / 2 - 128, 256, 256, rust_orange);
            d.draw_rectangle(w / 2 - 112, h / 2 - 112, 224, 224, ray_white);
            d.draw_text_ex(
                &font,
                "rust",
                Vector2::new((w / 2 - 69) as f32, (h / 2 + 18) as f32),
                50.0,
                1.0,
                rust_orange,
            );
            d.draw_text_ex(
                &font,
                "raylib",
                Vector2::new((w / 2 - 44) as f32, (h / 2 + 48) as f32),
                50.0,
                1.0,
                rust_orange,
            );
            // rl.take_screenshot(&thread, "logo.png");
        });
    }
}
