extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Input");
    let (_w, _h) = (opt.width, opt.height);
    let _rust_orange = Color::new(222, 165, 132, 255);
    let _ray_white = Color::new(255, 255, 255, 255);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let pressed_key = rl.get_key_pressed();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        if let Some(pressed_key) = pressed_key {
            // Certain keyboards may have keys raylib does not expect. Uncomment this line if so.
            // let pressed_key: u32 = unsafe { std::mem::transmute(pressed_key) };
            d.draw_text(&format!("{:?}", pressed_key), 100, 12, 10, Color::BLACK);
        }
    }
}
