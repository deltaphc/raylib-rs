/*******************************************************************************************
*
*   raygui - Controls test
*
*   TEST CONTROLS:
*       - GuiTextEditor()
*
*   DEPENDENCIES:
*       raylib 3.0  - Windowing/input management and drawing.
*       raygui 2.7  - Immediate-mode GUI controls.
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

    //----------------------------------------------------------------------------------
    // Defines and Macros
    //----------------------------------------------------------------------------------
    // ...

    //----------------------------------------------------------------------------------
    // Global Variables Definition
    //----------------------------------------------------------------------------------
    static char text01[128] = "Lorem ipsum dolor sit amet, \xE7\x8C\xBF\xE3\x82\x82\xE6\x9C\xA8\xE3\x81\x8B\xE3\x82\x89\xE8\x90\xBD\xE3\x81\xA1\xE3\x82\x8B consectetur adipiscing elit...\0"; // including some hiragana/kanji
static char text02[128] = "Here's another, much bigger textbox extended.\xf4\xa1\xa1\xff TIP: try COPY/PASTE ;)\0";                                                                            // Including some invalid UTF8

bool GuiTextEditor(Rectangle bounds, char *text, int textLen, bool editMode);

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
int main(int argc, char **argv)
{
    // Initialization
    //---------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - gui text editor test");


    Font font = {0};

    bool textEditor01EditMode = false;
    bool textEditor02EditMode = false;

    rl.set_target_fps(60);
    //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        Vector2 mouse = rl.get_mouse_position();

        // Fonts drag & drop logic
        if rl.is_file_dropped()
        {
            int count = 0;
            char **files = rl.get_dropped_files(&count);

            if IsFileExtension(files[0], ".ttf" ||
                IsFileExtension(files[0], ".otf") ||
                IsFileExtension(files[0], ".fnt"))
            {
                Font fnt = LoadFont(files[0]);

                if fnt.texture.id != 0
                {
                    // Font was loaded, only change font on success
                    GuiSetFont(fnt);

                    // Remove old font
                    if font.texture.id != 0
                        UnloadFont(font);
                    font = fnt;
                }
            }

            UnloadDroppedFiles();
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        // Draw textboxes extended
        //---------------------------------------------------------------------------------------
        if GuiTextEditor(rrect(20, 20, 380, 410), text01, strlen(text01), textEditor01EditMode)
            textEditor01EditMode = !textEditor01EditMode;
        if GuiTextEditor(rrect(420, 20, 360, 410), text02, strlen(text02), textEditor02EditMode)
            textEditor02EditMode = !textEditor02EditMode;
        //---------------------------------------------------------------------------------------

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}

// Text editor control (Advanced text box)
bool GuiTextEditor(Rectangle bounds, char *text, int textSize, bool editMode)
{
    static Rectangle cursor = {0}; // Cursor position and size
    static int framesCounter = 0;  // Blinking cursor frames counter
    static int cursorCodepoint = -1;
    static int selectStartCp = -1;
    static int selectLengthCp = 0;

    GuiControlState state = guiState;
    bool pressed = false;

    bool textWrap = true; // TODO: Word-Wrap vs Char-Wrap -> textWrapMode { NO_WRAP_LOCK, NO_WRAP_OVERFLOW, CHAR_WRAP, WORD_WRAP }

    // WARNING: First string full traversal
    int codepointCount = GetCodepointsCount(text);

    int textLen = strlen(text); // Text length in bytes

    // Update control
    //--------------------------------------------------------------------
    if (state != GUI_STATE_DISABLED) && !guiLocked
    {
        Vector2 mousePoint = rl.get_mouse_position();

        if editMode
        {
            state = GUI_STATE_PRESSED;
            framesCounter+=1;

            // TODO: Cursor position logic (mouse and keys)

            // Characters selection logic
            if selectStartCp != -1
            {
                if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT_SHIFT) && rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT)
                {
                    selectLengthCp+=1;
                    if selectLengthCp >= (codepointCount - selectStartCp)
                        selectLengthCp = codepointCount - selectStartCp;
                }

                if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT_SHIFT) && rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT)
                {
                    selectLengthCp-=1;
                    if selectLengthCp < 0
                        selectLengthCp = 0;
                }
            }

            int key = GetKeyPressed();

            // TODO: On key pressed, place new character in cursor position

            // Exit edit mode logic
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) || (!CheckCollisionPointRec(mousePoint, bounds) && rl.is_mouse_button_pressed(0))
                pressed = true;
        }
        else
        {
            if CheckCollisionPointRec(mousePoint, bounds)
            {
                state = GUI_STATE_FOCUSED;
                if rl.is_mouse_button_pressed(0)
                    pressed = true;
            }
        }

        if pressed
        {
            // Exiting edit mode, reset temp variables
            framesCounter = 0;
            cursor = (Rectangle){0};

            cursorCodepoint = -1;
            selectStartCp = -1;
            selectLengthCp = 0;
        }
    }
    //--------------------------------------------------------------------

    // Draw control
    //--------------------------------------------------------------------
    d.draw_rectangle_lines_ex(bounds, GuiGetStyle(TEXTBOX, BORDER_WIDTH), Fade(GetColor(GuiGetStyle(TEXTBOX, BORDER + (state * 3))), guiAlpha));

    if state == GUI_STATE_PRESSED
        d.draw_rectangle(bounds.x + GuiGetStyle(TEXTBOX, BORDER_WIDTH), bounds.y + GuiGetStyle(TEXTBOX, BORDER_WIDTH), bounds.width - 2 * GuiGetStyle(TEXTBOX, BORDER_WIDTH), bounds.height - 2 * GuiGetStyle(TEXTBOX, BORDER_WIDTH), Fade(GetColor(GuiGetStyle(TEXTBOX, BASE_COLOR_PRESSED)), guiAlpha));
    else if state == GUI_STATE_DISABLED
        d.draw_rectangle(bounds.x + GuiGetStyle(TEXTBOX, BORDER_WIDTH), bounds.y + GuiGetStyle(TEXTBOX, BORDER_WIDTH), bounds.width - 2 * GuiGetStyle(TEXTBOX, BORDER_WIDTH), bounds.height - 2 * GuiGetStyle(TEXTBOX, BORDER_WIDTH), Fade(GetColor(GuiGetStyle(TEXTBOX, BASE_COLOR_DISABLED)), guiAlpha));

    Font font = GetFontDefault();

    int textOffsetY = 0;      // Offset between lines (on line break '\n')
    float textOffsetX = 0.0; // Offset X to next character to draw

    float scaleFactor = GuiGetStyle(DEFAULT, TEXT_SIZE) * 2 / font.baseSize; // Character quad scaling factor

    for (int i = 0, cp = 0; i < textLen; i+=1)
    {
        // Get next codepoint from byte string and glyph index in font
        int codepointByteCount = 0;
        int codepoint = GetNextCodepoint(&text[i], &codepointByteCount);
        int index = GetGlyphIndex(font, codepoint);

        Rectangle rec = {bounds.x + textOffsetX + font.chars[index].offsetX * scaleFactor,
                         bounds.y + textOffsetY + font.chars[index].offsetY * scaleFactor,
                         font.recs[index].width * scaleFactor, font.recs[index].height * scaleFactor};

        // Automatic line break to wrap text inside box
        if textWrap && ((rec.x + rec.width) >= (bounds.x + bounds.width))
        {
            textOffsetY += (int)((font.baseSize + font.baseSize / 2) * scaleFactor);
            textOffsetX = 0.0;

            // Recalculate drawing rectangle position
            rec = (Rectangle){bounds.x + textOffsetX + font.chars[index].offsetX * scaleFactor,
                              bounds.y + textOffsetY + font.chars[index].offsetY * scaleFactor,
                              font.recs[index].width * scaleFactor, font.recs[index].height * scaleFactor};
        }

        // Check selected codepoint
        if editMode
        {
            if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) && CheckCollisionPointRec(rl.get_mouse_position(), rec)
            {
                cursor = rec;
                cursorCodepoint = cp;
                selectStartCp = cursorCodepoint;
                selectLengthCp = 0;

                // TODO: Place cursor at the end if pressed out of text
            }

            // On mouse left button down allow text selection
            if (selectStartCp != -1) && rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) && CheckCollisionPointRec(rl.get_mouse_position(), rec)
            {
                if cp >= selectStartCp
                    selectLengthCp = cp - selectStartCp;
                else if cp < selectStartCp
                {
                    //int temp = selectStartCp;
                    //selectStartCp = cp;
                    //selectLengthCp = temp - selectStartCp;
                }
            }
        }

        if codepoint == '\n' // Line break character
        {
            // NOTE: Fixed line spacing of 1.5 line-height
            // TODO: Support custom line spacing defined by user
            textOffsetY += (int)((font.baseSize + font.baseSize / 2) * scaleFactor);
            textOffsetX = 0.0;
        }
        else
        {
            // Draw codepoint glyph
            if (codepoint != ' ') && (codepoint != '\t') && ((rec.x + rec.width) < (bounds.x + bounds.width))
            {
                d.draw_texture_pro(font.texture, font.recs[index], rec, rvec2(0,  0), 0.0, GetColor(GuiGetStyle(DEFAULT, TEXT_COLOR_NORMAL)));
            }

            // TODO: On text overflow do something... move text to the left?
        }

        // Draw codepoints selection from selectStartCp to selectLengthCp
        // TODO: Consider spacing when drawing selected characters background
        if editMode && (selectStartCp != -1) && ((cp >= selectStartCp) && (cp <= (selectStartCp + selectLengthCp)))
            d.draw_rectangle_rec(rec, Color::MAROON);

        if font.chars[index].advanceX == 0
            textOffsetX += ((float)font.recs[index].width * scaleFactor + GuiGetStyle(DEFAULT, TEXT_SPACING));
        else
            textOffsetX += ((float)font.chars[index].advanceX * scaleFactor + GuiGetStyle(DEFAULT, TEXT_SPACING));

        i += (codepointByteCount - 1); // Move text bytes counter to next codepoint
        cp+=1;
    }

    // Draw blinking cursor
    if editMode && ((framesCounter / 20) % 2 == 0)
        d.draw_rectangle_rec(cursor, Fade(GetColor(GuiGetStyle(TEXTBOX, BORDER_COLOR_PRESSED)), guiAlpha));

    //Guid.draw_text(text, GetTextBounds(TEXTBOX, bounds), GuiGetStyle(TEXTBOX, TEXT_ALIGNMENT), Fade(GetColor(GuiGetStyle(TEXTBOX, TEXT + (state*3))), guiAlpha));
    //--------------------------------------------------------------------

    return pressed;
}