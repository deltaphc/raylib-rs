/*******************************************************************************************
*
*   raylib [core] example - 3d camera first person
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_COLUMNS: usize = 20;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - 3d camera first person");

    // Define the camera to look into our 3d world (position, target, up vector)
    let mut camera = Camera3D::perspective(rvec3(4, 2, 4), rvec3(0, 1.8, 0), Vector3::up(), 60.0);

    // Generates some random columns
    let mut heights = [0.032; MAX_COLUMNS];
    let mut positions = [Vector3::zero(); MAX_COLUMNS];
    let mut colors = [Color::default(); MAX_COLUMNS];

    for i in 0..MAX_COLUMNS {
        heights[i] = raylib::get_random_value::<i32>(1, 12) as f32;
        positions[i] = rvec3(
            raylib::get_random_value::<i32>(-15, 15),
            heights[i] / 2.0,
            raylib::get_random_value::<i32>(-15, 15),
        );
        colors[i] = Color::new(
            raylib::get_random_value::<i32>(20, 255) as u8,
            raylib::get_random_value::<i32>(10, 55) as u8,
            30,
            255,
        );
    }

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FIRST_PERSON); // Set a first person camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()                // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);                  // Update camera
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {
                let mut d = d.begin_mode3D(&camera);
    
                    d.draw_plane(rvec3(0.0, 0.0, 0.0), rvec2(32.0, 32.0), Color::LIGHTGRAY); // Draw ground
                    d.draw_cube(rvec3(16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::BLUE);     // Draw a blue wall
                    d.draw_cube(rvec3(16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::LIME);      // Draw a green wall
                    d.draw_cube(rvec3(0.0, 2.5, 16.0), 32.0, 5.0, 1.0, Color::GOLD);      // Draw a yellow wall
    
                    // Draw some cubes around
                    for i in 0..MAX_COLUMNS
                    {
                        d.draw_cube(positions[i], 2.0, heights[i], 2.0, colors[i]);
                        d.draw_cube_wires(positions[i], 2.0, heights[i], 2.0, Color::MAROON);
                    }

            }


            d.draw_rectangle( 10, 10, 220, 70, Color::SKYBLUE.fade(0.5));
            d.draw_rectangle_lines( 10, 10, 220, 70, Color::BLUE);

            d.draw_text("First person camera default controls:", 20, 20, 10, Color::BLACK);
            d.draw_text("- Move with keys: W, A, S, D", 40, 40, 10, Color::DARKGRAY);
            d.draw_text("- Mouse move to look around", 40, 60, 10, Color::DARKGRAY);

        //----------------------------------------------------------------------------------
    }
    );
}
