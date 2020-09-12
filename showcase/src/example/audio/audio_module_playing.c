/*******************************************************************************************
*
*   raylib [audio] example - Module playing (streaming)
*
*   This example has been created using raylib 1.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2016 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_CIRCLES 64

    typedef struct
{
    Vector2 position;
    float radius;
    float alpha;
    float speed;
    Color color;
} CircleWave;

pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    SetConfigFlags(FLAG_MSAA_4X_HINT); // NOTE: Try to enable MSAA 4X

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [audio] example - module playing (streaming)");


    InitAudioDevice(); // Initialize audio device

    Color colors[14] = {ORANGE,Color::RED, Color::GOLD, Color::LIME, Color::BLUE, VIOLET, BROWN, Color::LIGHTGRAY, PINK,
                        YELLOW, Color::GREEN, Color::SKYBLUE, PURPLE, BEIGE};

    // Creates ome circles for visual effect
    CircleWave circles[MAX_CIRCLES] = {0};

    for (int i = MAX_CIRCLES - 1; i >= 0; i--)
    {
        circles[i].alpha = 0.0;
        circles[i].radius = raylib::get_random_value(10, 40);
        circles[i].position.x = raylib::get_random_value(circles[i].radius, screen_width - circles[i].radius);
        circles[i].position.y = raylib::get_random_value(circles[i].radius, screen_height - circles[i].radius);
        circles[i].speed = (float)raylib::get_random_value(1, 100) / 2000.0;
        circles[i].color = colors[raylib::get_random_value(0, 13)];
    }

    Music music = LoadMusicStream("resources/mini1111.xm");

    PlayMusicStream(music);

    float timePlayed = 0.0;
    bool pause = false;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        UpdateMusicStream(music); // Update music buffer with new stream data

        // Restart music playing (stop and play)
        if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_SPACE))
        {
            StopMusicStream(music);
            PlayMusicStream(music);
        }

        // Pause/Resume music playing
        if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_P))
        {
            pause = !pause;

            if (pause)
                PauseMusicStream(music);
            else
                ResumeMusicStream(music);
        }

        // Get timePlayed scaled to bar dimensions
        timePlayed = GetMusicTimePlayed(music) / GetMusicTimeLength(music) * (screen_width - 40);

        // Color circles animation
        for (int i = MAX_CIRCLES - 1; (i >= 0) && !pause; i--)
        {
            circles[i].alpha += circles[i].speed;
            circles[i].radius += circles[i].speed * 10.0;

            if (circles[i].alpha > 1.0)
                circles[i].speed *= -1;

            if (circles[i].alpha <= 0.0)
            {
                circles[i].alpha = 0.0;
                circles[i].radius = raylib::get_random_value(10, 40);
                circles[i].position.x = raylib::get_random_value(circles[i].radius, screen_width - circles[i].radius);
                circles[i].position.y = raylib::get_random_value(circles[i].radius, screen_height - circles[i].radius);
                circles[i].color = colors[raylib::get_random_value(0, 13)];
                circles[i].speed = (float)raylib::get_random_value(1, 100) / 2000.0;
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        for (int i = MAX_CIRCLES - 1; i >= 0; i--)
        {
            DrawCircleV(circles[i].position, circles[i].radius, Fade(circles[i].color, circles[i].alpha));
        }

        // Draw time bar
        d.draw_rectangle(20, screen_height - 20 - 12, screen_width - 40, 12, Color::LIGHTGRAY);
        d.draw_rectangle(20, screen_height - 20 - 12, (int)timePlayed, 12, Color::MAROON);
        d.draw_rectangle_lines(20, screen_height - 20 - 12, screen_width - 40, 12, Color::GRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadMusicStream(music); // Unload music stream buffers from RAM

    CloseAudioDevice(); // Close audio device (music streaming is automatically stopped)

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}