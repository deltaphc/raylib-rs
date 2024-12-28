use raylib::prelude::*;

macro_rules! add_menu_color {
    ($ui:tt, $name:tt, $src:tt, $dst:tt) => {
        if $ui.menu_item($name) {
            *$dst.lock().unwrap() = Color::$src;
        }
    };
}

use std::sync::Mutex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init().width(640).height(480).build();

    while (!rl.window_should_close()) {
        let mut d = rl.begin_drawing(&thread);

        d.start_imgui(|ui| {
            ui.window("A Window").build(|| {
                // imgui functions
            });
        });
    }

    Ok(())
}
