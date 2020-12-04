/*******************************************************************************************
*
*   raylib [shapes] example - draw ring (with gui options)
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
    rl.set_window_title(thread, "raylib [shapes] example - draw ring");


    let center = rvec2((rl.get_screen_width() - 300) / 2, rl.get_screen_height() / 2);

    float innerRadius = 80.0;
    float outerRadius = 190.0;

    int startAngle = 0;
    int endAngle = 360;
    int segments = 0;

    bool drawRing = true;
    bool drawRingLines = false;
    bool drawCircleLines = false;

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

        if drawRing
            DrawRing(center, innerRadius, outerRadius, startAngle, endAngle, segments, Color::MAROON.fade(0.3));
        if drawRingLines
            DrawRingLines(center, innerRadius, outerRadius, startAngle, endAngle, segments, Color::BLACK.fade(0.4));
        if drawCircleLines
            DrawCircleSectorLines(center, outerRadius, startAngle, endAngle, segments, Color::BLACK.fade(0.4));

        // Draw GUI controls
        //------------------------------------------------------------------------------
        startAngle = GuiSliderBar(rrect(600, 40, 120, 20), "StartAngle", startAngle, -450, 450, true);
        endAngle = GuiSliderBar(rrect(600, 70, 120, 20), "EndAngle", endAngle, -450, 450, true);

        innerRadius = GuiSliderBar(rrect(600, 140, 120, 20), "InnerRadius", innerRadius, 0, 100, true);
        outerRadius = GuiSliderBar(rrect(600, 170, 120, 20), "OuterRadius", outerRadius, 0, 200, true);

        segments = GuiSliderBar(rrect(600, 240, 120, 20), "Segments", segments, 0, 100, true);

        drawRing = GuiCheckBox(rrect(600, 320, 20, 20), "Draw Ring", drawRing);
        drawRingLines = GuiCheckBox(rrect(600, 350, 20, 20), "Draw RingLines", drawRingLines);
        drawCircleLines = GuiCheckBox(rrect(600, 380, 20, 20), "Draw CircleLines", drawCircleLines);
        //------------------------------------------------------------------------------

        d.draw_text(&format!("MODE: %s", (segments >= 4) ? "MANUAL" : "AUTO"), 600, 270, 10, (segments >= 4) ? Color::MAROON : Color::DARKGRAY);

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