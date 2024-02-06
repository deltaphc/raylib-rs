#[cfg(test)]
mod random_test {

    use crate::tests::*;
    use raylib::prelude::*;

    ray_test!(test_random_range);
    fn test_random_range(_: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        rl.set_random_seed(1);
        let r: i32 = rl.get_random_value(0..4);
        assert!(r == 2);
    }

    ray_test!(test_random_seq);
    fn test_random_seq(_: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        rl.set_random_seed(1);

        let rnd = rl.load_random_sequence(1..10, 10);
        let exp_rnd = vec![-8, 7, 0, 4, 8, -6, -3, 5, 6, 10];
        let mut i = 0;
        for r in rnd {
            assert!(r == *exp_rnd.get(i).unwrap());
            i += 1;
        }
    }
}
