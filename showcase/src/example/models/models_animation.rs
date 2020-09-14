/*******************************************************************************************
*
*   raylib [models] example - Load 3d model with animations and play them
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Culacant (@culacant) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Culacant (@culacant) and Ramon Santamaria (@raysan5)
*
********************************************************************************************
*
* To export a model from blender, make sure it is not posed, the vertices need to be in the
* same position as they would be in edit mode.
* and that the scale of your models is set to 0. Scaling can be done from the export menu.
*
********************************************************************************************/

use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [models] example - model animation");

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(10.0, 10.0, 10.0),
        rvec3(0.0, 0.0, 0.0),
        Vector3::up(),
        45.0,
    );

    let mut model = rl
        .load_model(thread, "original/models/resources/guy/guy.iqm")
        .unwrap(); // Load the animated model mesh and basic data
    let texture = rl
        .load_texture(thread, "original/models/resources/guy/guytex.png")
        .unwrap(); // Load model texture and set material
    model.materials_mut()[0]
        .set_material_texture(raylib::consts::MaterialMapType::MAP_ALBEDO, &texture);

    let position = rvec3(0.0, 0.0, 0.0); // Set model position

    // Load animation data
    let anims = rl
        .load_model_animations(thread, "original/models/resources/guy/guyanim.iqm")
        .unwrap();
    let mut anim_frame_counter = 0;

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FREE);

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        // If we don't capture texture, it won't get moved to the closure which means raylib will drop it.
        let _ = texture;

        
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);

        // Play animation when spacebar is held down
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
            anim_frame_counter += 1;
            rl.update_model_animation(thread, &mut model, &anims[0], anim_frame_counter);
            if anim_frame_counter >= anims[0].frameCount {
                anim_frame_counter = 0;
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(&camera);

            d.draw_model_ex(
                &model,
                position,
                rvec3(1.0, 0.0, 0.0),
                -90.0,
                rvec3(1.0, 1.0, 1.0),
                Color::WHITE,
            );

            for i in 0..model.bones().map(|b| b.len()).unwrap() {
                d.draw_cube(
                    anims[0].frame_poses()[anim_frame_counter as usize][i].translation,
                    0.2,
                    0.2,
                    0.2,
                    Color::RED,
                );
            }

            d.draw_grid(10, 1.0); // Draw a grid
        }

        d.draw_text(
            "PRESS SPACE to PLAY MODEL ANIMATION",
            10,
            10,
            20,
            Color::MAROON,
        );
        d.draw_text(
            "(c) Guy IQM 3D model by @culacant",
            screen_width - 200,
            screen_height - 20,
            10,
            Color::GRAY,
        );

        //----------------------------------------------------------------------------------
    });
}
