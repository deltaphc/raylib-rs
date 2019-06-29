use raylib::prelude::*;
use std::ffi::CString;

mod options;

pub fn main() {
    let opt = options::Opt::new();
    let (mut rl, thread) = opt.open_window("Camera 2D");
    let (w, h) = (opt.width, opt.height);

    let wb = raylib::rgui::WindowBox {
        bounds: Rectangle::new(0.0, 0.0, 100.0, 100.0),
        text: CString::new("Hello world").unwrap(),
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_gui(&wb);
        d.draw_icon(
            raylib::consts::rIconDescription::RICON_FILETYPE_AUDIO,
            Vector2::new(50.0, 50.0),
            8,
            Color::RED,
        );
    }
}
