/*******************************************************************************************
*
*   raygui - image exporter
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
********************************************************************************************/

use raylib::prelude::*;

const RAYGUI_IMPLEMENTATION const RAYGUI_SUPPORT_RICONS
#include "../../src/raygui.h"

    //------------------------------------------------------------------------------------
    // Program main entry point
    //------------------------------------------------------------------------------------
    int
    main(int argc, char *argv[0])
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - image exporter");


    // GUI controls initialization
    //----------------------------------------------------------------------------------
    let windowBoxRec  = rrect(screen_width / 2 - 110,  screen_height / 2 - 100,  220,  190);
    bool windowBoxActive = false;

    int fileFormatActive = 0;
    const char *file&format!List[3] = {"IMAGE (.png)", "DATA (.raw)", "CODE (.h)"};

    int pixelFormatActive = 0;
    const char *pixel&format!List[7] = {"GRAYSCALE", "GRAY ALPHA", "R5G6B5", "R8G8B8", "R5G5B5A1", "R4G4B4A4", "R8G8B8A8"};

    bool textBoxEditMode = false;
    char fileName[32] = "untitled";
    //--------------------------------------------------------------------------------------

    Image image = {0};
    Texture2D texture = {0};

    bool imageLoaded = false;
    float imageScale = 1.0;
    Rectangle imageRec = {0.0};

    bool btnExport = false;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if IsFileDropped()
        {
            int fileCount = 0;
            char **droppedFiles = GetDroppedFiles(&fileCount);

            if fileCount == 1
            {
                Image imTemp = LoadImage(droppedFiles[0]);

                if imTemp.data != NULL
                {
                    UnloadImage(image);
                    image = imTemp;

                    UnloadTexture(texture);
                    texture = LoadTextureFromImage(image);

                    imageLoaded = true;
                    pixelFormatActive = image.format - 1;

                    if texture.height > texture.width
                        imageScale = (float)(screen_height - 100) / (float)texture.height;
                    else
                        imageScale = (float)(screen_width - 100) / (float)texture.width;
                }
            }

            ClearDroppedFiles();
        }

        if btnExport
        {
            if imageLoaded
            {
                ImageFormat(&image, pixelFormatActive + 1);

                if fileFormatActive == 0 // PNG
                {
                    if (GetExtension(fileName) == NULL) || (!IsFileExtension(fileName, ".png"))
                        strcat(fileName, ".png\0"); // No extension provided
                    ExportImage(image, fileName);
                }
                else if fileFormatActive == 1 // RAW
                {
                    if (GetExtension(fileName) == NULL) || (!IsFileExtension(fileName, ".raw"))
                        strcat(fileName, ".raw\0"); // No extension provided

                    int dataSize = GetPixelDataSize(image.width, image.height, image.format);

                    FILE *rawFile = fopen(fileName, "wb");
                    fwrite(image.data, dataSize, 1, rawFile);
                    fclose(rawFile);
                }
                else if fileFormatActive == 2 // CODE
                {
                    ExportImageAsCode(image, fileName);
                }
            }

            windowBoxActive = false;
        }

        if imageLoaded
        {
            imageScale += (float)rl.get_mouse_wheel_move() * 0.05f; // Image scale control
            if imageScale <= 0.1
                imageScale = 0.1;
            else if imageScale >= 5
                imageScale = 5;

            imageRec = (Rectangle){screen_width / 2 - (float)image.width * imageScale / 2,
                                   screen_height / 2 - (float)image.height * imageScale / 2,
                                   (float)image.width * imageScale, (float)image.height * imageScale};
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if texture.id > 0
        {
            DrawTextureEx(texture, rvec2(screen_width / 2 - (float)texture.width * imageScale / 2,  screen_height / 2 - (float)texture.height * imageScale / 2), 0.0, imageScale, Color::WHITE);

            d.draw_rectangle_linesEx(imageRec, 1, CheckCollisionPointRec(rl.get_mouse_position(), imageRec) ?Color::RED : Color::DARKGRAY);
            d.draw_text(&format!("SCALE: %.2%%", imageScale * 100.0), 20, screen_height - 40, 20, GetColor(GuiGetStyle(DEFAULT, LINE_COLOR)));
        }
        else
        {
            d.draw_text("DRAG & DROP YOUR IMAGE!", 350, 200, 10, Color::DARKGRAY);
            GuiDisable();
        }

        if GuiButton(rrect(screen_width - 170, screen_height - 50, 150, 30), "Image Export")
            windowBoxActive = true;
        GuiEnable();

        // Draw window box: windowBoxName
        //-----------------------------------------------------------------------------
        if windowBoxActive
        {
            d.draw_rectangle(0, 0, screen_width, screen_height, Fade(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)), 0.7f));
            windowBoxActive = !GuiWindowBox(rrect(windowBoxRec.x, windowBoxRec.y, 220, 190), "Image Export Options");

            GuiLabel(rrect(windowBoxRec.x + 10, windowBoxRec.y + 35, 60, 25), "File format:");
            fileFormatActive = GuiComboBox(rrect(windowBoxRec.x + 80, windowBoxRec.y + 35, 130, 25), TextJoin(file&format!List, 3, ";"), fileFormatActive);
            GuiLabel(rrect(windowBoxRec.x + 10, windowBoxRec.y + 70, 63, 25), "Pixel format:");
            pixelFormatActive = GuiComboBox(rrect(windowBoxRec.x + 80, windowBoxRec.y + 70, 130, 25), TextJoin(pixel&format!List, 7, ";"), pixelFormatActive);
            GuiLabel(rrect(windowBoxRec.x + 10, windowBoxRec.y + 105, 50, 25), "File name:");
            if GuiTextBox(rrect(windowBoxRec.x + 80, windowBoxRec.y + 105, 130, 25), fileName, 64, textBoxEditMode)
                textBoxEditMode = !textBoxEditMode;

            btnExport = GuiButton(rrect(windowBoxRec.x + 10, windowBoxRec.y + 145, 200, 30), "Export Image");
        }
        else
            btnExport = false;

        if btnExport
            d.draw_text("Image exported!", 20, screen_height - 20, 20,Color::RED);
        //-----------------------------------------------------------------------------

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadImage(image);
    UnloadTexture(texture);

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
