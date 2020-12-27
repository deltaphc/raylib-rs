/*******************************************************************************************
*
*   raylib [core] examples - Mouse wheel input
*
*   This test has been created using raylib 1.1 (www.raylib.com)
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
    rl.set_window_title(thread, "raylib [core] example - input mouse wheel");

    let mut box_position_y = screen_height / 2 - 40;
    let scroll_speed = 4; // Scrolling speed in pixels

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        box_position_y -= rl.get_mouse_wheel_move() * scroll_speed;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_rectangle(screen_width / 2 - 40, box_position_y, 80, 80, Color::MAROON);

        d.draw_text("Use mouse wheel to move the cube up and down!", 10, 10, 20, Color::GRAY);
        d.draw_text(&format!("Box position Y: {:03}", box_position_y), 10, 40, 20, Color::LIGHTGRAY);

        //----------------------------------------------------------------------------------
    },
    );
}
