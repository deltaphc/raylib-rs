/*******************************************************************************************
*
*   raylib [models] example - first person maze
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;


pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - first person maze");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(rvec3( 0.2, 0.4, 0.2 ), rvec3( 0.0, 0.0, 0.0 ), rvec3( 0.0, 1.0, 0.0 ), 45.0);

    let imMap = Image::load_image("original/models/resources/cubicmap.png").unwrap();      // Load cubicmap image (RAM)
    let cubicmap = rl.load_texture_from_image(thread, &imMap).unwrap();       // Convert image to texture to display (VRAM)
    let mesh = unsafe { Mesh::gen_mesh_cubicmap(thread, &imMap, rvec3( 1.0, 1.0,1.0 )).make_weak() };
    let mut model = rl.load_model_from_mesh(thread, mesh.clone()).unwrap();

    // NOTE: By default each cube is mapped to one part of texture atlas
    let texture = unsafe { rl.load_texture(thread, "original/models/resources/cubicmap_atlas.png").unwrap().make_weak() };   // Load map texture
    model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref();             // Set map diffuse texture

    // Get map image data to be used for collision detection
    let mapPixels = imMap.get_image_data();

    let mapPosition = rvec3( -16.0, 0.0, -8.0 );  // Set model position
    let playerPosition = camera.position;       // Set player position

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FIRST_PERSON);     // Set camera mode

    rl.set_target_fps(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        let oldCamPos = camera.position;    // Store old camera position

        rl.update_camera(&mut camera);      // Update camera

        // Check player collision (we simplify to 2D collision detection)
        let playerPos = rvec2( camera.position.x, camera.position.z );
        let playerRadius = 0.1;  // Collision radius (player is modelled as a cilinder for collision)

        let mut playerCellX = (playerPos.x - mapPosition.x + 0.5) as i32;
        let mut playerCellY = (playerPos.y - mapPosition.z + 0.5) as i32;

        // Out-of-limits security check
        if (playerCellX < 0) {playerCellX = 0;}
        else if (playerCellX >= cubicmap.width) {playerCellX = cubicmap.width - 1;}

        if (playerCellY < 0) {playerCellY = 0;}
        else if (playerCellY >= cubicmap.height) {playerCellY = cubicmap.height - 1;}

        // Check map collisions using image data and player position
        // TODO: Improvement: Just check player surrounding cells for collision
        for y in 0..cubicmap.height()
        {
            for x in 0..cubicmap.width()
            {
                if (mapPixels[(y*cubicmap.width + x) as usize].r == 255) &&       // Collision: white pixel, only check R channel
                     rrect( mapPosition.x - 0.5 + x as f32*1.0, mapPosition.z - 0.5 + y as f32*1.0, 1.0, 1.0 ).check_collision_circle_rec(playerPos, playerRadius)
                {
                    // Collision detected, reset camera position
                    camera.position = oldCamPos;
                }
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);
            {
                let mut d = d.begin_mode3D(&camera);
    
                    d.draw_model(&model, mapPosition, 1.0, Color::WHITE);                     // Draw maze map
                    //d.draw_cube_vplayerPosition, rvec3( 0.2f, 0.4,0.2f ), Color::RED);  // Draw player
    

            }


            d.draw_texture_ex(&cubicmap, rvec2( d.get_screen_width() - cubicmap.width*4 - 20, 20.0 ), 0.0, 4.0, Color::WHITE);
            d.draw_rectangle_lines(d.get_screen_width() - cubicmap.width*4 - 20, 20, cubicmap.width*4, cubicmap.height*4, Color::GREEN);

            // Draw player position radar
            d.draw_rectangle(d.get_screen_width() - cubicmap.width*4 - 20 + playerCellX*4, 20 + playerCellY*4, 4, 4, Color::RED);

            d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
        drop(d);
        if rl.is_key_pressed(crate::EXIT_KEY) {
            // free mouse
            rl.set_camera_mode(& camera, raylib::consts::CameraMode::CAMERA_FREE);
            unsafe {
                rl.unload_texture(thread, texture.clone());
                // Don't need to unload mesh because the model will.
                // rl.unload_mesh(thread, mesh.clone());
            }
        }
    });

}
