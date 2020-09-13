/*******************************************************************************************
*
*   raygui - custom file dialog to load image
*
*   DEPENDENCIES:
*       raylib 2.6-dev  - Windowing/input management and drawing.
*       raygui 2.6-dev  - Immediate-mode GUI controls.
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

const RAYGUI_IMPLEMENTATION const RAYGUI_SUPPORT_ICONS
#include "../../src/raygui.h"

#undef RAYGUI_IMPLEMENTATION // Avoid including raygui implementation again

    const GUI_FILE_DIALOG_IMPLEMENTATION
#include "gui_file_dialog.h"

    //------------------------------------------------------------------------------------
    // Program main entry point
    //------------------------------------------------------------------------------------
    int
    main()
{
    // Initialization
    //---------------------------------------------------------------------------------------
    int screen_width = 800;
    int screen_height = 560;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - custom modal dialog");

    SetExitKey(0);

    // Custom file dialog
    GuiFileDialogState fileDialogState = InitGuiFileDialog(420, 310, GetWorkingDirectory(), false);

    bool exitWindow = false;

    char fileNameToLoad[512] = {0};

    Texture texture = {0};

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    while (!exitWindow) // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        exitWindow = WindowShouldClose();

        if fileDialogState.SelectFilePressed
        {
            // Load image file (if supported extension)
            if IsFileExtension(fileDialogState.fileNameText, ".png")
            {
                strcpy(fileNameToLoad, &format!("%s/%s", fileDialogState.dirPathText, fileDialogState.fileNameText));
                UnloadTexture(texture);
                texture = LoadTexture(fileNameToLoad);
            }

            fileDialogState.SelectFilePressed = false;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        ClearBackground(GetColor(GuiGetStyle(DEFAULT, BACKGROUND_COLOR)));

        d.draw_texture(texture, rl.get_screen_width() / 2 - texture.width / 2, rl.get_screen_height() / 2 - texture.height / 2 - 5, Color::WHITE);
        d.draw_rectangle_lines(rl.get_screen_width() / 2 - texture.width / 2, rl.get_screen_height() / 2 - texture.height / 2 - 5, texture.width, texture.height, Color::BLACK);

        d.draw_text(fileNameToLoad, 208, rl.get_screen_height() - 20, 10, Color::GRAY);

        // raygui: controls drawing
        //----------------------------------------------------------------------------------
        if fileDialogState.fileDialogActive
            GuiLock();

        if GuiButton(rrect(20, 20, 140, 30), GuiIconText(RICON_FILE_OPEN, "Open Image"))
            fileDialogState.fileDialogActive = true;

        GuiUnlock();

        // GUI: Dialog Window
        //--------------------------------------------------------------------------------
        GuiFileDialog(&fileDialogState);
        //--------------------------------------------------------------------------------

        //----------------------------------------------------------------------------------

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(texture); // Unload texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
