use std::slice::from_raw_parts;

use raylib::prelude::*;

const NUM_PROCESSES: usize = 8;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - image processing");

    let mut current_process: usize = 0;
    let mut image = Image::load_image("original/textures/resources/parrots.png").unwrap(); // Loaded in CPU memory (RAM)
    image.set_format(crate::consts::PixelFormat::PIXELFORMAT_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
    let image_data_len = image.get_pixel_data_size();
    let mut texture = rl.load_texture_from_image(&thread, &image).unwrap();

    let toggle_rects: Vec<(&str, Rectangle)> = (0..NUM_PROCESSES)
        .zip([
            "NO PROCESSING",
            "COLOR GRAYSCALE",
            "COLOR TINT",
            "COLOR INVERT",
            "COLOR CONTRAST",
            "COLOR BRIGHTNESS",
            "FLIP VERTICAL",
            "FLIP HORIZONTAL",
        ])
        .map(|(i, text)| (text, rrect(40, 50 + 32 * i as u32, 150, 30)))
        .collect();

    let mut reload_texture = false;
    let mut hover_rect: i32 = -1;

    rl.set_target_fps(60);
    Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        // Mouse hover logic
        for (i, (_, rect)) in toggle_rects.iter().enumerate() {
            if rect.check_collision_point_rec(rl.get_mouse_position()) {
                hover_rect = i as i32;
                if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
                    current_process = i;
                    reload_texture = true;
                }
                break;
            } else {
                hover_rect = -1;
            }
        }

        // Keyboard toggle logic
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            current_process += 1;
            if current_process > NUM_PROCESSES - 1 {
                current_process = 0;
            }
            reload_texture = true;
        } else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            current_process = if current_process == 0 {
                NUM_PROCESSES - 1
            } else {
                current_process - 1
            };
            reload_texture = true;
        }

        // Reload texture when required
        if reload_texture {
            let mut imcopy = image.clone();
            match current_process {
                1 => imcopy.color_grayscale(),
                2 => imcopy.color_tint(Color::GREEN),
                3 => imcopy.color_invert(),
                4 => imcopy.color_contrast(-40.0),
                5 => imcopy.color_brightness(-80),
                6 => imcopy.flip_vertical(),
                7 => imcopy.flip_horizontal(),
                _ => {}
            }

            unsafe {
                let colors = ffi::LoadImageColors(*imcopy);
                texture.update_texture(from_raw_parts(colors as *const u8, image_data_len));
                ffi::UnloadImageColors(colors);
            }

            reload_texture = false;
        }

        // Draw
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("IMAGE PROCESSING:", 40, 30, 10, Color::DARKGRAY);
        // Draw rectangles
        for (i, (text, rect)) in toggle_rects.iter().enumerate() {
            d.draw_rectangle_rec(
                rect,
                if i == current_process || i as i32 == hover_rect {
                    Color::SKYBLUE
                } else {
                    Color::LIGHTGRAY
                },
            );
            d.draw_rectangle_lines(
                rect.x as i32,
                rect.y as i32,
                rect.width as i32,
                rect.height as i32,
                if i == current_process || i as i32 == hover_rect {
                    Color::BLUE
                } else {
                    Color::GRAY
                },
            );
            d.draw_text(
                text,
                (rect.x + rect.width / 2.0) as i32 - measure_text(text, 10) / 2,
                rect.y as i32 + 11,
                10,
                if i == current_process || i as i32 == hover_rect {
                    Color::DARKBLUE
                } else {
                    Color::DARKGRAY
                },
            );
        }

        d.draw_texture(
            &texture,
            screen_width - texture.width() - 60,
            screen_height / 2 - texture.height() / 2,
            Color::WHITE,
        );
        d.draw_rectangle_lines(
            screen_width - texture.width() - 60,
            screen_height / 2 - texture.height() / 2,
            texture.width(),
            texture.height(),
            Color::BLACK,
        );
    })
}
