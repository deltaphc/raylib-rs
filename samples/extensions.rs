extern crate raylib;
use nalgebra::Vector2;
use raylib::{core::text::Font, ffi::Color, prelude::*};
use structopt::StructOpt;

mod options;

trait RaylibDrawExt: RaylibDraw {
    fn custom_draw(&mut self, font: &Font) {
        self.draw_text_ex(
            font,
            "custom",
            Vector2::new(0.0, 0.0).into(),
            16.0,
            0.0,
            Color::GREEN,
        );
    }
}

impl<T> RaylibDrawExt for T where T: RaylibDraw {}

fn main() {
    let opt = options::Opt::from_args();
    let (rl, thread) = opt.open_window("Logo");
    let font = &rl.get_font_default();
    while !rl.window_should_close() {
        // Detect window close button or ESC key
        rl.frame(&thread, |mut d| {
            d.clear_background(Color::WHITE);
            d.custom_draw(&font);
        });
    }

    drop(rl);
}
