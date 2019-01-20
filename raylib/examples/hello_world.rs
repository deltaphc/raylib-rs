use raylib;

fn main() {
    let rl = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        rl.begin_drawing();

        rl.clear_background(raylib::WHITE);
        rl.draw_text("Hello, world!", 12, 12, 20, raylib::BLACK);

        rl.end_drawing();
    }
}
