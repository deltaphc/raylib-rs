/*******************************************************************************************
*
*   raylib [core] example - Initialize 3d camera mode
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
    rl.set_window_title(thread, "raylib [core] example - 3d camera mode");


    // Define the camera to look into our 3d world
    Camera3D camera = {0};
    camera.position = rvec3(0.0, 10.0, 10.0); // Camera position
    camera.target = rvec3(0.0, 0.0, 0.0);     // Camera looking at point
    camera.up = rvec3(0.0, 1.0, 0.0);         // Camera up vector (rotation towards target)
    camera.fovy = 45.0;                             // Camera field-of-view Y
    camera.type = CAMERA_PERSPECTIVE;                // Camera mode type

    Vector3 cubePosition = {0.0, 0.0, 0.0};

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

        let mut d = d.begin_mode3D(&camera);

        d.draw_cube(cubePosition, 2.0, 2.0, 2.0, RED);
        d.draw_cube_wires(cubePosition, 2.0, 2.0, 2.0, Color::MAROON);

        DrawGrid(10, 1.0);

        EndMode3D();

        d.draw_text("Welcome to the third dimension!", 10, 40, 20, Color::DARKGRAY);

        DrawFPS(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}