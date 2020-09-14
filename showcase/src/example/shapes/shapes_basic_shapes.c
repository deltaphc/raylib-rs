/*******************************************************************************************
*
*   raylib [shapes] example - Draw basic shapes 2d (rectangle, circle, line...)
*
*   This example has been created using raylib 1.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [shapes] example - basic shapes drawing");


    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // TODO: Update your variables here
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("some basic shapes available on raylib", 20, 20, 20, Color::DARKGRAY);

        d.draw_circle(screen_width / 4, 120, 35, Color::DARKBLUE);

        d.draw_rectangle(screen_width / 4 * 2 - 60, 100, 120, 60,Color::RED);
        d.draw_rectangle_lines(screen_width / 4 * 2 - 40, 320, 80, 60, Color::ORANGE); // NOTE: Uses QUADS internally, not lines
        d.draw_rectangleGradientH(screen_width / 4 * 2 - 90, 170, 180, 130, Color::MAROON, Color::GOLD);

        d.draw_triangle(rvec2(screen_width / 4 * 3,  80),
                     rvec2(screen_width / 4 * 3 - 60,  150),
                     rvec2(screen_width / 4 * 3 + 60,  150), Color::VIOLET);

        DrawPoly(rvec2(screen_width / 4 * 3,  320), 6, 80, 0, Color::BROWN);

        DrawCircleGradient(screen_width / 4, 220, 60, Color::GREEN, Color::SKYBLUE);

        // NOTE: We draw all LINES based shapes together to optimize internal drawing,
        // this way, all LINES are rendered in a single draw pass
        DrawLine(18, 42, screen_width - 18, 42, Color::BLACK);
        DrawCircleLines(screen_width / 4, 340, 80, Color::DARKBLUE);
        d.draw_triangleLines(rvec2(screen_width / 4 * 3,  160),
                          rvec2(screen_width / 4 * 3 - 20,  230),
                          rvec2(screen_width / 4 * 3 + 20,  230), Color::DARKBLUE);
        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}