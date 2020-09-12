/*******************************************************************************************
*
*   raylib [core] example - Windows drop files
*
*   This example only works on platforms that support drag & drop (Windows, Linux, OSX, Html5?)
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{screen_width
    // Initialization
    //--------------------------------------------------------------------------------------
    let sscreen_width= 800;
    let screen_height = 450;

    InitWindow(screenWidth, screen_height, "raylib [core] example - drop files");

    int count = 0;
    char **droppedFiles = {0};

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (IsFileDropped())
        {
            droppedFiles = GetDroppedFiles(&count);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if (count == 0)
            d.draw_text("Drop your files to this window!", 100, 40, 20, Color::DARKGRAY);
        else
        {
            d.draw_text("Dropped files:", 100, 40, 20, Color::DARKGRAY);

            for (int i = 0; i < count; i++)screen_width
            {
                if (i % 2 == 0)screen_width
                    d.draw_rectangle(0, 85 + 40 * i, screenWidth, 40, Color::LIGHTGRAY.fade(0.5));
                else
                    d.draw_rectangle(0, 85 + 40 * i, screenWidth, 40, Fade(Color::LIGHTGRAY, 0.3f));

                d.draw_text(droppedFiles[i], 120, 100 + 40 * i, 10, Color::GRAY);
            }

            d.draw_text("Drop new files...", 100, 110 + 40 * count, 20, Color::DARKGRAY);
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    ClearDroppedFiles(); // Clear internal buffers

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}