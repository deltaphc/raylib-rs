/*******************************************************************************************
*
*   raylib [textures] example - Bunnymark
*
*   This example has been created using raylib 1.6 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2014-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

// 50K bunnies limit
const MAX_BUNNIES: usize = 50000;

// This is the maximum amount of elements (quads) per batch
// NOTE: This value is defined in [rlgl] module and can be changed there
const MAX_BATCH_ELEMENTS: usize = 8192;

#[derive(Default, Clone)]
struct Bunny {
    position: Vector2,
    speed: Vector2,
    color: Color,
}

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - bunnymark");

    // Load bunny texture
    let tex_bunny = rl
        .load_texture(thread, "original/textures/resources/wabbit_alpha.png")
        .expect("texture missing: are you in the right directory?");
    let mut bunnies = vec![Bunny::default(); MAX_BUNNIES];

    let mut bunnies_count = 0; // Bunnies counter

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        use raylib::consts::MouseButton::*;
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
            // Create more bunnies
            for _ in 0..100 {
                if bunnies_count < MAX_BUNNIES {
                    bunnies[bunnies_count].position = rl.get_mouse_position();
                    bunnies[bunnies_count].speed.x =
                        get_random_value::<i32>(-250, 250) as f32 / 60.0;
                    bunnies[bunnies_count].speed.y =
                        get_random_value::<i32>(-250, 250) as f32 / 60.0;
                    bunnies[bunnies_count].color = Color::new(
                        get_random_value::<i32>(50, 240) as u8,
                        get_random_value::<i32>(80, 240) as u8,
                        get_random_value::<i32>(100, 240) as u8,
                        255,
                    );
                    bunnies_count += 1;
                }
            }
        }

        // Update bunnies
        for i in 0..bunnies_count {
            bunnies[i].position.x += bunnies[i].speed.x;
            bunnies[i].position.y += bunnies[i].speed.y;

            if (bunnies[i].position.x + tex_bunny.width as f32 / 2.0) > rl.get_screen_width() as f32
                || (bunnies[i].position.x + tex_bunny.width as f32 / 2.0) < 0.0
            {
                bunnies[i].speed.x *= -1.0;
            }
            if bunnies[i].position.y + tex_bunny.height as f32 / 2.0 > rl.get_screen_height() as f32
                || (bunnies[i].position.y + tex_bunny.height as f32 / 2.0 - 40.0) < 0.0
            {
                bunnies[i].speed.y *= -1.0;
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        for i in 0..bunnies_count {
            // NOTE: When internal batch buffer limit is reached (MAX_BATCH_ELEMENTS),
            // a draw call is launched and buffer starts being filled again;
            // before issuing a draw call, updated vertex data from internal CPU buffer is send to GPU...
            // Process of sending data is costly and it could happen that GPU data has not been completely
            // processed for drawing while new data is tried to be sent (updating current in-use buffers)
            // it could generates a stall and consequently a frame drop, limiting the number of drawn bunnies
            d.draw_texture(
                &tex_bunny,
                bunnies[i].position.x as i32,
                bunnies[i].position.y as i32,
                bunnies[i].color,
            );
        }

        d.draw_rectangle(0, 0, screen_width, 40, Color::BLACK);
        d.draw_text(
            &format!("bunnies: {}", bunnies_count),
            120,
            10,
            20,
            Color::GREEN,
        );
        d.draw_text(
            &format!(
                "batched draw calls: {}",
                1 + bunnies_count / MAX_BATCH_ELEMENTS,
            ),
            320,
            10,
            20,
            Color::MAROON,
        );

        d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    });
}
