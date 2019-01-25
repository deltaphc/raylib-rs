extern crate raylib;

use raylib::consts::*;
use raylib::Color;

fn main() {
    let rl = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    
    while !rl.window_should_close() {
        rl.begin_drawing();
        
        rl.clear_background(Color::WHITE);
        rl.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        rl.end_drawing();
    }
}