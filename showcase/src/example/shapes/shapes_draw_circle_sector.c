/*******************************************************************************************
*
*   raylib [shapes] example - draw circle sector (with gui options)
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Vlad Adrian (@demizdor) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2018 Vlad Adrian (@demizdor) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

#include <raylib.h>

const RAYGUI_IMPLEMENTATION
#include "raygui.h" // Required for GUI controls

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shapes] example - draw circle sector");


    let center = rvec2((rl.get_screen_width() - 300) / 2, rl.get_screen_height() / 2);

    float outerRadius = 180.f;
    int startAngle = 0;
    int endAngle = 180;
    int segments = 0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // NOTE: All variables update happens inside GUI control functions
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        DrawLine(500, 0, 500, rl.get_screen_height(), Fade(Color::LIGHTGRAY, 0.6f));
        d.draw_rectangle(500, 0, rl.get_screen_width() - 500, rl.get_screen_height(), Fade(Color::LIGHTGRAY, 0.3));

        DrawCircleSector(center, outerRadius, startAngle, endAngle, segments, Color::MAROON.fade(0.3));
        DrawCircleSectorLines(center, outerRadius, startAngle, endAngle, segments, Color::MAROON.fade(0.6));

        // Draw GUI controls
        //------------------------------------------------------------------------------
        startAngle = GuiSliderBar(rrect(600, 40, 120, 20), "StartAngle", startAngle, 0, 720, true);
        endAngle = GuiSliderBar(rrect(600, 70, 120, 20), "EndAngle", endAngle, 0, 720, true);

        outerRadius = GuiSliderBar(rrect(600, 140, 120, 20), "Radius", outerRadius, 0, 200, true);
        segments = GuiSliderBar(rrect(600, 170, 120, 20), "Segments", segments, 0, 100, true);
        //------------------------------------------------------------------------------

        d.draw_text(&format!("MODE: %s", (segments >= 4) ? "MANUAL" : "AUTO"), 600, 200, 10, (segments >= 4) ? Color::MAROON : Color::DARKGRAY);

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