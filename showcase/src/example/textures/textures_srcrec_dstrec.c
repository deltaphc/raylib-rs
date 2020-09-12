/*******************************************************************************************
*
*   raylib [textures] example - Texture source and destination rectangles
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
    rl.set_window_title(thread, "raylib [textures] examples - texture source and destination rectangles");


    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)

    Texture2D scarfy = LoadTexture("resources/scarfy.png"); // Texture loading

    int frameWidth = scarfy.width / 6;
    int frameHeight = scarfy.height;

    // Source rectangle (part of the texture to use for drawing)
    Rectangle sourceRec = {0.0, 0.0, frameWidth, frameHeight};

    // Destination rectangle (screen rectangle where drawing part of texture)
    Rectangle destRec = {screen_width / 2, screen_height / 2, frameWidth * 2, frameHeight * 2};

    // Origin of the texture (rotation/scale point), it's relative to destination rectangle size
    Vector2 origin = {frameWidth, frameHeight};

    int rotation = 0;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rotation++;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        // NOTE: Using DrawTexturePro() we can easily rotate and scale the part of the texture we draw
        // sourceRec defines the part of the texture we use for drawing
        // destRec defines the rectangle where our texture part will fit (scaling it to fit)
        // origin defines the point of the texture used as reference for rotation and scaling
        // rotation defines the texture rotation (using origin as rotation point)
        DrawTexturePro(scarfy, sourceRec, destRec, origin, (float)rotation, WHITE);

        DrawLine((int)destRec.x, 0, (int)destRec.x, screen_height, GRAY);
        DrawLine(0, (int)destRec.y, screen_width, (int)destRec.y, GRAY);

        d.draw_text("(c) Scarfy sprite by Eiden Marsal", screen_width - 200, screen_height - 20, 10, GRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(scarfy); // Texture unloading

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}