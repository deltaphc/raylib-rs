/*******************************************************************************************
*
*   raylib [core] example - Generate random values
*
*   This example has been created using raylib 1.1 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - generate random values");

    let mut framesCounter = 0; // Variable used to count frames

    let mut randValue: i32 = raylib::get_random_value(-8, 5); // Get a random integer number between -8 and 5 (both included)

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        framesCounter+=1;

        // Every two seconds (120 frames) a new random value is generated
        if ((framesCounter / 120) % 2) == 1
        {
            randValue = raylib::get_random_value(-8, 5);
            framesCounter = 0;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("Every 2 seconds a new random value is generated:", 130, 100, 20, Color::MAROON);

        d.draw_text(&format!("{}", randValue), 360, 180, 80, Color::LIGHTGRAY);

        //----------------------------------------------------------------------------------
    },
    );
}
