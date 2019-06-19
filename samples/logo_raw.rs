extern crate raylib;
use raylib::ffi::*;
use std::ffi::CString;

fn main() {
    let w = 800;
    let h = 450;
    let rust_orange = raylib::Color::new(222, 165, 132, 255);
    let ray_white = raylib::Color::new(255, 255, 255, 255);

    unsafe {
        InitWindow(
            800,
            450,
            CString::new("raylib [core] example - basic window")
                .unwrap()
                .as_ptr(),
        );

        SetTargetFPS(60); // Set our game to run at 60 frames-per-second
                          //--------------------------------------------------------------------------------------

        // Main game loop
        while !WindowShouldClose() {
            // Detect window close button or ESC key

            BeginDrawing();
            ClearBackground(ray_white.into());
            DrawRectangle(w / 2 - 128, h / 2 - 128, 256, 256, rust_orange.into());
            DrawRectangle(w / 2 - 112, h / 2 - 112, 224, 224, ray_white.into());
            DrawText(
                CString::new("rust").unwrap().as_ptr(),
                w / 2 - 69,
                h / 2 + 18,
                50,
                rust_orange.into(),
            );
            DrawText(
                CString::new("raylib").unwrap().as_ptr(),
                w / 2 - 44,
                h / 2 + 48,
                50,
                rust_orange.into(),
            );
            EndDrawing();
        }
    }
}
