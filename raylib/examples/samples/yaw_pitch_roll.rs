extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let mut opt = options::Opt::from_args();
    opt.width = 800;
    opt.height = 450;
    let (mut rl, thread) = opt.open_window("Yaw Pitch Roll");
    let (screen_width, _) = (opt.width, opt.height);
    let ray_white = Color::new(255, 255, 255, 255);

    let tex_angle_gauge = rl.load_texture(&thread, "static/angle_gauge.png").unwrap();
    let tex_background = rl.load_texture(&thread, "static/background.png").unwrap();
    let tex_pitch = rl.load_texture(&thread, "static/pitch.png").unwrap();
    let tex_plane = rl.load_texture(&thread, "static/plane.png").unwrap();

    let mut framebuffer = rl.load_render_texture(&thread, 192, 192).unwrap();

    let mut model = rl.load_model(&thread, "static/plane.obj").unwrap();
    {
        let materials = model.materials_mut();
        let mat = &mut materials[0];
        let mats = mat.maps_mut();
        let texture = unsafe {
            let mut t = rl
                .load_texture(&thread, "static/plane_diffuse.png")
                .unwrap();
            t.gen_texture_mipmaps();
            t.unwrap()
            // Because we are unwraping we are required to manually unload the texture and can't rely on Drop.
            // We don't do that here since we don't need to unload until the end of main anyway.
        };
        mats[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = texture;
    }

    let camera = Camera3D::perspective(
        Vector3::new(0.0, 60.0, -120.0),
        Vector3::new(0.0, 12.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        30.0,
    );
    let mut pitch = 0.0f32;
    let mut roll = 0.0f32;
    let mut yaw = 0.0f32;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Update
        //----------------------------------------------------------------------------------

        // Plane roll (x-axis) controls
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT) {
            roll += 1.0;
        } else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT) {
            roll -= 1.0;
        } else {
            if roll > 0.0 {
                roll -= 0.5;
            } else if roll < 0.0 {
                roll += 0.5;
            }
        }

        // Plane yaw (y-axis) controls
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            yaw += 1.0;
        } else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            yaw -= 1.0;
        } else {
            if yaw > 0.0 {
                yaw -= 0.5;
            } else if yaw < 0.0 {
                yaw += 0.5;
            }
        }

        // Plane pitch (z-axis) controls
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN) {
            pitch += 0.6;
        } else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP) {
            pitch -= 0.6;
        } else {
            if pitch > 0.3 {
                pitch -= 0.3;
            } else if pitch < -0.3 {
                pitch += 0.3;
            }
        }

        // Wraps the phase of an angle to fit between -180 and +180 degrees
        let mut pitch_offset = pitch;
        while pitch_offset > 180.0 {
            pitch_offset -= 360.0;
        }
        while pitch_offset < -180.0 {
            pitch_offset += 360.0;
        }
        pitch_offset *= 10.0;

        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(ray_white);

        let mat = Matrix::rotate_xyz(Vector3::new(
            pitch.to_radians(),
            yaw.to_radians(),
            roll.to_radians(),
        ));

        model.set_transform(&mat);

        // Draw framebuffer texture (Ahrs Display)
        let center_x = (framebuffer.texture().width() / 2) as f32;
        let center_y = (framebuffer.texture().height() / 2) as f32;
        let scale_factor = 0.5;
        {
            let mut d = d.begin_texture_mode(&thread, &mut framebuffer);
            {
                let mut d = d.begin_blend_mode(raylib::consts::BlendMode::BLEND_ALPHA);
                d.draw_texture_pro(
                    &tex_background,
                    Rectangle::new(
                        0.0,
                        0.0,
                        tex_background.width() as f32,
                        tex_background.height() as f32,
                    ),
                    Rectangle::new(
                        center_x,
                        center_y,
                        tex_background.width() as f32 * scale_factor,
                        tex_background.height() as f32 * scale_factor,
                    ),
                    Vector2::new(
                        tex_background.width() as f32 / 2.0 * scale_factor,
                        tex_background.height() as f32 / 2.0 * scale_factor
                            + pitch_offset * scale_factor,
                    ),
                    roll,
                    Color::WHITE,
                );

                d.draw_texture_pro(
                    &tex_pitch,
                    Rectangle::new(
                        0.0,
                        0.0,
                        tex_pitch.width() as f32,
                        tex_pitch.height() as f32,
                    ),
                    Rectangle::new(
                        center_x,
                        center_y,
                        tex_pitch.width() as f32 * scale_factor,
                        tex_pitch.height() as f32 * scale_factor,
                    ),
                    Vector2::new(
                        tex_pitch.width() as f32 / 2.0 * scale_factor,
                        tex_pitch.height() as f32 / 2.0 * scale_factor
                            + pitch_offset * scale_factor,
                    ),
                    roll,
                    Color::WHITE,
                );

                d.draw_texture_pro(
                    &tex_plane,
                    Rectangle::new(
                        0.0,
                        0.0,
                        tex_plane.width() as f32,
                        tex_plane.height() as f32,
                    ),
                    Rectangle::new(
                        center_x,
                        center_y,
                        tex_plane.width() as f32 * scale_factor,
                        tex_plane.height() as f32 * scale_factor,
                    ),
                    Vector2::new(
                        tex_plane.width() as f32 / 2.0 * scale_factor,
                        tex_plane.height() as f32 / 2.0 * scale_factor,
                    ),
                    0.0,
                    Color::WHITE,
                );
            }
        }
        // Draw 3D model (recomended to draw 3D always before 2D)
        {
            let mut d = d.begin_mode3D(camera);

            d.draw_model(&model, Vector3::new(0.0, 6.0, 0.0), 1.0, Color::WHITE); // Draw 3d model with texture
            d.draw_grid(10, 10.0);
        }

        // Draw 2D GUI stuff
        draw_angle_gauge(&mut d, &tex_angle_gauge, 80, 70, roll, "roll", Color::RED);
        draw_angle_gauge(
            &mut d,
            &tex_angle_gauge,
            190,
            70,
            pitch,
            "pitch",
            Color::GREEN,
        );
        draw_angle_gauge(
            &mut d,
            &tex_angle_gauge,
            300,
            70,
            yaw,
            "yaw",
            Color::SKYBLUE,
        );

        d.draw_rectangle(30, 360, 260, 70, Color::SKYBLUE.fade(0.5));
        d.draw_rectangle_lines(30, 360, 260, 70, Color::DARKBLUE.fade(0.5));
        d.draw_text(
            "Pitch controlled with: KEY_UP / KEY_DOWN",
            40,
            370,
            10,
            Color::DARKGRAY,
        );
        d.draw_text(
            "Roll controlled with: KEY_LEFT / KEY_RIGHT",
            40,
            390,
            10,
            Color::DARKGRAY,
        );
        d.draw_text(
            "Yaw controlled with: KEY_A / KEY_S",
            40,
            410,
            10,
            Color::DARKGRAY,
        );

        // Draw framebuffer texture
        d.draw_texture_rec(
            framebuffer.texture(),
            Rectangle::new(
                0.0,
                0.0,
                framebuffer.texture.width as f32,
                -framebuffer.texture.height as f32,
            ),
            Vector2::new(
                screen_width as f32 - framebuffer.texture.width as f32 - 20.0,
                20.0,
            ),
            Color::WHITE.fade(0.8),
        );
    }
}

// Draw angle gauge controls
fn draw_angle_gauge(
    d: &mut RaylibDrawHandle,
    angle_gauge: &Texture2D,
    x: i32,
    y: i32,
    angle: f32,
    title: &str,
    color: Color,
) {
    let src_rec = Rectangle::new(
        0.0,
        0.0,
        angle_gauge.width() as f32,
        angle_gauge.height() as f32,
    );
    let dst_rec = Rectangle::new(
        x as f32,
        y as f32,
        angle_gauge.width() as f32,
        angle_gauge.height() as f32,
    );
    let origin = Vector2::new(
        angle_gauge.width() as f32 / 2.0,
        angle_gauge.height() as f32 / 2.0,
    );
    let text_size = 20;

    d.draw_texture_pro(angle_gauge, src_rec, dst_rec, origin, angle, color);

    d.draw_text(
        &format!("{:5.1}", angle),
        x - d.measure_text(&format!("{:5.1}", angle), text_size) / 2,
        y + 10,
        text_size,
        Color::DARKGRAY,
    );
    d.draw_text(
        title,
        x - d.measure_text(title, text_size) / 2,
        y + 60,
        text_size,
        Color::DARKGRAY,
    );
}
