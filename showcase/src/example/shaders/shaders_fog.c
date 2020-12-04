/*******************************************************************************************
*
*   raylib [shaders] example - fog
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3).
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Camacho (@codifies) and reviewed by Ramon Santamaria (@raysan5)
*
*   Chris Camacho (@codifies -  http://bedroomcoders.co.uk/) notes:
*
*   This is based on the PBR lighting example, but greatly simplified to aid learning...
*   actually there is very little of the PBR example left!
*   When I first looked at the bewildering complexity of the PBR example I feared
*   I would never understand how I could do simple lighting with raylib however its
*   a testement to the authors of raylib (including rlights.h) that the example
*   came together fairly quickly.
*
*   Copyright (c) 2019 Chris Camacho (@codifies) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;




use raylib::rlights;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;


pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    SetConfigFlags(FLAG_MSAA_4X_HINT); // Enable Multi Sampling Anti Aliasing 4x (if available)
    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - fog");


    // Define the camera to look into our 3d world
    Camera camera = {
        rvec3(2.0, 2.0, 6.0), // position
        rvec3(0.0, 0.5, 0.0), // target
        rvec3(0.0, 1.0, 0.0), // up
        45.0, CAMERA_PERSPECTIVE};  // fov, type

    // Load models and texture
    let modelA = rl.load_model_from_mesh(thread, rl.gen_mesh_torus(thread,0.4, 1.0, 16, 32)).unwrap();
    let modelB = rl.load_model_from_mesh(thread, rl.gen_mesh_cube(thread,1.0, 1.0, 1.0)).unwrap();
    let modelC = rl.load_model_from_mesh(thread, rl.gen_mesh_sphere(thread,0.5, 32, 32)).unwrap();
    let texture = rl.load_texture(thread, "original/resources/texel_checker.png");

    // Assign texture to default model material
    modelA.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = *texture.as_ref();
    modelB.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = *texture.as_ref();
    modelC.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = *texture.as_ref();

    // Load shader and set up some uniforms
    let shader = rl.load_shader(thread,&format!("resources/shaders/glsl{}/base_lighting.vs", GLSL_VERSION),
                               &format!("resources/shaders/glsl{}/fog.fs", GLSL_VERSION));
    shader.locs[LOC_MATRIX_MODEL] = shader.get_shader_location( "matModel");
    shader.locs[LOC_VECTOR_VIEW] = shader.get_shader_location( "viewPos");

    // Ambient light level
    int ambientLoc = shader.get_shader_location( "ambient");
    shader.set_shader_value( ambientLoc, (float[4]){0.2, 0.2, 0.2, 1.0}, UNIFORM_VEC4);

    float fogDensity = 0.15f;
    int fogDensityLoc = shader.get_shader_location( "fogDensity");
    shader.set_shader_value( fogDensityLoc, &fogDensity, UNIFORM_FLOAT);

    // NOTE: All models share the same shader
    modelA.materials[0].shader = shader;
    modelB.materials[0].shader = shader;
    modelC.materials[0].shader = shader;

    // Using just 1 point lights
    CreateLight(LIGHT_POINT, rvec3(0, 2,  6), Vector3::zero()(), Color::WHITE, shader);

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
        {
            fogDensity += 0.001;
            if fogDensity > 1.0
                fogDensity = 1.0;
        }

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
        {
            fogDensity -= 0.001;
            if fogDensity < 0.0
                fogDensity = 0.0;
        }

        shader.set_shader_value( fogDensityLoc, &fogDensity, UNIFORM_FLOAT);

        // Rotate the torus
        modelA.transform = MatrixMultiply(modelA.transform, MatrixRotateX(-0.025));
        modelA.transform = MatrixMultiply(modelA.transform, MatrixRotateZ(0.012));

        // Update the light shader with the camera view position
        shader.set_shader_value( shader.locs[LOC_VECTOR_VIEW], &camera.position.x, UNIFORM_VEC3);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::GRAY);

        let mut d = d.begin_mode3D(&camera);

        // Draw the three models
        d.draw_model(modelA, Vector3::zero()(), 1.0, Color::WHITE);
        d.draw_model(modelB, rvec3(-2.6, 0,  0), 1.0, Color::WHITE);
        d.draw_model(modelC, rvec3(2.6, 0,  0), 1.0, Color::WHITE);

        for (int i = -20; i < 20; i += 2)
            d.draw_model(modelA, rvec3(i, 0,  2), 1.0, Color::WHITE);

        EndMode3D();

        d.draw_text(&format!("Use KEY_UP/KEY_DOWN to change fog density [%.2]", fogDensity), 10, 10, 20, Color::RAYWHITE);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadModel(modelA);    // Unload the model A
    UnloadModel(modelB);    // Unload the model B
    UnloadModel(modelC);    // Unload the model C
    UnloadTexture(texture); // Unload the texture
    UnloadShader(shader);   // Unload shader

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
