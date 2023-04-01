/*******************************************************************************************
*
*   raylib example - procedural mesh generation
*
*   This example has been created using raylib 1.8 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2017 Ramon Santamaria (Ray San)
*
********************************************************************************************/

pub use raylib::prelude::*;

const NUM_MODELS: usize = 8;      // Parametric 3d shapes to generate

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - mesh generation");
    rl.set_window_size(screen_width, screen_height);

    // We generate a checked image for texturing
    let checked = Image::gen_image_checked(2, 2, 1, 1, Color::RED, Color::GREEN);
    let texture = rl.load_texture_from_image(&thread, &checked).unwrap();


    let mut models = unsafe {
        [
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_plane(thread, 2.0, 2.0, 5, 5).make_weak()).unwrap(),
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_cube(thread,2.0, 1.0, 2.0).make_weak()).unwrap(),
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_sphere(thread,2.0, 32, 32).make_weak()).unwrap(),
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_hemisphere(thread,2.0, 16, 16).make_weak()).unwrap(),
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_cylinder(thread,1.0, 2.0, 16).make_weak()).unwrap(),
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_torus(thread,0.25, 4.0, 16, 32).make_weak()).unwrap(),
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_knot(thread,1.0, 2.0, 16, 128).make_weak()).unwrap(),
        rl.load_model_from_mesh(thread, Mesh::gen_mesh_poly(thread,5, 2.0).make_weak()).unwrap(),
    ]
    };


    // Set checked texture as default diffuse component for all models material
    for model in &mut models  {model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref();}

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(rvec3( 5.0, 5.0, 5.0 ), rvec3( 0.0, 0.0, 0.0 ), rvec3( 0.0, 1.0, 0.0 ), 45.0 );

    // Model drawing position
    let position = rvec3( 0.0, 0.0, 0.0 );

    let mut currentModel = 0;

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL);  // Set a orbital camera mode

    rl.set_target_fps(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()    // Detect window close button or ESC key
    {
        // prevent texture unloading;
        let _ = texture;
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);      // Update internal camera and our camera

        if (rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT))
        {
            currentModel = (currentModel + 1)%NUM_MODELS; // Cycle between the textures
        }

        if (rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT))
        {
            if (currentModel < NUM_MODELS) {currentModel += 1}
        }
        else if (rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT))
        {
            if (currentModel > 0) {currentModel -= 1}
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {

                let mut d = d.begin_mode3D(&camera);
    
                    d.draw_model(&models[currentModel], position, 1.0, Color::WHITE);
    
                    d.draw_grid(10, 1.0);
    
            }


            d.draw_rectangle(30, 400, 310, 30, Color::SKYBLUE.fade( 0.5));
            d.draw_rectangle_lines(30, 400, 310, 30, Color::DARKBLUE.fade(0.5));
            d.draw_text("MOUSE LEFT BUTTON to CYCLE PROCEDURAL MODELS", 40, 410, 10, Color::BLUE);

            match (currentModel)
            {
                0 => d.draw_text("PLANE", 680, 10, 20, Color::DARKBLUE),
                1 => d.draw_text("CUBE", 680, 10, 20, Color::DARKBLUE),
                2 => d.draw_text("SPHERE", 680, 10, 20, Color::DARKBLUE),
                3 => d.draw_text("HEMISPHERE", 640, 10, 20, Color::DARKBLUE),
                4 => d.draw_text("CYLINDER", 680, 10, 20, Color::DARKBLUE),
                5 => d.draw_text("TORUS", 680, 10, 20, Color::DARKBLUE),
                6 => d.draw_text("KNOT", 680, 10, 20, Color::DARKBLUE),
                7 => d.draw_text("POLY", 680, 10, 20, Color::DARKBLUE),
                _ => unimplemented!()
            }

    });
}