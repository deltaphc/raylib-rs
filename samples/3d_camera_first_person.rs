use arr_macro::arr;
use nalgebra::{Vector2, Vector3};
use rand::prelude::*;
use raylib::{
    ffi::{Camera3D, CameraMode, Color},
    prelude::*,
};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

struct Column {
    height: f32,
    position: Vector3<f32>,
    color: Color,
}

impl Column {
    fn create_random() -> Column {
        let mut rng = rand::thread_rng();
        let height: f32 = rng.gen_range(1.0, 12.0);
        let position = Vector3::new(
            rng.gen_range(-15.0, 15.0),
            height / 2.0,
            rng.gen_range(-15.0, 15.0),
        );
        let color = Color::new(rng.gen_range(20, 255), rng.gen_range(10, 55), 30, 255);

        Column {
            height,
            position,
            color,
        }
    }
}

fn main() {
    let (rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, world!")
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 2.0, 4.0).into(),
        Vector3::new(0.0, 1.8, 0.0).into(),
        Vector3::new(0.0, 1.0, 0.0).into(),
        60.0,
    );
    let columns: [Column; 20] = arr![Column::create_random(); 20];

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FIRST_PERSON);

        rl.frame(&thread, |d| {
            d.clear_background(Color::DARKGREEN);
            d.mode_camera_3d(&camera, || {
                d.draw_plane(
                    Vector3::new(0.0, 0.0, 0.0).into(),
                    Vector2::new(32.0, 32.0).into(),
                    Color::LIGHTGRAY,
                );
                d.draw_cube(
                    Vector3::new(-16.0, 2.5, 0.0).into(),
                    1.0,
                    5.0,
                    32.0,
                    Color::BLUE,
                );
                d.draw_cube(
                    Vector3::new(16.0, 2.5, 0.0).into(),
                    1.0,
                    5.0,
                    32.0,
                    Color::LIME,
                );
                d.draw_cube(
                    Vector3::new(0.0, 2.5, 16.0).into(),
                    32.0,
                    5.0,
                    1.0,
                    Color::GOLD,
                );

                for column in columns.iter() {
                    d.draw_cube(
                        column.position.into(),
                        2.0,
                        column.height,
                        2.0,
                        column.color,
                    );
                    d.draw_cube_wires(
                        column.position.into(),
                        2.0,
                        column.height,
                        2.0,
                        Color::MAROON,
                    );
                }
            });

            d.draw_rectangle(10, 10, 220, 70, Color::SKYBLUE);
            d.draw_rectangle_lines(10, 10, 220, 70, Color::BLUE);
            d.draw_text(
                "First person camera default controls:",
                20,
                20,
                10,
                Color::BLACK,
            );
            d.draw_text("- Move with keys: W, A, S, D", 40, 40, 10, Color::DARKGRAY);
            d.draw_text("- Mouse move to look around", 40, 60, 10, Color::DARKGRAY);
        });
    }
}
