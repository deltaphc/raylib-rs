/*******************************************************************************************
*
*   raylib [shaders] example - basic lighting
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
    rl.set_window_title(thread, "raylib [shaders] example - basic lighting");


    // Define the camera to look into our 3d world
    let camera = Camera3D::perspective(
    rvec3(2.0, 2.0, 6.0), // Camera position
    rvec3(0.0, 0.5, 0.0),   // Camera looking at point
    rvec3(0.0, 1.0, 0.0),       // Camera up vector (rotation towards target)
    45.0,                           // Camera field-of-view Y
    );              // Camera mode type

    // Load models
    let modelA = rl.load_model_from_mesh(thread, rl.gen_mesh_torus(thread,0.4, 1.0, 16, 32)).unwrap();
    let modelA = rl.load_model_from_mesh(thread, rl.gen_mesh_torus(thread,0.4, 1.0, 16, 32)).unwrap();
    let modelB = rl.load_model_from_mesh(thread, rl.gen_mesh_cube(thread,1.0, 1.0, 1.0)).unwrap();
    let modelC = rl.load_model_from_mesh(thread, rl.gen_mesh_sphere(thread,0.5, 32, 32)).unwrap();

    // Load models texture
    let texture = rl.load_texture(thread, "original/resources/texel_checker.png");

    // Assign texture to default model material
    modelA.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = *texture.as_ref();
    modelB.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = *texture.as_ref();
    modelC.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = *texture.as_ref();

    let shader = rl.load_shader(thread,&format!("resources/shaders/glsl{}/base_lighting.vs", GLSL_VERSION),
                               &format!("resources/shaders/glsl{}/lighting.fs", GLSL_VERSION)).unwrap();

    // Get some shader loactions
    shader.locs[LOC_MATRIX_MODEL] = shader.get_shader_location( "matModel");
    shader.locs[LOC_VECTOR_VIEW] = shader.get_shader_location( "viewPos");

    // ambient light level
    let ambientLoc = shader.get_shader_location( "ambient");
    shader.set_shader_value( ambientLoc, Vector4::new(0.2, 0.2, 0.2, 1.0));

    let angle = 6.282f;

    // All models use the same shader
    modelA.materials[0].shader = shader;
    modelB.materials[0].shader = shader;
    modelC.materials[0].shader = shader;

    // Using 4 point lights, white,Color::RED, green and blue
    let lights = [
        rlights::create_light(raylights::LightType::LIGHT_POINT, rvec3(4, 2,  4), Vector3::zero(), Color::WHITE, &shader),
         rlights::create_light(raylights::LightType::LIGHT_POINT, rvec3(4, 2,  4), Vector3::zero(),Color::RED, &shader),
         rlights::create_light(raylights::LightType::LIGHT_POINT, rvec3(0, 4,  2), Vector3::zero(), Color::GREEN, &shader),
         rlights::create_light(raylights::LightType::LIGHT_POINT, rvec3(0, 4,  2), Vector3::zero(), Color::BLUE, &shader),
    ];

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_W)
        {
            lights[0].enabled = !lights[0].enabled;
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_R)
        {
            lights[1].enabled = !lights[1].enabled;
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_G)
        {
            lights[2].enabled = !lights[2].enabled;
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_B)
        {
            lights[3].enabled = !lights[3].enabled;
        }

        rl.update_camera(&mut camera); // Update camera

        // Make the lights do differing orbits
        angle -= 0.02f;
        lights[0].position.x = cosf(angle) * 4.0;
        lights[0].position.z = sinf(angle) * 4.0;
        lights[1].position.x = cosf(-angle * 0.6f) * 4.0;
        lights[1].position.z = sinf(-angle * 0.6f) * 4.0;
        lights[2].position.y = cosf(angle * 0.2) * 4.0;
        lights[2].position.z = sinf(angle * 0.2) * 4.0;
        lights[3].position.y = cosf(-angle * 0.35f) * 4.0;
        lights[3].position.z = sinf(-angle * 0.35f) * 4.0;

        rlights::update_light_values(&shader, lights[0].clone());
        rlights::update_light_values(&shader, lights[1].clone());
        rlights::update_light_values(&shader, lights[2].clone());
        rlights::update_light_values(&shader, lights[3].clone());

        // Rotate the torus
        modelA.transform = MatrixMultiply(modelA.transform, MatrixRotateX(-0.025f));
        modelA.transform = MatrixMultiply(modelA.transform, MatrixRotateZ(0.012f));

        // Update the light shader with the camera view position
        let cameraPos = Vector3::new(camera.position.x, camera.position.y, camera.position.z);
        shader.set_shader_value( shader.locs[LOC_VECTOR_VIEW], cameraPos);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        let mut d = d.begin_mode3D(&camera);

        // Draw the three models
        d.draw_model(modelA, Vector3::zero()(), 1.0, Color::WHITE);
        d.draw_model(modelB, rvec3(-1.6, 0,  0), 1.0, Color::WHITE);
        d.draw_model(modelC, rvec3(1.6, 0,  0), 1.0, Color::WHITE);

        // Draw markers to show where the lights are
        if lights[0].enabled
        {
            DrawSphereEx(lights[0].position, 0.2, 8, 8, Color::WHITE);
        }
        if lights[1].enabled
        {
            DrawSphereEx(lights[1].position, 0.2, 8, 8,Color::RED);
        }
        if lights[2].enabled
        {
            DrawSphereEx(lights[2].position, 0.2, 8, 8, Color::GREEN);
        }
        if lights[3].enabled
        {
            DrawSphereEx(lights[3].position, 0.2, 8, 8, Color::BLUE);
        }

        d.draw_grid(10, 1.0);

        EndMode3D();

        d.draw_fps(10, 10);

        d.draw_text("Use keys RGBW to toggle lights", 10, 30, 20, Color::DARKGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    });

    // // De-Initialization
    // //--------------------------------------------------------------------------------------
    // UnloadModel(modelA); // Unload the modelA
    // UnloadModel(modelB); // Unload the modelB
    // UnloadModel(modelC); // Unload the modelC

    // UnloadTexture(texture); // Unload the texture
    // UnloadShader(shader);   // Unload shader

    // CloseWindow(); // Close window and OpenGL context
    // //--------------------------------------------------------------------------------------

    // return 0;
}
