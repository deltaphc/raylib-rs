#[cfg(test)]
mod core_test {
    
    use crate::tests::*;
    use raylib::camera::*;
    use raylib::math::*;
    use raylib::RaylibThread;
    #[test]
    fn test_clipboard() {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let s = "Hello, world!";
        rl.set_clipboard_text("Hello, world!").unwrap();
        let other = rl.get_clipboard_text().unwrap();
        assert_eq!(s, other);
    }

    #[test]
    fn test_screen_space() {
        let handle = TEST_HANDLE.read().unwrap();
        let rl = handle.as_ref().unwrap();
        let c = Camera::orthographic(
            Vector3::zero(),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::up(),
            90.0,
        );
        let _ = rl.get_mouse_ray(Vector2::zero(), &c);
        // Should be the middle of the screen
        let _ = rl.get_world_to_screen(Vector3::zero(), &c);
    }

    #[test]
    fn test_timing_functions() {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.set_target_fps(24);
        let _fps = rl.get_fps();
        rl.get_frame_time();
        rl.get_time();
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_window_ops() {
        // Call twice to make sure multiple calls won't panic
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        // double hide double show
        rl.hide_window();
        rl.hide_window();
        // TODO uncomment this when we can draw a frame
        // assert!(rl.is_window_hidden(), "window is not hidden!");

        rl.unhide_window();
        rl.unhide_window();
        // assert!(!rl.is_window_hidden(), "window is hidden!");
    }

    ray_test!(test_set_window_name);
    fn test_set_window_name(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        rl.set_window_title(thread, "raylib test");
        assert_eq!(
            rl.get_screen_width(),
            TEST_WIDTH,
            "screen width is not the expected size!"
        );
        assert_eq!(
            rl.get_screen_height(),
            TEST_HEIGHT,
            "screen height is not the expected size!"
        );
    }

    #[test]
    fn test_cursor() {
        // Call twice to make sure multiple calls won't panic
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        // double hide double show
        rl.hide_cursor();
        rl.hide_cursor();
        // TODO uncomment this when we can draw a frame
        // assert!(rl.is_cursor_hidden(), "window is not hidden!");

        rl.show_cursor();
        rl.show_cursor();
        // assert!(!rl.is_cursor_hidden(), "window is hidden!");

        rl.disable_cursor();
        rl.disable_cursor();
        rl.enable_cursor();
        rl.enable_cursor();
    }
}
