/*******************************************************************************************
*
*   raylib [textures] example - Bunnymark
*
*   This example has been created using raylib 1.6 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#include <stdlib.h> // Required for: malloc(), free()

const MAX_BUNNIES 50000 // 50K bunnies limit

    // This is the maximum amount of elements (quads) per batch
    // NOTE: This value is defined in [rlgl] module and can be changed there
    const MAX_BATCH_ELEMENTS 8192

    typedef struct Bunny
{
    Vector2 position;
    Vector2 speed;
    Color color;
} Bunny;

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
    rl.set_window_title(thread, "raylib [textures] example - bunnymark");


    // Load bunny texture
    let texBunny = rl.load_texture(thread, "resources/wabbit_alpha.png");

    Bunny *bunnies = (Bunny *)malloc(MAX_BUNNIES * sizeof(Bunny)); // Bunnies array

    int bunniesCount = 0; // Bunnies counter

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            // Create more bunnies
            for (int i = 0; i < 100; i+=1)
            {
                if bunniesCount < MAX_BUNNIES
                {
                    bunnies[bunniesCount].position = rl.get_mouse_position();
                    bunnies[bunniesCount].speed.x = (float)raylib::get_random_value::<i32>(-250, 250) / 60.0;
                    bunnies[bunniesCount].speed.y = (float)raylib::get_random_value::<i32>(-250, 250) / 60.0;
                    bunnies[bunniesCount].color = (Color){raylib::get_random_value::<i32>(50, 240),
                                                          raylib::get_random_value::<i32>(80, 240),
                                                          raylib::get_random_value::<i32>(100, 240), 255};
                    bunniesCount+=1;
                }
            }
        }

        // Update bunnies
        for (int i = 0; i < bunniesCount; i+=1)
        {
            bunnies[i].position.x += bunnies[i].speed.x;
            bunnies[i].position.y += bunnies[i].speed.y;

            if ((bunnies[i].position.x + texBunny.width / 2) > rl.get_screen_width() ||
                ((bunnies[i].position.x + texBunny.width / 2) < 0))
                bunnies[i].speed.x *= -1;
            if ((bunnies[i].position.y + texBunny.height / 2) > rl.get_screen_height() ||
                ((bunnies[i].position.y + texBunny.height / 2 - 40) < 0))
                bunnies[i].speed.y *= -1;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        for (int i = 0; i < bunniesCount; i+=1)
        {
            // NOTE: When internal batch buffer limit is reached (MAX_BATCH_ELEMENTS),
            // a draw call is launched and buffer starts being filled again;
            // before issuing a draw call, updated vertex data from internal CPU buffer is send to GPU...
            // Process of sending data is costly and it could happen that GPU data has not been completely
            // processed for drawing while new data is tried to be sent (updating current in-use buffers)
            // it could generates a stall and consequently a frame drop, limiting the number of drawn bunnies
            d.draw_texture(texBunny, bunnies[i].position.x, bunnies[i].position.y, bunnies[i].color);
        }

        d.draw_rectangle(0, 0, screen_width, 40, Color::BLACK);
        d.draw_text(&format!("bunnies: {}", bunniesCount), 120, 10, 20, Color::GREEN);
        d.draw_text(&format!("batched draw calls: {}", 1 + bunniesCount / MAX_BATCH_ELEMENTS), 320, 10, 20, Color::MAROON);

        d.draw_fps(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    free(bunnies); // Unload bunnies data array

    UnloadTexture(texBunny); // Unload bunny texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
