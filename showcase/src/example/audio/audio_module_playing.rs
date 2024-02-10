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

const MAX_CIRCLES: usize = 64;

#[derive(Default, Copy, Clone)]
struct CircleWave
{
    position: Vector2,
     radius: f32,
     alpha: f32,
     speed: f32,
    color: Color,
}

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
    rl.set_window_title(thread, "raylib [audio] example - module playing (streaming)");


    let mut audio = RaylibAudio::init_audio_device(); // Initialize audio device

    let colors = [Color::ORANGE,Color::RED, Color::GOLD, Color::LIME, Color::BLUE, Color::VIOLET, Color::BROWN, Color::LIGHTGRAY, Color::PINK,
                        Color::YELLOW, Color::GREEN, Color::SKYBLUE, Color::PURPLE, Color::BEIGE];

    // Creates ome circles for visual effect
    let mut circles = [CircleWave::default(); MAX_CIRCLES];

    for i in 0..MAX_CIRCLES 
    {
        circles[i].alpha = 0.0;
        circles[i].radius = raylib::get_random_value::<i32>(10, 40) as f32 ;
        circles[i].position.x = raylib::get_random_value::<i32>(circles[i].radius  as i32, screen_width - circles[i].radius as i32) as f32 ;
        circles[i].position.y = raylib::get_random_value::<i32>(circles[i].radius as i32, screen_height - circles[i].radius as i32) as f32 ;
        circles[i].speed = raylib::get_random_value::<i32>(1, 100) as f32 / 2000.0;
        circles[i].color = colors[raylib::get_random_value::<i32>(0, 13) as usize];
    }

    let mut music = Music::load_music_stream(thread, "original/audio/resources/mini1111.xm").unwrap();

    audio.play_music_stream(&mut music);

    let mut pause = false;

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        audio.update_music_stream(&mut music); // Update music buffer with new stream data

        // Restart music playing (stop and play)
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
        {
            audio.stop_music_stream(&mut music);
            audio.play_music_stream(&mut music);
        }

        // Pause/Resume music playing
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_P)
        {
            pause = !pause;

            if pause
                {

                    audio.pause_music_stream(&mut music);
                }
            else
                {

                    audio.resume_music_stream(&mut music);
                }
        }

        // Get timePlayed scaled to bar dimensions
        let time_played = audio.get_music_time_played(&music) / audio.get_music_time_length(&music) * (screen_width - 40) as f32;

        // Color circles animation
        for i in 0..MAX_CIRCLES 
        {
            circles[i].alpha += circles[i].speed;
            circles[i].radius += circles[i].speed * 10.0;

            if circles[i].alpha > 1.0
                {
                    circles[i].speed *= -1.0;

                }

            if circles[i].alpha <= 0.0
            {
                circles[i].alpha = 0.0;
                circles[i].radius = raylib::get_random_value::<i32>(10, 40) as f32;
                circles[i].position.x = raylib::get_random_value::<i32>(circles[i].radius as i32, screen_width - circles[i].radius as i32) as f32;
                circles[i].position.y = raylib::get_random_value::<i32>(circles[i].radius as i32, screen_height - circles[i].radius as i32) as f32;
                circles[i].color = colors[raylib::get_random_value::<i32>(0, 13) as usize];
                circles[i].speed = raylib::get_random_value::<i32>(1, 100) as f32 / 2000.0;
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        for i in 0..MAX_CIRCLES
        {
            d.draw_circle_v(circles[i].position, circles[i].radius, circles[i].color.fade( circles[i].alpha));
        }

        // Draw time bar
        d.draw_rectangle(20, screen_height - 20 - 12, screen_width - 40, 12, Color::LIGHTGRAY);
        d.draw_rectangle(20, screen_height - 20 - 12, time_played as i32, 12, Color::MAROON);
        d.draw_rectangle_lines(20, screen_height - 20 - 12, screen_width - 40, 12, Color::GRAY);

        //----------------------------------------------------------------------------------
    });
}