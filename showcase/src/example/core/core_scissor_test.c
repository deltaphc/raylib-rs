/*******************************************************************************************
*
*   raylib [core] example - Scissor test
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Dill (@MysteriousSpace) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Chris Dill (@MysteriousSpace)
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
    rl.set_window_title(thread, "raylib [core] example - scissor test");


    Rectangle scissorArea = {0, 0, 300, 300};
    bool scissorMode = true;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (IsKeyPressed(KEY_S))
            scissorMode = !scissorMode;

        // Centre the scissor area around the mouse position
        scissorArea.x = GetMouseX() - scissorArea.width / 2;
        scissorArea.y = GetMouseY() - scissorArea.height / 2;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if (scissorMode)
            BeginScissorMode(scissorArea.x, scissorArea.y, scissorArea.width, scissorArea.height);

        // Draw full screen rectangle and some text
        // NOTE: Only part defined by scissor area will be rendered
        d.draw_rectangle(0, 0, Getscreen_width(), Getscreen_height(), RED);
        d.draw_text("Move the mouse around to reveal this text!", 190, 200, 20, Color::LIGHTGRAY);

        if (scissorMode)
            EndScissorMode();

        d.draw_rectangle_linesEx(scissorArea, 1, Color::BLACK);
        d.draw_text("Press S to toggle scissor test", 10, 10, 20, Color::BLACK);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
