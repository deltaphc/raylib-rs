extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

pub fn main() {
    use raylib::consts;
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Camera 2D");
    let (w, h) = (opt.width, opt.height);

    let mut camera = Camera3D::perspective(
        Vector3::new(2.5, 2.5, 3.0),
        Vector3::new(0.0, 0.0, 0.7),
        Vector3::new(0.0, 1.0, 0.0),
        65.0,
    );

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);
    let mut shader = rl
        .load_shader(&thread, None, Some("raymarch-static/raymarching.fs"))
        .expect("couldn't load shader");
    // let s = std::fs::read_to_string("raymarch-static/raymarching.fs").expect("couldn't read");
    // println!("{}", s);

    let viewEyeLoc = shader.get_shader_location("viewEye");
    let viewCenterLoc = shader.get_shader_location("viewCenter");
    let viewUpLoc = shader.get_shader_location("viewUp");
    let deltaTimeLoc = shader.get_shader_location("deltaTime");
    let runTimeLoc = shader.get_shader_location("runTime");
    let resolutionLoc = shader.get_shader_location("resolution");

    let resolution = vec![w as f32, h as f32];
    shader.set_shader_value(resolutionLoc, &resolution);

    let mut runTime = 0.0;

    while !rl.window_should_close() {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        let cameraPos = vec![camera.position.x, camera.position.y, camera.position.z];
        let cameraTarget = vec![camera.target.x, camera.target.y, camera.target.z];
        let cameraUp = vec![camera.up.x, camera.up.y, camera.up.z];

        let deltaTime = rl.get_frame_time();
        runTime += deltaTime;

        // Set shader required uniform values
        shader.set_shader_value(viewEyeLoc, &cameraPos);
        shader.set_shader_value(viewCenterLoc, &cameraTarget);
        shader.set_shader_value(viewUpLoc, &cameraUp);
        shader.set_shader_value(deltaTimeLoc, &vec![deltaTime]);
        shader.set_shader_value(runTimeLoc, &vec![runTime]);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        // We only draw a white full-screen rectangle,
        // frame is generated in shader using raymarching
        {
            let mut d = d.begin_shader_mode(&shader);
            d.draw_rectangle(0, 0, w, h, Color::WHITE);
        }

        d.draw_text(
            "(c) Raymarching shader by Iñigo Quilez. MIT License.",
            w - 280,
            h - 20,
            10,
            Color::GRAY,
        );

        //----------------------------------------------------------------------------------
    }
}
