pub use raylib::prelude::*;

pub mod example;

fn main() {
    let screen_width = 800;
    let screen_height = 640;
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Showcase")
        // .vsync()
        .build();

    rl.set_exit_key(None);
    example::others::rlgl_standalone::run(&mut rl, &thread);
    // example::controls_test_suite::controls_test_suite::run(&mut rl, &thread);

    // let samples = &[example::core::core_2d_camera::run];
    // let mut sample = None;

    // let mut run = example::core::core_2d_camera::run(&mut rl, &thread);
    // while !rl.window_should_close() {
    //     match &mut sample {
    //         None => {
    //             let mut init = None;
    //             {
    //                 let mut d = rl.begin_drawing(&thread);
    //                 d.clear_background(Color::WHITE);
    //                 if d.gui_button(rrect(400, 320, 100, 30), "Core") {
    //                     init = Some(samples[0]);
    //                 }
    //             }
    //             match init {
    //                 Some(i) => sample = Some(i(&mut rl, &thread)),
    //                 _ => {}
    //             }
    //         }

    //         Some(ref mut run) => {
    //             (*run)(&mut rl, &thread);
    //             if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
    //                 sample = None
    //             }
    //         }
    //     }
    // }
}
