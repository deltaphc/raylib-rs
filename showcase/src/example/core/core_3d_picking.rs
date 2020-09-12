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

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - 3d picking");

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(10.0, 10.0, 10.0), // Camera position
        rvec3(0.0, 0.0, 0.0),    // Camera looking at point
        rvec3(0.0, 1.0, 0.0),    // Camera up vector (rotation towards target)
        45.0,                    // Camera field-of-view Y
    );

    let cube_position = rvec3(0.0, 1.0, 0.0);
    let cube_size = rvec3(2.0, 2.0, 2.0);

    let mut ray = Ray::default(); // Picking line ray

    let mut collision = false;

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

        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            if !collision
            {
                ray = rl.get_mouse_ray(rl.get_mouse_position(), camera);

                // Check collision between ray and box
                collision = BoundingBox::new(rvec3(cube_position.x - cube_size.x / 2.0, cube_position.y - cube_size.y / 2.0, cube_position.z - cube_size.z / 2.0),
                    rvec3(cube_position.x + cube_size.x / 2.0, cube_position.y + cube_size.y / 2.0, cube_position.z + cube_size.z / 2.0)).check_collision_ray_box(ray);
            }            else
             {
                 collision = false;

             }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        {

            let mut d = d.begin_mode3D(&camera);
    
            if collision
            {
                d.draw_cube(cube_position, cube_size.x, cube_size.y, cube_size.z,Color::RED);
                d.draw_cube_wires(cube_position, cube_size.x, cube_size.y, cube_size.z, Color::MAROON);
    
                d.draw_cube_wires(cube_position, cube_size.x + 0.2, cube_size.y + 0.2, cube_size.z + 0.2, Color::GREEN);
            }
            else
            {
                d.draw_cube(cube_position, cube_size.x, cube_size.y, cube_size.z, Color::GRAY);
                d.draw_cube_wires(cube_position, cube_size.x, cube_size.y, cube_size.z, Color::DARKGRAY);
            }
    
            d.draw_ray(ray, Color::MAROON);
            d.draw_grid(10, 1.0);
        }


        d.draw_text("Try selecting the box with mouse!", 240, 10, 20, Color::DARKGRAY);

        if collision {
            d.draw_text("BOX SELECTED", (screen_width  - raylib::text::measure_text("BOX SELECTED", 30)) / 2, (screen_height as f32 * 0.1) as i32, 30, Color::GREEN);

        }

        d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    }
    );
    
}
