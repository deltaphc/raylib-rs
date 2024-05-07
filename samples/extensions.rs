extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

trait RaylibDrawExt: RaylibDraw {
    fn custom_draw(&mut self, font: &WeakFont) {
        self.draw_text_ex(font, "custom", rvec2(0, 0), 16.0, 0.0, Color::GREEN);
    }
}

impl<T> RaylibDrawExt for T where T: RaylibDraw {}

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Logo");
    let font = rl.get_font_default();
    while !rl.window_should_close() {
        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.custom_draw(&font);
    }
}
