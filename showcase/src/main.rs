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
    ];
    let mut sample = None;
    let mut listViewActive = -1;
    let mut listViewFocus = -1;
    let mut listViewScrollIndex = -1;

    let boxLength = (50 * samples.len() as i32).min(500);
    let yMargin = (screen_height - boxLength) / 2;
    dbg!(boxLength);

    while !rl.window_should_close() {
        match &mut sample {
            None => {
                let mut toRun = None;
                {
                    let mut d = rl.begin_drawing(&thread);
                    d.clear_background(Color::WHITE);

                    let listViewExList: Vec<_> = samples.iter().map(|(s, _)| *s).collect();

                    listViewActive = d.gui_list_view_ex(
                        rrect(200.0, yMargin, 400, boxLength),
                        listViewExList.as_slice(),
                        &mut listViewFocus,
                        &mut listViewScrollIndex,
                        listViewActive,
                    );

                    if listViewActive >= 0 {
                        toRun.replace(samples[listViewActive as usize].1);
                    }
                }

                match toRun {
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
                    listViewActive = -1;
                }
            }
        }
    }
}
