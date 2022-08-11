/*******************************************************************************************
*
*   raylib [core] example - Mouse input
*
*   This example has been created using raylib 1.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - mouse input");

    let mut ball_color = Color::DARKBLUE;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        let ball_position = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
            {

                ball_color = Color::MAROON;
            }
        else if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON)
            {

                ball_color = Color::LIME;
            }
        else if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON)
            {

                ball_color = Color::DARKBLUE;
            }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_circle_v(ball_position, 40.0, ball_color);

        d.draw_text("move ball with mouse and click mouse button to change color", 10, 10, 20, Color::DARKGRAY);

        //----------------------------------------------------------------------------------
    },
    );
}
