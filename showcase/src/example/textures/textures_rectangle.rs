/*******************************************************************************************
*
*   raylib [textures] example - Texture loading and drawing a part defined by a rectangle
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_FRAME_SPEED: i32 = 15;
const MIN_FRAME_SPEED: i32 = 1;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [texture] example - texture rectangle");

    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)
    let scarfy = rl
        .load_texture(thread, "original/textures/resources/scarfy.png")
        .unwrap(); // Texture loading

    let position = rvec2(350.0, 280.0);
    let mut frame_rec = rrect(0.0, 0.0, scarfy.width() / 6, scarfy.height());
    let mut current_frame = 0;

    let mut frames_counter = 0;
    let mut frames_speed = 8; // Number of spritesheet frames shown by second

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        // Update
        //----------------------------------------------------------------------------------
        frames_counter += 1;

        if frames_counter >= (60 / frames_speed) {
            frames_counter = 0;
            current_frame += 1;

            if current_frame > 5 {
                current_frame = 0;
            }

            frame_rec.x = (current_frame * scarfy.width() / 6) as f32;
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT) {
            frames_speed += 1;
        } else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT) {
            frames_speed -= 1;
        }

        if frames_speed > MAX_FRAME_SPEED {
            frames_speed = MAX_FRAME_SPEED;
        } else if frames_speed < MIN_FRAME_SPEED {
            frames_speed = MIN_FRAME_SPEED;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_texture(&scarfy, 15, 40, Color::WHITE);
        d.draw_rectangle_lines(15, 40, scarfy.width, scarfy.height, Color::LIME);
        d.draw_rectangle_lines(
            15 + frame_rec.x as i32,
            40 + frame_rec.y as i32,
            frame_rec.width as i32,
            frame_rec.height as i32,
            Color::RED,
        );

        d.draw_text("FRAME SPEED: ", 165, 210, 10, Color::DARKGRAY);
        d.draw_text(
            &format!("{:.2} FPS", frames_speed),
            575,
            210,
            10,
            Color::DARKGRAY,
        );
        d.draw_text(
            "PRESS RIGHT/LEFT KEYS to CHANGE SPEED!",
            290,
            240,
            10,
            Color::DARKGRAY,
        );

        for i in 0..MAX_FRAME_SPEED {
            if i < frames_speed {
                d.draw_rectangle(250 + 21 * i, 205, 20, 20, Color::RED);
            }
            d.draw_rectangle_lines(250 + 21 * i, 205, 20, 20, Color::MAROON);
        }

        d.draw_texture_rec(&scarfy, frame_rec, position, Color::WHITE); // Draw part of the texture

        d.draw_text(
            "(c) Scarfy sprite by Eiden Marsal",
            screen_width - 200,
            screen_height - 20,
            10,
            Color::GRAY,
        );

        //----------------------------------------------------------------------------------
    });
}
