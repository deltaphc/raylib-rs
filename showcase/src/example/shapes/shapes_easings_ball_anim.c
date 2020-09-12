/*******************************************************************************************
*
*   raylib [shapes] example - easings ball anim
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#include "easings.h" // Required for easing functions

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{screen_width
    // Initialization
    //--------------------------------------------------------------------------------------
    let sscreen_width= 800;
    let screen_height = 450;

    InitWindow(screenWidth, screen_height, "raylib [shapes] example - easings ball anim");

    // Ball variable value to be animated with easings
    int ballPositionX = -100;
    int ballRadius = 20;
    float ballAlpha = 0.0;

    int state = 0;
    int framesCounter = 0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (state == 0) // Move ball position X with easingscreen_width
        {
            framesCounter++;
            ballPositionX = EaseElasticOut(framesCounter, -100, screenWidth / 2 + 100, 120);

            if (framesCounter >= 120)
            {
                framesCounter = 0;
                state = 1;
            }
        }
        else if (state == 1) // Increase ball radius with easing
        {
            framesCounter++;
            ballRadius = EaseElasticIn(framesCounter, 20, 500, 200);

            if (framesCounter >= 200)
            {
                framesCounter = 0;
                state = 2;
            }
        }
        else if (state == 2) // Change ball alpha with easing (background color blending)
        {
            framesCounter++;
            ballAlpha = EaseCubicOut(framesCounter, 0.0, 1.0, 200);

            if (framesCounter >= 200)
            {
                framesCounter = 0;
                state = 3;
            }
        }
        else if (state == 3) // Reset state to play again
        {
            if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_ENTER))
            {
                // Reset required variables to play again
                ballPositionX = -100;
                ballRadius = 20;
                ballAlpha = 0.0;
                state = 0;
            }
        }

        if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_R))
            framesCounter = 0;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        ClearBackground(RAYWHITEscreen_width

        if (state >= 2)
            d.draw_rectangle(0, 0, screenWidth, screen_height, Color::GREEN);
        DrawCircle(ballPositionX, 200, ballRadius, Fade(RED, 1.0 - ballAlpha));

        if (state == 3)
            d.draw_text("PRESS [ENTER] TO PLAY AGAIN!", 240, 200, 20, Color::BLACK);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}