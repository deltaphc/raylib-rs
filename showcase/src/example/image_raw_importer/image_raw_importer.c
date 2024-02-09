/*******************************************************************************************
*
*   raygui - image raw importer
*
*   DEPENDENCIES:
*       raylib 2.1  - Windowing/input management and drawing.
*       raygui 2.0  - Immediate-mode GUI controls.
*
*   COMPILATION (Windows - MinGW):
*       gcc -o $(NAME_PART).exe $(FILE_NAME) -I../../src -lraylib -lopengl32 -lgdi32 -std=c99
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2020 Ramon Santamaria (@raysan5)
*
**********************************************************************************************/

use raylib::prelude::*;

const RAYGUI_IMPLEMENTATION const RAYGUI_SUPPORT_RICONS
#include "../../src/raygui.h"

#include <string.h> // Required for: strcpy()
#include <stdlib.h> // Required for: atoi()
#include <math.h>   // Required for: round()

    //------------------------------------------------------------------------------------
    // Program main entry point
    //------------------------------------------------------------------------------------
    int
    main()
{
    // Initialization
    //---------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 600;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - image raw importer");


    Texture2D texture = {0};

    // GUI controls initialization
    //----------------------------------------------------------------------------------
    let windowOffset = rvec2(screen_width / 2 - 200 / 2, screen_height / 2 - 465 / 2);

    bool importWindowActive = false;

    int widthValue = 0;
    bool widthEditMode = false;
    int heightValue = 0;
    bool heightEditMode = false;

    int pixelFormatActive = 0;
    const char *pixel&format!List[8] = {"CUSTOM", "GRAYSCALE", "GRAY ALPHA", "R5G6B5", "R8G8B8", "R5G5B5A1", "R4G4B4A4", "R8G8B8A8"};

    int channelsActive = 3;
    const char *channelsTextList[4] = {"1", "2", "3", "4"};
    int bitDepthActive = 0;
    const char *bitDepthTextList[3] = {"8", "16", "32"};

    int headerSizeValue = 0;
    bool headerSizeEditMode = false;
    //----------------------------------------------------------------------------------

    // Image file info
    int dataSize = 0;
    char fileNamePath[256] = "\0";
    char fileName[64] = "\0";

    bool btnLoadPressed = false;

    bool imageLoaded = false;
    float imageScale = 1.0;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // Check if a file is dropped
        if rl.is_file_dropped()
        {
            int fileCount = 0;
            char **droppedFiles = rl.load_dropped_files(&fileCount);

            // Check file extensions for drag-and-drop
            if (fileCount == 1) && IsFileExtension(droppedFiles[0], ".raw")
            {
                FILE *imageFile = fopen(droppedFiles[0], "rb");
                fseek(imageFile, 0L, SEEK_END);
                dataSize = ftell(imageFile);
                fclose(imageFile);

                // NOTE: Returned string is just a pointer to droppedFiles[0],
                // we need to make a copy of that data somewhere else: fileName
                strcpy(fileNamePath, droppedFiles[0]);
                strcpy(fileName, GetFileName(droppedFiles[0]));

                // Try to guess possible raw values
                // Let's assume image is square, RGBA, 8 bit per channel
                widthValue = round(sqrt(dataSize / 4));
                heightValue = widthValue;
                headerSizeValue = dataSize - widthValue * heightValue * 4;
                if headerSizeValue < 0
                    headerSizeValue = 0;

                importWindowActive = true;
            }

            UnloadDroppedFiles();
        }

        // Check if load button has been pressed
        if btnLoadPressed
        {
            // Depending on channels and bit depth, select correct pixel format
            if (widthValue != 0) && (heightValue != 0)
            {
                int format = -1;

                if pixelFormatActive == 0
                {
                    int channels = atoi(channelsTextList[channelsActive]);
                    int bpp = atoi(bitDepthTextList[bitDepthActive]);

                    // Select correct format depending on channels and bpp
                    if bpp == 8
                    {
                        if channels == 1
                            format = UNCOMPRESSED_GRAYSCALE;
                        else if channels == 2
                            format = UNCOMPRESSED_GRAY_ALPHA;
                        else if channels == 3
                            format = UNCOMPRESSED_R8G8B8;
                        else if channels == 4
                            format = UNCOMPRESSED_R8G8B8A8;
                    }
                    else if bpp == 32
                    {
                        if channels == 1
                            format = UNCOMPRESSED_R32;
                        else if channels == 2
                            TraceLog(LOG_WARNING, "Channel bit-depth not supported!");
                        else if channels == 3
                            format = UNCOMPRESSED_R32G32B32;
                        else if channels == 4
                            format = UNCOMPRESSED_R32G32B32A32;
                    }
                    else if bpp == 16
                        TraceLog(LOG_WARNING, "Channel bit-depth not supported!");
                }
                else
                    format = pixelFormatActive;

                if format != -1
                {
                    Image image = LoadImageRaw(fileNamePath, widthValue, heightValue, format, headerSizeValue);
                    texture = LoadTextureFromImage(image);
                    UnloadImage(image);

                    importWindowActive = false;
                    btnLoadPressed = false;

                    if texture.id > 0
                    {
                        imageLoaded = true;
                        imageScale = (float)(screen_height - 100) / texture.height;
                    }
                }
            }
        }

        if imageLoaded
            imageScale += (float)rl.get_mouse_wheel_move(); // Image scale control
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        ClearBackground(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)));

        if texture.id != 0
        {
            d.draw_texture_ex(texture, rvec2(screen_width / 2 - texture.width * imageScale / 2,  screen_height / 2 - texture.height * imageScale / 2), 0, imageScale, Color::WHITE);
            d.draw_text(&format!("SCALE x%.0", imageScale), 20, screen_height - 40, 20, GetColor(GuiGetStyle(DEFAULT, LINE_COLOR)));
        }
        else
            d.draw_text("drag & drop RAW image file", 320, 180, 10, GetColor(GuiGetStyle(DEFAULT, LINE_COLOR)));

        // raygui: controls drawing
        //----------------------------------------------------------------------------------
        if importWindowActive
        {
            importWindowActive = !GuiWindowBox(rrect(windowOffset.x + 0, windowOffset.y + 0, 200, 465), "Image RAW Import Options");

            GuiLabel(rrect(windowOffset.x + 10, windowOffset.y + 30, 65, 20), "Import file:");
            GuiLabel(rrect(windowOffset.x + 85, windowOffset.y + 30, 75, 20), fileName);
            GuiLabel(rrect(windowOffset.x + 10, windowOffset.y + 50, 65, 20), "File size:");
            GuiLabel(rrect(windowOffset.x + 85, windowOffset.y + 50, 75, 20), &format!("{} bytes", dataSize));
            GuiGroupBox(rrect(windowOffset.x + 10, windowOffset.y + 85, 180, 80), "Resolution");
            GuiLabel(rrect(windowOffset.x + 20, windowOffset.y + 100, 33, 25), "Width:");
            if GuiValueBox(rrect(windowOffset.x + 60, windowOffset.y + 100, 80, 25), NULL, &widthValue, 0, 8192, widthEditMode)
                widthEditMode = !widthEditMode;
            GuiLabel(rrect(windowOffset.x + 145, windowOffset.y + 100, 30, 25), "pixels");
            GuiLabel(rrect(windowOffset.x + 20, windowOffset.y + 130, 33, 25), "Height:");
            if GuiValueBox(rrect(windowOffset.x + 60, windowOffset.y + 130, 80, 25), NULL, &heightValue, 0, 8192, heightEditMode)
                heightEditMode = !heightEditMode;
            GuiLabel(rrect(windowOffset.x + 145, windowOffset.y + 130, 30, 25), "pixels");
            GuiGroupBox(rrect(windowOffset.x + 10, windowOffset.y + 180, 180, 160), "Pixel Format");
            pixelFormatActive = GuiComboBox(rrect(windowOffset.x + 20, windowOffset.y + 195, 160, 25), TextJoin(pixel&format!List, 8, ";"), pixelFormatActive);
            GuiLine(rrect(windowOffset.x + 20, windowOffset.y + 220, 160, 20), NULL);

            if pixelFormatActive != 0
                GuiDisable();
            GuiLabel(rrect(windowOffset.x + 20, windowOffset.y + 235, 50, 20), "Channels:");
            channelsActive = GuiToggleGroup(rrect(windowOffset.x + 20, windowOffset.y + 255, 156 / 4, 25), TextJoin(channelsTextList, 4, ";"), channelsActive);
            GuiLabel(rrect(windowOffset.x + 20, windowOffset.y + 285, 50, 20), "Bit Depth:");
            bitDepthActive = GuiToggleGroup(rrect(windowOffset.x + 20, windowOffset.y + 305, 160 / 3, 25), TextJoin(bitDepthTextList, 3, ";"), bitDepthActive);
            GuiEnable();

            GuiGroupBox(rrect(windowOffset.x + 10, windowOffset.y + 355, 180, 50), "Header");
            GuiLabel(rrect(windowOffset.x + 25, windowOffset.y + 370, 27, 25), "Size:");
            if GuiValueBox(rrect(windowOffset.x + 55, windowOffset.y + 370, 85, 25), NULL, &headerSizeValue, 0, 10000, headerSizeEditMode)
                headerSizeEditMode = !headerSizeEditMode;
            GuiLabel(rrect(windowOffset.x + 145, windowOffset.y + 370, 30, 25), "bytes");

            btnLoadPressed = GuiButton(rrect(windowOffset.x + 10, windowOffset.y + 420, 180, 30), "Import RAW");
        }
        //----------------------------------------------------------------------------------

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    if texture.id != 0
        UnloadTexture(texture);

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
