fn main() {
    if std::env::var("TARGET")
        .expect("Cargo build scripts always have TARGET")
        .contains("emscripten")
    {
        println!("cargo:rustc-link-lib=glfw");
    }
}
