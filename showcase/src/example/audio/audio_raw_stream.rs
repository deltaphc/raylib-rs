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

const MAX_SAMPLES: usize = 512;
const MAX_SAMPLES_PER_UPDATE: usize = 4096;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [audio] example - raw audio streaming");

    let mut audio = RaylibAudio::init_audio_device(); // Initialize audio device

    // Init raw audio stream (sample rate: 22050, sample size: 16bit-short, channels: 1-mono)
    let mut stream = AudioStream::init_audio_stream(thread, 22050, 16, 1);

    // Buffer for the single cycle waveform we are synthesizing
    let mut data = [0i16; MAX_SAMPLES / std::mem::size_of::<i16>()];

    // Frame buffer, describing the waveform when repeated over the course of a frame
    let mut writeBuf = [0i16; MAX_SAMPLES_PER_UPDATE / std::mem::size_of::<i16>()];

    audio.play_audio_stream(&mut stream); // Start processing stream buffer (no data loaded currently)

    // Cycles per second (hz)
    let mut frequency = 440.0;

    // Previous value, used to test if sine needs to be rewritten, and to smoothly modulate frequency
    let mut oldFrequency = 1.0;

    // Cursor to read and copy the samples of the sine wave buffer
    let mut readCursor = 0;

    // Computed size in samples of the sine wave
    let mut waveLength = 1;

    let mut position = rvec2(0, 0);

    rl.set_target_fps(30); // Set our game to run at 30 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------

        // Sample mouse input.
        let mouse_position = rl.get_mouse_position();

        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            let fp = mouse_position.y;
            frequency = 40.0 + fp;
        }

        // Rewrite the sine wave.
        // Compute two cycles to allow the buffer padding, simplifying any modulation, resampling, etc.
        if frequency != oldFrequency
        {
            // Compute wavelength. Limit size in both directions.
            let oldWavelength = waveLength;
            waveLength = 22050 / frequency as usize;
            if waveLength > MAX_SAMPLES / 2
             {

                 waveLength = MAX_SAMPLES / 2;
             }
            if waveLength < 1
                {waveLength = 1;
                }
            // Write sine wave.
            for i in 0..waveLength*2
            {
                data[i] = (((2.0 * std::f32::consts::PI * i as f32 / waveLength as f32)).sin() * 32000.0) as i16;
            }

            // Scale read cursor's position to minimize transition artifacts
            readCursor = readCursor * (waveLength / oldWavelength);
            oldFrequency = frequency;
        }

        // Refill audio stream if required
        if audio.is_audio_stream_processed(&stream)
        {
            // Synthesize a buffer that is exactly the requested size
            let mut  writeCursor = 0;

            while writeCursor < MAX_SAMPLES_PER_UPDATE  / std::mem::size_of::<i16>()
            {
                // Start by trying to write the whole chunk at once
                let mut writeLength = MAX_SAMPLES_PER_UPDATE  / std::mem::size_of::<i16>() - writeCursor;

                // Limit to the maximum readable size
                let readLength = waveLength - readCursor;

                if writeLength > readLength
                {
                    writeLength = readLength;

                }

                // Write the slice
                let _ = &mut writeBuf[writeCursor..writeCursor+writeLength].copy_from_slice(&data[readCursor..readCursor+writeLength]);
                // memcpy(writeBuf + writeCursor, data + readCursor, writeLength * sizeof(short));

                // Update cursors and loop audio
                readCursor = (readCursor + writeLength) % waveLength;

                writeCursor += writeLength;
            }

            // Copy finished frame to audio stream
            stream.update_audio_stream( &writeBuf);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text(&format!("sine frequency: {}", frequency), d.get_screen_width() - 220, 10, 20,Color::RED);
        d.draw_text("click mouse button to change frequency", 10, 10, 20, Color::DARKGRAY);

        // Draw the current buffer state proportionate to the screen
        for i in 0..screen_width
        {
            position.x = i as f32;
            position.y =( 250 + 50 * data[i as usize * MAX_SAMPLES / std::mem::size_of::<i16>() / screen_width as usize] as i32 / 32000) as f32;

            d.draw_pixel_v(position,Color::RED);
        }

        //----------------------------------------------------------------------------------
    },
    );
}
