/*******************************************************************************************
*
*   raylib [text] example - Draw text inside a rectangle
*
*   This example has been created using raylib 2.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Vlad Adrian (@demizdor) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2018 Vlad Adrian (@demizdor) and Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [text] example - draw text inside a rectangle");


    const char text[] = "Text cannot escape\tthis container\t...word wrap also works when active so here's \
a long text for testing.\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
tempor incididunt ut labore et dolore magna aliqua. Nec ullamcorper sit amet risus nullam eget felis eget.";

    bool resizing = false;
    bool wordWrap = true;

    Rectangle container = {25, 25, screen_width - 50, screen_height - 250};
    Rectangle resizer = {container.x + container.width - 17, container.y + container.height - 17, 14, 14};

    // Minimum width and heigh for the container rectangle
    let minWidth = 60;
    let minHeight = 60;
    let maxWidth = screen_width - 50;
    let maxHeight = screen_height - 160;

    Vector2 lastMouse = {0.0, 0.0}; // Stores last mouse coordinates
    Color borderColor = Color::MAROON;       // Container border color
    Font font = GetFontDefault();     // Get default system font

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (IsKeyPressed(KEY_SPACE))
            wordWrap = !wordWrap;

        Vector2 mouse = GetMousePosition();

        // Check if the mouse is inside the container and toggle border color
        if (CheckCollisionPointRec(mouse, container))
            borderColor = Fade(Color::MAROON, 0.4f);
        else if (!resizing)
            borderColor = Color::MAROON;

        // Container resizing logic
        if (resizing)
        {
            if (IsMouseButtonReleased(MOUSE_LEFT_BUTTON))
                resizing = false;

            int width = container.width + (mouse.x - lastMouse.x);
            container.width = (width > minWidth) ? ((width < maxWidth) ? width : maxWidth) : minWidth;

            int height = container.height + (mouse.y - lastMouse.y);
            container.height = (height > minHeight) ? ((height < maxHeight) ? height : maxHeight) : minHeight;
        }
        else
        {
            // Check if we're resizing
            if (IsMouseButtonDown(MOUSE_LEFT_BUTTON) && CheckCollisionPointRec(mouse, resizer))
                resizing = true;
        }

        // Move resizer rectangle properly
        resizer.x = container.x + container.width - 17;
        resizer.y = container.y + container.height - 17;

        lastMouse = mouse; // Update mouse
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_rectangle_linesEx(container, 3, borderColor); // Draw container border

        // Draw text in container (add some padding)
        DrawTextRec(font, text,
                    (Rectangle){container.x + 4, container.y + 4, container.width - 4, container.height - 4},
                    20.0, 2.0, wordWrap, GRAY);

        d.draw_rectangleRec(resizer, borderColor); // Draw the resize box

        // Draw bottom info
        d.draw_rectangle(0, screen_height - 54, screen_width, 54, GRAY);
        d.draw_rectangleRec((Rectangle){382, screen_height - 34, 12, 12}, Color::MAROON);

        d.draw_text("Word Wrap: ", 313, screen_height - 115, 20, Color::BLACK);
        if (wordWrap)
            d.draw_text("ON", 447, screen_height - 115, 20, RED);
        else
            d.draw_text("OFF", 447, screen_height - 115, 20, Color::BLACK);

        d.draw_text("Press [SPACE] to toggle word wrap", 218, screen_height - 86, 20, GRAY);

        d.draw_text("Click hold & drag the    to resize the container", 155, screen_height - 38, 20, RAYWHITE);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}