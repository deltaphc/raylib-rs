/*******************************************************************************************
*
*   raylib [shaders] example - Apply a shader to a 3d model
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3), to test this example
*         on OpenGL ES 2.0 platforms (Android, Raspberry Pi, HTML5), use #version 100 shaders
*         raylib comes with shaders ready for both versions, check raylib/shaders install folder
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#if defined(PLATFORM_DESKTOP)
const GLSL_VERSION 330
#else // PLATFORM_RPI, PLATFORM_ANDROID, PLATFORM_WEB
const GLSL_VERSION 100
#endif

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    SetConfigFlags(FLAG_MSAA_4X_HINT); // Enable Multi Sampling Anti Aliasing 4x (if available)

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - model shader");


    // Define the camera to look into our 3d world
    Camera camera = {0};
    camera.position = rvec3(4.0, 4.0, 4.0);
    camera.target = rvec3(0.0, 1.0, 1.0);
    camera.up = rvec3(0.0, 1.0, 0.0);
    camera.fovy = 45.0;
    camera.type = CAMERA_PERSPECTIVE;

    Model model = LoadModel("resources/models/watermill.obj");                 // Load OBJ model
    Texture2D texture = LoadTexture("resources/models/watermill_diffuse.png"); // Load model texture

    // Load shader for model
    // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
    Shader shader = LoadShader(0, FormatText("resources/shaders/glsl%i/grayscale.fs", GLSL_VERSION));

    model.materials[0].shader = shader;                     // Set shader effect to 3d model
    model.materials[0].maps[MAP_DIFFUSE].texture = texture; // Bind texture to model

    Vector3 position = {0.0, 0.0, 0.0}; // Set model position

    SetCameraMode(camera, CAMERA_FREE); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        let mut d = d.begin_mode3D(&camera);

        DrawModel(model, position, 0.2f, WHITE); // Draw 3d model with texture

        DrawGrid(10, 1.0); // Draw a grid

        EndMode3D();

        d.draw_text("(c) Watermill 3D model by Alberto Cano", screen_width - 210, screen_height - 20, 10, GRAY);

        DrawFPS(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadShader(shader);   // Unload shader
    UnloadTexture(texture); // Unload texture
    UnloadModel(model);     // Unload model

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}