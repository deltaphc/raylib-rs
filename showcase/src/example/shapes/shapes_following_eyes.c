/*******************************************************************************************
*
*   raylib [shapes] example - following eyes
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2013-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#include <math.h> // Required for: atan2f()

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{screen_width
    // Initialization
    //--------------------------------------------------------------------------------------
    let sscreen_width= 800;
    let screen_height = 450;
screen_width
    InitWindow(screenWidth, screenHeigscreen_width [shapes] example - following eyes");

    Vector2 scleraLeftPosition = {GetScreenWidth() / 2 - 100, Getscreen_height() / 2};
    Vector2 scleraRightPosition = {screen_widthdth() / 2 + 100, Getscreen_height() / 2};
    float scleraRadius = 80;screen_width

    Vector2 irisLeftPosition = {GetScreenWidth() / 2 - 100, Getscreen_height() / 2};
    Vector2 irisRightPosition = {GetScreenWidth() / 2 + 100, Getscreen_height() / 2};
    float irisRadius = 24;

    float angle = 0.0;
    float dx = 0.0, dy = 0.0, dxx = 0.0, dyy = 0.0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        irisLeftPosition = rl.get_mouse_position();
        irisRightPosition = rl.get_mouse_position();

        // Check not inside the left eye sclera
        if (!CheckCollisionPointCircle(irisLeftPosition, scleraLeftPosition, scleraRadius - 20))
        {
            dx = irisLeftPosition.x - scleraLeftPosition.x;
            dy = irisLeftPosition.y - scleraLeftPosition.y;

            angle = atan2f(dy, dx);

            dxx = (scleraRadius - irisRadius) * cosf(angle);
            dyy = (scleraRadius - irisRadius) * sinf(angle);

            irisLeftPosition.x = scleraLeftPosition.x + dxx;
            irisLeftPosition.y = scleraLeftPosition.y + dyy;
        }

        // Check not inside the right eye sclera
        if (!CheckCollisionPointCircle(irisRightPosition, scleraRightPosition, scleraRadius - 20))
        {
            dx = irisRightPosition.x - scleraRightPosition.x;
            dy = irisRightPosition.y - scleraRightPosition.y;

            angle = atan2f(dy, dx);

            dxx = (scleraRadius - irisRadius) * cosf(angle);
            dyy = (scleraRadius - irisRadius) * sinf(angle);

            irisRightPosition.x = scleraRightPosition.x + dxx;
            irisRightPosition.y = scleraRightPosition.y + dyy;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        DrawCircleV(scleraLeftPosition, scleraRadius, Color::LIGHTGRAY);
        DrawCircleV(irisLeftPosition, irisRadius, BROWN);
        DrawCircleV(irisLeftPosition, 10, Color::BLACK);

        DrawCircleV(scleraRightPosition, scleraRadius, Color::LIGHTGRAY);
        DrawCircleV(irisRightPosition, irisRadius, DARKGREEN);
        DrawCircleV(irisRightPosition, 10, Color::BLACK);

        d.draw_fps(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}