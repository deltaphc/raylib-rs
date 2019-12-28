use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Model shader example")
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 4.0, 4.0),  // Position
        Vector3::new(0.0, 1.0, -1.0), // Target
        Vector3::new(0.0, 1.0, 0.0),  // Up vector
        45.0                          // FOV
    );

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);
    rl.set_target_fps(60);

    // Load shader
    let shader = rl.load_shader(&thread, None, Some("static/model_shader/grayscale.fs")).unwrap();

    // Load model
    let mut model = rl.load_model(&thread, "static/model_shader/watermill.obj").unwrap();

    // Load texture and generate mipmaps
    let texture = unsafe {
        let mut t = rl.load_texture(&thread, "static/model_shader/watermill_diffuse.png").unwrap();
        t.gen_texture_mipmaps();
        t.unwrap()
    };

    let materials = model.materials_mut();
    let material = &mut materials[0];

    // Assign shader to model materials
    material.shader = *shader.as_ref();

    // Assign loaded texture to material albedo map
    let mut maps = material.maps_mut();
    maps[MaterialMapType::MAP_ALBEDO as usize].texture = texture;

    let model_position = Vector3::new(0.0, 0.0, 0.0);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        let mut drawing = rl.begin_drawing(&thread);
        drawing.clear_background(Color::WHITE);
        {
            let mut mode_3d = drawing.begin_mode_3D(camera);

            mode_3d.draw_model(&model, model_position, 0.2, Color::WHITE);
            mode_3d.draw_grid(10, 1.0);
        }

        drawing.draw_text("(c) Watermill 3D model by Alberto Cano", WINDOW_WIDTH - 210, WINDOW_HEIGHT - 20, 10, Color::GRAY);
        drawing.draw_fps(10, 10)
    }
}