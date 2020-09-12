/*******************************************************************************************
*
*   raylib [textures] example - N-patch drawing
*
*   NOTE: Images are loaded in CPU memory (RAM); textures are loaded in GPU memory (VRAM)
*
*   This example has been created using raylib 2.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Jorge A. Gomes (@overdev) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2018 Jorge A. Gomes (@overdev) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - N-patch drawing");


    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)
    Texture2D nPatchTexture = LoadTexture("resources/ninepatch_button.png");

    Vector2 mousePosition = {0};
    Vector2 origin = {0.0, 0.0};

    // Position and size of the n-patches
    Rectangle dstRec1 = {480.0, 160.0, 32.0, 32.0};
    Rectangle dstRec2 = {160.0, 160.0, 32.0, 32.0};
    Rectangle dstRecH = {160.0, 93.0, 32.0, 32.0};
    Rectangle dstRecV = {92.0, 160.0, 32.0, 32.0};

    // A 9-patch (NPT_9PATCH) changes its sizes in both axis
    NPatchInfo ninePatchInfo1 = {(Rectangle){0.0, 0.0, 64.0, 64.0}, 12, 40, 12, 12, NPT_9PATCH};
    NPatchInfo ninePatchInfo2 = {(Rectangle){0.0, 128.0, 64.0, 64.0}, 16, 16, 16, 16, NPT_9PATCH};

    // A horizontal 3-patch (NPT_3PATCH_HORIZONTAL) changes its sizes along the x axis only
    NPatchInfo h3PatchInfo = {(Rectangle){0.0, 64.0, 64.0, 64.0}, 8, 8, 8, 8, NPT_3PATCH_HORIZONTAL};

    // A vertical 3-patch (NPT_3PATCH_VERTICAL) changes its sizes along the y axis only
    NPatchInfo v3PatchInfo = {(Rectangle){0.0, 192.0, 64.0, 64.0}, 6, 6, 6, 6, NPT_3PATCH_VERTICAL};

    rl.set_target_fps(60);
    //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        mousePosition = rl.get_mouse_position();

        // Resize the n-patches based on mouse position
        dstRec1.width = mousePosition.x - dstRec1.x;
        dstRec1.height = mousePosition.y - dstRec1.y;
        dstRec2.width = mousePosition.x - dstRec2.x;
        dstRec2.height = mousePosition.y - dstRec2.y;
        dstRecH.width = mousePosition.x - dstRecH.x;
        dstRecV.height = mousePosition.y - dstRecV.y;

        // Set a minimum width and/or height
        if (dstRec1.width < 1.0)
            dstRec1.width = 1.0;
        if (dstRec1.width > 300.0)
            dstRec1.width = 300.0;
        if (dstRec1.height < 1.0)
            dstRec1.height = 1.0;
        if (dstRec2.width < 1.0)
            dstRec2.width = 1.0;
        if (dstRec2.width > 300.0)
            dstRec2.width = 300.0;
        if (dstRec2.height < 1.0)
            dstRec2.height = 1.0;
        if (dstRecH.width < 1.0)
            dstRecH.width = 1.0;
        if (dstRecV.height < 1.0)
            dstRecV.height = 1.0;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        // Draw the n-patches
        DrawTextureNPatch(nPatchTexture, ninePatchInfo2, dstRec2, origin, 0.0, WHITE);
        DrawTextureNPatch(nPatchTexture, ninePatchInfo1, dstRec1, origin, 0.0, WHITE);
        DrawTextureNPatch(nPatchTexture, h3PatchInfo, dstRecH, origin, 0.0, WHITE);
        DrawTextureNPatch(nPatchTexture, v3PatchInfo, dstRecV, origin, 0.0, WHITE);

        // Draw the source texture
        d.draw_rectangle_lines(5, 88, 74, 266, Color::BLUE);
        DrawTexture(nPatchTexture, 10, 93, WHITE);
        d.draw_text("TEXTURE", 15, 360, 10, Color::DARKGRAY);

        d.draw_text("Move the mouse to stretch or shrink the n-patches", 10, 20, 20, Color::DARKGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(nPatchTexture); // Texture unloading

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
