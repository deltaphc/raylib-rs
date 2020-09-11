/*******************************************************************************************
*
*   raylib [core] example - Scissor test
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Dill (@MysteriousSpace) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Chris Dill (@MysteriousSpace)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [core] example - scissor test");
    rl.set_window_size(screen_width, screen_height);

    let mut scissorArea = rrect(0, 0, 300, 300);
    let mut scissorMode = true;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        use raylib::consts::KeyboardKey::*;
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_key_pressed(KEY_S) {
            scissorMode = !scissorMode;
        }

        // Centre the scissor area around the mouse position
        scissorArea.x = rl.get_mouse_x() as f32 - scissorArea.width / 2.0;
        scissorArea.y = rl.get_mouse_y() as f32 - scissorArea.height / 2.0;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if scissorMode {
            let mut d = d.begin_scissor_mode(
                scissorArea.x as i32,
                scissorArea.y as i32,
                scissorArea.width as i32,
                scissorArea.height as i32,
            );
            // Draw full screen rectangle and some text
            // NOTE: Only part defined by scissor area will be rendered
            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::RED,
            );
            d.draw_text(
                "Move the mouse around to reveal this text!",
                190,
                200,
                20,
                Color::LIGHTGRAY,
            );
        } else {
            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::RED,
            );
            d.draw_text(
                "Move the mouse around to reveal this text!",
                190,
                200,
                20,
                Color::LIGHTGRAY,
            );
        }

        d.draw_rectangle_lines_ex(scissorArea, 1, Color::BLACK);
        d.draw_text("Press S to toggle scissor test", 10, 10, 20, Color::BLACK);

        //----------------------------------------------------------------------------------
    });
}
