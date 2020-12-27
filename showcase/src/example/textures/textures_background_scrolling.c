/*******************************************************************************************
*
*   raylib [textures] example - Background scrolling
*
*   This example has been created using raylib 2.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2019 Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [textures] example - background scrolling");


    // NOTE: Be careful, background width must be equal or bigger than screen width
    // if not, texture should be draw more than two times for scrolling effect
    let background = rl.load_texture(thread, "resources/cyberpunk_street_background.png");
    let midground = rl.load_texture(thread, "resources/cyberpunk_street_midground.png");
    let foreground = rl.load_texture(thread, "resources/cyberpunk_street_foreground.png");

    float scrollingBack = 0.0;
    float scrollingMid = 0.0;
    float scrollingFore = 0.0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        scrollingBack -= 0.1;
        scrollingMid -= 0.5;
        scrollingFore -= 1.0;

        // NOTE: Texture is scaled twice its size, so it sould be considered on scrolling
        if scrollingBack <= -background.width * 2
            scrollingBack = 0;
        if scrollingMid <= -midground.width * 2
            scrollingMid = 0;
        if scrollingFore <= -foreground.width * 2
            scrollingFore = 0;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        ClearBackground(GetColor(0x052c46ff));

        // Draw background image twice
        // NOTE: Texture is scaled twice its size
        d.draw_texture_ex(background, rvec2(scrollingBack,  20), 0.0, 2.0, Color::WHITE);
        d.draw_texture_ex(background, rvec2(background.width * 2 + scrollingBack,  20), 0.0, 2.0, Color::WHITE);

        // Draw midground image twice
        d.draw_texture_ex(midground, rvec2(scrollingMid,  20), 0.0, 2.0, Color::WHITE);
        d.draw_texture_ex(midground, rvec2(midground.width * 2 + scrollingMid,  20), 0.0, 2.0, Color::WHITE);

        // Draw foreground image twice
        d.draw_texture_ex(foreground, rvec2(scrollingFore,  70), 0.0, 2.0, Color::WHITE);
        d.draw_texture_ex(foreground, rvec2(foreground.width * 2 + scrollingFore,  70), 0.0, 2.0, Color::WHITE);

        d.draw_text("BACKGROUND SCROLLING & PARALLAX", 10, 10, 20,Color::RED);
        d.draw_text("(c) Cyberpunk Street Environment by Luis Zuno (@ansimuz)", screen_width - 330, screen_height - 20, 10, Color::RAYWHITE);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(background); // Unload background texture
    UnloadTexture(midground);  // Unload midground texture
    UnloadTexture(foreground); // Unload foreground texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}