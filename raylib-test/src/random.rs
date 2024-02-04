#[cfg(test)]
mod random_test {

    use crate::tests::*;
    use raylib::prelude::*;

    ray_test!(test_random_range);
    fn test_random_range(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        rl.set_random_seed(1);
        let r: i32 = rl.get_random_value(0..4);
        assert!(r == 2);
    }
}
