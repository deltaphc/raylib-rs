/*******************************************************************************************
*
*   raylib [shapes] example - rectangle scaling by mouse
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Vlad Adrian (@demizdor) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2018 Vlad Adrian (@demizdor) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MOUSE_SCALE_MARK_SIZE 12

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shapes] example - rectangle scaling mouse");


    Rectangle rec = {100, 100, 200, 80};

    Vector2 mousePosition = {0};

    bool mouseScaleReady = false;
    bool mouseScaleMode = false;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        mousePosition = GetMousePosition();

        if (CheckCollisionPointRec(mousePosition, rec) &&
            CheckCollisionPointRec(mousePosition, (Rectangle){rec.x + rec.width - MOUSE_SCALE_MARK_SIZE, rec.y + rec.height - MOUSE_SCALE_MARK_SIZE, MOUSE_SCALE_MARK_SIZE, MOUSE_SCALE_MARK_SIZE}))
        {
            mouseScaleReady = true;
            if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON))
                mouseScaleMode = true;
        }
        else
            mouseScaleReady = false;

        if (mouseScaleMode)
        {
            mouseScaleReady = true;

            rec.width = (mousePosition.x - rec.x);
            rec.height = (mousePosition.y - rec.y);

            if (rec.width < MOUSE_SCALE_MARK_SIZE)
                rec.width = MOUSE_SCALE_MARK_SIZE;
            if (rec.height < MOUSE_SCALE_MARK_SIZE)
                rec.height = MOUSE_SCALE_MARK_SIZE;

            if (IsMouseButtonReleased(MOUSE_LEFT_BUTTON))
                mouseScaleMode = false;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("Scale rectangle dragging from bottom-right corner!", 10, 10, 20, GRAY);

        d.draw_rectangleRec(rec, GREEN.fade(0.5));

        if (mouseScaleReady)
        {
            d.draw_rectangle_linesEx(rec, 1, RED);
            DrawTriangle((Vector2){rec.x + rec.width - MOUSE_SCALE_MARK_SIZE, rec.y + rec.height},
                         (Vector2){rec.x + rec.width, rec.y + rec.height},
                         (Vector2){rec.x + rec.width, rec.y + rec.height - MOUSE_SCALE_MARK_SIZE}, RED);
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