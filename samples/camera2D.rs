#![allow(non_snake_case)]
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const MAX_BUILDINGS: usize = 100;

fn main() {
    use raylib::consts::KeyboardKey::*;
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Camera 2D");
    let (w, h) = (opt.width, opt.height);

    let mut player = Rectangle::new(400.0, 280.0, 40.0, 40.0);
    let mut buildings = Vec::with_capacity(MAX_BUILDINGS);
    let mut build_colors = Vec::with_capacity(MAX_BUILDINGS);
    let mut spacing = 0.0;

    for i in 0..MAX_BUILDINGS {
        let bh: i32 = rl.get_random_value(100..800);
        buildings.push(Rectangle::new(
            -6000.0 + spacing,
            (h - 130 - bh) as f32,
            rl.get_random_value::<i32>(50..200) as f32,
            bh as f32,
        ));

        spacing += buildings[i].width;
        build_colors.push(Color::new(
            rl.get_random_value::<i32>(200..240) as u8,
            rl.get_random_value::<i32>(200..240) as u8,
            rl.get_random_value::<i32>(200..240) as u8,
            255,
        ));
    }

    let mut camera = Camera2D {
        target: Vector2::new(player.x + 20.0, player.y + 20.0),
        offset: Vector2::new(player.x, player.y),
        rotation: 0.0,
        zoom: 1.0,
    };

    while !rl.window_should_close() {
        if rl.is_key_down(KEY_RIGHT) {
            player.x += 2.0;
        } else if rl.is_key_down(KEY_LEFT) {
            player.x -= 2.0;
        }

        // Camera follows player
        camera.target = Vector2::new(player.x + 20.0, player.y + 20.0);

        // Camera rotation controls
        if rl.is_key_down(KEY_A) {
            camera.rotation -= 1.0;
        } else if rl.is_key_down(KEY_S) {
            camera.rotation += 1.0;
        }

        // Limit camera rotation to 80 degrees
        camera.rotation = camera.rotation.max(-40.0).min(40.0);

        // zoom controls
        camera.zoom += rl.get_mouse_wheel_move() * 0.05;
        camera.zoom = camera.zoom.max(0.1).min(3.0);

        if rl.is_key_pressed(KEY_R) {
            camera.zoom = 1.0;
            camera.rotation = 0.0;
        }

        rl.start_drawing(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
            d.start_mode2D(camera, |mut d2, _camera| {
                d2.draw_rectangle(-6000, 320, 13000, 8000, Color::DARKGRAY);

                for i in 0..MAX_BUILDINGS {
                    d2.draw_rectangle_rec(buildings[i], build_colors[i]);
                }
                d2.draw_rectangle_rec(player, Color::RED);

                d2.draw_line(
                    camera.target.x as i32,
                    -h * 10,
                    camera.target.x as i32,
                    h * 10,
                    Color::GREEN,
                );
                d2.draw_line(
                    -w * 10,
                    camera.target.y as i32,
                    w * 10,
                    camera.target.y as i32,
                    Color::GREEN,
                );
            });

            d.draw_text("SCREEN AREA", 640, 10, 20, Color::RED);

            d.draw_rectangle(0, 0, w, 5, Color::RED);
            d.draw_rectangle(0, 5, 5, h - 10, Color::RED);
            d.draw_rectangle(w - 5, 5, 5, h - 10, Color::RED);
            d.draw_rectangle(0, h - 5, w, 5, Color::RED);

            d.draw_rectangle(10, 10, 250, 113, Color::SKYBLUE.alpha(0.5));
            d.draw_rectangle_lines(10, 10, 250, 113, Color::BLUE);

            d.draw_text("Free 2d camera controls:", 20, 20, 10, Color::BLACK);
            d.draw_text("- Right/Left to move Offset", 40, 40, 10, Color::DARKGRAY);
            d.draw_text("- Mouse Wheel to Zoom in-out", 40, 60, 10, Color::DARKGRAY);
            d.draw_text("- A / S to Rotate", 40, 80, 10, Color::DARKGRAY);
            d.draw_text(
                "- R to reset Zoom and Rotation",
                40,
                100,
                10,
                Color::DARKGRAY,
            );
        });
    }
}
