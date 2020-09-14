/*******************************************************************************************
*
*   raylib [core] example - 2d camera
*
*   This example has been created using raylib 1.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2016 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;

const MAX_BUILDINGS: i32 = 100;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;
    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - 2d camera");

    let mut player = Rectangle::new(400.0, 280.0, 40.0, 40.0);
    let mut buildings = [Rectangle::default(); MAX_BUILDINGS as usize];
    let mut build_colors = [Color::default(); MAX_BUILDINGS as usize];

    let mut spacing = 0f32;

    for i in 0..MAX_BUILDINGS as usize {
        buildings[i].width = get_random_value::<i32>(50, 200) as f32;
        buildings[i].height = get_random_value::<i32>(100, 800) as f32;
        buildings[i].y = screen_height as f32 - 130.0 - buildings[i].height;
        buildings[i].x = -6000.0 + spacing;

        spacing += buildings[i].width;

        build_colors[i] = Color::new(
            get_random_value::<i32>(200, 240) as u8,
            get_random_value::<i32>(200, 240) as u8,
            get_random_value::<i32>(200, 250) as u8,
            255,
        );
    }

    let mut camera = Camera2D {
        target: rvec2(player.x + 20.0, player.y + 20.0),
        offset: rvec2(screen_width as f32 / 2.0, screen_height as f32 / 2.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    // Detect window close button or ESC key
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        
        // Update
        //----------------------------------------------------------------------------------

        // Player movement
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT) {
            player.x += 2.0;
        } else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT) {
            player.x -= 2.0;
        }

        // Camera target follows player
        camera.target = Vector2::new(player.x + 20.0, player.y + 20.0);

        // Camera rotation controls
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            camera.rotation -= 1.0;
        } else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            camera.rotation += 1.0;
        }
        // Limit camera rotation to 80 degrees (-40 to 40)
        if camera.rotation > 40.0 {
            camera.rotation = 40.0;
        } else if camera.rotation < -40.0 {
            camera.rotation = -40.0;
        }

        // Camera zoom controls
        camera.zoom += rl.get_mouse_wheel_move() as f32 * 0.05;

        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        } else if camera.zoom < 0.1 {
            camera.zoom = 0.1;
        }

        // Camera reset (zoom and rotation)
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_R) {
            camera.zoom = 1.0;
            camera.rotation = 0.0;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode2D(camera);

            d.draw_rectangle(-6000, 320, 13000, 8000, Color::DARKGRAY);

            for i in 0..MAX_BUILDINGS as usize {
                d.draw_rectangle_rec(buildings[i], build_colors[i]);
            }

            d.draw_rectangle_rec(player, Color::RED);

            d.draw_line(
                camera.target.x as i32,
                -screen_height * 10,
                camera.target.x as i32,
                screen_height * 10,
                Color::GREEN,
            );
            d.draw_line(
                -screen_width * 10,
                camera.target.y as i32,
                screen_width * 10,
                camera.target.y as i32,
                Color::GREEN,
            );
        }

        d.draw_text("SCREEN AREA", 640, 10, 20, Color::RED);

        d.draw_rectangle(0, 0, screen_width, 5, Color::RED);
        d.draw_rectangle(0, 5, 5, screen_height - 10, Color::RED);
        d.draw_rectangle(screen_width - 5, 5, 5, screen_height - 10, Color::RED);
        d.draw_rectangle(0, screen_height - 5, screen_width, 5, Color::RED);

        d.draw_rectangle(10, 10, 250, 113, Color::SKYBLUE.fade(0.5));
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

        return ();
        //----------------------------------------------------------------------------------
    });
}
