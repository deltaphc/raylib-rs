/*******************************************************************************************
*
*   raylib [core] example - Gamepad input
*
*   NOTE: This example requires a Gamepad connected to the system
*         raylib is configured to work with the following gamepads:
*                - Xbox 360 Controller (Xbox 360, Xbox One)
*                - PLAYSTATION(R)3 Controller
*         Check raylib.h for buttons configuration
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2013-2019 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

// NOTE: Gamepad name ID depends on drivers and OS
// #if defined(PLATFORM_RPI)
// const XBOX360_NAME_ID "Microsoft X-Box 360 pad" const PS3_NAME_ID "PLAYSTATION(R)3 Controller"
// #else
// const XBOX360_NAME_ID "Xbox 360 Controller" const PS3_NAME_ID "PLAYSTATION(R)3 Controller"
// #endif
const XBOX360_NAME_ID: &'static str = "Xbox 360 Controller";
const PS3_NAME_ID: &'static str = "PLAYSTATION(R)3 Controller";

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialscreen_width
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - gamepad input");

    let tex_ps3_pad = rl
        .load_texture(thread, "original/core/resources/ps3.png")
        .unwrap();
    let tex_xbox_pad = rl
        .load_texture(thread, "original/core/resources/xbox.png")
        .unwrap();

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // ...
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if d.is_gamepad_available(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1)
        {
            d.draw_text(&format!("GP1: {}", d.get_gamepad_name(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1).unwrap()), 10, 10, 10, Color::BLACK);

            if d.is_gamepad_name(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, XBOX360_NAME_ID)
            {
                d.draw_texture(&tex_xbox_pad, 0, 0, Color::DARKGRAY);

                // Draw buttons: xbox home
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_MIDDLE)
                {

                    d.draw_circle(394, 89, 19.0,Color::RED);
                }

                // Draw buttons: basic
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)
                    {
                        d.draw_circle(436, 150, 9.0,Color::RED);

                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT)
                    {

                        d.draw_circle(352, 150, 9.0,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT)
                    {

                        d.draw_circle(501, 151, 15.0, Color::BLUE);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN)
                    {

                        d.draw_circle(536, 187, 15.0, Color::LIME);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT)
                    {

                        d.draw_circle(572, 151, 15.0, Color::MAROON);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP)
                    {

                        d.draw_circle(536, 115, 15.0, Color::GOLD);
                    }

                // Draw buttons: d-pad
                d.draw_rectangle(317, 202, 19, 71, Color::BLACK);
                d.draw_rectangle(293, 228, 69, 19, Color::BLACK);
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP)
                    {

                        d.draw_rectangle(317, 202, 19, 26,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN)
                    {

                        d.draw_rectangle(317, 202 + 45, 19, 26,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT)
                    {

                        d.draw_rectangle(292, 228, 25, 19,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT)
                    {

                        d.draw_rectangle(292 + 44, 228, 26, 19,Color::RED);
                    }

                // Draw buttons: left-right back
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1)
                    {

                        d.draw_circle(259, 61, 20.0,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1)
                    {

                        d.draw_circle(536, 61, 20.0,Color::RED);
                    }

                // Draw axis: left joystick
                d.draw_circle(259, 152, 39.0, Color::BLACK);
                d.draw_circle(259, 152, 34.0, Color::LIGHTGRAY);
                d.draw_circle(259 + (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_LEFT_X) * 20.0) as i32,
                           152 - (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_LEFT_Y) * 2.00) as i32, 25.0, Color::BLACK);

                // Draw axis: right joystick
                d.draw_circle(461, 237, 38.0, Color::BLACK);
                d.draw_circle(461, 237, 33.0, Color::LIGHTGRAY);
                d.draw_circle(461 + (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_RIGHT_X) * 20.0) as i32,
                           237 - (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_RIGHT_Y) * 20.0) as i32, 25.0, Color::BLACK);

                // Draw axis: left-right triggers
                d.draw_rectangle(170, 30, 15, 70, Color::GRAY);
                d.draw_rectangle(604, 30, 15, 70, Color::GRAY);
                d.draw_rectangle(170, 30, 15, (((1.0 + d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER)) / 2.0) * 70.0) as i32,Color::RED);
                d.draw_rectangle(604, 30, 15, (((1.0 + d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER)) / 2.0) * 70.0) as i32,Color::RED);

                //d.draw_text(format!("Xbox axis LT: %02.02f", d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER)), 10, 40, 10, Color::BLACK);
                //d.draw_text(format!("Xbox axis RT: %02.02f", d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER)), 10, 60, 10, Color::BLACK);
            }
            else if d.is_gamepad_name(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, PS3_NAME_ID)
            {
                d.draw_texture(&tex_ps3_pad, 0, 0, Color::DARKGRAY);

                // Draw buttons: ps
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_MIDDLE)
                    {

                        d.draw_circle(396, 222, 13.0,Color::RED);
                    }

                // Draw buttons: basic
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT)
                    {

                        d.draw_rectangle(328, 170, 32, 13,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)
                    {

                        d.draw_triangle(rvec2(436, 168), rvec2(436, 185), rvec2(464, 177),Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP)
                    {

                        d.draw_circle(557, 144, 13.0, Color::LIME);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT)
                    {

                        d.draw_circle(586, 173, 13.0,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN)
                    {

                        d.draw_circle(557, 203, 13.0, Color::VIOLET);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT)
                    {

                        d.draw_circle(527, 173, 13.0, Color::PINK);
                    }

                // Draw buttons: d-pad
                d.draw_rectangle(225, 132, 24, 84, Color::BLACK);
                d.draw_rectangle(195, 161, 84, 25, Color::BLACK);
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP)
                    {

                        d.draw_rectangle(225, 132, 24, 29,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN)
                    {

                        d.draw_rectangle(225, 132 + 54, 24, 30,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT)
                    {

                        d.draw_rectangle(195, 161, 30, 25,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT)
                    {

                        d.draw_rectangle(195 + 54, 161, 30, 25,Color::RED);
                    }

                // Draw buttons: left-right back buttons
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1)
                    {

                        d.draw_circle(239, 82, 20.0,Color::RED);
                    }
                if d.is_gamepad_button_down(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1)
                    {

                        d.draw_circle(557, 82, 20.0,Color::RED);
                    }

                // Draw axis: left joystick
                d.draw_circle(319, 255, 35.0, Color::BLACK);
                d.draw_circle(319, 255, 31.0, Color::LIGHTGRAY);
                d.draw_circle(319 + (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_LEFT_X) * 20.0) as i32,
                           255 + (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_LEFT_Y) * 20.0) as i32, 25.0, Color::BLACK);

                // Draw axis: right joystick
                d.draw_circle(475, 255, 35.0, Color::BLACK);
                d.draw_circle(475, 255, 31.0, Color::LIGHTGRAY);
                d.draw_circle(475 + (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_RIGHT_X) * 20.0) as i32,
                           255 + (d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_RIGHT_Y) * 20.0) as i32, 25.0, Color::BLACK);

                // Draw axis: left-right triggers
                d.draw_rectangle(169, 48, 15, 70, Color::GRAY);
                d.draw_rectangle(611, 48, 15, 70, Color::GRAY);
                d.draw_rectangle(169, 48, 15, (((1.0 - d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER)) / 2.0) * 70.0) as i32,Color::RED);
                d.draw_rectangle(611, 48, 15, (((1.0 - d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, raylib::consts::GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER)) / 2.0) * 70.0) as i32,Color::RED);
            }
            else
            {
                d.draw_text("- GENERIC GAMEPAD -", 280, 180, 20, Color::GRAY);

                // TODO: Draw generic gamepad
            }

            d.draw_text(&format!("DETECTED AXIS [{}]:", d.get_gamepad_axis_count(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1)), 10, 50, 10, Color::MAROON);

            for i in 0..d.get_gamepad_axis_count(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1)
            {
                d.draw_text(&format!("AXIS {}: {:.02}", i, d.get_gamepad_axis_movement(raylib::consts::GamepadNumber::GAMEPAD_PLAYER1, unsafe {std::mem::transmute(i)})), 20, 70 + 20 * i, 10, Color::DARKGRAY);
            }

            if let Some(button) = d.get_gamepad_button_pressed()
                {

                    d.draw_text(&format!("DETECTED BUTTON: {:?}", button), 10, 430, 10,Color::RED);
                }
            else
                {

                    d.draw_text("DETECTED BUTTON: NONE", 10, 430, 10, Color::GRAY);
                }
        }
        else
        {
            d.draw_text("GP1: NOT DETECTED", 10, 10, 10, Color::GRAY);

            d.draw_texture(&tex_xbox_pad, 0, 0, Color::LIGHTGRAY);
        }

        //----------------------------------------------------------------------------------
    },
    );
}
