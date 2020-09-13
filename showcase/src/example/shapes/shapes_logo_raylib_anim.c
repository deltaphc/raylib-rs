/*******************************************************************************************
*
*   raylib [shapes] example - raylib logo animation
*
*   This example has been created using raylib 2.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [shapes] example - raylib logo animation");


    int logoPositionX = screen_width / 2 - 128;
    int logoPositionY = screen_height / 2 - 128;

    int framesCounter = 0;
    int lettersCount = 0;

    int topSideRecWidth = 16;
    int leftSideRecHeight = 16;

    int bottomSideRecWidth = 16;
    int rightSideRecHeight = 16;

    int state = 0;      // Tracking animation states (State Machine)
    float alpha = 1.0; // Useful for fading

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if state == 0 // State 0: Small box blinking
        {
            framesCounter++;

            if framesCounter == 120
            {
                state = 1;
                framesCounter = 0; // Reset counter... will be used later...
            }
        }
        else if state == 1 // State 1: Top and left bars growing
        {
            topSideRecWidth += 4;
            leftSideRecHeight += 4;

            if topSideRecWidth == 256
                state = 2;
        }
        else if state == 2 // State 2: Bottom and right bars growing
        {
            bottomSideRecWidth += 4;
            rightSideRecHeight += 4;

            if bottomSideRecWidth == 256
                state = 3;
        }
        else if state == 3) // State 3: Letters appearing (one by one
        {
            framesCounter++;

            if framesCounter / 12 // Every 12 frames, one more letter!
            {
                lettersCount++;
                framesCounter = 0;
            }

            if lettersCount >= 10 // When all letters have appeared, just fade out everything
            {
                alpha -= 0.02f;

                if alpha <= 0.0
                {
                    alpha = 0.0;
                    state = 4;
                }
            }
        }
        else if state == 4 // State 4: Reset and Replay
        {
            if rl.is_key_pressed('R')
            {
                framesCounter = 0;
                lettersCount = 0;

                topSideRecWidth = 16;
                leftSideRecHeight = 16;

                bottomSideRecWidth = 16;
                rightSideRecHeight = 16;

                alpha = 1.0;
                state = 0; // Return to State 0
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if state == 0
        {
            if (framesCounter / 15) % 2
                d.draw_rectangle(logoPositionX, logoPositionY, 16, 16, Color::BLACK);
        }
        else if state == 1
        {
            d.draw_rectangle(logoPositionX, logoPositionY, topSideRecWidth, 16, Color::BLACK);
            d.draw_rectangle(logoPositionX, logoPositionY, 16, leftSideRecHeight, Color::BLACK);
        }
        else if state == 2
        {
            d.draw_rectangle(logoPositionX, logoPositionY, topSideRecWidth, 16, Color::BLACK);
            d.draw_rectangle(logoPositionX, logoPositionY, 16, leftSideRecHeight, Color::BLACK);

            d.draw_rectangle(logoPositionX + 240, logoPositionY, 16, rightSideRecHeight, Color::BLACK);
            d.draw_rectangle(logoPositionX, logoPositionY + 240, bottomSideRecWidth, 16, Color::BLACK);
        }
        else if state == 3
        {
            d.draw_rectangle(logoPositionX, logoPositionY, topSideRecWidth, 16, Fade(BLACK, alpha));
            d.draw_rectangle(logoPositionX, logoPositionY + 16, 16, leftSideRecHeight - 32, Fade(BLACK, alpha));

            d.draw_rectangle(logoPositionX + 240, logoPositionY + 16, 16, rightSideRecHeight - 32, Fade(BLACK, alpha));
            d.draw_rectangle(logoPositionX, logoPositionY + 240, bottomSideRecWidth, 16, Fade(BLACK, alpha));

            d.draw_rectangle(screen_width / 2 - 112, screen_height / 2 - 112, 224, 224, Fade(RAYWHITE, alpha));

            d.draw_text(TextSubtext("raylib", 0, lettersCount), screen_width / 2 - 44, screen_height / 2 + 48, 50, Fade(BLACK, alpha));
        }
        else if state == 4
        {
            d.draw_text("[R] REPLAY", 340, 200, 20, Color::GRAY);
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}