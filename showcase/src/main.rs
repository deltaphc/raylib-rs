pub mod example;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 640).title("Showcase").build();
    // let logo = raylib::prelude::Image::load_image("static/logo.png").unwrap();
    // rl.set_window_icon(&logo);
    // rl.set_target_fps(self.fps);

    let mut run = example::core::core_2d_camera::run(&mut rl, &thread);
    while !rl.window_should_close() {
        run(&mut rl, &thread);
    }
}
