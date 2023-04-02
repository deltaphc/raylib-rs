/*******************************************************************************************
*
*   raylib [text] example - TTF loading and usage
*
*   This example has been created using raylib 1.3.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION 330
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION 100
#endif

#include <stdlib.h>

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [text] example - SDF fonts");


    // NOTE: Textures/Fonts MUST be loaded after Window initialization (OpenGL context is required)

    const char msg[50] = "Signed Distance Fields";

    // Default font generation from TTF font
    Font fontDefault = {0};
    fontDefault.baseSize = 16;
    fontDefault.charsCount = 95;
    // Parameters > font size: 16, no chars array provided (0), chars count: 95 (autogenerate chars array)
    fontDefault.chars = LoadFontData("resources/AnonymousPro-Bold.ttf", 16, 0, 95, FONT_DEFAULT);
    // Parameters > chars count: 95, font size: 16, chars padding in image: 4 px, pack method: 0 (default)
    Image atlas = GenImageFontAtlas(fontDefault.chars, &fontDefault.recs, 95, 16, 4, 0);
    fontDefault.texture = LoadTextureFromImage(atlas);
    UnloadImage(atlas);

    // SDF font generation from TTF font
    Font fontSDF = {0};
    fontSDF.baseSize = 16;
    fontSDF.charsCount = 95;
    // Parameters > font size: 16, no chars array provided (0), chars count: 0 (defaults to 95)
    fontSDF.chars = LoadFontData("resources/AnonymousPro-Bold.ttf", 16, 0, 0, FONT_SDF);
    // Parameters > chars count: 95, font size: 16, chars padding in image: 0 px, pack method: 1 (Skyline algorythm)
    atlas = GenImageFontAtlas(fontSDF.chars, &fontSDF.recs, 95, 16, 0, 1);
    fontSDF.texture = LoadTextureFromImage(atlas);
    UnloadImage(atlas);

    // Load SDF required shader (we use default vertex shader)
    let shader = rl.load_shader(thread,0, &format!("resources/shaders/glsl{}/sdf.fs", GLSL_VERSION));
    SetTextureFilter(fontSDF.texture, FILTER_BILINEAR); // Required for SDF font

    let fontPosition = rvec2(40, screen_height / 2 - 50);
    let textSize = rvec2(0.0, 0.0);
    float fontSize = 16.0;
    int currentFont = 0; // 0 - fontDefault, 1 - fontSDF

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        fontSize += rl.get_mouse_wheel_move() * 8.0;

        if fontSize < 6
            fontSize = 6;

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE)
            currentFont = 1;
        else
            currentFont = 0;

        if currentFont == 0
            textSize = MeasureTextEx(fontDefault, msg, fontSize, 0);
        else
            textSize = MeasureTextEx(fontSDF, msg, fontSize, 0);

        fontPosition.x = rl.get_screen_width() / 2 - textSize.x / 2;
        fontPosition.y = rl.get_screen_height() / 2 - textSize.y / 2 + 80;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if currentFont == 1
        {
            // NOTE: SDF fonts require a custom SDf shader to compute fragment color
            let mut d = d.begin_shader_mode(&shader); // Activate SDF font shader
            DrawTextEx(fontSDF, msg, fontPosition, fontSize, 0, Color::BLACK);
            EndShaderMode(); // Activate our default shader for next drawings

            d.draw_texture(fontSDF.texture, 10, 10, Color::BLACK);
        }
        else
        {
            DrawTextEx(fontDefault, msg, fontPosition, fontSize, 0, Color::BLACK);
            d.draw_texture(fontDefault.texture, 10, 10, Color::BLACK);
        }

        if currentFont == 1
            d.draw_text("SDF!", 320, 20, 80,Color::RED);
        else
            d.draw_text("default font", 315, 40, 30, Color::GRAY);

        d.draw_text("FONT SIZE: 16.0", rl.get_screen_width() - 240, 20, 20, Color::DARKGRAY);
        d.draw_text(&format!("RENDER SIZE: %02.02f", fontSize), rl.get_screen_width() - 240, 50, 20, Color::DARKGRAY);
        d.draw_text("Use MOUSE WHEEL to SCALE TEXT!", rl.get_screen_width() - 240, 90, 10, Color::DARKGRAY);

        d.draw_text("HOLD SPACE to USE SDF FONT VERSION!", 340, rl.get_screen_height() - 30, 20, Color::MAROON);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadFont(fontDefault); // Default font unloading
    UnloadFont(fontSDF);     // SDF font unloading

    UnloadShader(shader); // Unload SDF shader

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}