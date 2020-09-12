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


    Vector2 ballPosition = {Getscreen_width() / 2, Getscreen_height() / 2};
    Vector2 ballSpeed = {5.0, 4.0};
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
        if (IsKeyPressed(KEY_SPACE))
            pause = !pause;

        if (!pause)
        {
            ballPosition.x += ballSpeed.x;
            ballPosition.y += ballSpeed.y;

            // Check walls collision for bouncing
            if ((ballPosition.x >= (Getscreen_width() - ballRadius)) || (ballPosition.x <= ballRadius))
                ballSpeed.x *= -1.0;
            if ((ballPosition.y >= (Getscreen_height() - ballRadius)) || (ballPosition.y <= ballRadius))
                ballSpeed.y *= -1.0;
        }
        else
            framesCounter++;
        //-----------------------------------------------------

        // Draw
        //-----------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        DrawCircleV(ballPosition, ballRadius, Color::MAROON);
        d.draw_text("PRESS SPACE to PAUSE BALL MOVEMENT", 10, Getscreen_height() - 25, 20, Color::LIGHTGRAY);

        // On pause, we draw a blinking message
        if (pause && ((framesCounter / 30) % 2))
            d.draw_text("PAUSED", 350, 200, 30, GRAY);

        DrawFPS(10, 10);

        EndDrawing();
        //-----------------------------------------------------
    }

    // De-Initialization
    //---------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //----------------------------------------------------------

    return 0;
}