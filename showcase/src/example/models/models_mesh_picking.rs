/*******************************************************************************************
*
*   raylib [models] example - Mesh picking in 3d mode, ground plane, triangle, mesh
*
*   This example has been created using raylib 1.7 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Joel Davis (@joeld42) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2017 Joel Davis (@joeld42) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - mesh picking");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
     rvec3( 20.0, 20.0,20.0 ), // Camera position
     rvec3( 0.0, 8.0,0.0 ),      // Camera looking at point
    rvec3( 0.0, 1.6,0.0 ),          // Camera up vector (rotation towards target)
     45.0,                                // Camera field-of-view Y
    );

    let mut ray = Ray::default();        // Picking ray

   let mut tower  = rl.load_model(thread, "original/model/resources/models/turret.obj").unwrap();                 // Load OBJ model
    let texture = rl.load_texture(thread, "original/models/resources/models/turret_diffuse.png").unwrap(); // Load model texture
    tower.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref();                 // Set model diffuse texture

    let  towerPos = rvec3( 0.0, 0.0, 0.0 );                    // Set model position
    let  towerBBox = tower.meshes_mut()[0].get_mesh_bounding_box();   // Get mesh bounding box
    let mut hitMeshBBox = false;
    let mut hitTriangle = false;

    // Ground quad
    let g0 = rvec3( -50.0, 0.0, -50.0 );
    let g1 = rvec3( -50.0, 0.0,  50.0 );
    let g2 = rvec3(  50.0, 0.0,  50.0 );
    let g3 = rvec3(  50.0, 0.0, -50.0 );

    // Test triangle
    let ta = rvec3( -25.0, 0.5,0.0 );
    let tb = rvec3( -4.0, 2.5,1.0 );
    let tc = rvec3( -8.0, 6.5,0.0 );

    let mut bary = rvec3( 0.0, 0.0, 0.0 );

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FREE); // Set a free camera mode

    rl.set_target_fps(60);                   // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------
    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()        // Detect window close button or ESC key
    {
        let _ = texture;
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);          // Update camera

        // Display information about closest hit
        let mut nearestHit = RayCollision::default();
        let mut hitObjectName = "None";
        nearestHit.distance = std::f32::MAX;
        nearestHit.hit = false;
        let mut cursorColor = Color::WHITE;

        // Get ray and test against ground, triangle, and mesh
        ray = rl.get_mouse_ray(rl.get_mouse_position(), camera);

        // Check ray collision aginst ground plane
        let groundHitInfo = get_ray_collision_quad(ray, g0, g1, g2, g3);

        if ((groundHitInfo.hit) && (groundHitInfo.distance < nearestHit.distance))
        {
            nearestHit = groundHitInfo;
            cursorColor = Color::GREEN;
            hitObjectName = "Ground";
        }

        // Check ray collision against test triangle
        let triHitInfo = get_ray_collision_triangle(ray, ta, tb, tc);

        if ((triHitInfo.hit) && (triHitInfo.distance < nearestHit.distance))
        {
            nearestHit = triHitInfo;
            cursorColor = Color::PURPLE;
            hitObjectName = "Triangle";

            bary = nearestHit.point.barycenter( ta, tb, tc);
            hitTriangle = true;
        }
        else {hitTriangle = false;}

        let mut meshHitInfo = RayCollision::default();

        // Check ray collision against bounding box first, before trying the full ray-mesh test
        if (towerBBox.get_ray_collision_box(ray).hit)
        {
            hitMeshBBox = true;

            // Check ray collision against model
            // NOTE: It considers model.transform matrix!
            meshHitInfo = get_ray_collision_model(ray, &tower);

            if ((meshHitInfo.hit) && (meshHitInfo.distance < nearestHit.distance))
            {
                nearestHit = meshHitInfo;
                cursorColor = Color::ORANGE;
                hitObjectName = "Mesh";
            }
        }

        hitMeshBBox = false;
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {

                let mut d = d.begin_mode3D(&camera);
    
                    // Draw the tower
                    // WARNING: If scale is different than 1.0,
                    // not considered by GetCollisionRayModel()
                    d.draw_model(&tower, towerPos, 1.0, Color::WHITE);
    
                    // Draw the test triangle
                    d.draw_line_3D(ta, tb, Color::PURPLE);
                    d.draw_line_3D(tb, tc, Color::PURPLE);
                    d.draw_line_3D(tc, ta, Color::PURPLE);
    
                    // Draw the mesh bbox if we hit it
                    if (hitMeshBBox){ d.draw_bounding_box(towerBBox, Color::LIME);}
    
                    // If we hit something, draw the cursor at the hit point
                    if (nearestHit.hit)
                    {
                        d.draw_cube(nearestHit.point, 0.3, 0.3, 0.3, cursorColor);
                        d.draw_cube_wires(nearestHit.point, 0.3, 0.3, 0.3, Color::RED);
    
                        let mut normalEnd = Vector3::default();
                        normalEnd.x = nearestHit.point.x + nearestHit.normal.x;
                        normalEnd.y = nearestHit.point.y + nearestHit.normal.y;
                        normalEnd.z = nearestHit.point.z + nearestHit.normal.z;
    
                        d.draw_line_3D(nearestHit.point, normalEnd, Color::RED);
                    }
    
                    d.draw_ray(ray, Color::MAROON);
    
                    d.draw_grid(10, 10.0);
    
            }

            // Draw some debug GUI text
            d.draw_text(&format!("Hit Object: {}", hitObjectName), 10, 50, 10, Color::BLACK);

            if (nearestHit.hit)
            {
                let ypos = 70;

                d.draw_text(&format!("Distance: {:3.2}", nearestHit.distance), 10, ypos, 10, Color::BLACK);

                d.draw_text(&format!("Hit Point: {:3.2} {:3.2} {:3.2}",
                                    nearestHit.point.x,
                                    nearestHit.point.y,
                                    nearestHit.point.z), 10, ypos + 15, 10, Color::BLACK);

                d.draw_text(&format!("Hit Norm: {:3.2} {:3.2} {:3.2}",
                                    nearestHit.normal.x,
                                    nearestHit.normal.y,
                                    nearestHit.normal.z), 10, ypos + 30, 10, Color::BLACK);

                if (hitTriangle){ d.draw_text(&format!("Barycenter: {:3.2} {:3.2} {:3.2}",  bary.x, bary.y, bary.z), 10, ypos + 45, 10, Color::BLACK);}
            }

            d.draw_text("Use Mouse to Move Camera", 10, 430, 10, Color::GRAY);

            d.draw_text("(c) Turret 3D model by Alberto Cano", screen_width - 200, screen_height - 20, 10, Color::GRAY);

            d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    });
}