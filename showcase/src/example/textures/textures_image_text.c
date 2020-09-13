/*******************************************************************************************
*
*   raylib [texture] example - Image text drawing using TTF generated spritefont
*
*   This example has been created using raylib 1.8 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2017 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

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
    rl.set_window_title(thread, "raylib [texture] example - image text drawing");


    Image parrots = LoadImage("resources/parrots.png"); // Load image in CPU memory (RAM)

    // TTF Font loading with custom generation parameters
    Font font = LoadFontEx("resources/KAISG.ttf", 64, 0, 0);

    // Draw over image using custom font
    ImageDrawTextEx(&parrots, rvec2(20.0, 20.0), font, "[Parrots font drawing]", (float)font.baseSize, 0.0,Color::RED);

    Texture2D texture = LoadTextureFromImage(parrots); // Image converted to texture, uploaded to GPU memory (VRAM)
    UnloadImage(parrots);                              // Once image has been converted to texture and uploaded to VRAM, it can be unloaded from RAM

    let position = rvec2((float)(screen_width / 2 - texture.width / 2), (float)(screen_height / 2 - texture.height / 2 - 20));

    bool showFont = false;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE)
            showFont = true;
        else
            showFont = false;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if !showFont
        {
            // Draw texture with text already drawn inside
            DrawTextureV(texture, position, Color::WHITE);

            // Draw text directly using sprite font
            DrawTextEx(font, "[Parrots font drawing]", rvec2(position.x + 20,  position.y + 20 + 280), (float)font.baseSize, 0.0, Color::WHITE);
        }
        else
            d.draw_texture(font.texture, screen_width / 2 - font.texture.width / 2, 50, Color::BLACK);

        d.draw_text("PRESS SPACE to SEE USED SPRITEFONT ", 290, 420, 10, Color::DARKGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(texture); // Texture unloading

    UnloadFont(font); // Unload custom spritefont

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}