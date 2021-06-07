/*******************************************************************************************
*
*   raylib [models] example - Show the difference between perspective and orthographic projection
*
*   This program is heavily based on the geometric objects example
*
*   This example has been created using raylib 2.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Max Danielsson (@autious) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2018 Max Danielsson (@autious) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;

const FOVY_PERSPECTIVE: f32 = 45.0;
const WIDTH_ORTHOGRAPHIC: f32 = 10.0;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - orthographic projection");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(0.0, 10.0, 10.0),
        rvec3(0.0, 0.0, 0.0),
        rvec3(0.0, 1.0, 0.0),
        FOVY_PERSPECTIVE,
    );

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE))
        {
            if (camera.camera_type() == raylib::consts::CameraProjection::CAMERA_PERSPECTIVE)
            {
                camera.fovy = WIDTH_ORTHOGRAPHIC;
                camera = Camera3D::orthographic(camera.position, camera.target, camera.up, camera.fovy);
            }
            else
            {
                camera.fovy = FOVY_PERSPECTIVE;
                camera = Camera3D::perspective(camera.position, camera.target, camera.up, camera.fovy);
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {

                let mut d = d.begin_mode3D(&camera);
    
                    d.draw_cube(rvec3(-4.0, 0.0,2.0), 2.0, 5.0, 2.0, Color::RED);
                    d.draw_cube_wires(rvec3(-4.0, 0.0,2.0), 2.0, 5.0, 2.0, Color::GOLD);
                    d.draw_cube_wires(rvec3(-4.0, 0.0,-2.0), 3.0, 6.0, 2.0, Color::MAROON);
    
                    d.draw_sphere(rvec3(-1.0, 0.0,-2.0), 1.0, Color::GREEN);
                    d.draw_sphere_wires(rvec3(1.0, 0.0,2.0), 2.0, 16, 16, Color::LIME);
    
                    d.draw_cylinder(rvec3(4.0, 0.0,-2.0), 1.0, 2.0, 3.0, 4, Color::SKYBLUE);
                    d.draw_cylinder_wires(rvec3(4.0, 0.0,-2.0), 1.0, 2.0, 3.0, 4, Color::DARKBLUE);
                    d.draw_cylinder_wires(rvec3(4.5, -1.0,2.0), 1.0, 1.0, 2.0, 6, Color::BROWN);
    
                    d.draw_cylinder(rvec3(1.0, 0.0,-4.0), 0.0, 1.5, 3.0, 8, Color::GOLD);
                    d.draw_cylinder_wires(rvec3(1.0, 0.0,-4.0), 0.0, 1.5, 3.0, 8, Color::PINK);
    
                    d.draw_grid(10, 1.0);        // Draw a grid
    
            }


            d.draw_text("Press Spacebar to switch camera type", 10, d.get_screen_height() - 30, 20, Color::DARKGRAY);

            if (camera.camera_type() == raylib::consts::CameraProjection::CAMERA_ORTHOGRAPHIC) {d.draw_text("ORTHOGRAPHIC", 10, 40, 20, Color::BLACK);}
            else if (camera.camera_type() == raylib::consts::CameraProjection::CAMERA_PERSPECTIVE){ d.draw_text("PERSPECTIVE", 10, 40, 20, Color::BLACK);}

            d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    },
    );
}
