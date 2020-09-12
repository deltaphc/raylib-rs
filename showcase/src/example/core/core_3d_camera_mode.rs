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

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - 3d camera mode");

    // Define the camera to look into our 3d world
    let  camera = Camera3D::perspective(
        rvec3(0.0, 10.0, 10.0), // Camera position
        rvec3(0.0, 0.0, 0.0),   // Camera looking at point
        rvec3(0.0, 1.0, 0.0),   // Camera up vector (rotation towards target)
        45.0,                   // Camera field-of-view Y
    );

    let cube_position = Vector3::zero();

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // TODO: Update your variables here
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(&camera);
    
            d.draw_cube(cube_position, 2.0, 2.0, 2.0,Color::RED);
            d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
    
            d.draw_grid(10, 1.0);

        }


        d.draw_text("Welcome to the third dimension!", 10, 40, 20, Color::DARKGRAY);

        d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    }
    );
}
