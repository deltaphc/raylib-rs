/*******************************************************************************************
*
*   raylib [textures] example - Procedural images generation
*
*   This example has been created using raylib 1.8 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2O17 Wilhem Barbier (@nounoursheureux)
*
********************************************************************************************/

use raylib::prelude::*;

const NUM_TEXTURES 7 // Currently we have 7 generation algorithms

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - procedural images generation");


    Image verticalGradient = GenImageGradientV(screen_width, screen_height,Color::RED, Color::BLUE);
    Image horizontalGradient = GenImageGradientH(screen_width, screen_height,Color::RED, Color::BLUE);
    Image radialGradient = GenImageGradientRadial(screen_width, screen_height, 0.0, WHITE, Color::BLACK);
    Image checked = GenImageChecked(screen_width, screen_height, 32, 32,Color::RED, Color::BLUE);
    Image whiteNoise = GenImageWhiteNoise(screen_width, screen_height, 0.5);
    Image perlinNoise = GenImagePerlinNoise(screen_width, screen_height, 50, 50, 4.0);
    Image cellular = GenImageCellular(screen_width, screen_height, 32);

    Texture2D textures[NUM_TEXTURES] = {0};

    textures[0] = LoadTextureFromImage(verticalGradient);
    textures[1] = LoadTextureFromImage(horizontalGradient);
    textures[2] = LoadTextureFromImage(radialGradient);
    textures[3] = LoadTextureFromImage(checked);
    textures[4] = LoadTextureFromImage(whiteNoise);
    textures[5] = LoadTextureFromImage(perlinNoise);
    textures[6] = LoadTextureFromImage(cellular);

    // Unload image data (CPU RAM)
    UnloadImage(verticalGradient);
    UnloadImage(horizontalGradient);
    UnloadImage(radialGradient);
    UnloadImage(checked);
    UnloadImage(whiteNoise);
    UnloadImage(perlinNoise);
    UnloadImage(cellular);

    int currentTexture = 0;

    rl.set_target_fps(60);
    //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()
    {
        // Update
        //----------------------------------------------------------------------------------
        if (rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) || IsKeyPressed(raylib::consts::KeyboardKey::KEY_RIGHT))
        {
            currentTexture = (currentTexture + 1) % NUM_TEXTURES; // Cycle between the textures
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        DrawTexture(textures[currentTexture], 0, 0, WHITE);

        d.draw_rectangle(30, 400, 325, 30, Color::SKYBLUE.fade(0.5));
        d.draw_rectangle_lines(30, 400, 325, 30, WHITE.fade(0.5));
        d.draw_text("MOUSE LEFT BUTTON to CYCLE PROCEDURAL TEXTURES", 40, 410, 10, WHITE);

        switch (currentTexture)
        {
        case 0:
            d.draw_text("VERTICAL GRADIENT", 560, 10, 20, RAYWHITE);
            break;
        case 1:
            d.draw_text("HORIZONTAL GRADIENT", 540, 10, 20, RAYWHITE);
            break;
        case 2:
            d.draw_text("RADIAL GRADIENT", 580, 10, 20, Color::LIGHTGRAY);
            break;
        case 3:
            d.draw_text("CHECKED", 680, 10, 20, RAYWHITE);
            break;
        case 4:
            d.draw_text("WHITE NOISE", 640, 10, 20,Color::RED);
            break;
        case 5:
            d.draw_text("PERLIN NOISE", 630, 10, 20, RAYWHITE);
            break;
        case 6:
            d.draw_text("CELLULAR", 670, 10, 20, RAYWHITE);
            break;
        default:
            break;
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------

    // Unload textures data (GPU VRAM)
    for (int i = 0; i < NUM_TEXTURES; i++)
        UnloadTexture(textures[i]);

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
