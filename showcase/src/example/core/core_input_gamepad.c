/*******************************************************************************************
*
*   raylib [core] example - Gamepad input
*
*   NOTE: This example requires a Gamepad connected to the system
*         raylib is configured to work with the following gamepads:
*                - Xbox 360 Controller (Xbox 360, Xbox One)
*                - PLAYSTATION(R)3 Controller
*         Check raylib.h for buttons configuration
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2013-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

// NOTE: Gamepad name ID depends on drivers and OS
#if defined(PLATFORM_RPI)
const XBOX360_NAME_ID "Microsoft X-Box 360 pad" const PS3_NAME_ID "PLAYSTATION(R)3 Controller"
#else
const XBOX360_NAME_ID "Xbox 360 Controller" const PS3_NAME_ID "PLAYSTATION(R)3 Controller"
#endif

    pub fn
    run(rl
        : &mut RaylibHandle, thread
        : &RaylibThread)
        ->crate::SampleOut
{
    // Initialscreen_width
    //--------------------------------------------------------------------------------------
    let screenWidth = 800;
    let screen_height = 450;

    SetConfigFlscreen_widthAA_4X_HINT); // Set MSAA 4X hint before windows creation

    InitWindow(screenWidth, screen_height, "raylib [core] example - gamepad input");

    Texture2D texPs3Pad = LoadTexture("resources/ps3.png");
    Texture2D texXboxPad = LoadTexture("resources/xbox.png");

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // ...
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if (IsGamepadAvailable(GAMEPAD_PLAYER1))
        {
            d.draw_text(FormatText("GP1: %s", GetGamepadName(GAMEPAD_PLAYER1)), 10, 10, 10, Color::BLACK);

            if (IsGamepadName(GAMEPAD_PLAYER1, XBOX360_NAME_ID))
            {
                DrawTexture(texXboxPad, 0, 0, Color::DARKGRAY);

                // Draw buttons: xbox home
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_MIDDLE))
                    DrawCircle(394, 89, 19, RED);

                // Draw buttons: basic
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_MIDDLE_RIGHT))
                    DrawCircle(436, 150, 9, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_MIDDLE_LEFT))
                    DrawCircle(352, 150, 9, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_LEFT))
                    DrawCircle(501, 151, 15, Color::BLUE);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
                    DrawCircle(536, 187, 15, Color::LIME);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT))
                    DrawCircle(572, 151, 15, Color::MAROON);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_UP))
                    DrawCircle(536, 115, 15, Color::GOLD);

                // Draw buttons: d-pad
                d.draw_rectangle(317, 202, 19, 71, Color::BLACK);
                d.draw_rectangle(293, 228, 69, 19, Color::BLACK);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_UP))
                    d.draw_rectangle(317, 202, 19, 26, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_DOWN))
                    d.draw_rectangle(317, 202 + 45, 19, 26, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_LEFT))
                    d.draw_rectangle(292, 228, 25, 19, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_RIGHT))
                    d.draw_rectangle(292 + 44, 228, 26, 19, RED);

                // Draw buttons: left-right back
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_TRIGGER_1))
                    DrawCircle(259, 61, 20, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_TRIGGER_1))
                    DrawCircle(536, 61, 20, RED);

                // Draw axis: left joystick
                DrawCircle(259, 152, 39, Color::BLACK);
                DrawCircle(259, 152, 34, Color::LIGHTGRAY);
                DrawCircle(259 + (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_LEFT_X) * 20),
                           152 - (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_LEFT_Y) * 20), 25, Color::BLACK);

                // Draw axis: right joystick
                DrawCircle(461, 237, 38, Color::BLACK);
                DrawCircle(461, 237, 33, Color::LIGHTGRAY);
                DrawCircle(461 + (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_RIGHT_X) * 20),
                           237 - (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_RIGHT_Y) * 20), 25, Color::BLACK);

                // Draw axis: left-right triggers
                d.draw_rectangle(170, 30, 15, 70, GRAY);
                d.draw_rectangle(604, 30, 15, 70, GRAY);
                d.draw_rectangle(170, 30, 15, (((1.0 + GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_LEFT_TRIGGER)) / 2.0) * 70), RED);
                d.draw_rectangle(604, 30, 15, (((1.0 + GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_RIGHT_TRIGGER)) / 2.0) * 70), RED);

                //d.draw_text(FormatText("Xbox axis LT: %02.02f", GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_LEFT_TRIGGER)), 10, 40, 10, Color::BLACK);
                //d.draw_text(FormatText("Xbox axis RT: %02.02f", GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_RIGHT_TRIGGER)), 10, 60, 10, Color::BLACK);
            }
            else if (IsGamepadName(GAMEPAD_PLAYER1, PS3_NAME_ID))
            {
                DrawTexture(texPs3Pad, 0, 0, Color::DARKGRAY);

                // Draw buttons: ps
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_MIDDLE))
                    DrawCircle(396, 222, 13, RED);

                // Draw buttons: basic
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_MIDDLE_LEFT))
                    d.draw_rectangle(328, 170, 32, 13, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_MIDDLE_RIGHT))
                    DrawTriangle(rvec2(436, 168), rvec2(436, 185), rvec2(464, 177), RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_UP))
                    DrawCircle(557, 144, 13, Color::LIME);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT))
                    DrawCircle(586, 173, 13, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
                    DrawCircle(557, 203, 13, VIOLET);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_FACE_LEFT))
                    DrawCircle(527, 173, 13, PINK);

                // Draw buttons: d-pad
                d.draw_rectangle(225, 132, 24, 84, Color::BLACK);
                d.draw_rectangle(195, 161, 84, 25, Color::BLACK);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_UP))
                    d.draw_rectangle(225, 132, 24, 29, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_DOWN))
                    d.draw_rectangle(225, 132 + 54, 24, 30, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_LEFT))
                    d.draw_rectangle(195, 161, 30, 25, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_FACE_RIGHT))
                    d.draw_rectangle(195 + 54, 161, 30, 25, RED);

                // Draw buttons: left-right back buttons
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_LEFT_TRIGGER_1))
                    DrawCircle(239, 82, 20, RED);
                if (IsGamepadButtonDown(GAMEPAD_PLAYER1, GAMEPAD_BUTTON_RIGHT_TRIGGER_1))
                    DrawCircle(557, 82, 20, RED);

                // Draw axis: left joystick
                DrawCircle(319, 255, 35, Color::BLACK);
                DrawCircle(319, 255, 31, Color::LIGHTGRAY);
                DrawCircle(319 + (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_LEFT_X) * 20),
                           255 + (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_LEFT_Y) * 20), 25, Color::BLACK);

                // Draw axis: right joystick
                DrawCircle(475, 255, 35, Color::BLACK);
                DrawCircle(475, 255, 31, Color::LIGHTGRAY);
                DrawCircle(475 + (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_RIGHT_X) * 20),
                           255 + (GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_RIGHT_Y) * 20), 25, Color::BLACK);

                // Draw axis: left-right triggers
                d.draw_rectangle(169, 48, 15, 70, GRAY);
                d.draw_rectangle(611, 48, 15, 70, GRAY);
                d.draw_rectangle(169, 48, 15, (((1.0 - GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_LEFT_TRIGGER)) / 2.0) * 70), RED);
                d.draw_rectangle(611, 48, 15, (((1.0 - GetGamepadAxisMovement(GAMEPAD_PLAYER1, GAMEPAD_AXIS_RIGHT_TRIGGER)) / 2.0) * 70), RED);
            }
            else
            {
                d.draw_text("- GENERIC GAMEPAD -", 280, 180, 20, GRAY);

                // TODO: Draw generic gamepad
            }

            d.draw_text(FormatText("DETECTED AXIS [%i]:", GetGamepadAxisCount(GAMEPAD_PLAYER1)), 10, 50, 10, Color::MAROON);

            for (int i = 0; i < GetGamepadAxisCount(GAMEPAD_PLAYER1); i++)
            {
                d.draw_text(FormatText("AXIS %i: %.02f", i, GetGamepadAxisMovement(GAMEPAD_PLAYER1, i)), 20, 70 + 20 * i, 10, Color::DARKGRAY);
            }

            if (GetGamepadButtonPressed() != -1)
                d.draw_text(FormatText("DETECTED BUTTON: %i", GetGamepadButtonPressed()), 10, 430, 10, RED);
            else
                d.draw_text("DETECTED BUTTON: NONE", 10, 430, 10, GRAY);
        }
        else
        {
            d.draw_text("GP1: NOT DETECTED", 10, 10, 10, GRAY);

            DrawTexture(texXboxPad, 0, 0, Color::LIGHTGRAY);
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(texPs3Pad);
    UnloadTexture(texXboxPad);

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}