/*******************************************************************************************
*
*   raylib [models] example - Draw some basic geometric shapes (cube, sphere, cylinder...)
*
*   This example has been created using raylib 1.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - geometric shapes");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let camera = Camera3D::perspective(
     rvec3( 0.0, 10.0,10.0 ),
    rvec3( 0.0, 0.0,0.0 ),
     rvec3( 0.0, 1.0,0.0 ),
     45.0,
    );

    rl.set_target_fps(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()    // Detect window close button or ESC key
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


            d.draw_fps(10, 10);

    });
}