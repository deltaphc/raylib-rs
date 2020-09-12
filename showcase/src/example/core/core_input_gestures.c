/*******************************************************************************************
*
*   raylib [core] example - Input Gestures Detection
*
*   This example has been created using raylib 1.4 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2016 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;
#include <string.h>

const MAX_GESTURE_STRINGS 20

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - input gestures");


    Vector2 touchPosition = {0, 0};
    Rectangle touchArea = {220, 10, screen_width - 230, screen_height - 20};

    int gesturesCount = 0;
    char gestureStrings[MAX_GESTURE_STRINGS][32];

    int currentGesture = GESTURE_NONE;
    int lastGesture = GESTURE_NONE;

    //SetGesturesEnabled(0b0000000000001001);   // Enable only some gestures to be detected

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        lastGesture = currentGesture;
        currentGesture = GetGestureDetected();
        touchPosition = GetTouchPosition(0);

        if (CheckCollisionPointRec(touchPosition, touchArea) && (currentGesture != GESTURE_NONE))
        {
            if (currentGesture != lastGesture)
            {
                // Store gesture string
                switch (currentGesture)
                {
                case GESTURE_TAP:
                    strcpy(gestureStrings[gesturesCount], "GESTURE TAP");
                    break;
                case GESTURE_DOUBLETAP:
                    strcpy(gestureStrings[gesturesCount], "GESTURE DOUBLETAP");
                    break;
                case GESTURE_HOLD:
                    strcpy(gestureStrings[gesturesCount], "GESTURE HOLD");
                    break;
                case GESTURE_DRAG:
                    strcpy(gestureStrings[gesturesCount], "GESTURE DRAG");
                    break;
                case GESTURE_SWIPE_RIGHT:
                    strcpy(gestureStrings[gesturesCount], "GESTURE SWIPE RIGHT");
                    break;
                case GESTURE_SWIPE_LEFT:
                    strcpy(gestureStrings[gesturesCount], "GESTURE SWIPE LEFT");
                    break;
                case GESTURE_SWIPE_UP:
                    strcpy(gestureStrings[gesturesCount], "GESTURE SWIPE UP");
                    break;
                case GESTURE_SWIPE_DOWN:
                    strcpy(gestureStrings[gesturesCount], "GESTURE SWIPE DOWN");
                    break;
                case GESTURE_PINCH_IN:
                    strcpy(gestureStrings[gesturesCount], "GESTURE PINCH IN");
                    break;
                case GESTURE_PINCH_OUT:
                    strcpy(gestureStrings[gesturesCount], "GESTURE PINCH OUT");
                    break;
                default:
                    break;
                }

                gesturesCount++;

                // Reset gestures strings
                if (gesturesCount >= MAX_GESTURE_STRINGS)
                {
                    for (int i = 0; i < MAX_GESTURE_STRINGS; i++)
                        strcpy(gestureStrings[i], "\0");

                    gesturesCount = 0;
                }
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_rectangleRec(touchArea, GRAY);
        d.draw_rectangle(225, 15, screen_width - 240, screen_height - 30, RAYWHITE);

        d.draw_text("GESTURES TEST AREA", screen_width - 270, screen_height - 40, 20, GRAY.fade(0.5));

        for (int i = 0; i < gesturesCount; i++)
        {
            if (i % 2 == 0)
                d.draw_rectangle(10, 30 + 20 * i, 200, 20, Color::LIGHTGRAY.fade(0.5));
            else
                d.draw_rectangle(10, 30 + 20 * i, 200, 20, Fade(Color::LIGHTGRAY, 0.3f));

            if (i < gesturesCount - 1)
                d.draw_text(gestureStrings[i], 35, 36 + 20 * i, 10, Color::DARKGRAY);
            else
                d.draw_text(gestureStrings[i], 35, 36 + 20 * i, 10, Color::MAROON);
        }

        d.draw_rectangle_lines(10, 29, 200, screen_height - 50, GRAY);
        d.draw_text("DETECTED GESTURES", 50, 15, 10, GRAY);

        if (currentGesture != GESTURE_NONE)
            DrawCircleV(touchPosition, 30, Color::MAROON);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------
}