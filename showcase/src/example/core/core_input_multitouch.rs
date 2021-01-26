/*******************************************************************************************
*
*   raylib [core] example - Input multitouch
*
*   This example has been created using raylib 2.1 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Berni (@Berni8k) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Berni (@Berni8k) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_TOUCH_POINTS: u32 = 0;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - input multitouch");

    let mut ball_position = rvec2(-100.0, -100.0);
    let mut ball_color = Color::BEIGE;

    let mut touch_counter = 0;
    let mut touch_position = Vector2::zero();

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        ball_position = rl.get_mouse_position();

        ball_color = Color::BEIGE;

        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
            {

                ball_color = Color::MAROON;
            }
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON)
            {

                ball_color = Color::LIME;
            }
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON)
            {

                ball_color = Color::DARKBLUE;
            }

        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
            {

                touch_counter = 10;
            }
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON)
            {

                touch_counter = 10;
            }
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON)
            {

                touch_counter = 10;
            }

        if touch_counter > 0
            {

                touch_counter-=1;
            }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        // Multitouch
        for i in 0..MAX_TOUCH_POINTS
        {
            touch_position = d.get_touch_position(i); // Get the touch point

            if (touch_position.x >= 0.0) && (touch_position.y >= 0.0) // Make sure point is not (-1,-1 as this means there is no touch for it
            {
                // Draw circle and touch index number
                d.draw_circle_v(touch_position, 34.0, Color::ORANGE);
                d.draw_text(&format!("{}", i), touch_position.x as i32 - 10, touch_position.y as i32 - 70, 40, Color::BLACK);
            }
        }

        // Draw the normal mouse location
        d.draw_circle_v(ball_position, 30.0 + (touch_counter * 3) as f32, ball_color);

        d.draw_text("move ball with mouse and click mouse button to change color", 10, 10, 20, Color::DARKGRAY);
        d.draw_text("touch the screen at multiple locations to get multiple balls", 10, 30, 20, Color::DARKGRAY);

    },
    );
}
