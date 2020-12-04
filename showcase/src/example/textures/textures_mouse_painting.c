/*******************************************************************************************
*
*   raylib [textures] example - Mouse painting
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Dill (@MysteriousSpace) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Chris Dill (@MysteriousSpace) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_COLORS_COUNT 23 // Number of colors available

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - mouse painting");


    // Colours to choose from
    Color colors[MAX_COLORS_COUNT] = {
        Color::RAYWHITE, Color::YELLOW, Color::GOLD, Color::ORANGE, Color::PINK,Color::RED, Color::MAROON, Color::GREEN, Color::LIME, DARKGREEN,
        Color::SKYBLUE, Color::BLUE, Color::DARKBLUE, Color::PURPLE, Color::VIOLET, DARKPURPLE, Color::BEIGE, Color::BROWN, DARKBROWN,
        Color::LIGHTGRAY, Color::GRAY, Color::DARKGRAY, Color::BLACK};

    // Define colorsRecs data (for every rectangle)
    Rectangle colorsRecs[MAX_COLORS_COUNT] = {0};

    for (int i = 0; i < MAX_COLORS_COUNT; i++)
    {
        colorsRecs[i].x = 10 + 30 * i + 2 * i;
        colorsRecs[i].y = 10;
        colorsRecs[i].width = 30;
        colorsRecs[i].height = 30;
    }

    int colorSelected = 0;
    int colorSelectedPrev = colorSelected;
    int colorMouseHover = 0;
    int brushSize = 20;

    let btnSaveRec  = rrect(750,  10,  40,  30);
    bool btnSaveMouseHover = false;
    bool showSaveMessage = false;
    int saveMessageCounter = 0;

    // Create a RenderTexture2D to use as a canvas
    RenderTexture2D target = LoadRenderTexture(screen_width, screen_height);

    // Clear render texture before entering the game loop
    let mut d = d.begin_texture_mode(thread, &target);
    ClearBackground(colors[0]);
    EndTextureMode();

    rl.set_target_fps(120); // Set our game to run at 120 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        Vector2 mousePos = rl.get_mouse_position();

        // Move between colors with keys
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT)
            colorSelected++;
        else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT)
            colorSelected--;

        if colorSelected >= MAX_COLORS_COUNT
            colorSelected = MAX_COLORS_COUNT - 1;
        else if colorSelected < 0
            colorSelected = 0;

        // Choose color with mouse
        for (int i = 0; i < MAX_COLORS_COUNT; i++)
        {
            if CheckCollisionPointRec(mousePos, colorsRecs[i])
            {
                colorMouseHover = i;
                break;
            }
            else
                colorMouseHover = -1;
        }

        if (colorMouseHover >= 0) && rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            colorSelected = colorMouseHover;
            colorSelectedPrev = colorSelected;
        }

        // Change brush size
        brushSize += rl.get_mouse_wheel_move() * 5;
        if brushSize < 2
            brushSize = 2;
        if brushSize > 50
            brushSize = 50;

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_C)
        {
            // Clear render texture to clear color
            let mut d = d.begin_texture_mode(thread, &target);
            ClearBackground(colors[0]);
            EndTextureMode();
        }

        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) || (GetGestureDetected() == GESTURE_DRAG)
        {
            // Paint circle into render texture
            // NOTE: To avoid discontinuous circles, we could store
            // previous-next mouse points and just draw a line using brush size
            let mut d = d.begin_texture_mode(thread, &target);
            if mousePos.y > 50
                d.draw_circle(mousePos.x, mousePos.y, brushSize, colors[colorSelected]);
            EndTextureMode();
        }

        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON)
        {
            colorSelected = 0;

            // Erase circle from render texture
            let mut d = d.begin_texture_mode(thread, &target);
            if mousePos.y > 50
                d.draw_circle(mousePos.x, mousePos.y, brushSize, colors[0]);
            EndTextureMode();
        }
        else
            colorSelected = colorSelectedPrev;

        // Check mouse hover save button
        if CheckCollisionPointRec(mousePos, btnSaveRec)
            btnSaveMouseHover = true;
        else
            btnSaveMouseHover = false;

        // Image saving logic
        // NOTE: Saving painted texture to a default named image
        if (btnSaveMouseHover && rl.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)) || rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S)
        {
            Image image = GetTextureData(target.texture);
            ImageFlipVertical(&image);
            ExportImage(image, "my_amazing_texture_painting.png");
            UnloadImage(image);
            showSaveMessage = true;
        }

        if showSaveMessage
        {
            // On saving, show a full screen message for 2 seconds
            saveMessageCounter++;
            if saveMessageCounter > 240
            {
                showSaveMessage = false;
                saveMessageCounter = 0;
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        DrawTextureRec(target.texture, rrect(0, 0, target.texture.width, -target.texture.height), rvec2(0,  0), Color::WHITE);

        // Draw drawing circle for reference
        if mousePos.y > 50
        {
            if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON)
                DrawCircleLines(mousePos.x, mousePos.y, brushSize, Color::GRAY);
            else
                d.draw_circle(GetMouseX(), GetMouseY(), brushSize, colors[colorSelected]);
        }

        // Draw top panel
        d.draw_rectangle(0, 0, rl.get_screen_width(), 50, Color::RAYWHITE);
        DrawLine(0, 50, rl.get_screen_width(), 50, Color::LIGHTGRAY);

        // Draw color selection rectangles
        for (int i = 0; i < MAX_COLORS_COUNT; i++)
            d.draw_rectangle_rec(colorsRecs[i], colors[i]);
        d.draw_rectangle_lines(10, 10, 30, 30, Color::LIGHTGRAY);

        if colorMouseHover >= 0
            d.draw_rectangle_rec(colorsRecs[colorMouseHover], Fade(WHITE, 0.6f));

        d.draw_rectangle_lines_ex((Rectangle){colorsRecs[colorSelected].x - 2, colorsRecs[colorSelected].y - 2,
                                         colorsRecs[colorSelected].width + 4, colorsRecs[colorSelected].height + 4},
                             2, Color::BLACK);

        // Draw save image button
        d.draw_rectangle_lines_ex(btnSaveRec, 2, btnSaveMouseHover ?Color::RED : Color::BLACK);
        d.draw_text("SAVE!", 755, 20, 10, btnSaveMouseHover ?Color::RED : Color::BLACK);

        // Draw save image message
        if showSaveMessage
        {
            d.draw_rectangle(0, 0, rl.get_screen_width(), rl.get_screen_height(), Fade(RAYWHITE, 0.8f));
            d.draw_rectangle(0, 150, rl.get_screen_width(), 80, Color::BLACK);
            d.draw_text("IMAGE SAVED:  my_amazing_texture_painting.png", 150, 180, 20, Color::RAYWHITE);
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadRenderTexture(target); // Unload render texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
