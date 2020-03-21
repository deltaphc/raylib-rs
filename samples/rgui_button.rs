use raylib::prelude::*;
use std::ffi::CString;

mod options;

pub fn main() {
    let opt = options::Opt::new();
    let (mut rl, thread) = opt.open_window("Camera 2D");
    let (_w, _h) = (opt.width, opt.height);

    let mut wb = rgui::WindowBox {
        bounds: Rectangle::new(64.0, 64.0, 128.0, 72.0),
        text: CString::new("Hello World").unwrap(),
    };

    rl.set_target_fps(200);

    let btn = rgui::Button {
        bounds: Rectangle::new(72.0, 172.0 + 24.0, 64.0, 16.0),
        text: CString::new("Click Me").unwrap(),
    };

    let mut exit_program = false;

    while !exit_program && !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        if let rgui::DrawResult::Bool(b) = d.draw_gui(&wb) {
            if b {
                exit_program = true;
            }
        }

        if let rgui::DrawResult::Bool(b) = d.draw_gui(&btn) {
            if b {
                wb.bounds.x += 8.0;
            }
        }
    }
}
