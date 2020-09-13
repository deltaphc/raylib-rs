/*******************************************************************************************
*
*   raylib [audio] example - Raw audio streaming
*
*   This example has been created using raylib 1.6 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example created by Ramon Santamaria (@raysan5) and reviewed by James Hofmann (@triplefox)
*
*   Copyright (c) 2015-2019 Ramon Santamaria (@raysan5) and James Hofmann (@triplefox)
*
********************************************************************************************/

use raylib::prelude::*;

#include <stdlib.h> // Required for: malloc(), free()
#include <math.h>   // Required for: sinf()
#include <string.h> // Required for: memcpy()

const MAX_SAMPLES 512 const MAX_SAMPLES_PER_UPDATE 4096

    int
    main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [audio] example - raw audio streaming");


    InitAudioDevice(); // Initialize audio device

    // Init raw audio stream (sample rate: 22050, sample size: 16bit-short, channels: 1-mono)
    AudioStream stream = InitAudioStream(22050, 16, 1);

    // Buffer for the single cycle waveform we are synthesizing
    short *data = (short *)malloc(sizeof(short) * MAX_SAMPLES);

    // Frame buffer, describing the waveform when repeated over the course of a frame
    short *writeBuf = (short *)malloc(sizeof(short) * MAX_SAMPLES_PER_UPDATE);

    PlayAudioStream(stream); // Start processing stream buffer (no data loaded currently)

    // Position read in to determine next frequency
    let mousePosition = rvec2(-100.0, -100.0);

    // Cycles per second (hz)
    float frequency = 440.0;

    // Previous value, used to test if sine needs to be rewritten, and to smoothly modulate frequency
    float oldFrequency = 1.0;

    // Cursor to read and copy the samples of the sine wave buffer
    int readCursor = 0;

    // Computed size in samples of the sine wave
    int waveLength = 1;

    let position = rvec2(0, 0);

    SetTargetFPS(30); // Set our game to run at 30 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------

        // Sample mouse input.
        mousePosition = rl.get_mouse_position();

        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            float fp = (float)(mousePosition.y);
            frequency = 40.0 + (float)(fp);
        }

        // Rewrite the sine wave.
        // Compute two cycles to allow the buffer padding, simplifying any modulation, resampling, etc.
        if frequency != oldFrequency
        {
            // Compute wavelength. Limit size in both directions.
            int oldWavelength = waveLength;
            waveLength = (int)(22050 / frequency);
            if waveLength > MAX_SAMPLES / 2
                waveLength = MAX_SAMPLES / 2;
            if waveLength < 1
                waveLength = 1;

            // Write sine wave.
            for (int i = 0; i < waveLength * 2; i++)
            {
                data[i] = (short)(sinf(((2 * PI * (float)i / waveLength))) * 32000);
            }

            // Scale read cursor's position to minimize transition artifacts
            readCursor = (int)(readCursor * ((float)waveLength / (float)oldWavelength));
            oldFrequency = frequency;
        }

        // Refill audio stream if required
        if IsAudioStreamProcessed(stream)
        {
            // Synthesize a buffer that is exactly the requested size
            int writeCursor = 0;

            while (writeCursor < MAX_SAMPLES_PER_UPDATE)
            {
                // Start by trying to write the whole chunk at once
                int writeLength = MAX_SAMPLES_PER_UPDATE - writeCursor;

                // Limit to the maximum readable size
                int readLength = waveLength - readCursor;

                if writeLength > readLength
                    writeLength = readLength;

                // Write the slice
                memcpy(writeBuf + writeCursor, data + readCursor, writeLength * sizeof(short));

                // Update cursors and loop audio
                readCursor = (readCursor + writeLength) % waveLength;

                writeCursor += writeLength;
            }

            // Copy finished frame to audio stream
            UpdateAudioStream(stream, writeBuf, MAX_SAMPLES_PER_UPDATE);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text(&format!("sine frequency: {}", (int)frequency), rl.get_screen_width() - 220, 10, 20,Color::RED);
        d.draw_text("click mouse button to change frequency", 10, 10, 20, Color::DARKGRAY);

        // Draw the current buffer state proportionate to the screen
        for (int i = 0; i < screen_width; i++)
        {
            position.x = i;
            position.y = 250 + 50 * data[i * MAX_SAMPLES / screen_width] / 32000;

            DrawPixelV(position,Color::RED);
        }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    free(data);     // Unload sine wave data
    free(writeBuf); // Unload write buffer

    CloseAudioStream(stream); // Close raw audio stream and delete buffers from RAM
    CloseAudioDevice();       // Close audio device (music streaming is automatically stopped)

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}
