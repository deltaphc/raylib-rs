/*******************************************************************************************
*
*   raylib [core] example - Picking in 3d mode
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
    rl.set_window_title(thread, "raylib [core] example - 3d picking");


    // Define the camera to look into our 3d world
    Camera camera = {0};
    camera.position = rvec3(10.0, 10.0, 10.0); // Camera position
    camera.target = rvec3(0.0, 0.0, 0.0);      // Camera looking at point
    camera.up = rvec3(0.0, 1.0, 0.0);          // Camera up vector (rotation towards target)
    camera.fovy = 45.0;                              // Camera field-of-view Y
    camera.type = CAMERA_PERSPECTIVE;                 // Camera mode type

    Vector3 cubePosition = {0.0, 1.0, 0.0};
    Vector3 cubeSize = {2.0, 2.0, 2.0};

    Ray ray = {0}; // Picking line ray

    bool collision = false;

    SetCameraMode(camera, CAMERA_FREE); // Set a free camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON))
        {
            if (!collision)
            {
                ray = GetMouseRay(GetMousePosition(), camera);

                // Check collision between ray and box
                collision = CheckCollisionRayBox(ray,
                                                 (BoundingBox){(Vector3){cubePosition.x - cubeSize.x / 2, cubePosition.y - cubeSize.y / 2, cubePosition.z - cubeSize.z / 2},
                                                               (Vector3){cubePosition.x + cubeSize.x / 2, cubePosition.y + cubeSize.y / 2, cubePosition.z + cubeSize.z / 2}});
            }
            else
                collision = false;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        let mut d = d.begin_mode3D(&camera);

        if (collision)
        {
            d.draw_cube(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, RED);
            d.draw_cube_wires(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, Color::MAROON);

            d.draw_cube_wires(cubePosition, cubeSize.x + 0.2f, cubeSize.y + 0.2f, cubeSize.z + 0.2f, GREEN);
        }
        else
        {
            d.draw_cube(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, GRAY);
            d.draw_cube_wires(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, Color::DARKGRAY);
        }

        DrawRay(ray, Color::MAROON);
        DrawGrid(10, 1.0);

        EndMode3D();

        d.draw_text("Try selecting the box with mouse!", 240, 10, 20, Color::DARKGRAY);

        if (collision)
            d.draw_text("BOX SELECTED", (screen_width - MeasureText("BOX SELECTED", 30)) / 2, screen_height * 0.1f, 30, GREEN);

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
