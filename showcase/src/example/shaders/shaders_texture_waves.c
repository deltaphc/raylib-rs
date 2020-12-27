/*******************************************************************************************
*
*   raylib [shaders] example - Texture Waves
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3), to test this example
*         on OpenGL ES 2.0 platforms (Android, Raspberry Pi, HTML5), use #version 100 shaders
*         raylib comes with shaders ready for both versions, check raylib/shaders install folder
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Anata (@anatagawa) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Anata (@anatagawa) and Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [shaders] example - texture waves");


    // Load texture texture to apply shaders
    let texture = rl.load_texture(thread, "original/shaders/resources/space.png");

    // Load shader and setup location points and values
    let shader = rl.load_shader(thread,0, &format!("original/shaders/resources/shaders/glsl{}/wave.fs", GLSL_VERSION));

    int secondsLoc = shader.get_shader_location( "secondes");
    int freqXLoc = shader.get_shader_location( "freqX");
    int freqYLoc = shader.get_shader_location( "freqY");
    int ampXLoc = shader.get_shader_location( "ampX");
    int ampYLoc = shader.get_shader_location( "ampY");
    int speedXLoc = shader.get_shader_location( "speedX");
    int speedYLoc = shader.get_shader_location( "speedY");

    // let uniform values that can be updated at any time
    float freqX = 25.0;
    float freqY = 25.0;
    float ampX = 5.0;
    float ampY = 5.0;
    float speedX = 8.0;
    float speedY = 8.0;

    float screenSize[2] = {(float)rl.get_screen_width(), (float)rl.get_screen_height()};
    shader.set_shader_value( shader.get_shader_location( "size"), &screenSize, UNIFORM_VEC2);
    shader.set_shader_value( freqXLoc, &freqX, UNIFORM_FLOAT);
    shader.set_shader_value( freqYLoc, &freqY, UNIFORM_FLOAT);
    shader.set_shader_value( ampXLoc, &ampX, UNIFORM_FLOAT);
    shader.set_shader_value( ampYLoc, &ampY, UNIFORM_FLOAT);
    shader.set_shader_value( speedXLoc, &speedX, UNIFORM_FLOAT);
    shader.set_shader_value( speedYLoc, &speedY, UNIFORM_FLOAT);

    float seconds = 0.0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    // -------------------------------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        seconds += GetFrameTime();

        shader.set_shader_value( secondsLoc, &seconds, UNIFORM_FLOAT);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        let mut d = d.begin_shader_mode(&shader);

        d.draw_texture(texture, 0, 0, Color::WHITE);
        d.draw_texture(texture, texture.width, 0, Color::WHITE);

        EndShaderMode();

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadShader(shader);   // Unload shader
    UnloadTexture(texture); // Unload texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
