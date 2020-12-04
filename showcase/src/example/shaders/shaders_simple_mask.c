/*******************************************************************************************
*
*   raylib [shaders] example - Simple shader mask
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Camacho (@codifies) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Chris Camacho (@codifies) and Ramon Santamaria (@raysan5)
*
********************************************************************************************
*
*   After a model is loaded it has a default material, this material can be
*   modified in place rather than creating one from scratch...
*   While all of the maps have particular names, they can be used for any purpose
*   except for three maps that are applied as cubic maps (see below)
*
********************************************************************************************/

use raylib::prelude::*;


#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;


pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib - simple shader mask");


    // Define the camera to look into our 3d world
    let camera = Camera3D::perspective(
    rvec3(0.0, 1.0, 2.0),
    rvec3(0.0, 0.0, 0.0),
    rvec3(0.0, 1.0, 0.0),
    45.0,
    );

    // Define our three models to show the shader on
    Mesh torus = Mesh::gen_mesh_torus(thread,.3, 1, 16, 32);
    let model1 = rl.load_model_from_mesh(thread, torus).unwrap();

    Mesh cube = Mesh::gen_mesh_cube(thread,.8, .8, .8);
    let model2 = rl.load_model_from_mesh(thread, cube).unwrap();

    // Generate model to be shaded just to see the gaps in the other two
    Mesh sphere = Mesh::gen_mesh_sphere(thread,1, 16, 16);
    let model3 = rl.load_model_from_mesh(thread, sphere).unwrap();

    // Load the shader
    let shader = rl.load_shader(thread,0, &format!("resources/shaders/glsl{}/mask.fs", GLSL_VERSION));

    // Load and apply the diffuse texture (colour map)
    let texDiffuse = rl.load_texture(thread, "original/resources/plasma.png");
    model1.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = texDiffuse;
    model2.materials[0].maps[raylib::consts::MaterialMapType::MAP_ALBEDO].texture = texDiffuse;

    // Using MAP_EMISSION as a spare slot to use for 2nd texture
    // NOTE: Don't use MAP_IRRADIANCE, MAP_PREFILTER or  MAP_CUBEMAP
    // as they are bound as cube maps
    let texMask = rl.load_texture(thread, "original/resources/mask.png");
    model1.materials[0].maps[MAP_EMISSION].texture = texMask;
    model2.materials[0].maps[MAP_EMISSION].texture = texMask;
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::LOC_MAP_EMISSION] = shader.get_shader_location( "mask");

    // Frame is incremented each frame to animate the shader
    int shaderFrame = shader.get_shader_location( "frame");

    // Apply the shader to the two models
    model1.materials[0].shader = shader;
    model2.materials[0].shader = shader;

    int framesCounter = 0;
    Vector3 rotation = {0}; // Model rotation angles

    rl.set_target_fps(60); // Set  to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        framesCounter++;
        rotation.x += 0.01f;
        rotation.y += 0.005f;
        rotation.z -= 0.0025;

        // Send frames counter to shader for animation
        shader.set_shader_value( shaderFrame, &framesCounter, UNIFORM_INT);

        // Rotate one of the models
        model1.transform = Matrix::rotate_xYZ(rotation);

        rl.update_camera(&mut camera);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::Color::DARKBLUE);

        let mut d = d.begin_mode3D(&camera);

        d.draw_model(model1, rvec3(0.5, 0,  0), 1, Color::WHITE);
        d.draw_modelEx(model2, rvec3(-.5, 0,  0), rvec3(1, 1,  0), 50, rvec3(1, 1,  1), Color::WHITE);
        d.draw_model(model3, rvec3(0, 0,  -1.5), 1, Color::WHITE);
        d.draw_grid(10, 1.0); // Draw a grid

        EndMode3D();

        d.draw_rectangle(16, 698, raylib::text::measure_text&format!("Frame: {}", framesCounter), 20) + 8, 42, Color::BLUE);
        d.draw_text(&format!("Frame: {}", framesCounter), 20, 700, 20, Color::WHITE);

        d.draw_fps(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadModel(model1);
    UnloadModel(model2);
    UnloadModel(model3);

    UnloadTexture(texDiffuse); // Unload default diffuse texture
    UnloadTexture(texMask);    // Unload texture mask

    UnloadShader(shader); // Unload shader

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
