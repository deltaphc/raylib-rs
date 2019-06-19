extern crate raylib;
use raylib::ffi::*;
use std::ffi::CString;

fn main() {
    let w = 800;
    let h = 450;
    // let rust_orange = raylib::Color::new(222, 165, 132, 255);
    let ray_white = raylib::Color::new(255, 255, 255, 255);

    unsafe {
        InitWindow(
            w,
            h,
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
            DrawTriangle(
                Vector2 { x: 0.0, y: 30.0 },
                Vector2 { x: 15.0, y: 0.0 },
                Vector2 { x: 30.0, y: 30.0 },
                Color {
                    r: 230,
                    g: 41,
                    b: 55,
                    a: 255,
                },
            );
            EndDrawing();
        }
    }
}
