/*******************************************************************************************
*
*   raylib [shaders] example - Simple shader mask
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Camacho (@codifies) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Chris Camacho (@codifies) and Ramon Santamaria (@raysan5)
*
********************************************************************************************
*
*   After a model is loaded it has a default material, this material can be
*   modified in place rather than creating one from scratch...
*   While all of the maps have particular names, they can be used for any purpose
*   except for three maps that are applied as cubic maps (see below)
*
********************************************************************************************/

use raylib::prelude::*;
#include "raymath.h"

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

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib - simple shader mask");


    // Define the camera to look into our 3d world
    Camera camera = {0};
    camera.position = rvec3(0.0, 1.0, 2.0);
    camera.target = rvec3(0.0, 0.0, 0.0);
    camera.up = rvec3(0.0, 1.0, 0.0);
    camera.fovy = 45.0;
    camera.type = CAMERA_PERSPECTIVE;

    // Define our three models to show the shader on
    Mesh torus = GenMeshTorus(.3, 1, 16, 32);
    Model model1 = LoadModelFromMesh(torus);

    Mesh cube = GenMeshCube(.8, .8, .8);
    Model model2 = LoadModelFromMesh(cube);

    // Generate model to be shaded just to see the gaps in the other two
    Mesh sphere = GenMeshSphere(1, 16, 16);
    Model model3 = LoadModelFromMesh(sphere);

    // Load the shader
    Shader shader = LoadShader(0, FormatText("resources/shaders/glsl%i/mask.fs", GLSL_VERSION));

    // Load and apply the diffuse texture (colour map)
    Texture texDiffuse = LoadTexture("resources/plasma.png");
    model1.materials[0].maps[MAP_DIFFUSE].texture = texDiffuse;
    model2.materials[0].maps[MAP_DIFFUSE].texture = texDiffuse;

    // Using MAP_EMISSION as a spare slot to use for 2nd texture
    // NOTE: Don't use MAP_IRRADIANCE, MAP_PREFILTER or  MAP_CUBEMAP
    // as they are bound as cube maps
    Texture texMask = LoadTexture("resources/mask.png");
    model1.materials[0].maps[MAP_EMISSION].texture = texMask;
    model2.materials[0].maps[MAP_EMISSION].texture = texMask;
    shader.locs[LOC_MAP_EMISSION] = GetShaderLocation(shader, "mask");

    // Frame is incremented each frame to animate the shader
    int shaderFrame = GetShaderLocation(shader, "frame");

    // Apply the shader to the two models
    model1.materials[0].shader = shader;
    model2.materials[0].shader = shader;

    int framesCounter = 0;
    Vector3 rotation = {0}; // Model rotation angles

    rl.set_target_fps(60); // Set  to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        framesCounter++;
        rotation.x += 0.01f;
        rotation.y += 0.005f;
        rotation.z -= 0.0025f;

        // Send frames counter to shader for animation
        SetShaderValue(shader, shaderFrame, &framesCounter, UNIFORM_INT);

        // Rotate one of the models
        model1.transform = MatrixRotateXYZ(rotation);

        rl.update_camera(&mut camera);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::DARKColor::BLUE);

        let mut d = d.begin_mode3D(&camera);

        DrawModel(model1, (Vector3){0.5, 0, 0}, 1, WHITE);
        DrawModelEx(model2, (Vector3){-.5, 0, 0}, (Vector3){1, 1, 0}, 50, (Vector3){1, 1, 1}, WHITE);
        DrawModel(model3, (Vector3){0, 0, -1.5}, 1, WHITE);
        DrawGrid(10, 1.0); // Draw a grid

        EndMode3D();

        d.draw_rectangle(16, 698, MeasureText(FormatText("Frame: %i", framesCounter), 20) + 8, 42, Color::BLUE);
        d.draw_text(FormatText("Frame: %i", framesCounter), 20, 700, 20, WHITE);

        DrawFPS(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadModel(model1);
    UnloadModel(model2);
    UnloadModel(model3);

    UnloadTexture(texDiffuse); // Unload default diffuse texture
    UnloadTexture(texMask);    // Unload texture mask

    UnloadShader(shader); // Unload shader

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
