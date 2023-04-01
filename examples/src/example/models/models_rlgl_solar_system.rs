/*******************************************************************************************
*
*   raylib [models] example - rlgl module usage with push/pop matrix transformations
*
*   This example uses [rlgl] module funtionality (pseudo-OpenGL 1.1 style coding)
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2018 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;
use raylib::ffi;


//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    let sunRadius = 4.0;
    let earthRadius = 0.6;
    let earthOrbitRadius = 8.0;
    let moonRadius = 0.16;
    let moonOrbitRadius = 1.5;

    rl.set_window_title(thread, "raylib [models] example - rlgl module usage with push/pop matrix transformations");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
     rvec3( 16.0, 16.0,16.0 ),
     rvec3( 0.0, 0.0,0.0 ),
     rvec3( 0.0, 1.0,0.0 ),
     45.0,
    );

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FREE);

    let rotationSpeed = 0.2;         // General system rotation speed

    let mut earthRotation = 0.0;         // Rotation of earth around itself (days) in degrees
    let mut earthOrbitRotation = 0.0;    // Rotation of earth around the Sun (years) in degrees
    let mut moonRotation = 0.0;          // Rotation of moon around itself
    let mut moonOrbitRotation = 0.0;     // Rotation of moon around earth in degrees

    rl.set_target_fps(60);                   // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()        // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera);

        earthRotation += (5.0*rotationSpeed);
        earthOrbitRotation += (365.0/360.0*(5.0*rotationSpeed)*rotationSpeed);
        moonRotation += (2.0*rotationSpeed);
        moonOrbitRotation += (8.0*rotationSpeed);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {
                let mut d = d.begin_mode3D(&camera);

                unsafe {
                    ffi::rlPushMatrix();
                        ffi::rlScalef(sunRadius, sunRadius, sunRadius);          // Scale Sun
                        DrawSphereBasic(Color::GOLD);                              // Draw the Sun
                    ffi::rlPopMatrix();

                    ffi::rlPushMatrix();
                        ffi::rlRotatef(earthOrbitRotation, 0.0, 1.0, 0.0);    // Rotation for Earth orbit around Sun
                        ffi::rlTranslatef(earthOrbitRadius, 0.0, 0.0);         // Translation for Earth orbit
                        ffi::rlRotatef(-earthOrbitRotation, 0.0, 1.0, 0.0);   // Rotation for Earth orbit around Sun inverted

                        ffi::rlPushMatrix();
                            ffi::rlRotatef(earthRotation, 0.25, 1.0, 0.0);       // Rotation for Earth itself
                            ffi::rlScalef(earthRadius, earthRadius, earthRadius);// Scale Earth

                            DrawSphereBasic(Color::BLUE);                          // Draw the Earth
                        ffi::rlPopMatrix();

                        ffi::rlRotatef(moonOrbitRotation, 0.0, 1.0, 0.0);     // Rotation for Moon orbit around Earth
                        ffi::rlTranslatef(moonOrbitRadius, 0.0, 0.0);          // Translation for Moon orbit
                        ffi::rlRotatef(-moonOrbitRotation, 0.0, 1.0, 0.0);    // Rotation for Moon orbit around Earth inverted
                        ffi::rlRotatef(moonRotation, 0.0, 1.0, 0.0);          // Rotation for Moon itself
                        ffi::rlScalef(moonRadius, moonRadius, moonRadius);       // Scale Moon

                        DrawSphereBasic(Color::LIGHTGRAY);                         // Draw the Moon
                    ffi::rlPopMatrix();

                }
    
                    // Some reference elements (not affected by previous matrix transformations)
                    d.draw_circle_3D(rvec3( 0.0, 0.0, 0.0 ), earthOrbitRadius, rvec3( 1, 0,0 ), 90.0, Color::RED.fade( 0.5));
                    d.draw_grid(20, 1.0);
    
            }

            d.draw_text("EARTH ORBITING AROUND THE SUN!", 400, 10, 20, Color::MAROON);
            d.draw_fps(10, 10);

    });
}

//--------------------------------------------------------------------------------------------
// Module Functions Definitions (local)
//--------------------------------------------------------------------------------------------

// Draw sphere without any matrix transformation
// NOTE: Sphere is drawn in woffi::rld position ( 0, 0, 0 ) with radius 1.0
fn DrawSphereBasic(color: Color)
{
    let rings = 16;
    let slices = 16;

    unsafe {
        ffi::rlCheckRenderBatchLimit((rings + 2) * slices * 6);

        ffi::rlBegin(ffi::RL_TRIANGLES as i32);
            ffi::rlColor4ub(color.r, color.g, color.b, color.a);

            for i in 0..(rings + 2)
            {
                for j in 0..slices
                {
                    let deg2rad: f32 = consts::DEG2RAD as f32;

                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*(j*360/slices) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*i) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*(j*360/slices) as f32).cos());
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).cos());
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*(j*360/slices) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*(j*360/slices) as f32).cos());

                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*(j*360/slices) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*i) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*(j*360/slices) as f32).cos());
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i)) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).cos());
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).sin(),
                                    (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).cos());
                }
            }
        ffi::rlEnd();
    }

}
