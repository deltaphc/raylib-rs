use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Model shader example");
    let (w, h) = (opt.width, opt.height);

    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 4.0, 4.0),  // Position
        Vector3::new(0.0, 1.0, -1.0), // Target
        Vector3::new(0.0, 1.0, 0.0),  // Up vector
        45.0,                         // FOV
    );

    rl.set_target_fps(60);

    // Load shader
    let shader = rl.load_shader(&thread, None, Some("static/model_shader/grayscale.fs"));

    // Load model
    let mut model = rl
        .load_model(&thread, "static/model_shader/watermill.obj")
        .unwrap();

    // Load texture and generate mipmaps
    let texture = unsafe {
        let mut t = rl
            .load_texture(&thread, "static/model_shader/watermill_diffuse.png")
            .unwrap();
        t.gen_texture_mipmaps();
        t.unwrap()
    };

    let materials = model.materials_mut();
    let material = &mut materials[0];

    // Assign shader to model materials
    material.shader = *shader.as_ref();

    // Assign loaded texture to material albedo map
    let maps = material.maps_mut();
    maps[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = texture;

    let model_position = Vector3::new(0.0, 0.0, 0.0);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

        rl.start_drawing(&thread, |mut drawing| {
            drawing.clear_background(Color::WHITE);
            drawing.start_mode3D(camera, |mut mode_3d, _camera| {
                mode_3d.draw_model(&model, model_position, 0.2, Color::WHITE);
                mode_3d.draw_grid(10, 1.0);
            });

            drawing.draw_text(
                "(c) Watermill 3D model by Alberto Cano",
                w - 210,
                h - 20,
                10,
                Color::GRAY,
            );
            drawing.draw_fps(10, 10)
        });
    }
}
