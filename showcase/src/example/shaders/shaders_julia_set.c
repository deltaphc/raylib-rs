/*******************************************************************************************
*
*   raylib [shaders] example - julia sets
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3).
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by eggmund (@eggmund) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 eggmund (@eggmund) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#if defined(PLATFORM_DESKTOP)
const GLSL_VERSION 330
#else // PLATFORM_RPI, PLATFORM_ANDROID, PLATFORM_WEB
const GLSL_VERSION 100
#endif

    // A few good julia sets
    const float POINTS_OF_INTEREST[6][2] =
        {
            {-0.348827, 0.607167},
            {-0.786268, 0.169728},
            {-0.8, 0.156},
            {0.285, 0.0},
            {-0.835, -0.2321},
            {-0.70176, -0.3842},
};

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
    rl.set_window_title(thread, "raylib [shaders] example - julia sets");


    // Load julia set shader
    // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
    Shader shader = LoadShader(0, &format!("resources/shaders/glsl{}/julia_set.fs", GLSL_VERSION));

    // c constant to use in z^2 + c
    float c[2] = {POINTS_OF_INTEREST[0][0], POINTS_OF_INTEREST[0][1]};

    // Offset and zoom to draw the julia set at. (centered on screen and default size)
    float offset[2] = {-(float)screen_width / 2, -(float)screen_height / 2};
    float zoom = 1.0;

    let offsetSpeed = rvec2(0.0, 0.0);

    // Get variable (uniform) locations on the shader to connect with the program
    // NOTE: If uniform variable could not be found in the shader, function returns -1
    int cLoc = GetShaderLocation(shader, "c");
    int zoomLoc = GetShaderLocation(shader, "zoom");
    int offsetLoc = GetShaderLocation(shader, "offset");

    // Tell the shader what the screen dimensions, zoom, offset and c are
    float screenDims[2] = {(float)screen_width, (float)screen_height};
    SetShaderValue(shader, GetShaderLocation(shader, "screenDims"), screenDims, UNIFORM_VEC2);

    SetShaderValue(shader, cLoc, c, UNIFORM_VEC2);
    SetShaderValue(shader, zoomLoc, &zoom, UNIFORM_FLOAT);
    SetShaderValue(shader, offsetLoc, offset, UNIFORM_VEC2);

    // Create a RenderTexture2D to be used for render to texture
    RenderTexture2D target = LoadRenderTexture(screen_width, screen_height);

    int incrementSpeed = 0;   // Multiplier of speed to change c value
    bool showControls = true; // Show controls
    bool pause = false;       // Pause animation

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // Press [1 - 6] to reset c to a point of interest
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ONE ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_TWO) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_THREE) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FOUR) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FIVE) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SIX))
        {
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ONE)
                c[0] = POINTS_OF_INTEREST[0][0], c[1] = POINTS_OF_INTEREST[0][1];
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_TWO)
                c[0] = POINTS_OF_INTEREST[1][0], c[1] = POINTS_OF_INTEREST[1][1];
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_THREE)
                c[0] = POINTS_OF_INTEREST[2][0], c[1] = POINTS_OF_INTEREST[2][1];
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FOUR)
                c[0] = POINTS_OF_INTEREST[3][0], c[1] = POINTS_OF_INTEREST[3][1];
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FIVE)
                c[0] = POINTS_OF_INTEREST[4][0], c[1] = POINTS_OF_INTEREST[4][1];
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SIX)
                c[0] = POINTS_OF_INTEREST[5][0], c[1] = POINTS_OF_INTEREST[5][1];

            SetShaderValue(shader, cLoc, c, UNIFORM_VEC2);
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
            pause = !pause; // Pause animation (c change)
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_F1)
            showControls = !showControls; // Toggle whether or not to show controls

        if !pause
        {
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT)
                incrementSpeed++;
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT)
                incrementSpeed--;

            // TODO: The idea is to zoom and move around with mouse
            // Probably offset movement should be proportional to zoom level
            if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) || rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON)
            {
                if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
                    zoom += zoom * 0.003f;
                if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON)
                    zoom -= zoom * 0.003f;

                Vector2 mousePos = rl.get_mouse_position();

                offsetSpeed.x = mousePos.x - (float)screen_width / 2;
                offsetSpeed.y = mousePos.y - (float)screen_height / 2;

                // Slowly move camera to targetOffset
                offset[0] += GetFrameTime() * offsetSpeed.x * 0.8f;
                offset[1] += GetFrameTime() * offsetSpeed.y * 0.8f;
            }
            else
                offsetSpeed = rvec2(0.0, 0.0);

            SetShaderValue(shader, zoomLoc, &zoom, UNIFORM_FLOAT);
            SetShaderValue(shader, offsetLoc, offset, UNIFORM_VEC2);

            // Increment c value with time
            float amount = GetFrameTime() * incrementSpeed * 0.0005f;
            c[0] += amount;
            c[1] += amount;

            SetShaderValue(shader, cLoc, c, UNIFORM_VEC2);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::BLACK); // Clear the screen of the previous frame.

        // Using a render texture to draw Julia set
        let mut d = d.begin_texture_mode(thread, &target); // Enable drawing to texture
        d.clear_background(Color::BLACK);   // Clear the render texture

        // Draw a rectangle in shader mode to be used as shader canvas
        // NOTE: Rectangle uses font white character texture coordinates,
        // so shader can not be applied here directly because input vertexTexCoord
        // do not represent full screen coordinates (space where want to apply shader)
        d.draw_rectangle(0, 0, rl.get_screen_width(), rl.get_screen_height(), Color::BLACK);
        EndTextureMode();

        // Draw the saved texture and rendered julia set with shader
        // NOTE: We do not invert texture on Y, already considered inside shader
        BeginShaderMode(shader);
        d.draw_texture(target.texture, 0, 0, Color::WHITE);
        EndShaderMode();

        if showControls
        {
            d.draw_text("Press Mouse buttons right/left to zoom in/out and move", 10, 15, 10, Color::RAYWHITE);
            d.draw_text("Press KEY_F1 to toggle these controls", 10, 30, 10, Color::RAYWHITE);
            d.draw_text("Press KEYS [1 - 6] to change point of interest", 10, 45, 10, Color::RAYWHITE);
            d.draw_text("Press KEY_LEFT | KEY_RIGHT to change speed", 10, 60, 10, Color::RAYWHITE);
            d.draw_text("Press KEY_SPACE to pause movement animation", 10, 75, 10, Color::RAYWHITE);
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadShader(shader);        // Unload shader
    UnloadRenderTexture(target); // Unload render texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
