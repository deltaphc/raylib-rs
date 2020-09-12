/*******************************************************************************************
*
*   raylib [textures] example - sprite button
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const NUM_FRAMES 3 // Number of frames (rectangles) for the button sprite texture

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - sprite button");


    InitAudioDevice(); // Initialize audio device

    Sound fxButton = LoadSound("resources/buttonfx.wav");   // Load button sound
    Texture2D button = LoadTexture("resources/button.png"); // Load button texture

    // Define frame rectangle for drawing
    int frameHeight = button.height / NUM_FRAMES;
    Rectangle sourceRec = {0, 0, button.width, frameHeight};

    // Define button bounds on screen
    Rectangle btnBounds = {screen_width / 2 - button.width / 2, screen_height / 2 - button.height / NUM_FRAMES / 2, button.width, frameHeight};

    int btnState = 0;       // Button state: 0-NORMAL, 1-MOUSE_HOVER, 2-PRESSED
    bool btnAction = false; // Button action should be activated

    Vector2 mousePoint = {0.0, 0.0};

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        mousePoint = rl.get_mouse_position();
        btnAction = false;

        // Check button state
        if (CheckCollisionPointRec(mousePoint, btnBounds))
        {
            if (IsMouseButtonDown(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON))
                btnState = 2;
            else
                btnState = 1;

            if (IsMouseButtonReleased(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON))
                btnAction = true;
        }
        else
            btnState = 0;

        if (btnAction)
        {
            PlaySound(fxButton);

            // TODO: Any desired action
        }

        // Calculate button frame rectangle to draw depending on button state
        sourceRec.y = btnState * frameHeight;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        DrawTextureRec(button, sourceRec, (Vector2){btnBounds.x, btnBounds.y}, WHITE); // Draw button frame

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(button); // Unload button texture
    UnloadSound(fxButton); // Unload sound

    CloseAudioDevice(); // Close audio device

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}