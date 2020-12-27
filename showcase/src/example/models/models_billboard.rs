/*******************************************************************************************
*
*   raylib [models] example - Drawing billboards
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - drawing billboards");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(5.0, 4.0, 5.0),
        rvec3(0.0, 2.0, 0.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );

    let bill = rl.load_texture(thread, "original/models/resources/billboard.png").unwrap(); // Our texture billboard
    let billPosition = rvec3(0.0, 2.0, 0.0); // Position where draw billboard

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()
    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera
                               //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        {

            let mut d = d.begin_mode3D(&camera);
    
            d.draw_grid(10, 1.0); // Draw a grid
    
            d.draw_billboard(&camera, &bill, billPosition, 2.0, Color::WHITE);
        }



        d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    });
}
