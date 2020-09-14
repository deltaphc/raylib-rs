/*******************************************************************************************
*
*   raylib [textures] example - sprite explosion
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2019 Anata and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const NUM_FRAMES 8 const NUM_LINES 6

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - sprite explosion");


    let mut audio = RaylibAudio::init_audio_device();

    // Load explosion sound
    Sound fxBoom = Sound::load_sound("resources/boom.wav");

    // Load explosion texture
    let explosion = rl.load_texture(thread, "resources/explosion.png");

    // Init variables for animation
    int frameWidth = explosion.width / NUM_FRAMES;  // Sprite one frame rectangle width
    int frameHeight = explosion.height / NUM_LINES; // Sprite one frame rectangle height
    int currentFrame = 0;
    int currentLine = 0;

    let frameRec  = rrect(0,  0,  frameWidth,  frameHeight);
    let position = rvec2(0.0, 0.0);

    bool active = false;
    int framesCounter = 0;

    SetTargetFPS(120);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------

        // Check for mouse button pressed and activate explosion (if not active)
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) && !active
        {
            position = rl.get_mouse_position();
            active = true;

            position.x -= frameWidth / 2;
            position.y -= frameHeight / 2;

            PlaySound(fxBoom);
        }

        // Compute explosion animation frames
        if active
        {
            framesCounter++;

            if framesCounter > 2
            {
                currentFrame++;

                if currentFrame >= NUM_FRAMES
                {
                    currentFrame = 0;
                    currentLine++;

                    if currentLine >= NUM_LINES
                    {
                        currentLine = 0;
                        active = false;
                    }
                }

                framesCounter = 0;
            }
        }

        frameRec.x = frameWidth * currentFrame;
        frameRec.y = frameHeight * currentLine;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        // Draw explosion required frame rectangle
        if active
            DrawTextureRec(explosion, frameRec, position, Color::WHITE);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(explosion); // Unload texture
    UnloadSound(fxBoom);      // Unload sound

    CloseAudioDevice();

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}