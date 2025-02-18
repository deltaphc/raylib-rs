extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Texture");
    let (_w, _h) = (opt.width, opt.height);
    let t = rl
        .load_texture(&thread, "static/billboard.png")
        .expect("could not load texture billboard");

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&t, 0, 0, Color::WHITE);
    }
}
