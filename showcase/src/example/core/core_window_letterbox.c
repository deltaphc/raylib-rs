/*******************************************************************************************
*
*   raylib [core] example - window scale letterbox (and virtual mouse)
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Anata (@anatagawa) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Anata (@anatagawa) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const max(a, b)((a) > (b) ? (a) : (b))
    const min(a, b)((a) < (b) ? (a) : (b))

    // Clamp Vector2 value with min and max and return a new vector2
    // NOTE: Required for virtual mouse, to clamp inside virtual game size
    Vector2 ClampValue(Vector2 value, Vector2 min, Vector2 max)
{
    Vector2 result = value;
    result.x = (result.x > max.x) ? max.x : result.x;
    result.x = (result.x < min.x) ? min.x : result.x;
    result.y = (result.y > max.y) ? max.y : result.y;
    result.y = (result.y < min.y) ? min.y : result.y;
    return result;
}

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{
    let windowWidth = 800;
    let windowHeight = 450;

    // Enable config flags for resizable window and vertical synchro
    SetConfigFlags(FLAG_WINDOW_RESIZABLE | FLAG_VSYNC_HINT);
    InitWindow(windowWidth, windowHeight, "raylib [core] example - window scale letterbox");
    SetWindowMinSize(320, 240);

    int gamescreen_width = 640;
    int gamescreen_height = 480;

    // Render texture initialization, used to hold the rendering result so we can easily resize it
    RenderTexture2D target = LoadRenderTexture(gamescreen_width, gamescreen_height);
    SetTextureFilter(target.texture, FILTER_BILINEAR); // Texture scale filter to use

    Color colors[10] = {0};
    for (int i = 0; i < 10; i++)
        colors[i] = (Color){raylib::get_random_value(100, 250), raylib::get_random_value(50, 150), raylib::get_random_value(10, 100), 255};

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // Compute required framebuffer scaling
        float scale = min((float)Getscreen_width() / gamescreen_width, (float)Getscreen_height() / gamescreen_height);

        if (IsKeyPressed(KEY_SPACE))
        {
            // Recalculate random colors for the bars
            for (int i = 0; i < 10; i++)
                colors[i] = (Color){raylib::get_random_value(100, 250), raylib::get_random_value(50, 150), raylib::get_random_value(10, 100), 255};
        }

        // Update virtual mouse (clamped mouse value behind game screen)
        Vector2 mouse = GetMousePosition();
        Vector2 virtualMouse = {0};
        virtualMouse.x = (mouse.x - (Getscreen_width() - (gamescreen_width * scale)) * 0.5) / scale;
        virtualMouse.y = (mouse.y - (Getscreen_height() - (gamescreen_height * scale)) * 0.5) / scale;
        virtualMouse = ClampValue(virtualMouse, (Vector2){0, 0}, (Vector2){gamescreen_width, gamescreen_height});
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        // Draw everything in the render texture, note this will not be rendered on screen, yet
        BeginTextureMode(target);

        d.clear_background(Color::RAYWHITE); // Clear render texture background color

        for (int i = 0; i < 10; i++)
            d.draw_rectangle(0, (gamescreen_height / 10) * i, gamescreen_width, gamescreen_height / 10, colors[i]);

        d.draw_text("If executed inside a window,\nyou can resize the window,\nand see the screen scaling!", 10, 25, 20, WHITE);

        d.draw_text(TextFormat("Default Mouse: [%i , %i]", (int)mouse.x, (int)mouse.y), 350, 25, 20, GREEN);
        d.draw_text(TextFormat("Virtual Mouse: [%i , %i]", (int)virtualMouse.x, (int)virtualMouse.y), 350, 55, 20, YELLOW);

        EndTextureMode();

        // Draw RenderTexture2D to window, properly scaled
        DrawTexturePro(target.texture, (Rectangle){0.0, 0.0, (float)target.texture.width, (float)-target.texture.height},
                       (Rectangle){(Getscreen_width() - ((float)gamescreen_width * scale)) * 0.5, (Getscreen_height() - ((float)gamescreen_height * scale)) * 0.5,
                                   (float)gamescreen_width * scale, (float)gamescreen_height * scale},
                       (Vector2){0, 0}, 0.0, WHITE);

        EndDrawing();
        //--------------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadRenderTexture(target); // Unload render texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
