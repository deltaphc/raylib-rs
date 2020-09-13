#![allow(non_snake_case)]

pub use raylib::prelude::*;

pub mod example;

type SampleOut = Box<dyn for<'a> FnMut(&'a mut RaylibHandle, &'a RaylibThread) -> ()>;
type Sample = fn(&mut RaylibHandle, &RaylibThread) -> SampleOut;

use std::cell::RefCell;
thread_local! (static APP: RefCell<Option<Box<dyn FnMut() -> bool>>> = RefCell::new(None));

pub const EXIT_KEY: raylib::consts::KeyboardKey = raylib::consts::KeyboardKey::KEY_ESCAPE;

fn main() {
    // Set the emscripten main loop before setting up raylib so that raylib has something
    // to configure
    // #[cfg(target_arch = "wasm32")]
    // unsafe {
    //     wasm::emscripten_set_main_loop(wasm::_nothing_wasm, 0, 1);
    // }

    let title = "Showcase";
    let screen_width = 800;
    let screen_height = 640;
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title(title)
        .resizable()
        .vsync()
        .msaa_4x()
        .build();

    rl.set_exit_key(None);

    let samples: Vec<(&std::ffi::CStr, Sample)> = vec![
        (rstr!("Core2D Camera"), example::core::core_2d_camera::run),
        (
            rstr!("Core2D Camera Platformer"),
            example::core::core_2d_camera_platformer::run,
        ),
        (
            rstr!("raylib [core] example - 3d camera first person"),
            example::core::core_3d_camera_first_person::run,
        ),
        (
            rstr!("raylib [core] example - 3d camera free"),
            example::core::core_3d_camera_free::run,
        ),
        (
            rstr!("raylib [core] example - 3d camera mode"),
            example::core::core_3d_camera_mode::run,
        ),
        (
            rstr!("raylib [core] example - 3d picking"),
            example::core::core_3d_picking::run,
        ),
        (
            rstr!("raylib [core] example - basic window"),
            example::core::core_basic_window::run,
        ),
        (
            rstr!("raylib [core] example - custom logging"),
            example::core::core_custom_logging::run,
        ),
        (
            rstr!("raylib [core] example - drop files"),
            example::core::core_drop_files::run,
        ),
        (
            rstr!("raylib [core] example - gamepad input"),
            example::core::core_input_gamepad::run,
        ),
        (
            rstr!("raylib [core] example - input gestures"),
            example::core::core_input_gestures::run,
        ),
        (
            rstr!("raylib [core] example - keyboard input"),
            example::core::core_input_keys::run,
        ),
        (
            rstr!("raylib [core] example - input mouse wheel"),
            example::core::core_input_mouse_wheel::run,
        ),
        (
            rstr!("raylib [core] example - mouse input"),
            example::core::core_input_mouse::run,
        ),
        (
            rstr!("raylib [core] example - input multitouch"),
            example::core::core_input_multitouch::run,
        ),
        (
            rstr!("raylib [core] example - generate random values"),
            example::core::core_random_values::run,
        ),
        (
            rstr!("raylib [core] example - window scale letterbox"),
            example::core::core_window_letterbox::run,
        ),
        (
            rstr!("raylib [core] example - core world screen"),
            example::core::core_world_screen::run,
        ),
        // VR is Buggy AF. Take a look at it
        // (
        //     rstr!("raylib [core] example - vr simulator"),
        //     example::core::core_vr_simulator::run,
        // ),
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
        (
            rstr!("raylib [core] example - scissor test"),
            example::core::core_scissor_test::run,
        ),
        (
            rstr!("raylib [audio] example - music playing (streaming)"),
            example::audio::audio_music_stream::run,
        ),
        (
            rstr!("raylib [shaders] example - postprocessing shader"),
            example::shaders::shaders_postprocessing::run,
        ),
        (
            rstr!("raylib [texture] example - texture rectangle"),
            example::textures::textures_rectangle::run,
        ),
        (
            rstr!("raylib [textures] example - mouse painting"),
            example::textures::textures_mouse_painting::run,
        ),
    ];
    let mut sample = None;
    let mut list_view_active = -1;
    let mut list_view_focus = -1;
    let mut list_view_scroll_index = -1;

    let box_length = (50 * samples.len() as i32).min(500);
    let y_margin = (screen_height - box_length) / 2;

    let frame: Box<dyn FnMut() -> bool> = Box::new(move || {
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
                if rl.is_key_down(EXIT_KEY) {
                    sample = None;
                    rl.set_window_size(screen_width, screen_height);
                    rl.set_window_title(&thread, title);
                    list_view_active = -1;
                }
            }
        };
        return rl.window_should_close();
    });

    APP.with(|app| {
        app.borrow_mut().replace(frame);
    });

    // absolutely NONE of this is necessary. You could use a while !update() {} loop in
    // wasm without any problems as long as you compile with ASYNCIFY.
    // This shows you how to do it using emscripten_set_main_loop.
    #[cfg(not(target_arch = "wasm32"))]
    {
        while !update() {}
    }
    #[cfg(target_arch = "wasm32")]
    unsafe {
        wasm::emscripten_set_main_loop(wasm::_update_wasm, 0, 1);
    }
}

fn update() -> bool {
    APP.with(|app| match *app.borrow_mut() {
        None => false,
        Some(ref mut frame) => frame(),
    })
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
mod wasm {
    use std::os::raw::{c_int, c_uchar};

    #[allow(non_camel_case_types)]
    type em_callback_func = unsafe extern "C" fn();

    extern "C" {
        // This extern is built in by Emscripten.
        pub fn emscripten_sample_gamepad_data();
        pub fn emscripten_run_script_int(x: *const c_uchar) -> c_int;
        pub fn emscripten_cancel_main_loop();
        pub fn emscripten_set_main_loop(
            func: em_callback_func,
            fps: c_int,
            simulate_infinite_loop: c_int,
        );
    }

    pub extern "C" fn _update_wasm() {
        super::update();
    }

    pub extern "C" fn _nothing_wasm() {}
}
