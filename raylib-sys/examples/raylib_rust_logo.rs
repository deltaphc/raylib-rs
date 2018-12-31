extern crate raylib_sys;

use std::ffi::CString;

use raylib_sys::{
    BeginDrawing, ClearBackground, CloseWindow, Color, DrawRectangle, DrawText, EndDrawing,
    InitWindow, SetTargetFPS, WindowShouldClose,
};

fn main() {
    let w = 800;
    let h = 450;
    let ray_white = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };
    let rust_orange = Color {
        r: 222,
        g: 165,
        b: 132,
        a: 255,
    };
    let raw_window_title = CString::new("raylib-rust logo").unwrap();
    let raw_text1 = CString::new("rust").unwrap();
    let raw_text2 = CString::new("raylib").unwrap();
    unsafe {
        InitWindow(w, h, raw_window_title.as_ptr());
        SetTargetFPS(60);
        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(ray_white);
            DrawRectangle(w / 2 - 128, h / 2 - 128, 256, 256, rust_orange);
            DrawRectangle(w / 2 - 112, h / 2 - 112, 224, 224, ray_white);
            DrawText(raw_text1.as_ptr(), w / 2 - 69, h / 2 + 18, 50, rust_orange);
            DrawText(raw_text2.as_ptr(), w / 2 - 44, h / 2 + 48, 50, rust_orange);
            EndDrawing();
        }
        CloseWindow();
    }
}
