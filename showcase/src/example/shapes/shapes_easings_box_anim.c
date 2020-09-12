/*******************************************************************************************
*
*   raylib [shapes] example - easings box anim
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

    InitWindow(screenWidscreen_widtheight, "raylib [shapes] example - easings box anim");

    // Box variables to be animated with easings
    Rectangle rec = {GetScreenWidth() / 2, -100, 100, 100};
    float rotation = 0.0;
    float alpha = 1.0;

    int state = 0;
    int framesCounter = 0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        switch (state)
        {
        case 0: // Move box down to center of screen
        {
            framesCounter++;

            // NOTE: Remember that 3rd parameter of easing function refers to
            // desired value variation, do not confuse it with expected final value!
            rec.y = EaseElasticOut(framesCounter, -100, Getscreen_height() / 2 + 100, 120);

            if (framesCounter >= 120)
            {
                framesCounter = 0;
                state = 1;
            }
        }
        break;
        case 1: // Scale box to an horizontal bar
        {screen_width
            framesCounter++;
            rec.height = EaseBounceOut(framesCounter, 100, -90, 120);
            rec.width = EaseBounceOut(framesCounter, 100, GetScreenWidth(), 120);

            if (framesCounter >= 120)
            {
                framesCounter = 0;
                state = 2;
            }
        }
        break;
        case 2: // Rotate horizontal bar rectangle
        {
            framesCounter++;
            rotation = EaseQuadOut(framesCounter, 0.0, 270.0, 240);

            if (framesCounter >= 240)
            {
                framesCounter = 0;
                state = 3;
            }
        }
        break;
        case 3: // Increase bar size to fill all screenscreen_width
        {
            framesCounter++;
            rec.height = EaseCircOut(framesCounter, 10, GetScreenWidth(), 120);

            if (framesCounter >= 120)
            {
                framesCounter = 0;
                state = 4;
            }
        }
        break;
        case 4: // Fade out animation
        {
            framesCounter++;
            alpha = EaseSineOut(framesCounter, 1.0, -1.0, 160);

            if (framesCounter >= 160)
            {
                framesCounter = 0;
                state = 5;
            }
        }
        break;
        default:
            break;
        }

        // Reset animation at anyscreen_width
        if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_SPACE))
        {
            rec = (Rectangle){GetScreenWidth() / 2, -100, 100, 100};
            rotation = 0.0;
            alpha = 1.0;
            state = 0;
            framesCounter = 0;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_rectanglePro(rec, (Vector2){rec.width / 2, rec.height / 2}, rotation, Fade(BLACK, alpha));

        d.draw_text("PRESS [SPACE] TO RESET BOX ANIMATION!", 10, Getscreen_height() - 25, 20, Color::LIGHTGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}