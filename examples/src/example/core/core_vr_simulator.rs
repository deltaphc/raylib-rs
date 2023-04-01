/*******************************************************************************************
*
*   raylib [core] example - VR Simulator (Oculus Rift CV1 parameters)
*
*   This example has been created using raylib 1.7 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2017 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    // NOTE: screen_width/screen_height should match VR device aspect ratio

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [core] example - vr simulator");


    // VrDeviceInfo hmd = {0}; // VR device parameters (head-mounted-device)

    #[allow(non_snake_case)]
    let lensDistortionValues = [
        1.0,  // HMD lens distortion constant parameter 0
        0.22, // HMD lens distortion constant parameter 1
        0.24, // HMD lens distortion constant parameter 2
        0.0,  // HMD lens distortion constant parameter 3
    ];

    #[allow(non_snake_case)]
    let chromaAbCorrection = [
        0.996,  // HMD chromatic aberration correction parameter 0
        -0.004, // HMD chromatic aberration correction parameter 1
        1.014,  // HMD chromatic aberration correction parameter 2
        0.0,    // HMD chromatic aberration correction parameter 3
    ];

    let device = raylib::ffi::VrDeviceInfo {
        // Oculus Rift CV1 parameters for simulator
        hResolution: 2160,            // HMD horizontal resolution in pixels
        vResolution: 1200,            // HMD vertical resolution in pixels
        hScreenSize: 0.133793,        // HMD horizontal size in meters
        vScreenSize: 0.0669,          // HMD vertical size in meters
        vScreenCenter: 0.04678,       // HMD screen center in meters
        eyeToScreenDistance: 0.041,   // HMD distance between eye and display in meters
        lensSeparationDistance: 0.07, // HMD lens separation distance in meters
        interpupillaryDistance: 0.07, // HMD IPD (distance between pupils) in meters

        // NOTE: CV1 uses a Fresnel-hybrid-asymmetric lenses with specific distortion compute shaders.
        // Following parameters are an approximation to distortion stereo rendering but results differ from actual device.
        lensDistortionValues,
        chromaAbCorrection,
    };

    let mut config = rl.load_vr_stereo_config(thread, device.clone()); // Set Vr device parameters for stereo rendering


    // Distortion shader (uses device lens distortion and chroma)
    let mut distortion = rl
        .load_shader(
            thread,
            None,
            Some(&format!("original/core/resources/distortion{}.fs", GLSL_VERSION)),
        )
        .unwrap();

// Update distortion shader with lens and distortion-scale parameters
distortion.set_shader_value( distortion.get_shader_location( "leftLensCenter"),
config.leftLensCenter);
distortion.set_shader_value( distortion.get_shader_location( "rightLensCenter"),
config.rightLensCenter);
distortion.set_shader_value( distortion.get_shader_location( "leftScreenCenter"),
config.leftScreenCenter);
distortion.set_shader_value( distortion.get_shader_location( "rightScreenCenter"),
config.rightScreenCenter);

distortion.set_shader_value( distortion.get_shader_location( "scale"),
config.scale);
distortion.set_shader_value( distortion.get_shader_location( "scaleIn"),
config.scaleIn);
distortion.set_shader_value( distortion.get_shader_location( "deviceWarpParam"),
device.lensDistortionValues);
distortion.set_shader_value( distortion.get_shader_location( "chromaAbParam"),
device.chromaAbCorrection);

        let mut target = rl.load_render_texture(thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32).expect("couldn't make render texture");

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(5.0, 2.0, 5.0), // Camera position
        rvec3(0.0, 2.0, 0.0), // Camera looking at point
        rvec3(0.0, 1.0, 0.0), // Camera up vector (rotation towards target)
        60.0,                 // Camera field-of-view Y
    ); // Camera type

    let cube_position = Vector3::zero();


    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_FIRST_PERSON); // Set first person camera mode

    rl.set_target_fps(90); // Set our game to run at 90 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera (simulator mode)

        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::WHITE);

        {
            let mut d = d.begin_texture_mode(thread, &mut target);
                d.clear_background(Color::WHITE);
                    let mut d = d.begin_vr_stereo_mode(&mut config);
                        let mut d = d.begin_mode3D(camera);
        
                            d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
                            d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
                            d.draw_grid(40, 1.0);
        

        }
        {

            let mut d = d.begin_shader_mode(&distortion);
                d.draw_texture_rec(target.texture(), rrect( 0, 0, target.texture().width,
                              -target.texture().height ), rvec2( 0.0, 0.0 ), Color::WHITE);
        }

        d.draw_fps(10, 10);
    }
    );
}
