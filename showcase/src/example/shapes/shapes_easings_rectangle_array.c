/*******************************************************************************************
*
*   raylib [shapes] example - easings rectangle array
*
*   NOTE: This example requires 'easings.h' library, provided on raylib/src. Just copy
*   the library to same directory as example or make sure it's available on include path.
*
*   This example has been created using raylib 2.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#include "easings.h" // Required for easing functions

const RECS_WIDTH 50 const RECS_HEIGHT 50

    const MAX_RECS_X 800 /
    RECS_WIDTH const MAX_RECS_Y 450 / RECS_HEIGHT

    const PLAY_TIME_IN_FRAMES 240 // At 60 fps = 4 seconds

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shapes] example - easings rectangle array");


    Rectangle recs[MAX_RECS_X * MAX_RECS_Y] = {0};

    for (int y = 0; y < MAX_RECS_Y; y+=1)
    {
        for (int x = 0; x < MAX_RECS_X; x+=1)
        {
            recs[y * MAX_RECS_X + x].x = RECS_WIDTH / 2 + RECS_WIDTH * x;
            recs[y * MAX_RECS_X + x].y = RECS_HEIGHT / 2 + RECS_HEIGHT * y;
            recs[y * MAX_RECS_X + x].width = RECS_WIDTH;
            recs[y * MAX_RECS_X + x].height = RECS_HEIGHT;
        }
    }

    float rotation = 0.0;
    int framesCounter = 0;
    int state = 0; // Rectangles animation state: 0-Playing, 1-Finished

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if state == 0
        {
            framesCounter+=1;

            for (int i = 0; i < MAX_RECS_X * MAX_RECS_Y; i+=1)
            {
                recs[i].height = EaseCircOut(framesCounter, RECS_HEIGHT, -RECS_HEIGHT, PLAY_TIME_IN_FRAMES);
                recs[i].width = EaseCircOut(framesCounter, RECS_WIDTH, -RECS_WIDTH, PLAY_TIME_IN_FRAMES);

                if recs[i].height < 0
                    recs[i].height = 0;
                if recs[i].width < 0
                    recs[i].width = 0;

                if (recs[i].height == 0) && (recs[i].width == 0)
                    state = 1; // Finish playing

                rotation = EaseLinearIn(framesCounter, 0.0, 360.0, PLAY_TIME_IN_FRAMES);
            }
        }
        else if (state == 1) && rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
        {
            // When animation has finished, press space to restart
            framesCounter = 0;

            for (int i = 0; i < MAX_RECS_X * MAX_RECS_Y; i+=1)
            {
                recs[i].height = RECS_HEIGHT;
                recs[i].width = RECS_WIDTH;
            }

            state = 0;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if state == 0
        {
            for (int i = 0; i < MAX_RECS_X * MAX_RECS_Y; i+=1)
            {
                d.draw_rectanglePro(recs[i], rvec2(recs[i].width / 2,  recs[i].height / 2), rotation,Color::RED);
            }
        }
        else if state == 1
            d.draw_text("PRESS [SPACE] TO PLAY AGAIN!", 240, 200, 20, Color::GRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}