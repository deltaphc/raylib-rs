#[cfg(test)]
mod data_test {
    use crate::tests::*;
    use raylib::prelude::*;

    ray_test!(data_test);
    fn data_test(_: &RaylibThread) {
        //let mut handle = TEST_HANDLE.write().unwrap();
        //let rl = handle.as_mut().unwrap();

        export_data_as_code(
            "The quick brown fox jumped over the lazy dog.".as_bytes(),
            "./test_out/export_data.txt",
        );
    }

    ray_test!(application_dir);
    fn application_dir(_: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        println!(
            "\n\n=====\nApplication directory is {}\n=====\n\n",
            rl.application_directory()
        );
    }

    ray_test!(file_length_test);
    fn file_length_test(_: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        let len = rl.get_file_length("./resources/just_exists.txt").unwrap();
        assert!(len == 18);
    }

    ray_test!(is_path_file);
    fn is_path_file(_: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();

        let len = rl.is_path_file("./resources/just_exists.txt").unwrap();
        assert!(len == true);
        let len = rl.is_path_file("./resources/").unwrap();
        assert!(len == false);
    }
}
