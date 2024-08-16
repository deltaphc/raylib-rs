/*******************************************************************************************
*
*   raylib [models] example - Heightmap loading and drawing
*
*   This example has been created using raylib 1.8 (www.raylib.com)
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

    rl.set_window_title(thread, "raylib [models] example - heightmap loading and drawing");
    rl.set_window_size(screen_width, screen_height);

    // Define our custom camera to look into our 3d world
    let mut camera = Camera3D::perspective(rvec3( 18.0, 18.0, 18.0 ), rvec3( 0.0, 0.0, 0.0 ), rvec3( 0.0, 1.0, 0.0 ), 45.0);

    let image = Image::load_image("original/models/resources/heightmap.png").unwrap();             // Load heightmap image (RAM)
    let texture = unsafe { rl.load_texture_from_image(thread, &image).unwrap().make_weak() };                // Convert image to texture (VRAM)

    let mesh = unsafe { Mesh::gen_mesh_heightmap(thread, &image, rvec3( 16, 8,16 )).make_weak() };    // Generate heightmap mesh (RAM and VRAM)
    let mut model = rl.load_model_from_mesh(thread, mesh).unwrap();                          // Load model from generated mesh

    model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref();         // Set map diffuse texture
    let mapPosition = rvec3( -8.0, 0.0, -8.0 );                   // Define model position


    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL);  // Set an orbital camera mode

    rl.set_target_fps(60);                       // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()            // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);              // Update camera
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {
                let mut d = d.begin_mode3D(&camera);
    
                    d.draw_model(&model, mapPosition, 1.0, Color::RED);
    
                    d.draw_grid(20, 1.0);
    

            }


            d.draw_texture(&texture, screen_width - texture.width - 20, 20, Color::WHITE);
            d.draw_rectangle_lines(screen_width - texture.width - 20, 20, texture.width, texture.height, Color::GREEN);

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