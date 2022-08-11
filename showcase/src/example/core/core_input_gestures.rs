/*******************************************************************************************
*
*   raylib [core] example - Input Gestures Detection
*
*   This example has been created using raylib 1.4 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2016 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

const MAX_GESTURE_STRINGS: usize = 20;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    use raylib::consts::Gestures::*;
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - input gestures");

    let touch_area = rrect(220, 10, screen_width - 230, screen_height - 20);

    let mut gestures_count = 0;
    let mut gesture_strings = [raylib::consts::Gestures::GESTURE_NONE; MAX_GESTURE_STRINGS];

    let mut current_gesture = GESTURE_NONE;

    //SetGesturesEnabled(0b0000000000001001);   // Enable only some gestures to be detected

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        let last_gesture = current_gesture;
        current_gesture = rl.get_gesture_detected();
        let touch_position = rl.get_touch_position(0);

        if touch_area.check_collision_point_rec(touch_position) && (current_gesture != GESTURE_NONE)
        {
            if current_gesture != last_gesture
            {
                gesture_strings[gestures_count] = current_gesture;

                gestures_count+=1;

                // Reset gestures strings
                if gestures_count >= MAX_GESTURE_STRINGS
                {
                    for gesture in &mut gesture_strings {
                        *gesture = GESTURE_NONE;
                    }

                    gestures_count = 0;
                }
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_rectangle_rec(touch_area, Color::GRAY);
        d.draw_rectangle(225, 15, screen_width - 240, screen_height - 30, Color::RAYWHITE);

        d.draw_text("GESTURES TEST AREA", screen_width - 270, screen_height - 40, 20, Color::GRAY.fade(0.5));

        for i in 0..gestures_count as i32
        {
            if i % 2 == 0
                {

                    d.draw_rectangle(10, 30 + 20 * i, 200, 20, Color::LIGHTGRAY.fade(0.5));
                }
            else
                {

                    d.draw_rectangle(10, 30 + 20 * i, 200, 20, Color::LIGHTGRAY.fade( 0.3));
                }

            if i < gestures_count as i32 - 1
                {

                    d.draw_text(&format!("{:?}",gesture_strings[i as usize]), 35, 36 + 20 * i, 10, Color::DARKGRAY);
                }
            else
                {

                    d.draw_text(&format!("{:?}",gesture_strings[i as usize]), 35, 36 + 20 * i, 10, Color::MAROON);
                }
        }

        d.draw_rectangle_lines(10, 29, 200, screen_height - 50, Color::GRAY);
        d.draw_text("DETECTED GESTURES", 50, 15, 10, Color::GRAY);

        if current_gesture != GESTURE_NONE
            {
                d.draw_circle_v(touch_position, 30.0, Color::MAROON);
            }

    },
    );
}
