/*******************************************************************************************
*
*   raylib [models] example - Skybox loading and drawing
*
*   This example has been created using raylib 1.8 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2017 Ramon Santamaria (@raysan5)
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
        "raylib [models] example - skybox loading and drawing",
    );
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(1.0, 1.0, 1.0),
        rvec3(4.0, 1.0, 4.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );

    // Load skybox model
    let cube = unsafe { Mesh::gen_mesh_cube(thread, 1.0, 1.0, 1.0).make_weak() };
    let mut skybox = rl.load_model_from_mesh(thread, cube).unwrap();

    // Load skybox shader and set required locations
    // NOTE: Some locations are automatically set at shader loading
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        *skybox.materials_mut()[0].shader_mut() = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/skybox.vs"),
                Some("original/models/resources/shaders/glsl330/skybox.fs"),
            )
            .unwrap()
            .make_weak();
    }
    #[cfg(target_arch = "wasm32")] // PLATFORM_RPI, PLATFORM_ANDROID, PLATFORM_WEB
    unsafe {
        *skybox.materials_mut()[0].shader_mut() = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl100/skybox.vs"),
                Some("original/models/resources/shaders/glsl100/skybox.fs"),
            )
            .unwrap()
            .make_weak();
    }

    let loc = skybox.materials()[0]
        .shader()
        .get_shader_location("environmentMap");
    skybox.materials_mut()[0]
        .shader_mut()
        .set_shader_value(loc, raylib::consts::MaterialMapIndex::MATERIAL_MAP_CUBEMAP as i32 );

    // Load cubemap shader and setup required shader locations
    #[cfg(not(target_arch = "wasm32"))]
    let mut shdrCubemap = unsafe {
         rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/cubemap.vs"),
                Some("original/models/resources/shaders/glsl330/cubemap.fs"),
            )
            .unwrap()
    };
    #[cfg(target_arch = "wasm32")] // PLATFORM_RPI, PLATFORM_ANDROID, PLATFORM_WEB
    let mut shdrCubemap = unsafe {
         rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl100/cubemap.vs"),
                Some("original/models/resources/shaders/glsl100/cubemap.fs"),
            )
            .unwrap()
    };

    let loc = shdrCubemap.get_shader_location("equirectangularMap");
    shdrCubemap.set_shader_value(loc, 0i32);

    // Load HDR panorama (sphere) texture
    let texHDR = rl.load_texture(thread, "original/models/resources/dresden_square.hdr").unwrap();

    // Generate cubemap (texture with 6 quads-cube-mapping) from panorama HDR texture
    // NOTE: New texture is generated rendering to texture, shader computes the sphre->cube coordinates mapping
    skybox.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_CUBEMAP as usize].texture = unsafe { *rl.gen_texture_cubemap(thread, &shdrCubemap, &texHDR, 512, ffi::PixelFormat::UNCOMPRESSED_R8G8B8A8).make_weak().as_ref()};


    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FIRST_PERSON); // Set a first person camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()            // Detect window close button or ESC key
    {
        // keep stuff captured
        let _ = texHDR;
        let _ = skybox;
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
    
                    d.draw_model(&skybox, rvec3(0, 0,0), 1.0, Color::WHITE);
    
                    d.draw_grid(10, 1.0);
    
            }


            d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
        drop(d);
        if rl.is_key_pressed(crate::EXIT_KEY) {
            // free mouse
            rl.set_camera_mode(& camera, raylib::consts::CameraMode::CAMERA_FREE);
            
        }
    },
    );
}
