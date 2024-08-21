#[cfg(test)]
mod data_test {
    use crate::tests::*;
    use colored::Colorize;
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

    ray_test!(base64);
    fn base64(_: &RaylibThread) {
        let encoded = encode_data_base64("This is a test".as_bytes());
        let enc: Vec<u8> = encoded.to_vec().iter().map(|f| *f as u8).collect();
        let decoded = decode_data_base64(&enc);

        let fin = std::str::from_utf8(&decoded).unwrap();
        assert!(fin == "This is a test")
    }
}
