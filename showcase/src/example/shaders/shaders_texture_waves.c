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

#if defined(PLATFORM_DESKTOP)
#defconstL_VERSION 330
#else // PLATFORM_RPI, PLATFORM_ANDROID, PLATFORM_WEB
#defconstL_VERSION 100
#endif

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
    let texture = rl.load_texture(thread, "resources/space.png");

    // Load shader and setup location points and values
    Shader shader = LoadShader(0, &format!("resources/shaders/glsl{}/wave.fs", GLSL_VERSION));

    int secondsLoc = GetShaderLocation(shader, "secondes");
    int freqXLoc = GetShaderLocation(shader, "freqX");
    int freqYLoc = GetShaderLocation(shader, "freqY");
    int ampXLoc = GetShaderLocation(shader, "ampX");
    int ampYLoc = GetShaderLocation(shader, "ampY");
    int speedXLoc = GetShaderLocation(shader, "speedX");
    int speedYLoc = GetShaderLocation(shader, "speedY");

    // Shader uniform values that can be updated at any time
    float freqX = 25.0;
    float freqY = 25.0;
    float ampX = 5.0;
    float ampY = 5.0;
    float speedX = 8.0;
    float speedY = 8.0;

    float screenSize[2] = {(float)rl.get_screen_width(), (float)rl.get_screen_height()};
    SetShaderValue(shader, GetShaderLocation(shader, "size"), &screenSize, UNIFORM_VEC2);
    SetShaderValue(shader, freqXLoc, &freqX, UNIFORM_FLOAT);
    SetShaderValue(shader, freqYLoc, &freqY, UNIFORM_FLOAT);
    SetShaderValue(shader, ampXLoc, &ampX, UNIFORM_FLOAT);
    SetShaderValue(shader, ampYLoc, &ampY, UNIFORM_FLOAT);
    SetShaderValue(shader, speedXLoc, &speedX, UNIFORM_FLOAT);
    SetShaderValue(shader, speedYLoc, &speedY, UNIFORM_FLOAT);

    float seconds = 0.0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    // -------------------------------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        seconds += GetFrameTime();

        SetShaderValue(shader, secondsLoc, &seconds, UNIFORM_FLOAT);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        BeginShaderMode(shader);

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
