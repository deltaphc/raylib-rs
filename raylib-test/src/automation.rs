#[cfg(test)]
pub(crate) mod automation_test {
    use crate::tests::*;
    use raylib::prelude::*;

    pub(crate) fn automation_test(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();

        let rl = handle.as_mut().unwrap();
        let camera_base = Camera3D::perspective(
            Vector3::new(4.0, 2.0, 4.0),
            Vector3::new(0.0, 1.8, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            60.0,
        );
        let mut camera = Camera3D::perspective(
            camera_base.position,
            camera_base.target,
            camera_base.up,
            camera_base.fovy,
        );

        let mut aelist = rl.load_automation_event_list(None);
        rl.set_automation_event_list(&mut aelist);

        let mut is_recording = false;
        let mut is_event_playing = false;

        let mut current_play_frame = 0;
        let mut play_frame_counter = 0;
        rl.set_target_fps(60);

        let mut events = aelist.events();

        while !rl.window_should_close() {
            rl.update_camera(&mut camera, CameraMode::CAMERA_FIRST_PERSON);

            if rl.is_key_released(KeyboardKey::KEY_SPACE) {
                if !is_recording {
                    rl.set_automation_event_base_frame(0);
                    rl.start_automation_event_recording();
                } else {
                    rl.stop_automation_event_recording();
                    aelist.export("test_out/automation.rae");
                    println!("RECORDED FRAMES: {}", aelist.count());
                    events = aelist.events();
                }
                is_recording = !is_recording;
            }

            if rl.is_key_released(KeyboardKey::KEY_BACKSPACE) {
                if !is_event_playing && aelist.count() > 0 {
                    is_event_playing = true;
                    play_frame_counter = 0;
                    current_play_frame = 0;
                    camera.position = camera_base.position;
                    camera.target = camera_base.target;
                    camera.up = camera_base.up;
                    camera.fovy = camera_base.fovy;
                }
            }

            if is_event_playing {
                while play_frame_counter == events[current_play_frame as usize].frame() {
                    if let Some(ev) = events.get(current_play_frame as usize) {
                        println!("PLAYING: PlayFrameCount: {} | currentPlayFrame: {} | Event Frame: {}, param: {}",play_frame_counter,current_play_frame,ev.frame(), ev.params()[0]);
                        ev.play();
                        current_play_frame += 1;

                        if current_play_frame == aelist.count() as usize {
                            is_event_playing = false;
                            current_play_frame = 0;
                            play_frame_counter = 0;
                            break;
                        }
                    }
                }
                play_frame_counter += 1;
            }
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::DARKGREEN);
            {
                let mut d2 = d.begin_mode3D(camera);

                d2.draw_plane(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector2::new(32.0, 32.0),
                    Color::LIGHTGRAY,
                );
                d2.draw_cube(Vector3::new(-16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::BLUE);
                d2.draw_cube(Vector3::new(16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::LIME);
                d2.draw_cube(Vector3::new(0.0, 2.5, 16.0), 32.0, 5.0, 1.0, Color::GOLD);
            }
            d.draw_rectangle(10, 10, 260, 120, Color::SKYBLUE);
            d.draw_rectangle_lines(10, 10, 260, 120, Color::BLUE);
            d.draw_text(
                "First person camera default controls:",
                20,
                20,
                10,
                Color::BLACK,
            );
            d.draw_text("- Move with keys: W, A, S, D", 40, 40, 10, Color::DARKGRAY);
            d.draw_text("- Mouse move to look around", 40, 60, 10, Color::DARKGRAY);
            d.draw_text(
                "- SPACE to start recording movement.",
                40,
                80,
                10,
                Color::DARKGRAY,
            );
            d.draw_text(
                "- BACKSPACE to play recorded movement.",
                40,
                100,
                10,
                Color::DARKGRAY,
            );
        }
    }
}
