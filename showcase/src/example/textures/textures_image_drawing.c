/*******************************************************************************************
*
*   raylib [textures] example - Image loading and drawing on it
*
*   NOTE: Images are loaded in CPU memory (RAM); textures are loaded in GPU memory (VRAM)
*
*   This example has been created using raylib 1.4 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2016 Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [textures] example - image drawing");


    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)

    Image cat = LoadImage("resources/cat.png");      // Load image in CPU memory (RAM)
    ImageCrop(&cat, rrect(100, 10, 280, 380)); // Crop an image piece
    ImageFlipHorizontal(&cat);                       // Flip cropped image horizontally
    ImageResize(&cat, 150, 200);                     // Resize flipped-cropped image

    Image parrots = LoadImage("resources/parrots.png"); // Load image in CPU memory (RAM)

    // Draw one image over the other with a scaling of 1.5
    ImageDraw(&parrots, cat, rrect(0, 0, cat.width, cat.height), rrect(30, 40, cat.width * 1.5, cat.height * 1.5), Color::WHITE);
    ImageCrop(&parrots, rrect(0, 50, parrots.width, parrots.height - 100)); // Crop resulting image

    // Draw on the image with a few image draw methods
    ImageDrawPixel(&parrots, 10, 10, Color::RAYWHITE);
    Imaged.draw_circle(&parrots, 10, 10, 5, Color::RAYWHITE);
    Imaged.draw_rectangle(&parrots, 5, 20, 10, 10, Color::RAYWHITE);

    UnloadImage(cat); // Unload image from RAM

    // Load custom font for frawing on image
    Font font = LoadFont("resources/custom_jupiter_crash.png");

    // Draw over image using custom font
    ImageDrawTextEx(&parrots, rvec2(300, 230), font, "PARROTS & CAT", font.baseSize, -2, Color::WHITE);

    UnloadFont(font); // Unload custom spritefont (already drawn used on image)

    Texture2D texture = LoadTextureFromImage(parrots); // Image converted to texture, uploaded to GPU memory (VRAM)
    UnloadImage(parrots);                              // Once image has been converted to texture and uploaded to VRAM, it can be unloaded from RAM

    rl.set_target_fps(60);
    //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // TODO: Update your variables here
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_texture(texture, screen_width / 2 - texture.width / 2, screen_height / 2 - texture.height / 2 - 40, Color::WHITE);
        d.draw_rectangle_lines(screen_width / 2 - texture.width / 2, screen_height / 2 - texture.height / 2 - 40, texture.width, texture.height, Color::DARKGRAY);

        d.draw_text("We are drawing only one texture from various images composed!", 240, 350, 10, Color::DARKGRAY);
        d.draw_text("Source images have been cropped, scaled, flipped and copied one over the other.", 190, 370, 10, Color::DARKGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(texture); // Texture unloading

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}