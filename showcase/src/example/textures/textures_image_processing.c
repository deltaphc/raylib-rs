/*******************************************************************************************
*
*   raylib [textures] example - Image processing
*
*   NOTE: Images are loaded in CPU memory (RAM); textures are loaded in GPU memory (VRAM)
*
*   This example has been created using raylib 1.4 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2016 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#include <stdlib.h> // Required for: free()

const NUM_PROCESSES 8

    typedef enum {
        NONE = 0,
        COLOR_GRAYSCALE,
        COLOR_TINT,
        COLOR_INVERT,
        COLOR_CONTRAST,
        COLOR_BRIGHTNESS,
        FLIP_VERTICAL,
        FLIP_HORIZONTAL
    } ImageProcess;

static const char *processText[] = {
    "NO PROCESSING",
    "COLOR Color::GRAYSCALE",
    "COLOR TINT",
    "COLOR INVERT",
    "COLOR CONTRAST",
    "COLOR BRIGHTNESS",
    "FLIP VERTICAL",
    "FLIP HORIZONTAL"};

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - image processing");


    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)

    Image image = LoadImage("resources/parrots.png"); // Loaded in CPU memory (RAM)
    ImageFormat(&image, UNCOMPRESSED_R8G8B8A8);       // Format image to RGBA 32bit (required for texture update) <-- ISSUE
    Texture2D texture = LoadTextureFromImage(image);  // Image converted to texture, GPU memory (VRAM)

    int currentProcess = NONE;
    bool textureReload = false;

    Rectangle selectRecs[NUM_PROCESSES] = {0};

    for (int i = 0; i < NUM_PROCESSES; i++)
        selectRecs[i] = (Rectangle){40.0, (float)(50 + 32 * i), 150.0, 30.0};

    rl.set_target_fps(60);
    //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_DOWN))
        {
            currentProcess++;
            if (currentProcess > 7)
                currentProcess = 0;
            textureReload = true;
        }
        else if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_UP))
        {
            currentProcess--;
            if (currentProcess < 0)
                currentProcess = 7;
            textureReload = true;
        }

        if (textureReload)
        {
            UnloadImage(image);                         // Unload current image data
            image = LoadImage("resources/parrots.png"); // Re-load image data

            // NOTE: Image processing is a costly CPU process to be done every frame,
            // If image processing is required in a frame-basis, it should be done
            // with a texture and by shaders
            switch (currentProcess)
            {
            case COLOR_GRAYSCALE:
                ImageColorGrayscale(&image);
                break;
            case COLOR_TINT:
                ImageColorTint(&image, Color::GREEN);
                break;
            case COLOR_INVERT:
                ImageColorInvert(&image);
                break;
            case COLOR_CONTRAST:
                ImageColorContrast(&image, -40);
                break;
            case COLOR_BRIGHTNESS:
                ImageColorBrightness(&image, -80);
                break;
            case FLIP_VERTICAL:
                ImageFlipVertical(&image);
                break;
            case FLIP_HORIZONTAL:
                ImageFlipHorizontal(&image);
                break;
            default:
                break;
            }

            Color *pixels = GetImageData(image); // Get pixel data from image (RGBA 32bit)
            UpdateTexture(texture, pixels);      // Update texture with new image data
            free(pixels);                        // Unload pixels data from RAM

            textureReload = false;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("IMAGE PROCESSING:", 40, 30, 10, Color::DARKGRAY);

        // Draw rectangles
        for (int i = 0; i < NUM_PROCESSES; i++)
        {
            d.draw_rectangleRec(selectRecs[i], (i == currentProcess) ? Color::SKYBLUE : Color::LIGHTGRAY);
            d.draw_rectangle_lines((int)selectRecs[i].x, (int)selectRecs[i].y, (int)selectRecs[i].width, (int)selectRecs[i].height, (i == currentProcess) ? Color::BLUE : Color::GRAY);
            d.draw_text(processText[i], (int)(selectRecs[i].x + selectRecs[i].width / 2 - raylib::text::measure_textprocessText[i], 10) / 2), (int)selectRecs[i].y + 11, 10, (i == currentProcess) ? DARKColor::BLUE : Color::DARKGRAY);
        }

        DrawTexture(texture, screen_width - texture.width - 60, screen_height / 2 - texture.height / 2, WHITE);
        d.draw_rectangle_lines(screen_width - texture.width - 60, screen_height / 2 - texture.height / 2, texture.width, texture.height, Color::BLACK);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(texture); // Unload texture from VRAM
    UnloadImage(image);     // Unload image from RAM

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}