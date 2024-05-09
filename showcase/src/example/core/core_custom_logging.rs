/*******************************************************************************************
*
*   raylib [core] example - Custom logging
*
*   This example has been created using raylib 2.1 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Pablo Marcos Oltra (@pamarcos) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2018 Pablo Marcos Oltra (@pamarcos) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

// Custom logging funtion
pub fn log_custom(msg_type: TraceLogLevel, text: &str) {
    match msg_type {
        0 => println!("[INFO] : {}", s),

        1 => println!("[ERROR]: {}", s),

        2 => println!("[WARN] : {}", s),

        3 => println!("[DEBUG]: {}", s),

        _ => println!("", s),
    }
}

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    {
        // Initialization
        //--------------------------------------------------------------------------------------
        let screen_width = 800;
        let screen_height = 450;

        rl.set_trace_log_callback(log_custom);

        rl.set_window_size(screen_width, screen_height);
        rl.set_window_title(thread, "raylib [core] example - custom logging");

        rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                               //--------------------------------------------------------------------------------------

        // Main game loop
        return Box::new(
            move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // TODO: Update your variables here
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("Check out the console output to see the custom logger in action!", 60, 200, 20, Color::LIGHTGRAY);

    },
        );
    }
}
