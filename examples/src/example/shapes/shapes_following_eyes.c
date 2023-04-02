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
    InitWindow(screen_width, screenHeigscreen_width [shapes] example - following eyes");

    let scleraLeftPosition = rvec2(GetScreenWidth() / 2 - 100, rl.get_screen_height() / 2);
    let scleraRightPosition = rvec2(screen_widthdth() / 2 + 100, rl.get_screen_height() / 2);
    float scleraRadius = 80;screen_width

    let irisLeftPosition = rvec2(GetScreenWidth() / 2 - 100, rl.get_screen_height() / 2);
    let irisRightPosition = rvec2(GetScreenWidth() / 2 + 100, rl.get_screen_height() / 2);
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
        if !CheckCollisionPointCircle(irisLeftPosition, scleraLeftPosition, scleraRadius - 20)
        {
            dx = irisLeftPosition.x - scleraLeftPosition.x;
            dy = irisLeftPosition.y - scleraLeftPosition.y;

            angle = atan2f(dy, dx);

            dxx = (scleraRadius - irisRadius) * (angle).cos();
            dyy = (scleraRadius - irisRadius) * (angle).sin();

            irisLeftPosition.x = scleraLeftPosition.x + dxx;
            irisLeftPosition.y = scleraLeftPosition.y + dyy;
        }

        // Check not inside the right eye sclera
        if !CheckCollisionPointCircle(irisRightPosition, scleraRightPosition, scleraRadius - 20)
        {
            dx = irisRightPosition.x - scleraRightPosition.x;
            dy = irisRightPosition.y - scleraRightPosition.y;

            angle = atan2f(dy, dx);

            dxx = (scleraRadius - irisRadius) * (angle).cos();
            dyy = (scleraRadius - irisRadius) * (angle).sin();

            irisRightPosition.x = scleraRightPosition.x + dxx;
            irisRightPosition.y = scleraRightPosition.y + dyy;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_circle_v(scleraLeftPosition, scleraRadius, Color::LIGHTGRAY);
        d.draw_circle_v(irisLeftPosition, irisRadius, Color::BROWN);
        d.draw_circle_v(irisLeftPosition, 10, Color::BLACK);

        d.draw_circle_v(scleraRightPosition, scleraRadius, Color::LIGHTGRAY);
        d.draw_circle_v(irisRightPosition, irisRadius, DARKGREEN);
        d.draw_circle_v(irisRightPosition, 10, Color::BLACK);

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