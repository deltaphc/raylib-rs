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

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - core world screen");

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(10.0, 10.0, 10.0),
        rvec3(0.0, 0.0, 0.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );

    let  cube_position = Vector3::zero();

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FREE); // Set a free camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        // Calculate cube screen space position (with a little offset to be in top)
        let cube_screen_position = rl.get_world_to_screen(rvec3(cube_position.x, cube_position.y + 2.5,  cube_position.z), camera);
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

        d.draw_text("Enemy: 100 / 100", cube_screen_position.x  as i32 - (raylib::text::measure_text("Enemy: 100/100", 20) / 2), cube_screen_position.y as i32, 20, Color::BLACK);
        d.draw_text("Text is always on top of the cube", (screen_width - raylib::text::measure_text("Text is always on top of the cube", 20)) / 2, 25, 20, Color::GRAY);

        //----------------------------------------------------------------------------------
    },
    );
}
