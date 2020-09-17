/*******************************************************************************************
*
*   raylib [text] example - Input Box
*
*   This example has been created using raylib 1.7 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2017 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_INPUT_CHARS 9

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [text] example - input box");


    char name[MAX_INPUT_CHARS + 1] = "\0"; // NOTE: One extra space required for line ending char '\0'
    int letterCount = 0;

    let textBox  = rrect(screen_width / 2 - 100,  180,  225,  50);
    bool mouseOnText = false;

    int framesCounter = 0;

    rl.set_target_fps(10); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if CheckCollisionPointRec(rl.get_mouse_position(), textBox)
            mouseOnText = true;
        else
            mouseOnText = false;

        if mouseOnText
        {
            // Get pressed key (character) on the queue
            int key = GetKeyPressed();

            // Check if more characters have been pressed on the same frame
            while (key > 0)
            {
                // NOTE: Only allow keys in range [32..125]
                if (key >= 32) && (key <= 125) && (letterCount < MAX_INPUT_CHARS)
                {
                    name[letterCount] = (char)key;
                    letterCount++;
                }

                key = GetKeyPressed(); // Check next character in the queue
            }

            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_BACKSPACE)
            {
                letterCount--;
                name[letterCount] = '\0';

                if letterCount < 0
                    letterCount = 0;
            }
        }

        if mouseOnText
            framesCounter++;
        else
            framesCounter = 0;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("PLACE MOUSE OVER INPUT BOX!", 240, 140, 20, Color::GRAY);

        d.draw_rectangle_rec(textBox, Color::LIGHTGRAY);
        if mouseOnText
            d.draw_rectangle_lines(textBox.x, textBox.y, textBox.width, textBox.height,Color::RED);
        else
            d.draw_rectangle_lines(textBox.x, textBox.y, textBox.width, textBox.height, Color::DARKGRAY);

        d.draw_text(name, textBox.x + 5, textBox.y + 8, 40, Color::MAROON);

        d.draw_text(&format!("INPUT CHARS: {}/{}", letterCount, MAX_INPUT_CHARS), 315, 250, 20, Color::DARKGRAY);

        if mouseOnText
        {
            if letterCount < MAX_INPUT_CHARS
            {
                // Draw blinking underscore char
                if ((framesCounter / 20) % 2) == 0
                    d.draw_text("_", textBox.x + 8 + raylib::text::measure_textname, 40), textBox.y + 12, 40, Color::MAROON);
            }
            else
                d.draw_text("Press BACKSPACE to delete chars...", 230, 300, 20, Color::GRAY);
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

// Check if any key is pressed
// NOTE: We limit keys check to keys between 32 (raylib::consts::KeyboardKey::KEY_SPACE) and 126
bool IsAnyKeyPressed()
{
    bool keyPressed = false;
    int key = GetKeyPressed();

    if (key >= 32) && (key <= 126)
        keyPressed = true;

    return keyPressed;
}