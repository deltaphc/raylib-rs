pub use raylib::prelude::*;

pub mod example;

type SampleOut = Box<dyn for<'a> FnMut(&'a mut RaylibHandle, &'a RaylibThread) -> ()>;
type Sample = fn(&mut RaylibHandle, &RaylibThread) -> SampleOut;

fn main() {
    let title = "Showcase";
    let screen_width = 800;
    let screen_height = 640;
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title(title)
        .vsync()
        .msaa_4x()
        .build();

    rl.set_exit_key(None);

    let samples: &[(&std::ffi::CStr, Sample)] = &[
        (rstr!("Core2D Camera"), example::core::core_2d_camera::run),
        (
            rstr!("Core2D Camera Platformer"),
            example::core::core_2d_camera_platformer::run,
        ),
        (
            rstr!("raygui - controls test suite"),
            example::controls_test_suite::controls_test_suite::run,
        ),
        (
            rstr!("raylib [models] example - pbr material"),
            example::models::models_material_pbr::run,
        ),
        (
            rstr!("rlgl standalone"),
            example::others::rlgl_standalone::run,
        ),
        (
            rstr!("raylib [textures] example - bunnymark"),
            example::textures::textures_bunnymark::run,
        ),
        (
            rstr!("raylib [models] example - model animation"),
            example::models::models_animation::run,
        ),
    ];
    let mut sample = None;
    let mut list_view_active = -1;
    let mut list_view_focus = -1;
    let mut list_view_scroll_index = -1;

    let box_length = (50 * samples.len() as i32).min(500);
    let y_margin = (screen_height - box_length) / 2;

    while !rl.window_should_close() {
        match &mut sample {
            None => {
                let mut to_run = None;
                {
                    let mut d = rl.begin_drawing(&thread);
                    d.clear_background(Color::WHITE);

                    let list: Vec<_> = samples.iter().map(|(s, _)| *s).collect();

                    list_view_active = d.gui_list_view_ex(
                        rrect(200.0, y_margin, 400, box_length),
                        list.as_slice(),
                        &mut list_view_focus,
                        &mut list_view_scroll_index,
                        list_view_active,
                    );

                    if list_view_active >= 0 {
                        to_run.replace(samples[list_view_active as usize].1);
                    }
                }

                match to_run {
                    Some(run) => sample = Some(run(&mut rl, &thread)),
                    _ => {}
                }
            }

            Some(ref mut run) => {
                (*run)(&mut rl, &thread);
                if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
                    sample = None;
                    rl.set_window_size(screen_width, screen_height);
                    rl.set_window_title(&thread, title);
                    list_view_active = -1;
                }
            }
        }
    }
}
