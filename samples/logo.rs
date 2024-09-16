extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Logo");
    let (w, h) = (opt.width, opt.height);
    let rust_orange = Color::new(222, 165, 132, 255);
    let ray_white = Color::new(255, 255, 255, 255);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Detect window close button or ESC key

        rl.start_drawing(&thread, |mut d| {
            d.clear_background(ray_white);
            d.draw_rectangle(w / 2 - 128, h / 2 - 128, 256, 256, rust_orange);
            d.draw_rectangle(w / 2 - 112, h / 2 - 112, 224, 224, ray_white);
            d.draw_text("rust", w / 2 - 69, h / 2 + 18, 50, rust_orange);
            d.draw_text("raylib", w / 2 - 44, h / 2 + 48, 50, rust_orange);
        });
    }
}
