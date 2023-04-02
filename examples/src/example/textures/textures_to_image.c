/*******************************************************************************************
*
*   raylib [textures] example - Retrieve image data from texture: GetTextureData()
*
*   NOTE: Images are loaded in CPU memory (RAM); textures are loaded in GPU memory (VRAM)
*
*   This example has been created using raylib 1.3 (www.raylib.com)
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
    rl.set_window_title(thread, "raylib [textures] example - texture to image");


    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)

    let image = Image::load_image("original/models/resources/raylib_logo.png").unwrap(); // Load image data into CPU memory (RAM)
    let texture = rl.load_texture_from_image(thread, &image).unwrap();      // Image converted to texture, GPU memory (RAM -> VRAM)
    UnloadImage(image);                                   // Unload image data from CPU memory (RAM)

    image = GetTextureData(texture); // Retrieve image data from GPU memory (VRAM -> RAM)
    UnloadTexture(texture);          // Unload texture from GPU memory (VRAM)

    texture = LoadTextureFromImage(image); // Recreate texture from retrieved image data (RAM -> VRAM)
    UnloadImage(image);                    // Unload retrieved image data from CPU memory (RAM)
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

        d.draw_texture(texture, screen_width / 2 - texture.width / 2, screen_height / 2 - texture.height / 2, Color::WHITE);

        d.draw_text("this IS a texture loaded from an image!", 300, 370, 10, Color::GRAY);

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