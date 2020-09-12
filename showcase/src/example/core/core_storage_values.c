/*******************************************************************************************
*
*   raylib [core] example - Storage save/load values
*
*   This example has been created using raylib 1.4 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

// NOTE: Storage positions must start with 0, directly related to file memory layout
typedef enum
{
    STORAGE_POSITION_SCORE = 0,
    STORAGE_POSITION_HISCORE = 1
} StorageData;

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
    rl.set_window_title(thread, "raylib [core] example - storage save/load values");


    int score = 0;
    int hiscore = 0;
    int framesCounter = 0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (IsKeyPressed(KEY_R))
        {
            score = raylib::get_random_value(1000, 2000);
            hiscore = raylib::get_random_value(2000, 4000);
        }

        if (IsKeyPressed(KEY_ENTER))
        {
            SaveStorageValue(STORAGE_POSITION_SCORE, score);
            SaveStorageValue(STORAGE_POSITION_HISCORE, hiscore);
        }
        else if (IsKeyPressed(KEY_SPACE))
        {
            // NOTE: If requested position could not be found, value 0 is returned
            score = LoadStorageValue(STORAGE_POSITION_SCORE);
            hiscore = LoadStorageValue(STORAGE_POSITION_HISCORE);
        }

        framesCounter++;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text(FormatText("SCORE: %i", score), 280, 130, 40, Color::MAROON);
        d.draw_text(FormatText("HI-SCORE: %i", hiscore), 210, 200, 50, Color::BLACK);

        d.draw_text(FormatText("frames: %i", framesCounter), 10, 10, 20, Color::LIME);

        d.draw_text("Press R to generate random numbers", 220, 40, 20, Color::LIGHTGRAY);
        d.draw_text("Press ENTER to SAVE values", 250, 310, 20, Color::LIGHTGRAY);
        d.draw_text("Press SPACE to LOAD values", 252, 350, 20, Color::LIGHTGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}