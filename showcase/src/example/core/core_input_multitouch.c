/*******************************************************************************************
*
*   raylib [core] example - Input multitouch
*
*   This example has been created using raylib 2.1 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Berni (@Berni8k) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Berni (@Berni8k) and Ramon Santamaria (@raysan5)
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
    rl.set_window_title(thread, "raylib [core] example - input multitouch");


    Vector2 ballPosition = {-100.0, -100.0};
    Color ballColor = BEIGE;

    int touchCounter = 0;
    Vector2 touchPosition = {0.0};

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        ballPosition = rl.get_mouse_position();

        ballColor = BEIGE;

        if (IsMouseButtonDown(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON))
            ballColor = Color::MAROON;
        if (IsMouseButtonDown(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON))
            ballColor = Color::LIME;
        if (IsMouseButtonDown(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON))
            ballColor = DARKColor::BLUE;

        if (rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON))
            touchCounter = 10;
        if (rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON))
            touchCounter = 10;
        if (rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON))
            touchCounter = 10;

        if (touchCounter > 0)
            touchCounter--;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        // Multitouch
        for (int i = 0; i < MAX_TOUCH_POINTS; ++i)
        {
            touchPosition = GetTouchPosition(i); // Get the touch point

            if ((touchPosition.x >= 0) && (touchPosition.y >= 0)) // Make sure point is not (-1,-1) as this means there is no touch for it
            {
                // Draw circle and touch index number
                DrawCircleV(touchPosition, 34, ORANGE);
                d.draw_text(FormatText("%d", i), touchPosition.x - 10, touchPosition.y - 70, 40, Color::BLACK);
            }
        }

        // Draw the normal mouse location
        DrawCircleV(ballPosition, 30 + (touchCounter * 3), ballColor);

        d.draw_text("move ball with mouse and click mouse button to change color", 10, 10, 20, Color::DARKGRAY);
        d.draw_text("touch the screen at multiple locations to get multiple balls", 10, 30, 20, Color::DARKGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}