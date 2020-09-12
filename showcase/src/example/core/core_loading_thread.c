/*******************************************************************************************
*
*   raylib example - loading thread
*
*   NOTE: This example requires linking with pthreads library,
*   on MinGW, it can be accomplished passing -static parameter to compiler
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#include "pthread.h" // POSIX style threads management

#include <stdatomic.h> // C11 atomic data types

#include <time.h> // Required for: clock()

// Using C11 atomics for synchronization
// NOTE: A plain bool (or any plain data type for that matter) can't be used for inter-thread synchronization
static atomic_bool dataLoaded = ATOMIC_VAR_INIT(false); // Data Loaded completion indicator
static void *LoadDataThread(void *arg);                 // Loading data thread function declaration

static int dataProgress = 0; // Data progress accumulator

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
    rl.set_window_title(thread, "raylib [core] example - loading thread");


    pthread_t threadId; // Loading data thread id

    enum
    {
        STATE_WAITING,
        STATE_LOADING,
        STATE_FINISHED
    } state = STATE_WAITING;
    int framesCounter = 0;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        switch (state)
        {
        case STATE_WAITING:
        {
            if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_ENTER))
            {
                int error = pthread_create(&threadId, NULL, &LoadDataThread, NULL);
                if (error != 0)
                    TraceLog(LOG_ERROR, "Error creating loading thread");
                else
                    TraceLog(LOG_INFO, "Loading thread initialized successfully");

                state = STATE_LOADING;
            }
        }
        break;
        case STATE_LOADING:
        {
            framesCounter++;
            if (atomic_load(&dataLoaded))
            {
                framesCounter = 0;
                state = STATE_FINISHED;
            }
        }
        break;
        case STATE_FINISHED:
        {
            if (IsKeyPressed(raylib::consts::KeyboardKey::KEY_ENTER))
            {
                // Reset everything to launch again
                atomic_store(&dataLoaded, false);
                dataProgress = 0;
                state = STATE_WAITING;
            }
        }
        break;
        default:
            break;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        switch (state)
        {
        case STATE_WAITING:
            d.draw_text("PRESS ENTER to START LOADING DATA", 150, 170, 20, Color::DARKGRAY);
            break;
        case STATE_LOADING:
        {
            d.draw_rectangle(150, 200, dataProgress, 60, Color::SKYBLUE);
            if ((framesCounter / 15) % 2)
                d.draw_text("LOADING DATA...", 240, 210, 40, DARKColor::BLUE);
        }
        break;
        case STATE_FINISHED:
        {
            d.draw_rectangle(150, 200, 500, 60, Color::LIME);
            d.draw_text("DATA LOADED!", 250, 210, 40, Color::GREEN);
        }
        break;
        default:
            break;
        }

        d.draw_rectangle_lines(150, 200, 500, 60, Color::DARKGRAY);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}

// Loading data thread function definition
static void *LoadDataThread(void *arg)
{
    int timeCounter = 0;        // Time counted in ms
    clock_t prevTime = clock(); // Previous time

    // We simulate data loading with a time counter for 5 seconds
    while (timeCounter < 5000)
    {
        clock_t currentTime = clock() - prevTime;
        timeCounter = currentTime * 1000 / CLOCKS_PER_SEC;

        // We accumulate time over a global variable to be used in
        // main thread as a progress bar
        dataProgress = timeCounter / 10;
    }

    // When data has finished loading, we set global variable
    atomic_store(&dataLoaded, true);

    return NULL;
}
