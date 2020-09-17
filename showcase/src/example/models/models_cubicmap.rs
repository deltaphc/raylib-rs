/*******************************************************************************************
*
*   raylib [models] example - Cubicmap loading and drawing
*
*   This example has been created using raylib 1.8 (www.raylib.com)
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

    rl.set_window_title(
        thread,
        "raylib [models] example - cubesmap loading and drawing",
    );
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(16.0, 14.0, 16.0),
        rvec3(0.0, 0.0, 0.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );

    let image = Image::load_image("original/models/resources/cubicmap.png").unwrap(); // Load cubicmap image (RAM)
    let cubicmap =  rl.load_texture_from_image(thread, &image).unwrap(); // Convert image to texture to display (VRAM)

    // Because model depends on mesh, we have to make sure mesh lives as long as the model. We make it weak
    // to manually control it's lifespan.
    let mesh = unsafe { Mesh::gen_mesh_cubicmap(thread, &image, rvec3(1.0, 1.0, 1.0)).make_weak() };
    let mut model = rl.load_model_from_mesh(thread, mesh.clone()).unwrap();

    // NOTE: By default each cube is mapped to one part of texture atlas
    // make texture weak so it lives as long as the model
    let texture = unsafe {
        rl
            .load_texture(thread, "original/models/resources/cubicmap_atlas.png")
            .unwrap().make_weak() // Load map texture
            
        }; 
        model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapType::MAP_ALBEDO as usize]
        .texture = *texture.as_ref(); // Set map diffuse texture
    let mapPosition = rvec3(-16.0, 0.0, -8.0); // Set model position

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()            // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);              // Update camera
        //----------------------------------------------------------------------------------

        {
            // Draw
            //----------------------------------------------------------------------------------
            let mut d = rl.begin_drawing(thread);
    
                d.clear_background(Color::RAYWHITE);
    
                {
                    let mut d = d.begin_mode3D(&camera);
        
                        d.draw_model(&model, mapPosition, 1.0, Color::WHITE);
    
                }
    
    
                d.draw_texture_ex(&cubicmap, rvec2( screen_width - cubicmap.width*4 - 20, 20 ), 0.0, 4.0, Color::WHITE);
                d.draw_rectangle_lines(screen_width - cubicmap.width*4 - 20, 20, cubicmap.width*4, cubicmap.height*4, Color::GREEN);
    
                d.draw_text("cubicmap image used to", 658, 90, 10, Color::GRAY);
                d.draw_text("generate map 3d model", 658, 104, 10, Color::GRAY);
    
                d.draw_fps(10, 10);
    
            //----------------------------------------------------------------------------------
        }
        if rl.is_key_pressed(crate::EXIT_KEY) {
            unsafe {
                rl.unload_texture(thread, texture.clone());
                // Don't need to unload mesh because the model will.
                // rl.unload_mesh(thread, mesh.clone());
            }
        }
    },
    );
}
