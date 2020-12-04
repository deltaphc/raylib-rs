/*******************************************************************************************
*
*   raylib [shaders] example - Sieve of Eratosthenes
*
*   Sieve of Eratosthenes, the earliest known (ancient Greek) prime number sieve.
*
*   "Sift the twos and sift the threes,
*    The Sieve of Eratosthenes.
*    When the multiples sublime,
*    the numbers that are left are prime."
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3).
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by ProfJski and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 ProfJski and Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [shaders] example - Sieve of Eratosthenes");


    RenderTexture2D target = LoadRenderTexture(screen_width, screen_height);

    // Load Eratosthenes shader
    // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
    let shader = rl.load_shader(thread,0, &format!("resources/shaders/glsl{}/eratosthenes.fs", GLSL_VERSION));

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // Nothing to do here, everything is happening in the shader
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        let mut d = d.begin_texture_mode(thread, &target); // Enable drawing to texture
        d.clear_background(Color::BLACK);   // Clear the render texture

        // Draw a rectangle in shader mode to be used as shader canvas
        // NOTE: Rectangle uses font white character texture coordinates,
        // so shader can not be applied here directly because input vertexTexCoord
        // do not represent full screen coordinates (space where want to apply shader)
        d.draw_rectangle(0, 0, rl.get_screen_width(), rl.get_screen_height(), Color::BLACK);
        EndTextureMode(); // End drawing to texture (now we have a blank texture available for the shader)

        BeginShaderMode(shader);
        // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        DrawTextureRec(target.texture, rrect(0, 0, target.texture.width, -target.texture.height), rvec2(0.0, 0.0), Color::WHITE);
        EndShaderMode();

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadShader(shader);        // Unload shader
    UnloadRenderTexture(target); // Unload texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
