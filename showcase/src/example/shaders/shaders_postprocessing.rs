/*******************************************************************************************
*
*   raylib [shaders] example - Apply a postprocessing shader to a scene
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3), to test this example
*         on OpenGL ES 2.0 platforms (Android, Raspberry Pi, HTML5), use #version 100 shaders
*         raylib comes with shaders ready for both versions, check raylib/shaders install folder
*
*   This example has been created using raylib 1.3 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Copyright (c) 2015 Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;

const MAX_POSTPRO_SHADERS: usize = 12;

#[repr(usize)]
#[allow(non_camel_case_types)]
enum PostproShader {
    FX_GRAYSCALE = 0,
    FX_POSTERIZATION,
    FX_DREAM_VISION,
    FX_PIXELIZER,
    FX_CROSS_HATCHING,
    FX_CROSS_STITCHING,
    FX_PREDATOR_VIEW,
    FX_SCANLINES,
    FX_FISHEYE,
    FX_SOBEL,
    FX_BLOOM,
    FX_BLUR,
    //FX_FXAA
}

const POST_PRO_SHADER_TEXT: [&'static str; MAX_POSTPRO_SHADERS] = [
    "GRAYSCALE",
    "POSTERIZATION",
    "DREAM_VISION",
    "PIXELIZER",
    "CROSS_HATCHING",
    "CROSS_STITCHING",
    "PREDATOR_VIEW",
    "SCANLINES",
    "FISHEYE",
    "SOBEL",
    "BLOOM",
    "BLUR",
    //"FXAA"
];

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    use PostproShader::*;
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [shaders] example - postprocessing shader");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera::perspective(
        rvec3(2.0, 3.0, 2.0),
        rvec3(0.0, 1.0, 0.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );

    let mut model = rl
        .load_model(thread, "original/shaders/resources/models/church.obj")
        .unwrap(); // Load OBJ model
    let texture = rl
        .load_texture(
            thread,
            "original/shaders/resources/models/church_diffuse.png",
        )
        .unwrap(); // Load model texture (diffuse map)
                   // Raylib-rs assumes that textures aren't shared or used by models can be unloaded on drop.
                   // You convert the Texture2D to WeakTexture2D to stop raylib-rs from unloading a shared texture.
    let texture = unsafe { texture.make_weak() };
    model.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapType::MAP_ALBEDO as usize]
        .texture = *texture.as_ref(); // Set model diffuse texture

    let position = rvec3(0.0, 0.0, 0.0); // Set model position

    // Load all postpro shaders
    // NOTE 1: All postpro shader use the base vertex shader (DEFAULT_VERTEX_SHADER)
    // NOTE 2: We load the correct shader depending on GLSL version

    let shaders = {
        use std::mem::MaybeUninit;
        let mut shaders: [MaybeUninit<Shader>; MAX_POSTPRO_SHADERS] =
            unsafe { MaybeUninit::uninit().assume_init() };

        // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
        shaders[FX_GRAYSCALE as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/grayscale.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_POSTERIZATION as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/posterization.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_DREAM_VISION as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/dream_vision.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_PIXELIZER as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/pixelizer.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_CROSS_HATCHING as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/cross_hatching.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_CROSS_STITCHING as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/cross_stitching.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_PREDATOR_VIEW as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/predator.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_SCANLINES as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/scanlines.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_FISHEYE as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/fisheye.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_SOBEL as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/sobel.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        shaders[FX_BLOOM as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/bloom.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );
        // "original/shaders/resources/shaders/glsl330/blur.fs";
        shaders[FX_BLUR as usize] = MaybeUninit::new(
            rl.load_shader(
                thread,
                None,
                Some(&format!(
                    "original/shaders/resources/shaders/glsl{}/blur.fs",
                    GLSL_VERSION
                )),
            )
            .unwrap(),
        );

        unsafe { std::mem::transmute::<_, [Shader; MAX_POSTPRO_SHADERS]>(shaders) }
    };

    let mut current_shader = FX_GRAYSCALE as isize;

    // Create a RenderTexture2D to be used for render to texture
    let mut target = rl
        .load_render_texture(thread, screen_width as u32, screen_height as u32)
        .unwrap();

    // Setup orbital camera
    rl.set_camera_mode(camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () {
        use raylib::consts::KeyboardKey::*;
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        if rl.is_key_pressed(KEY_RIGHT) {
            current_shader += 1;
        } else if rl.is_key_pressed(KEY_LEFT) {
            current_shader -= 1;
        }

        if current_shader >= MAX_POSTPRO_SHADERS as isize {
            current_shader = 0;
        } else if current_shader < 0 {
            current_shader = MAX_POSTPRO_SHADERS as isize - 1;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        {
            let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);
            {
                let mut d = d.begin_texture_mode(&mut target); // Enable drawing to texture

                d.clear_background(Color::RAYWHITE); // Clear texture background
                {
                    let mut d = d.begin_mode3D(&camera); // Begin 3d mode drawing

                    d.draw_model(&model, position, 0.1, Color::WHITE); // Draw 3d model with texture

                    d.draw_grid(10, 1.0); // Draw a grid
                }
            }

            {
                // Render previously generated texture using selected postpro shader
                let mut d = d.begin_shader_mode(&shaders[current_shader as usize]);

                // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
                d.draw_texture_rec(
                    target.texture(),
                    rrect(0, 0, target.texture.width, -target.texture.height),
                    rvec2(0, 0),
                    Color::WHITE,
                );
            }

            // Draw 2d shapes and text over drawn texture
            d.draw_rectangle(0, 9, 580, 30, Color::LIGHTGRAY.fade(0.7));

            d.draw_text(
                "(c) Church 3D model by Alberto Cano",
                screen_width - 200,
                screen_height - 20,
                10,
                Color::GRAY,
            );

            d.draw_text("CURRENT POSTPRO SHADER:", 10, 15, 20, Color::BLACK);
            d.draw_text(
                POST_PRO_SHADER_TEXT[current_shader as usize],
                330,
                15,
                20,
                Color::RED,
            );
            d.draw_text("< >", 540, 10, 30, Color::DARKBLUE);

            d.draw_fps(700, 15);

            //----------------------------------------------------------------------------------
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
            unsafe {
                rl.unload_texture(thread, texture.clone());
            }
        }
    });
}
