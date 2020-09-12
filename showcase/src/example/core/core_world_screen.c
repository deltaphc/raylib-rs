/*******************************************************************************************
*
*   raylib [core] example - World to screen
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
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - 3d camera free");


    // Define the camera to look into our 3d world
    Camera camera = {0};
    camera.position = rvec3(10.0, 10.0, 10.0);
    camera.target = rvec3(0.0, 0.0, 0.0);
    camera.up = rvec3(0.0, 1.0, 0.0);
    camera.fovy = 45.0;
    camera.type = CAMERA_PERSPECTIVE;

    Vector3 cubePosition = {0.0, 0.0, 0.0};
    Vector2 cubeScreenPosition = {0.0, 0.0};

    SetCameraMode(camera, CAMERA_FREE); // Set a free camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        // Calculate cube screen space position (with a little offset to be in top)
        cubeScreenPosition = GetWorldToScreen((Vector3){cubePosition.x, cubePosition.y + 2.5, cubePosition.z}, camera);
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

        d.draw_text("Enemy: 100 / 100", cubeScreenPosition.x - MeasureText("Enemy: 100/100", 20) / 2, cubeScreenPosition.y, 20, Color::BLACK);
        d.draw_text("Text is always on top of the cube", (screen_width - MeasureText("Text is always on top of the cube", 20)) / 2, 25, 20, GRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}