/*******************************************************************************************
*
*   raylib [audio] example - Music playing (streaming)
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [audio] example - music playing (streaming)");
    rl.set_window_size(screen_width, screen_height);

    let mut audio = RaylibAudio::init_audio_device();

    let mut music =
        Music::load_music_stream(thread, "original/audio/resources/guitar_noodling.ogg").unwrap();

    audio.play_music_stream(&mut music);

    let mut time_played = 0.0;
    let mut pause = false;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        use crate::consts::KeyboardKey::*;
        // Update
        //----------------------------------------------------------------------------------
        audio.update_music_stream(&mut music); // Update music buffer with new stream data

        // Restart music playing (stop and play)
        if rl.is_key_pressed(KEY_SPACE) {
            audio.stop_music_stream(&mut music);
            audio.play_music_stream(&mut music);
        }

        // Pause/Resume music playing
        if rl.is_key_pressed(KEY_P) {
            pause = !pause;

            if pause {
                audio.pause_music_stream(&mut music);
            } else {
                audio.resume_music_stream(&mut music);
            }
        }

        // Get time_played scaled to bar dimensions (400 pixels)
        time_played =
            audio.get_music_time_played(&music) / audio.get_music_time_length(&music) * 400.0;

        if time_played > 400.0 {
            audio.stop_music_stream(&mut music);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text("MUSIC SHOULD BE PLAYING!", 255, 150, 20, Color::LIGHTGRAY);

        d.draw_rectangle(200, 200, 400, 12, Color::LIGHTGRAY);
        d.draw_rectangle(200, 200, time_played as i32, 12, Color::MAROON);
        d.draw_rectangle_lines(200, 200, 400, 12, Color::GRAY);

        d.draw_text(
            "PRESS SPACE TO RESTART MUSIC",
            215,
            250,
            20,
            Color::LIGHTGRAY,
        );
        d.draw_text(
            "PRESS P TO PAUSE/RESUME MUSIC",
            208,
            280,
            20,
            Color::LIGHTGRAY,
        );

        //----------------------------------------------------------------------------------
    });
}
