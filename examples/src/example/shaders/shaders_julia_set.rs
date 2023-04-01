/*******************************************************************************************
*
*   raylib [shaders] example - julia sets
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3).
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by eggmund (@eggmund) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 eggmund (@eggmund) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: usize = 330;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: usize = 100;

// A few good julia sets
const POINTS_OF_INTEREST: &[Vector2] = &[
    Vector2::new(-0.348827, 0.607167),
    Vector2::new(-0.786268, 0.169728),
    Vector2::new(-0.8, 0.156),
    Vector2::new(0.285, 0.0),
    Vector2::new(-0.835, -0.2321),
    Vector2::new(-0.70176, -0.3842),
];

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - julia sets");

    // Load julia set shader
    // NOTE: Defining 0 (NULL) for vertex shader forces usage of internal default vertex shader
    let mut shader = rl
        .load_shader(
            thread,
            None,
            Some(&format!(
                "original/shaders/resources/shaders/glsl{}/julia_set.fs",
                GLSL_VERSION
            )),
        )
        .unwrap();

    // c constant to use in z^2 + c
    let mut c = POINTS_OF_INTEREST[0];

    // Offset and zoom to draw the julia set at. (centered on screen and default size)
    let mut offset = rvec2(screen_width as f32 / 2.0, screen_height as f32 / 2.0);
    let mut zoom = 1.0;

    let mut offsetSpeed = rvec2(0.0, 0.0);

    // Get variable (uniform) locations on the shader to connect with the program
    // NOTE: If uniform variable could not be found in the shader, function returns -1
    let cLoc = shader.get_shader_location("c");
    let zoomLoc = shader.get_shader_location("zoom");
    let offsetLoc = shader.get_shader_location("offset");

    // Tell the shader what the screen dimensions, zoom, offset and c are
    let screenDims = rvec2(screen_width, screen_height);
    shader.set_shader_value(shader.get_shader_location("screenDims"), screenDims);

    shader.set_shader_value(cLoc, c);
    shader.set_shader_value(zoomLoc, zoom);
    shader.set_shader_value(offsetLoc, offset);

    // Create a RenderTexture2D to be used for render to texture
    let mut target = rl
        .load_render_texture(thread, screen_width as u32, screen_height as u32)
        .unwrap();

    let mut incrementSpeed = 0.0; // Multiplier of speed to change c value
    let mut showControls = true; // Show controls
    let mut pause = false; // Pause animation

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // Press [1 - 6] to reset c to a point of interest
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ONE) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_TWO) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_THREE) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FOUR) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FIVE) ||
            rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SIX)
        {
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ONE)
                {c = POINTS_OF_INTEREST[0];}
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_TWO)
                {c = POINTS_OF_INTEREST[1];}
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_THREE)
                {c = POINTS_OF_INTEREST[2];}
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FOUR)
                {c = POINTS_OF_INTEREST[3];}
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_FIVE)
               { c = POINTS_OF_INTEREST[4];}
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SIX)
               { c = POINTS_OF_INTEREST[5];}

            shader.set_shader_value( cLoc, c);
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
            {pause = !pause; // Pause animation (c change)
            }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_F1)
            {showControls = !showControls; // Toggle whether or not to show controls
            }

        if !pause
        {
            if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT)
                {incrementSpeed+=1.0;}
            else if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT)
                {incrementSpeed-=1.0;}

            // TODO: The idea is to zoom and move around with mouse
            // Probably offset movement should be proportional to zoom level
            if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) || rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_RIGHT)
            {
                if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT)
                   { zoom += zoom * 0.003;}
                if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_RIGHT)
                   { zoom -= zoom * 0.003;}

                let mousePos = rl.get_mouse_position();

                offsetSpeed.x = mousePos.x - screen_width as f32 / 2.0;
                offsetSpeed.y = mousePos.y - screen_height as f32 / 2.0;

                // Slowly move camera to targetOffset
                offset.x += rl.get_frame_time() * offsetSpeed.x * 0.8;
                offset.y += rl.get_frame_time() * offsetSpeed.y * 0.8;
            }
            else
               { offsetSpeed = rvec2(0.0, 0.0);
}
            shader.set_shader_value( zoomLoc, zoom);
            shader.set_shader_value( offsetLoc, offset);

            // Increment c value with time
            let amount = rl.get_frame_time() as f32 * incrementSpeed * 0.0005;
            c.x += amount;
            c.y += amount;

            shader.set_shader_value( cLoc, c);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::BLACK); // Clear the screen of the previous frame.

        {// Using a render texture to draw Julia set
        let mut d = d.begin_texture_mode(thread, &mut target); // Enable drawing to texture
        d.clear_background(Color::BLACK);   // Clear the render texture

        // Draw a rectangle in shader mode to be used as shader canvas
        // NOTE: Rectangle uses font white character texture coordinates,
        // so shader can not be applied here directly because input vertexTexCoord
        // do not represent full screen coordinates (space where want to apply shader)
        d.draw_rectangle(0, 0, d.get_screen_width(), d.get_screen_height(), Color::BLACK);
}
       { // Draw the saved texture and rendered julia set with shader
        // NOTE: We do not invert texture on Y, already considered inside shader
        let mut d = d.begin_shader_mode(&shader);
        d.draw_texture(target.texture(), 0, 0, Color::WHITE);
}
        if showControls
        {
            d.draw_text("Press Mouse buttons right/left to zoom in/out and move", 10, 15, 10, Color::RAYWHITE);
            d.draw_text("Press KEY_F1 to toggle these controls", 10, 30, 10, Color::RAYWHITE);
            d.draw_text("Press KEYS [1 - 6] to change point of interest", 10, 45, 10, Color::RAYWHITE);
            d.draw_text("Press KEY_LEFT | KEY_RIGHT to change speed", 10, 60, 10, Color::RAYWHITE);
            d.draw_text("Press KEY_SPACE to pause movement animation", 10, 75, 10, Color::RAYWHITE);
        }

        //----------------------------------------------------------------------------------
    },
    );

    // // De-Initialization
    // //--------------------------------------------------------------------------------------
    // UnloadShader(shader);        // Unload shader
    // UnloadRenderTexture(target); // Unload render texture

    // CloseWindow(); // Close window and OpenGL context
    // //--------------------------------------------------------------------------------------

    // return 0;
}
