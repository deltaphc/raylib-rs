/*******************************************************************************************
*
*   raylib [shaders] example - Apply a postprocessing shader and connect a custom uniform variable
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3), to test this example
*         on OpenGL ES 2.0 platforms (Android, Raspberry Pi, HTML5), use #version 100 shaders
*         raylib comes with shaders ready for both versions, check raylib/shaders install folder
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - custom uniform variable");

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(8.0, 8.0, 8.0),
        rvec3(0.0, 1.5, 0.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );

    let mut model = rl
        .load_model(&thread, "original/shaders/resources/models/barracks.obj")
        .unwrap(); // Load OBJ model
    let texture = rl
        .load_texture(
            thread,
            "original/shaders/resources/models/barracks_diffuse.png",
        )
        .unwrap(); // Load model texture (diffuse map)
    model.materials_mut()[0].maps_mut()
        [raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *texture.as_ref(); // Set model diffuse texture

    let position = Vector3::zero(); // Set model position

    // Load postprocessing shader
    // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
    let mut shader = rl
        .load_shader(
            thread,
            None,
            Some(&format!(
                "original/shaders/resources/shaders/glsl{}/swirl.fs",
                GLSL_VERSION
            )),
        )
        .unwrap();

    // Get variable (uniform) location on the shader to connect with the program
    // NOTE: If uniform variable could not be found in the shader, function returns -1
    let swirlCenterLoc = shader.get_shader_location("center");

    let mut swirlCenter = Vector2::new(screen_width as f32 / 2.0, screen_height as f32 / 2.0);

    // Create a RenderTexture2D to be used for render to texture
    let mut target = rl
        .load_render_texture(thread, screen_width as u32, screen_height as u32)
        .unwrap();

    // Setup orbital camera
    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        let _ = texture;
        // Update
        //----------------------------------------------------------------------------------
        let mousePosition = rl.get_mouse_position();

        swirlCenter.x = mousePosition.x;
        swirlCenter.y = screen_height as f32 - mousePosition.y;

        // Send new value to the shader to be used on drawing
        shader.set_shader_value( swirlCenterLoc, swirlCenter);

        rl.update_camera(&mut camera); // Update camera
        //----------------------------------------------------------------------------------
{
        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);
{
        let mut d = d.begin_texture_mode(thread, &mut target); // Enable drawing to texture

        d.clear_background(Color::RAYWHITE); // Clear texture background
{
        let mut d = d.begin_mode3D(&camera); // Begin 3d mode drawing

        d.draw_model(&model, position, 0.5, Color::WHITE); // Draw 3d model with texture

        d.draw_grid(10, 1.0); // Draw a grid
}

        d.draw_text("TEXT DRAWN IN RENDER TEXTURE", 200, 10, 30,Color::RED);

}
{
        let mut d = d.begin_shader_mode(&shader);

        // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        d.draw_texture_rec(target.texture(), rrect(0, 0, target.texture().width, -target.texture().height), rvec2(0,  0), Color::WHITE);

}
        // Draw some 2d text over drawn texture
        d.draw_text("(c) Barracks 3D model by Alberto Cano", screen_width - 220, screen_height - 20, 10, Color::GRAY);

        d.draw_fps(10, 10);
    }
        //----------------------------------------------------------------------------------
    },
    );
}
