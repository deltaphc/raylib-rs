/*******************************************************************************************
*
*   raylib [text] example - Text Writing Animation
*
*   This example has been created using raylib 2.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2016 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [text] example - text writing anim");


    const char message[128] = "This sample illustrates a text writing\nanimation effect! Check it out! ;)";

    int framesCounter = 0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE))
            framesCounter += 8;
        else
            framesCounter++;

        if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_ENTER))
            framesCounter = 0;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text(TextSubtext(message, 0, framesCounter / 10), 210, 160, 20, Color::MAROON);

        d.draw_text("PRESS [ENTER] to RESTART!", 240, 260, 20, Color::LIGHTGRAY);
        d.draw_text("PRESS [SPACE] to SPEED UP!", 239, 300, 20, Color::LIGHTGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}