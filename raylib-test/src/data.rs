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
}
