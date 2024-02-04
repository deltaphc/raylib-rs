#[cfg(test)]
pub(crate) mod manual_test {
    use crate::tests::*;
    use raylib::prelude::*;

    pub(crate) fn test_manual(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();

        let mut rl = handle.as_mut().unwrap();

        let mut prev_time = rl.get_time();
        let mut cur_time = 0.0;
        let mut wait_time = 0.0;
        let mut delta_time = 0.0;
        let mut update_draw_time = 0.0;
        let mut should_update = true;

        let default_font = rl.get_font_default();

        let target_fps = 60;
        while !rl.window_should_close() {
            rl.poll_input_events();

            // place this in a block to simulate BeginDrawing/EndDrawing
            {
                let mut d = rl.begin_drawing(&thread);

                d.draw_rectangle(
                    0,
                    0,
                    d.get_screen_width(),
                    d.get_screen_height(),
                    if d.is_key_down(KeyboardKey::KEY_SPACE) {
                        Color::RED
                    } else {
                        Color::WHITE
                    },
                );

                if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                    should_update = !should_update;
                }

                d.draw_text_ex(&default_font,format!("You should be able to hold SPACE\n\nto turn the background red\n\nand release it to turn it white.\n\nPressing BACKSPACE will toggle whether the\n\nthe latter part of the code and\n\nmake it act weird.\n\nweird: {}, last wait_time: {}",!should_update,wait_time).as_str(),Vector2::new(25.0,25.0),25.0,3.0,Color::BLACK);
            }

            if should_update {
                rl.swap_screen_buffer();
                cur_time = rl.get_time();
                update_draw_time = cur_time - prev_time;
                if target_fps > 0 {
                    wait_time = (1.0 / target_fps as f64) - update_draw_time;
                    if wait_time > 0.0 {
                        rl.wait_time(wait_time);
                        cur_time = rl.get_time();
                        delta_time = cur_time - prev_time;
                    }
                } else {
                    delta_time = update_draw_time;
                }
                prev_time = cur_time;
            }
        }
        /*if (IsKeyPressed(KEY_SPACE)) pause = !pause;

           if (IsKeyPressed(KEY_UP)) targetFPS += 20;
           else if (IsKeyPressed(KEY_DOWN)) targetFPS -= 20;

           if (targetFPS < 0) targetFPS = 0;

           if (!pause)
           {
               position += 200*deltaTime;  // We move at 200 pixels per second
               if (position >= GetScreenWidth()) position = 0;
               timeCounter += deltaTime;   // We count time (seconds)
           }
           //----------------------------------------------------------------------------------

           // Draw
           //----------------------------------------------------------------------------------
           BeginDrawing();

               ClearBackground(RAYWHITE);

               for (int i = 0; i < GetScreenWidth()/200; i++) DrawRectangle(200*i, 0, 1, GetScreenHeight(), SKYBLUE);

               DrawCircle((int)position, GetScreenHeight()/2 - 25, 50, RED);

               DrawText(TextFormat("%03.0f ms", timeCounter*1000.0f), (int)position - 40, GetScreenHeight()/2 - 100, 20, MAROON);
               DrawText(TextFormat("PosX: %03.0f", position), (int)position - 50, GetScreenHeight()/2 + 40, 20, BLACK);

               DrawText("Circle is moving at a constant 200 pixels/sec,\nindependently of the frame rate.", 10, 10, 20, DARKGRAY);
               DrawText("PRESS SPACE to PAUSE MOVEMENT", 10, GetScreenHeight() - 60, 20, GRAY);
               DrawText("PRESS UP | DOWN to CHANGE TARGET FPS", 10, GetScreenHeight() - 30, 20, GRAY);
               DrawText(TextFormat("TARGET FPS: %i", targetFPS), GetScreenWidth() - 220, 10, 20, LIME);
               DrawText(TextFormat("CURRENT FPS: %i", (int)(1.0f/deltaTime)), GetScreenWidth() - 220, 40, 20, GREEN);

           EndDrawing();

           // NOTE: In case raylib is configured to SUPPORT_CUSTOM_FRAME_CONTROL,
           // Events polling, screen buffer swap and frame time control must be managed by the user

           SwapScreenBuffer();         // Flip the back buffer to screen (front buffer)

           currentTime = GetTime();
           updateDrawTime = currentTime - previousTime;

           if (targetFPS > 0)          // We want a fixed frame rate
           {
               waitTime = (1.0f/(float)targetFPS) - updateDrawTime;
               if (waitTime > 0.0)
               {
                   WaitTime((float)waitTime);
                   currentTime = GetTime();
                   deltaTime = (float)(currentTime - previousTime);
               }
           }
           else deltaTime = (float)updateDrawTime;    // Framerate could be variable

           previousTime = currentTime;
           //----------------------------------------------------------------------------------
        */
    }
}
