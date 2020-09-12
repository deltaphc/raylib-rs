/*******************************************************************************************
*
*   raygui - portable window
*
*   DEPENDENCIES:
*       raylib 2.1  - Windowing/input management and drawing.
*       raygui 2.0  - Immediate-mode GUI controls.
*
*   COMPILATION (Windows - MinGW):
*       gcc -o $(NAME_PART).exe $(FILE_NAME) -I../../src -lraylib -lopengl32 -lgdi32 -std=c99
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2020 Ramon Santamaria (@raysan5)
*
**********************************************************************************************/

use raylib::prelude::*;

const RAYGUI_IMPLEMENTATION
#include "../../src/raygui.h"

    //------------------------------------------------------------------------------------
    // Program main entry point
    //------------------------------------------------------------------------------------
    int
    main()
{
    // Initialization
    //---------------------------------------------------------------------------------------
    int screen_width = 800;
    int screen_height = 600;

    SetConfigFlags(FLAG_WINDOW_UNDECORATED);
    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - portable window");


    // General variables
    Vector2 mousePosition = {0};
    Vector2 windowPosition = {500, 200};
    Vector2 panOffset = mousePosition;
    bool dragWindow = false;

    SetWindowPosition(windowPosition.x, windowPosition.y);

    bool exitWindow = false;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    while (!exitWindow && !WindowShouldClose()) // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        mousePosition = rl.get_mouse_position();

        if (rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON))
        {
            if (CheckCollisionPointRec(mousePosition, (Rectangle){0, 0, screen_width, 20}))
            {
                dragWindow = true;
                panOffset = mousePosition;
            }
        }

        if (dragWindow)
        {
            windowPosition.x += (mousePosition.x - panOffset.x);
            windowPosition.y += (mousePosition.y - panOffset.y);

            if (IsMouseButtonReleased(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON))
                dragWindow = false;

            SetWindowPosition(windowPosition.x, windowPosition.y);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        exitWindow = GuiWindowBox((Rectangle){0, 0, screen_width, screen_height}, "PORTABLE WINDOW");

        d.draw_text(FormatText("Mouse Position: [ %.0, %.0 ]", mousePosition.x, mousePosition.y), 10, 40, 10, Color::DARKGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
