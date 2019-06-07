extern crate raylib;

fn main() {
    let w = 800;
    let h = 450;
    let rust_orange = raylib::Color::new(222, 165, 132, 255);
    let ray_white = raylib::Color::new(255, 255, 255, 255);
    let rl = raylib::init().size(w, h).title("Logo").build();

    while !rl.window_should_close() {
        rl.begin_drawing();
        rl.clear_background(ray_white);
        rl.draw_rectangle(w / 2 - 128, h / 2 - 128, 256, 256, rust_orange);
        rl.draw_rectangle(w / 2 - 112, h / 2 - 112, 224, 224, ray_white);
        rl.draw_text("rust", w / 2 - 69, h / 2 + 18, 50, rust_orange);
        rl.draw_text("raylib", w / 2 - 44, h / 2 + 48, 50, rust_orange);
        rl.end_drawing();
    }
}