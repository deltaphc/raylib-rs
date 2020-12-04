/*******************************************************************************************
*
*   raylib [textures] example - Texture drawing
*
*   This example illustrates how to draw on a blank texture using a shader
*
*   This example has been created using raylib 2.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Michał Ciesielski and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Michał Ciesielski and Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [shaders] example - texture drawing");


    Image imBlank = GenImageColor(1024, 1024, BLANK);
    let texture = rl.load_texture_from_image(&thread, &imBlank); // Load blank texture to fill on shader
    UnloadImage(imBlank);

    // NOTE: Using GLSL 330 shader version, on OpenGL ES 2.0 use GLSL 100 shader version
    let shader = rl.load_shader(thread,0, &format!("resources/shaders/glsl{}/cubes_panning.fs", GLSL_VERSION));

    float time = 0.0;
    int timeLoc = shader.get_shader_location( "uTime");
    shader.set_shader_value( timeLoc, &time, UNIFORM_FLOAT);

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    // -------------------------------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        time = GetTime();
        shader.set_shader_value( timeLoc, &time, UNIFORM_FLOAT);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        BeginShaderMode(shader);           // Enable our custom shader for next shapes/textures drawings
        d.draw_texture(texture, 0, 0, Color::WHITE); // Drawing BLANK texture, all magic happens on shader
        EndShaderMode();                   // Disable our custom shader, return to default shader

        d.draw_text("BACKGROUND is PAINTED and ANIMATED on SHADER!", 10, 10, 20, Color::MAROON);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadShader(shader);

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
