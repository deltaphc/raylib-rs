/*******************************************************************************************
*
*   raylib [shapes] example - Colors palette
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_COLORS_COUNT 21 // Number of colors available

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shapes] example - colors palette");


    Color colors[MAX_COLORS_COUNT] = {
        Color::DARKGRAY, Color::MAROON, Color::ORANGE, DARKGREEN, Color::DARKBLUE, DARKPURPLE, DARKBROWN,
        Color::GRAY,Color::RED, Color::GOLD, Color::LIME, Color::BLUE, Color::VIOLET, BROWN, Color::LIGHTGRAY, Color::PINK, Color::YELLOW,
        Color::GREEN, Color::SKYBLUE, PURPLE, Color::BEIGE};

    const char *colorNames[MAX_COLORS_COUNT] = {
        "DARKGRAY", "Color::MAROON", "ORANGE", "DARKGREEN", "Color::DARKBLUE", "DARKPURPLE",
        "DARKBROWN", "GRAY", "RED", "Color::GOLD", "Color::LIME", "Color::BLUE", "VIOLET", "BROWN",
        "Color::LIGHTGRAY", "PINK", "YELLOW", "GREEN", "Color::SKYBLUE", "PURPLE", "BEIGE"};

    Rectangle colorsRecs[MAX_COLORS_COUNT] = {0}; // Rectangles array

    // Fills colorsRecs data (for every rectangle)
    for (int i = 0; i < MAX_COLORS_COUNT; i++)
    {
        colorsRecs[i].x = 20 + 100 * (i % 7) + 10 * (i % 7);
        colorsRecs[i].y = 80 + 100 * (i / 7) + 10 * (i / 7);
        colorsRecs[i].width = 100;
        colorsRecs[i].height = 100;
    }

    int colorState[MAX_COLORS_COUNT] = {0}; // Color state: 0-DEFAULT, 1-MOUSE_HOVER

    let mousePoint = rvec2(0.0, 0.0);

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        mousePoint = rl.get_mouse_position();

        for (int i = 0; i < MAX_COLORS_COUNT; i++)
        {
            if CheckCollisionPointRec(mousePoint, colorsRecs[i])
                colorState[i] = 1;
            else
                colorState[i] = 0;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("raylib colors palette", 28, 42, 20, Color::BLACK);
        d.draw_text("press SPACE to see all colors", rl.get_screen_width() - 180, rl.get_screen_height() - 40, 10, Color::GRAY);

        for (int i = 0; i < MAX_COLORS_COUNT; i++) // Draw all rectangles
        {
            d.draw_rectangle_rec(colorsRecs[i], Fade(colors[i], colorState[i] ? 0.6f : 1.0));

            if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) || colorState[i]
            {
                d.draw_rectangle(colorsRecs[i].x, colorsRecs[i].y + colorsRecs[i].height - 26, colorsRecs[i].width, 20, Color::BLACK);
                d.draw_rectangle_linesEx(colorsRecs[i], 6, Fade(BLACK, 0.3));
                d.draw_text(colorNames[i], colorsRecs[i].x + colorsRecs[i].width - raylib::text::measure_textcolorNames[i], 10) - 12,
                         colorsRecs[i].y + colorsRecs[i].height - 20, 10, colors[i]);
            }
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}