// Smooth Pixel-Perfect Camera
//
// Example originally created with raylib 3.7, last time updated with raylib 4.0
// and ported to raylib-rs 5.0
//
// Example contributed by Giancamillo Alessandroni (@NotManyIdeasDev),
// reviewed by Ramon Santamaria (@raysan5) and
// ported by Lev Abashichev (@levilovie)
//
// Example licensed under an unmodified zlib/libpng license, which is an OSI-certified,
// BSD-like license that allows static linking with closed source software
// Copyright (c) 2021-2024 Giancamillo Alessandroni (@NotManyIdeasDev),
// Ramon Santamaria (@raysan5) and Lev Abashichev (@levilovie)

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

const VIRTUAL_SCREEN_WIDTH: i32 = 160;
const VIRTUAL_SCREEN_HEIGHT: i32 = 90;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib [core] example - smooth pixel-perfect camera")
        .build();

    // Game world camera
    let mut world_space_camera = Camera2D {
        zoom: 1.0,
        ..Default::default()
    };
    // Screen camera
    let screen_space_camera = Camera2D {
        zoom: 1.0,
        ..Default::default()
    };

    // This is where we'll draw all our objects.
    let mut target = rl
        .load_render_texture(
            &thread,
            VIRTUAL_SCREEN_WIDTH as u32,
            VIRTUAL_SCREEN_HEIGHT as u32,
        )
        .expect("Failed to create render texture");

    let rec01 = Rectangle::new(70.0, 35.0, 20.0, 20.0);
    let rec02 = Rectangle::new(90.0, 55.0, 30.0, 10.0);
    let rec03 = Rectangle::new(80.0, 65.0, 15.0, 25.0);

    // The target's height is flipped (in the source Rectangle), due to OpenGL reasons.
    let source_rec = Rectangle::new(
        0.0,
        0.0,
        target.texture.width as f32,
        -target.texture.height as f32,
    );
    let dest_rec = Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let origin = Vector2::new(0.0, 0.0);

    let mut rotation = 0.0;

    rl.set_target_fps(60);

    // Main game loop
    // Detect window close button or ESC key
    while !rl.window_should_close() {
        // Update
        // Rotate the rectangles, 60 degrees per second
        rotation += 60.0 * rl.get_frame_time();

        // Make the camera move to demonstrate the effect
        world_space_camera.target = Vector2::new(
            (f64::sin(rl.get_time()) as f32 * 50.0) - 10.0,
            f64::cos(rl.get_time()) as f32 * 30.0,
        );

        // Draw
        let mut d = rl.begin_drawing(&thread);
        {
            let mut d = d.begin_texture_mode(&thread, &mut target);
            d.clear_background(Color::RAYWHITE);

            let mut d = d.begin_mode2D(world_space_camera);
            d.draw_rectangle_pro(rec01, origin, rotation, Color::BLACK);
            d.draw_rectangle_pro(rec02, origin, rotation + -70.0, Color::RED);
            d.draw_rectangle_pro(rec03, origin, rotation + 30.0, Color::BLUE);
        }

        d.clear_background(Color::RED);

        {
            let mut d = d.begin_mode2D(screen_space_camera);
            d.draw_texture_pro(
                &target.texture(),
                source_rec,
                dest_rec,
                origin,
                0.0,
                Color::WHITE,
            );
        }

        d.draw_text(
            format!("Screen resolution: {}x{}", SCREEN_WIDTH, SCREEN_HEIGHT).as_str(),
            10,
            10,
            20,
            Color::DARKBLUE,
        );
        d.draw_text(
            format!(
                "World resolution: {}x{}",
                VIRTUAL_SCREEN_WIDTH, VIRTUAL_SCREEN_HEIGHT
            )
            .as_str(),
            10,
            40,
            20,
            Color::DARKGREEN,
        );
        d.draw_fps(SCREEN_WIDTH - 95, 10);
    }
}
