/*******************************************************************************************
*
*   raylib [audio] example - Sound loading and playing
*
*   This example has been created using raylib 1.0 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [audio] example - sound loading and playing");

    let mut audio = RaylibAudio::init_audio_device(); // Initialize audio device

    let mut fxWav = Sound::load_sound("original/audio/resources/sound.wav").unwrap(); // Load WAV audio file
    let mut fxOgg = Sound::load_sound("original/audio/resources/tanatana.ogg").unwrap(); // Load OGG audio file

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
            {
                audio.play_sound(&mut fxWav); // Play WAV sound
            }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER)
            {
                audio.play_sound(&mut fxOgg); // Play OGG soundd
            }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("Press SPACE to PLAY the WAV sound!", 200, 180, 20, Color::LIGHTGRAY);
        d.draw_text("Press ENTER to PLAY the OGG sound!", 200, 220, 20, Color::LIGHTGRAY);

        //----------------------------------------------------------------------------------
    },
    );
}
