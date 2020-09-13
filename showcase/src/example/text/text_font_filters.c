/*******************************************************************************************
*
*   raylib [text] example - Font filters
*
*   After font loading, font texture atlas filter could be configured for a softer
*   display of the font when scaling it to different sizes, that way, it's not required
*   to generate multiple fonts at multiple sizes (as long as the scaling is not very different)
*
*   This example has been created using raylib 1.3.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [text] example - font filters");


    const char msg[50] = "Loaded Font";

    // NOTE: Textures/Fonts MUST be loaded after Window initialization (OpenGL context is required)

    // TTF Font loading with custom generation parameters
    Font font = LoadFontEx("resources/KAISG.ttf", 96, 0, 0);

    // Generate mipmap levels to use trilinear filtering
    // NOTE: On 2D drawing it won't be noticeable, it looks like FILTER_BILINEAR
    GenTextureMipmaps(&font.texture);

    float fontSize = font.baseSize;
    let fontPosition = rvec2(40, screen_height / 2 - 80);
    let textSize = rvec2(0.0, 0.0);

    // Setup texture scaling filter
    SetTextureFilter(font.texture, FILTER_POINT);
    int currentFontFilter = 0; // FILTER_POINT

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        fontSize += rl.get_mouse_wheel_move() * 4.0;

        // Choose font texture filter method
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ONE)
        {
            SetTextureFilter(font.texture, FILTER_POINT);
            currentFontFilter = 0;
        }
        else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_TWO)
        {
            SetTextureFilter(font.texture, FILTER_BILINEAR);
            currentFontFilter = 1;
        }
        else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_THREE)
        {
            // NOTE: Trilinear filter won't be noticed on 2D drawing
            SetTextureFilter(font.texture, FILTER_TRILINEAR);
            currentFontFilter = 2;
        }

        textSize = MeasureTextEx(font, msg, fontSize, 0);

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT)
            fontPosition.x -= 10;
        else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT)
            fontPosition.x += 10;

        // Load a dropped TTF file dynamically (at current fontSize)
        if IsFileDropped()
        {
            int count = 0;
            char **droppedFiles = GetDroppedFiles(&count);

            // NOTE: We only support first ttf file dropped
            if IsFileExtension(droppedFiles[0], ".ttf")
            {
                UnloadFont(font);
                font = LoadFontEx(droppedFiles[0], fontSize, 0, 0);
                ClearDroppedFiles();
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("Use mouse wheel to change font size", 20, 20, 10, Color::GRAY);
        d.draw_text("Use KEY_RIGHT and KEY_LEFT to move text", 20, 40, 10, Color::GRAY);
        d.draw_text("Use 1, 2, 3 to change texture filter", 20, 60, 10, Color::GRAY);
        d.draw_text("Drop a new TTF font for dynamic loading", 20, 80, 10, Color::DARKGRAY);

        DrawTextEx(font, msg, fontPosition, fontSize, 0, Color::BLACK);

        // TODO: It seems texSize measurement is not accurate due to chars offsets...
        //d.draw_rectangle_lines(fontPosition.x, fontPosition.y, textSize.x, textSize.y,Color::RED);

        d.draw_rectangle(0, screen_height - 80, screen_width, 80, Color::LIGHTGRAY);
        d.draw_text(&format!("Font size: %02.02f", fontSize), 20, screen_height - 50, 10, Color::DARKGRAY);
        d.draw_text(&format!("Text size: [%02.02f, %02.02f]", textSize.x, textSize.y), 20, screen_height - 30, 10, Color::DARKGRAY);
        d.draw_text("CURRENT TEXTURE FILTER:", 250, 400, 20, Color::GRAY);

        if currentFontFilter == 0
            d.draw_text("POINT", 570, 400, 20, Color::BLACK);
        else if currentFontFilter == 1
            d.draw_text("BILINEAR", 570, 400, 20, Color::BLACK);
        else if currentFontFilter == 2
            d.draw_text("TRILINEAR", 570, 400, 20, Color::BLACK);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    ClearDroppedFiles(); // Clear internal buffers

    UnloadFont(font); // Font unloading

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}