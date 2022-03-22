/*******************************************************************************************
*
*   raylib [textures] example - Mouse painting
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Dill (@MysteriousSpace) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Chris Dill (@MysteriousSpace) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_COLORS_COUNT: usize = 23; // Number of colors available

pub fn run(mut rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [textures] example - mouse painting");

    // Colours to choose from
    let colors = [
        Color::RAYWHITE,
        Color::YELLOW,
        Color::GOLD,
        Color::ORANGE,
        Color::PINK,
        Color::RED,
        Color::MAROON,
        Color::GREEN,
        Color::LIME,
        Color::DARKGREEN,
        Color::SKYBLUE,
        Color::BLUE,
        Color::DARKBLUE,
        Color::PURPLE,
        Color::VIOLET,
        Color::BEIGE,
        Color::BROWN,
        Color::LIGHTGRAY,
        Color::GRAY,
        Color::DARKGRAY,
        Color::BLACK,
    ];

    // Define colors_recs data (for every rectangle)
    let mut colors_recs = [Rectangle::default(); MAX_COLORS_COUNT];

    for i in 0..MAX_COLORS_COUNT {
        colors_recs[i].x = (10 + 30 * i + 2 * i) as f32;
        colors_recs[i].y = 10.0;
        colors_recs[i].width = 30.0;
        colors_recs[i].height = 30.0;
    }

    let mut color_selected = 0;
    let mut color_selected_prev = color_selected;
    let mut color_mouse_hover = None;
    let mut brush_size = 20;

    let btn_save_rec = rrect::<i32, i32, i32, i32>(750, 10, 40, 30);
    let mut btn_save_mouse_hover = false;
    let mut show_save_message = false;
    let mut save_message_counter = 0;

    // Create a RenderTexture2D to use as a canvas
    let mut target = rl
        .load_render_texture(thread, screen_width as u32, screen_height as u32)
        .unwrap();

    // Clear render texture before entering the game loop

    {
        let mut d = rl.begin_texture_mode(thread, &mut target);
        d.clear_background(colors[0]);
    }

    rl.set_target_fps(120); // Set our game to run at 120 frames-per-second
                            //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |mut rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
            use raylib::consts::Gestures::*;
            // Update
            //----------------------------------------------------------------------------------
            let mouse_pos = rl.get_mouse_position();

            // Move between colors with keys
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT)
                && color_selected < MAX_COLORS_COUNT
            {
                color_selected += 1;
            } else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT) && color_selected > 0
            {
                color_selected -= 1;
            }

            // Choose color with mouse
            for i in 0..MAX_COLORS_COUNT {
                if colors_recs[i].check_collision_point_rec(mouse_pos) {
                    color_mouse_hover = Some(i);
                    break;
                } else {
                    color_mouse_hover = None;
                }
            }

            if color_mouse_hover.is_some()
                && rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
            {
                color_selected = color_mouse_hover.unwrap();
                color_selected_prev = color_selected;
            }

            // Change brush size
            brush_size += rl.get_mouse_wheel_move() as i32 * 5;
            if brush_size < 2 {
                brush_size = 2;
            }
            if brush_size > 50 {
                brush_size = 50;
            }

            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_C) {
                // Clear render texture to clear color
                let mut d = rl.begin_texture_mode(thread, &mut target);
                d.clear_background(colors[0]);
            }

            if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
                || rl.get_gesture_detected() == GESTURE_DRAG
            {
                // Paint circle into render texture
                // NOTE: To avoid discontinuous circles, we could store
                // previous-next mouse points and just draw a line using brush size
                let mut d = rl.begin_texture_mode(thread, &mut target);

                if mouse_pos.y > 50.0 {
                    d.draw_circle(
                        mouse_pos.x as i32,
                        mouse_pos.y as i32,
                        brush_size as f32,
                        colors[color_selected],
                    );
                }
            }

            if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON) {
                color_selected = 0;

                // Erase circle from render texture
                let mut d = rl.begin_texture_mode(thread, &mut target);

                if mouse_pos.y > 50.0 {
                    d.draw_circle(
                        mouse_pos.x as i32,
                        mouse_pos.y as i32,
                        brush_size as f32,
                        colors[0],
                    );
                }
            } else {
                color_selected = color_selected_prev;
            }

            // Check mouse hover save button
            if btn_save_rec.check_collision_point_rec(mouse_pos) {
                btn_save_mouse_hover = true;
            } else {
                btn_save_mouse_hover = false;
            }

            // Image saving logic
            // NOTE: Saving painted texture to a default named image
            if btn_save_mouse_hover
                && rl.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
                || rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S)
            {
                let mut image = target.get_texture_data().unwrap();
                image.flip_vertical();

                image.export_image("my_amazing_texture_painting.png");
                show_save_message = true;
            }

            if show_save_message {
                // On saving, show a full screen message for 2 seconds
                save_message_counter += 1;
                if save_message_counter > 240 {
                    show_save_message = false;
                    save_message_counter = 0;
                }
            }
            //----------------------------------------------------------------------------------

            // Draw
            //----------------------------------------------------------------------------------
            let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
            d.draw_texture_rec(
                &target,
                rrect::<i32, i32, i32, i32>(0, 0, target.texture.width, -target.texture.height),
                rvec2::<i32, i32>(0, 0),
                Color::WHITE,
            );

            // Draw drawing circle for reference
            if mouse_pos.y > 50.0 {
                if d.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON) {
                    d.draw_circle_lines(
                        mouse_pos.x as i32,
                        mouse_pos.y as i32,
                        brush_size as f32,
                        Color::GRAY,
                    );
                } else {
                    d.draw_circle(
                        d.get_mouse_x(),
                        d.get_mouse_y(),
                        brush_size as f32,
                        colors[color_selected],
                    );
                }
            }

            // Draw top panel
            d.draw_rectangle(0, 0, d.get_screen_width(), 50, Color::RAYWHITE);
            d.draw_line(0, 50, d.get_screen_width(), 50, Color::LIGHTGRAY);

            // Draw color selection rectangles
            for i in 0..MAX_COLORS_COUNT {
                d.draw_rectangle_rec(colors_recs[i], colors[i]);
            }
            d.draw_rectangle_lines(10, 10, 30, 30, Color::LIGHTGRAY);

            if let Some(color_mouse_hover) = color_mouse_hover {
                d.draw_rectangle_rec(
                    colors_recs[color_mouse_hover as usize],
                    Color::WHITE.fade(0.6),
                );
            }

            d.draw_rectangle_lines_ex(
                rrect(
                    colors_recs[color_selected].x - 2.0,
                    colors_recs[color_selected].y - 2.0,
                    colors_recs[color_selected].width + 4.0,
                    colors_recs[color_selected].height + 4.0,
                ),
                2,
                Color::BLACK,
            );

            // Draw save image button
            d.draw_rectangle_lines_ex(
                btn_save_rec,
                2,
                if btn_save_mouse_hover {
                    Color::RED
                } else {
                    Color::BLACK
                },
            );
            d.draw_text(
                "SAVE!",
                755,
                20,
                10,
                if btn_save_mouse_hover {
                    Color::RED
                } else {
                    Color::BLACK
                },
            );

            // Draw save image message
            if show_save_message {
                d.draw_rectangle(
                    0,
                    0,
                    d.get_screen_width(),
                    d.get_screen_height(),
                    Color::RAYWHITE.fade(0.8),
                );
                d.draw_rectangle(0, 150, d.get_screen_width(), 80, Color::BLACK);
                d.draw_text(
                    "IMAGE SAVED:  my_amazing_texture_painting.png",
                    150,
                    180,
                    20,
                    Color::RAYWHITE,
                );
            }

            //----------------------------------------------------------------------------------
        },
    );
}
