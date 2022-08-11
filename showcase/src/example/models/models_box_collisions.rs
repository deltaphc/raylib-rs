/*******************************************************************************************
*
*   raylib [models] example - Detect basic 3d collisions (box vs sphere vs box)
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - box collisions");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let camera = Camera::perspective( rvec3( 0.0, 10.0, 10.0 ), rvec3( 0.0, 0.0, 0.0 ), rvec3( 0.0, 1.0, 0.0 ), 45.0 );

    let mut playerPosition = rvec3( 0.0, 1.0, 2.0 );
    let playerSize = rvec3( 1.0, 2.0, 1.0 );

    let enemyBoxPos = rvec3( -4.0, 1.0, 0.0 );
    let enemyBoxSize = rvec3( 2.0, 2.0, 2.0 );

    let enemySpherePos = rvec3( 4.0, 0.0, 0.0 );
    let enemySphereSize = 1.5;

    rl.set_target_fps(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------

        // Move player
        if (rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT)){ playerPosition.x += 0.2;}
        else if (rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT)) {playerPosition.x -= 0.2;}
        else if (rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)) {playerPosition.z += 0.2;}
        else if (rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)){ playerPosition.z -= 0.2;}

        let mut collision = false;

        // Check collisions player vs enemy-box
        if 
            BoundingBox::new(rvec3( playerPosition.x - playerSize.x/2.0,
                                     playerPosition.y - playerSize.y/2.0,
                                     playerPosition.z - playerSize.z/2.0 ),
                          rvec3( playerPosition.x + playerSize.x/2.0,
                                     playerPosition.y + playerSize.y/2.0,
                                     playerPosition.z + playerSize.z/2.0 )).check_collision_boxes(
            BoundingBox::new(rvec3( enemyBoxPos.x - enemyBoxSize.x/2.0,
                                     enemyBoxPos.y - enemyBoxSize.y/2.0,
                                     enemyBoxPos.z - enemyBoxSize.z/2.0 ),
                          rvec3( enemyBoxPos.x + enemyBoxSize.x/2.0,
                                     enemyBoxPos.y + enemyBoxSize.y/2.0,
                                     enemyBoxPos.z + enemyBoxSize.z/2.0 ))) {collision = true;}

        // Check collisions player vs enemy-sphere
        if 
            BoundingBox::new(rvec3( playerPosition.x - playerSize.x/2.0,
                                     playerPosition.y - playerSize.y/2.0,
                                     playerPosition.z - playerSize.z/2.0 ),
                          rvec3( playerPosition.x + playerSize.x/2.0,
                                     playerPosition.y + playerSize.y/2.0,
                                     playerPosition.z + playerSize.z/2.0 )).check_collision_box_sphere(
            enemySpherePos, enemySphereSize) {collision = true;}

        let playerColor = if collision { Color::RED } else { Color::GREEN };
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {

                let mut d = d.begin_mode3D(&camera);
    
                    // Draw enemy-box
                    d.draw_cube(enemyBoxPos, enemyBoxSize.x, enemyBoxSize.y, enemyBoxSize.z, Color::GRAY);
                    d.draw_cube_wires(enemyBoxPos, enemyBoxSize.x, enemyBoxSize.y, enemyBoxSize.z, Color::DARKGRAY);
    
                    // Draw enemy-sphere
                    d.draw_sphere(enemySpherePos, enemySphereSize, Color::GRAY);
                    d.draw_sphere_wires(enemySpherePos, enemySphereSize, 16, 16, Color::DARKGRAY);
    
                    // Draw player
                    d.draw_cube_v(playerPosition, playerSize, playerColor);
    
                    d.draw_grid(10, 1.0);        // Draw a grid
            }

            d.draw_text("Move player with cursors to collide", 220, 40, 20, Color::GRAY);

            d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    });
}