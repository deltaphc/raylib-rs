#![doc(
    html_logo_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust_256x256.png",
    html_favicon_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust.ico"
)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::GetRandomValue;

    #[test]
    fn call_random_function() {
        let result = unsafe { GetRandomValue(0, 10) };
        assert!(result >= 0);
        assert!(result <= 10);
    }
}
