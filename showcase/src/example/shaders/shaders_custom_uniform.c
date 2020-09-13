/*******************************************************************************************
*
*   raylib [shaders] example - Apply a postprocessing shader and connect a custom uniform variable
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
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#if defined(PLATFORM_DESKTOP)
#defconstL_VERSION 330
#else // PLATFORM_RPI, PLATFORM_ANDROID, PLATFORM_WEB
#defconstL_VERSION 100
#endif

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    SetConfigFlags(FLAG_MSAA_4X_HINT); // Enable Multi Sampling Anti Aliasing 4x (if available)

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - custom uniform variable");


    // Define the camera to look into our 3d world
    let camera = Camera3D::perspective(
    rvec3(8.0, 8.0, 8.0),
    rvec3(0.0, 1.5, 0.0),
    rvec3(0.0, 1.0, 0.0),
    45.0,
    );

    Model model = LoadModel("resources/models/barracks.obj");                 // Load OBJ model
    let texture = rl.load_texture(thread, "resources/models/barracks_diffuse.png"); // Load model texture (diffuse map)
    model.materials[0].maps[MAP_DIFFUSE].texture = texture;                   // Set model diffuse texture

    let position = Vector3::zero(); // Set model position

    // Load postprocessing shader
    // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
    Shader shader = LoadShader(0, &format!("resources/shaders/glsl{}/swirl.fs", GLSL_VERSION));

    // Get variable (uniform) location on the shader to connect with the program
    // NOTE: If uniform variable could not be found in the shader, function returns -1
    int swirlCenterLoc = GetShaderLocation(shader, "center");

    float swirlCenter[2] = {(float)screen_width / 2, (float)screen_height / 2};

    // Create a RenderTexture2D to be used for render to texture
    RenderTexture2D target = LoadRenderTexture(screen_width, screen_height);

    // Setup orbital camera
    SetCameraMode(camera, CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        Vector2 mousePosition = rl.get_mouse_position();

        swirlCenter[0] = mousePosition.x;
        swirlCenter[1] = screen_height - mousePosition.y;

        // Send new value to the shader to be used on drawing
        SetShaderValue(shader, swirlCenterLoc, swirlCenter, UNIFORM_VEC2);

        rl.update_camera(&mut camera); // Update camera
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        let mut d = d.begin_texture_mode(thread, &target); // Enable drawing to texture

        d.clear_background(Color::RAYWHITE); // Clear texture background

        let mut d = d.begin_mode3D(&camera); // Begin 3d mode drawing

        DrawModel(model, position, 0.5, Color::WHITE); // Draw 3d model with texture

        d.draw_grid(10, 1.0); // Draw a grid

        EndMode3D(); // End 3d mode drawing, returns to orthographic 2d mode

        d.draw_text("TEXT DRAWN IN RENDER TEXTURE", 200, 10, 30,Color::RED);

        EndTextureMode(); // End drawing to texture (now we have a texture available for next passes)

        BeginShaderMode(shader);

        // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        DrawTextureRec(target.texture, rrect(0, 0, target.texture.width, -target.texture.height), rvec2(0,  0), Color::WHITE);

        EndShaderMode();

        // Draw some 2d text over drawn texture
        d.draw_text("(c) Barracks 3D model by Alberto Cano", screen_width - 220, screen_height - 20, 10, Color::GRAY);

        d.draw_fps(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadShader(shader);        // Unload shader
    UnloadTexture(texture);      // Unload texture
    UnloadModel(model);          // Unload model
    UnloadRenderTexture(target); // Unload render texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}