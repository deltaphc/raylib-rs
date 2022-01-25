/*******************************************************************************************
*
*   raylib [models] example - Models loading
*
*   raylib supports multiple models file formats:
*
*     - OBJ > Text file, must include vertex position-texcoords-normals information,
*             if files references some .mtl materials file, it will be loaded (or try to)
*     - GLTF > Modern text/binary file format, includes lot of information and it could
*              also reference external files, raylib will try loading mesh and materials data
*     - IQM > Binary file format including mesh vertex data but also animation data,
*             raylib can load .iqm animations.  
*
*   This example has been created using raylib 2.6 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;
use std::path::Path;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - models loading");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
    rvec3( 50.0, 50.0,50.0 ), // Camera position
     rvec3( 0.0, 10.0,0.0 ),     // Camera looking at point
     rvec3( 0.0, 1.0,0.0 ),          // Camera up vector (rotation towards target)
     45.0,                                // Camera field-of-view Y
    );
    
    let mut model = rl.load_model(&thread, "original/models/resources/models/castle.obj").unwrap();                 // Load model
    let mut texture =  rl.load_texture(thread, "original/models/resources/models/castle_diffuse.png").unwrap(); // Load model texture
    model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref();                 // Set map diffuse texture

    let position = rvec3( 0.0, 0.0, 0.0 );                // Set model position
 
    let mut bounds = model.meshes()[0].get_mesh_bounding_box();  // Set model bounds

    // NOTE: bounds are calculated from the original size of the model,
    // if model is scaled on drawing, bounds must be also scaled

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FREE);     // Set a free camera mode

    let mut selected = false;          // Selected object flag

    rl.set_target_fps(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);
        
        // Load new models/textures on drag&drop
        if (rl.is_file_dropped())
        {
            let droppedFiles = rl.get_dropped_files();

            if (droppedFiles.len() == 1) // Only support one file dropped
            {
                if Path::new(&droppedFiles[0]).extension().map_or(false, |ext| {
                    return ext == "obj" ||
                        ext == "gltf" ||
                        ext == "iqm"       // Model file formats supported
                    
                }) 
                {
                    model = rl.load_model(thread, &droppedFiles[0]).unwrap();     // Load new model
                    model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref(); // Set current map diffuse texture

                    bounds = model.meshes()[0].get_mesh_bounding_box();
                    
                    // TODO: Move camera position from target enough distance to visualize model properly
                }
                else if Path::new(&droppedFiles[0]).extension().map_or(false, |ext| { ext == "png"})  // Texture file formats supported
                {
                    // Unload current model texture and load new one
                    texture = rl.load_texture(thread, &droppedFiles[0]).unwrap();
                    model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref();
                }
            }

            rl.clear_dropped_files();    // Clear internal buffers
        }

        // Select model on mouse click
        if (rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT))
        {
            // Check collision between ray and box
            if bounds.get_ray_collision_box(rl.get_mouse_ray(rl.get_mouse_position(), &camera)).hit { selected = !selected;}
            else {selected = false;}
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {

                let mut d = d.begin_mode3D(&camera);
    
                    d.draw_model(&model, position, 1.0, Color::WHITE);        // Draw 3d model with texture
    
                    d.draw_grid(20, 10.0);         // Draw a grid
    
                    if (selected) {d.draw_bounding_box(bounds, Color::GREEN);}   // Draw selection box
    
            }

            
            d.draw_text("Drag & drop model to load mesh/texture.", 10, d.get_screen_height() - 20, 10, Color::DARKGRAY);
            if (selected){ d.draw_text("MODEL SELECTED", d.get_screen_width() - 110, 10, 10, Color::GREEN);}

            d.draw_text("(c) Castle 3D model by Alberto Cano", screen_width - 200, screen_height - 20, 10, Color::GRAY);

            d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    });
}