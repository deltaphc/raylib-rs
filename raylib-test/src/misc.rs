#[cfg(test)]
mod core_test {
    use crate::tests::*;
    use raylib::prelude::*;
    ray_test!(test_screenshot);
    fn test_screenshot(t: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        rl.take_screenshot(t, "test_out/screenshot.png");
        assert!(std::path::Path::new("test_out/screenshot.png").exists());
    }

    ray_test!(test_screendata);
    fn test_screendata(t: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        // make sure it doesn't seg fault
        let _ = rl.get_screen_data(t);
    }
}
