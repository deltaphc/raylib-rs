/*******************************************************************************************
*
*   raylib [core] example - Windows drop files
*
*   This example only works on platforms that support drag & drop (Windows, Linux, OSX, Html5?)
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
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
    let screen_width= 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - drop files");


    let mut dropped_files = Vec::new();

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_file_dropped()
        {
            dropped_files = rl.load_dropped_files();
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        {

            let mut d = rl.begin_drawing(thread);
    
            d.clear_background(Color::RAYWHITE);
    
            if dropped_files.len() == 0 {
    
                d.draw_text("Drop your files to this window!", 100, 40, 20, Color::DARKGRAY);
            }
            else
            {
                d.draw_text("Dropped files:", 100, 40, 20, Color::DARKGRAY);
    
                for i in 0..dropped_files.len() as i32
                {
                    if i % 2 == 0{
    
                        d.draw_rectangle(0, 85 + 40 * i, screen_width, 40, Color::LIGHTGRAY.fade(0.5));
                    }
                    else {
                        d.draw_rectangle(0, 85 + 40 * i, screen_width, 40, Color::LIGHTGRAY.fade(0.3));
                    }
    
                    d.draw_text(&dropped_files[i as usize], 120, 100 + 40 * i, 10, Color::GRAY);
                }
    
                d.draw_text("Drop new files...", 100, 110 + 40 * dropped_files.len() as i32, 20, Color::DARKGRAY);
            }
    
            //----------------------------------------------------------------------------------
        }
        if rl.is_key_down(crate::EXIT_KEY) {
            rl.unload_dropped_files();
        }
    });

}