extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const SHADER: &str = include_str!("static/raymarching.fs");

pub fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Camera 2D");
    let (w, h) = (opt.width, opt.height);

    let mut camera = Camera3D::perspective(
        Vector3::new(2.5, 2.5, 3.0),
        Vector3::new(0.0, 0.0, 0.7),
        Vector3::new(0.0, 1.0, 0.0),
        65.0,
    );

    let mut shader = rl.load_shader_from_memory(&thread, None, Some(SHADER));
    // let s = std::fs::read_to_string("raymarch-static/raymarching.fs").expect("couldn't read");
    // println!("{}", s);

    let view_eye_loc = shader.get_shader_location("viewEye");
    let view_center_loc = shader.get_shader_location("viewCenter");
    let view_up_loc = shader.get_shader_location("viewUp");
    let delta_time_loc = shader.get_shader_location("deltaTime");
    let runtime_loc = shader.get_shader_location("runTime");
    let resolution_loc = shader.get_shader_location("resolution");

    let resolution: [f32; 2] = [w as f32, h as f32];
    shader.set_shader_value(resolution_loc, resolution);

    let mut run_time = 0.0;

    while !rl.window_should_close() {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        let camera_pos = Vector3::new(camera.position.x, camera.position.y, camera.position.z);
        let camera_target = Vector3::new(camera.target.x, camera.target.y, camera.target.z);
        let camera_up = Vector3::new(camera.up.x, camera.up.y, camera.up.z);

        let delta_time = rl.get_frame_time();
        run_time += delta_time;

        // Set shader required uniform values
        shader.set_shader_value(view_eye_loc, camera_pos);
        shader.set_shader_value(view_center_loc, camera_target);
        shader.set_shader_value(view_up_loc, camera_up);
        shader.set_shader_value(delta_time_loc, delta_time);
        shader.set_shader_value(runtime_loc, run_time);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        rl.start_drawing(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);

            // We only draw a white full-screen rectangle,
            // frame is generated in shader using raymarching
            {
                d.start_shader_mode(&mut shader, |mut d, _shader| {
                    d.draw_rectangle(0, 0, w, h, Color::WHITE);
                });
            }

            d.draw_text(
                "(c) Raymarching shader by Iñigo Quilez. MIT License.",
                w - 280,
                h - 20,
                10,
                Color::GRAY,
            );
        });

        //----------------------------------------------------------------------------------
    }
}
