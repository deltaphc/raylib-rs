#[cfg(test)]
mod data_test {
    use crate::tests::*;
    use raylib::prelude::*;

    ray_test!(data_test);
    fn data_test(_: &RaylibThread) {
        export_data_as_code(
            "The quick brown fox jumped over the lazy dog.".as_bytes(),
            "./test_out/export_data.txt",
        );
    }

    ray_test!(base64);
    fn base64(_: &RaylibThread) {
        let encoded = encode_data_base64("This is a test".as_bytes());
        let enc: Vec<u8> = encoded.expect("encode ok").to_vec().iter().map(|f| *f as u8).collect();
        let decoded = decode_data_base64(&enc).expect("decode ok");
        let fin = std::str::from_utf8(&decoded).unwrap();
        assert_eq!(fin, "This is a test")
    }

    ray_test!(base64_encode_and_decode);
    fn base64_encode_and_decode(_: &RaylibThread) {
        let encoded = encode_data_base64(b"This is a test").expect("encode ok");
        let mut enc = encoded.as_ref();
        if enc.ends_with(&[0]) {
            enc = &enc[..enc.len() - 1];
        }
        let decoded = decode_data_base64(enc).expect("decode ok");
        assert_eq!(decoded.as_ref(), b"This is a test");
    }

    ray_test!(base64_decode_plain_str);
    fn base64_decode_plain_str(_: &RaylibThread) {
        //non-trailing null
        let str = b"VGhpcyBpcyBhIHRlc3Q=";
        let out = decode_data_base64(str).expect("decode plain ok");
        assert_eq!(out.as_ref(), b"This is a test");
    }

    ray_test!(base64_decode_with_trailing_null);
    fn base64_decode_with_trailing_null(_: &RaylibThread) {
        // trailing null
        let mut c_str = b"SGVsbG8sIHdvcmxkIQ==".to_vec();
        c_str.push(0);
        let out = decode_data_base64(&c_str).expect("decode c-string ok");
        assert_eq!(out.as_ref(), b"Hello, world!");
    }
}