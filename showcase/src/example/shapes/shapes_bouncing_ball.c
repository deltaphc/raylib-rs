/*******************************************************************************************
*
*   raylib [shapes] example - bouncing ball
*
*   This example has been created using raylib 1.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2013 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{
    // Initialization
    //---------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shapes] example - bouncing ball");


    let ballPosition = rvec2(rl.get_screen_width() / 2, rl.get_screen_height() / 2);
    let ballSpeed = rvec2(5.0, 4.0);
    int ballRadius = 20;

    bool pause = 0;
    int framesCounter = 0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //----------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //-----------------------------------------------------
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
            pause = !pause;

        if !pause
        {
            ballPosition.x += ballSpeed.x;
            ballPosition.y += ballSpeed.y;

            // Check walls collision for bouncing
            if (ballPosition.x >= (rl.get_screen_width() - ballRadius)) || (ballPosition.x <= ballRadius)
                ballSpeed.x *= -1.0;
            if (ballPosition.y >= (rl.get_screen_height() - ballRadius)) || (ballPosition.y <= ballRadius)
                ballSpeed.y *= -1.0;
        }
        else
            framesCounter+=1;
        //-----------------------------------------------------

        // Draw
        //-----------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_circle_v(ballPosition, ballRadius, Color::MAROON);
        d.draw_text("PRESS SPACE to PAUSE BALL MOVEMENT", 10, rl.get_screen_height() - 25, 20, Color::LIGHTGRAY);

        // On pause, we draw a blinking message
        if pause && ((framesCounter / 30) % 2)
            d.draw_text("PAUSED", 350, 200, 30, Color::GRAY);

        d.draw_fps(10, 10);

        EndDrawing();
        //-----------------------------------------------------
    }

    // De-Initialization
    //---------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //----------------------------------------------------------

    return 0;
}