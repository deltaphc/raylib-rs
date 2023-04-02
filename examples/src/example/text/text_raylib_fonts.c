/*******************************************************************************************
*
*   raylib [text] example - raylib font loading and usage
*
*   NOTE: raylib is distributed with some free to use fonts (even for commercial pourposes!)
*         To view details and credits for those fonts, check raylib license file
*
*   This example has been created using raylib 1.7 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2017 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_FONTS 8

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [text] example - raylib fonts");


    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)
    Font fonts[MAX_FONTS] = {0};

    fonts[0] = LoadFont("resources/fonts/alagard.png");
    fonts[1] = LoadFont("resources/fonts/pixelplay.png");
    fonts[2] = LoadFont("resources/fonts/mecha.png");
    fonts[3] = LoadFont("resources/fonts/setback.png");
    fonts[4] = LoadFont("resources/fonts/romulus.png");
    fonts[5] = LoadFont("resources/fonts/pixantiqua.png");
    fonts[6] = LoadFont("resources/fonts/alpha_beta.png");
    fonts[7] = LoadFont("resources/fonts/jupiter_crash.png");

    const char *messages[MAX_FONTS] = {"ALAGARD FONT designed by Hewett Tsoi",
                                       "PIXELPLAY FONT designed by Aleksander Shevchuk",
                                       "MECHA FONT designed by Captain Falcon",
                                       "SETBACK FONT designed by Brian Kent (AEnigma)",
                                       "ROMULUS FONT designed by Hewett Tsoi",
                                       "PIXANTIQUA FONT designed by Gerhard Grossmann",
                                       "ALPHA_BETA FONT designed by Brian Kent (AEnigma)",
                                       "JUPITER_CRASH FONT designed by Brian Kent (AEnigma)"};

    let spacings[MAX_FONTS] = {2, 4, 8, 4, 3, 4, 4, 1};

    Vector2 positions[MAX_FONTS] = {0};

    for (int i = 0; i < MAX_FONTS; i+=1)
    {
        positions[i].x = screen_width / 2 - MeasureTextEx(fonts[i], messages[i], fonts[i].baseSize * 2, spacings[i]).x / 2;
        positions[i].y = 60 + fonts[i].baseSize + 45 * i;
    }

    // Small Y position corrections
    positions[3].y += 8;
    positions[4].y += 2;
    positions[7].y -= 8;

    let colors[MAX_FONTS] = {Color::MAROON, Color::ORANGE, DARKGREEN, Color::DARKBLUE, DARKPURPLE, Color::LIME, Color::GOLD,Color::RED};

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // TODO: Update your variables here
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("free fonts included with raylib", 250, 20, 20, Color::DARKGRAY);
        DrawLine(220, 50, 590, 50, Color::DARKGRAY);

        for (int i = 0; i < MAX_FONTS; i+=1)
        {
            DrawTextEx(fonts[i], messages[i], positions[i], fonts[i].baseSize * 2, spacings[i], colors[i]);
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------

    // Fonts unloading
    for (int i = 0; i < MAX_FONTS; i+=1)
        UnloadFont(fonts[i]);

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}