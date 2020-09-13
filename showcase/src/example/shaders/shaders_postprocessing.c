/*******************************************************************************************
*
*   raylib [shaders] example - Apply a postprocessing shader to a scene
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
const GLSL_VERSION 330
#else // PLATFORM_RPI, PLATFORM_ANDROID, PLATFORM_WEB
const GLSL_VERSION 100
#endif

    const MAX_POSTPRO_SHADERS 12

    typedef enum {
        FX_GRAYSCALE = 0,
        FX_POSTERIZATION,
        FX_DREAM_VISION,
        FX_PIXELIZER,
        FX_CROSS_HATCHING,
        FX_CROSS_STITCHING,
        FX_PREDATOR_VIEW,
        FX_SCANLINES,
        FX_FISHEYE,
        FX_SOBEL,
        FX_BLOOM,
        FX_BLUR,
        //FX_FXAA
    } PostproShader;

static const char *postproShaderText[] = {
    "GRAYSCALE",
    "POSTERIZATION",
    "DREAM_VISION",
    "PIXELIZER",
    "CROSS_HATCHING",
    "CROSS_STITCHING",
    "PREDATOR_VIEW",
    "SCANLINES",
    "FISHEYE",
    "SOBEL",
    "BLOOM",
    "BLUR",
    //"FXAA"
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

    SetConfigFlags(FLAG_MSAA_4X_HINT); // Enable Multi Sampling Anti Aliasing 4x (if available)

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - postprocessing shader");


    // Define the camera to look into our 3d world
    Camera camera = {{2.0, 3.0, 2.0}, {0.0, 1.0, 0.0}, {0.0, 1.0, 0.0}, 45.0, 0};

    Model model = LoadModel("resources/models/church.obj");                 // Load OBJ model
    let texture = rl.load_texture(thread, "resources/models/church_diffuse.png"); // Load model texture (diffuse map)
    model.materials[0].maps[MAP_DIFFUSE].texture = texture;                 // Set model diffuse texture

    let position = Vector3::zero(); // Set model position

    // Load all postpro shaders
    // NOTE 1: All postpro shader use the base vertex shader (DEFAULT_VERTEX_SHADER)
    // NOTE 2: We load the correct shader depending on GLSL version
    Shader shaders[MAX_POSTPRO_SHADERS] = {0};

    // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
    shaders[FX_GRAYSCALE] = LoadShader(0, &format!("resources/shaders/glsl{}/grayscale.fs", GLSL_VERSION));
    shaders[FX_POSTERIZATION] = LoadShader(0, &format!("resources/shaders/glsl{}/posterization.fs", GLSL_VERSION));
    shaders[FX_DREAM_VISION] = LoadShader(0, &format!("resources/shaders/glsl{}/dream_vision.fs", GLSL_VERSION));
    shaders[FX_PIXELIZER] = LoadShader(0, &format!("resources/shaders/glsl{}/pixelizer.fs", GLSL_VERSION));
    shaders[FX_CROSS_HATCHING] = LoadShader(0, &format!("resources/shaders/glsl{}/cross_hatching.fs", GLSL_VERSION));
    shaders[FX_CROSS_STITCHING] = LoadShader(0, &format!("resources/shaders/glsl{}/cross_stitching.fs", GLSL_VERSION));
    shaders[FX_PREDATOR_VIEW] = LoadShader(0, &format!("resources/shaders/glsl{}/predator.fs", GLSL_VERSION));
    shaders[FX_SCANLINES] = LoadShader(0, &format!("resources/shaders/glsl{}/scanlines.fs", GLSL_VERSION));
    shaders[FX_FISHEYE] = LoadShader(0, &format!("resources/shaders/glsl{}/fisheye.fs", GLSL_VERSION));
    shaders[FX_SOBEL] = LoadShader(0, &format!("resources/shaders/glsl{}/sobel.fs", GLSL_VERSION));
    shaders[FX_BLOOM] = LoadShader(0, &format!("resources/shaders/glsl{}/bloom.fs", GLSL_VERSION));
    shaders[FX_BLUR] = LoadShader(0, &format!("resources/shaders/glsl{}/blur.fs", GLSL_VERSION));

    int currentShader = FX_GRAYSCALE;

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
        rl.update_camera(&mut camera); // Update camera

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT)
            currentShader++;
        else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT)
            currentShader--;

        if currentShader >= MAX_POSTPRO_SHADERS
            currentShader = 0;
        else if currentShader < 0
            currentShader = MAX_POSTPRO_SHADERS - 1;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        let mut d = d.begin_texture_mode(thread, &target); // Enable drawing to texture

        d.clear_background(Color::RAYWHITE); // Clear texture background

        let mut d = d.begin_mode3D(&camera); // Begin 3d mode drawing

        DrawModel(model, position, 0.1, Color::WHITE); // Draw 3d model with texture

        d.draw_grid(10, 1.0); // Draw a grid

        EndMode3D(); // End 3d mode drawing, returns to orthographic 2d mode

        EndTextureMode(); // End drawing to texture (now we have a texture available for next passes)

        // Render previously generated texture using selected postpro shader
        BeginShaderMode(shaders[currentShader]);

        // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        DrawTextureRec(target.texture, rrect(0, 0, target.texture.width, -target.texture.height), rvec2(0,  0), Color::WHITE);

        EndShaderMode();

        // Draw 2d shapes and text over drawn texture
        d.draw_rectangle(0, 9, 580, 30, Fade(Color::LIGHTGRAY, 0.7f));

        d.draw_text("(c) Church 3D model by Alberto Cano", screen_width - 200, screen_height - 20, 10, Color::GRAY);

        d.draw_text("CURRENT POSTPRO SHADER:", 10, 15, 20, Color::BLACK);
        d.draw_text(postproShaderText[currentShader], 330, 15, 20,Color::RED);
        d.draw_text("< >", 540, 10, 30, Color::DARKBLUE);

        d.draw_fps(700, 15);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------

    // Unload all postpro shaders
    for (int i = 0; i < MAX_POSTPRO_SHADERS; i++)
        UnloadShader(shaders[i]);

    UnloadTexture(texture);      // Unload texture
    UnloadModel(model);          // Unload model
    UnloadRenderTexture(target); // Unload render texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
