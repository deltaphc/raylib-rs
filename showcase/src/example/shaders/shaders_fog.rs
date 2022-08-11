/*******************************************************************************************
*
*   raylib [shaders] example - fog
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3).
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Camacho (@codifies) and reviewed by Ramon Santamaria (@raysan5)
*
*   Chris Camacho (@codifies -  http://bedroomcoders.co.uk/) notes:
*
*   This is based on the PBR lighting example, but greatly simplified to aid learning...
*   actually there is very little of the PBR example left!
*   When I first looked at the bewildering complexity of the PBR example I feared
*   I would never understand how I could do simple lighting with raylib however its
*   a testement to the authors of raylib (including rlights.h) that the example
*   came together fairly quickly.
*
*   Copyright (c) 2019 Chris Camacho (@codifies) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - fog");

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(2.0, 2.0, 6.0), // position
        rvec3(0.0, 0.5, 0.0), // target
        rvec3(0.0, 1.0, 0.0), // up
        45.0,
    ); // fov, type

    // Load models and texture
    let mut modelA = unsafe {
        rl.load_model_from_mesh(
            thread,
            Mesh::gen_mesh_torus(thread, 0.4, 1.0, 16, 32).make_weak(),
        )
        .unwrap()
    };
    let mut modelB = unsafe {
        rl.load_model_from_mesh(
            thread,
            Mesh::gen_mesh_cube(thread, 1.0, 1.0, 1.0).make_weak(),
        )
        .unwrap()
    };
    let mut modelC = unsafe {
        rl.load_model_from_mesh(
            thread,
            Mesh::gen_mesh_sphere(thread, 0.5, 32, 32).make_weak(),
        )
        .unwrap()
    };
    let texture = rl
        .load_texture(thread, "original/shaders/resources/texel_checker.png")
        .unwrap();

    // Assign texture to default model material
    modelA.materials_mut()[0].maps_mut()
        [raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *texture.as_ref();
    modelB.materials_mut()[0].maps_mut()
        [raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *texture.as_ref();
    modelC.materials_mut()[0].maps_mut()
        [raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *texture.as_ref();

    // Load shader and set up some uniforms
    let mut shader = rl
        .load_shader(
            thread,
            Some(&format!(
                "original/shaders/resources/shaders/glsl{}/base_lighting.vs",
                GLSL_VERSION
            )),
            Some(&format!(
                "original/shaders/resources/shaders/glsl{}/fog.fs",
                GLSL_VERSION
            )),
        )
        .unwrap();
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
        shader.get_shader_location("matModel");
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
        shader.get_shader_location("viewPos");

    // Ambient light level
    let ambientLoc = shader.get_shader_location("ambient");
    shader.set_shader_value(ambientLoc, Vector4::new(0.2, 0.2, 0.2, 1.0));

    let mut fogDensity = 0.15;
    let fogDensityLoc = shader.get_shader_location("fogDensity");
    shader.set_shader_value(fogDensityLoc, fogDensity);

    // NOTE: All models share the same shader
    modelA.materials_mut()[0].shader = *shader.as_ref();
    modelB.materials_mut()[0].shader = *shader.as_ref();
    modelC.materials_mut()[0].shader = *shader.as_ref();

    // Using just 1 point lights
    create_light(
        LightType::LightPoint,
        rvec3(0, 2, 6),
        Vector3::zero(),
        Color::WHITE,
        &mut shader,
    );

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
        {
            fogDensity += 0.001;
            if fogDensity > 1.0
                {fogDensity = 1.0;}
        }

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
        {
            fogDensity -= 0.001;
            if fogDensity < 0.0
                {fogDensity = 0.0;}
        }

        shader.set_shader_value( fogDensityLoc, fogDensity);

        // Rotate the torus
        modelA.set_transform(&(*modelA.transform() * Matrix::rotate_x(-0.025)));
        modelA.set_transform(&(*modelA.transform() * Matrix::rotate_z(0.012)));

        // Update the light shader with the camera view position
        let loc = shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize];
        shader.set_shader_value( loc, camera.position);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::GRAY);
{
        let mut d = d.begin_mode3D(&camera);

        // Draw the three models
        d.draw_model(&modelA, Vector3::zero(), 1.0, Color::WHITE);
        d.draw_model(&modelB, rvec3(-2.6, 0,  0), 1.0, Color::WHITE);
        d.draw_model(&modelC, rvec3(2.6, 0,  0), 1.0, Color::WHITE);

        for  i in (-20..20).step_by(2){

            d.draw_model(&modelA, rvec3(i, 0,  2), 1.0, Color::WHITE);
        }

    }

        d.draw_text(&format!("Use KEY_UP/KEY_DOWN to change fog density [{:.2}]", fogDensity), 10, 10, 20, Color::RAYWHITE);

        //----------------------------------------------------------------------------------
    },
    );

    // // De-Initialization
    // //--------------------------------------------------------------------------------------
    // UnloadModel(modelA);    // Unload the model A
    // UnloadModel(modelB);    // Unload the model B
    // UnloadModel(modelC);    // Unload the model C
    // UnloadTexture(texture); // Unload the texture
    // UnloadShader(shader);   // Unload shader

    // CloseWindow(); // Close window and OpenGL context
    // //--------------------------------------------------------------------------------------

    // return 0;
}

const MAX_LIGHTS: u32 = 4;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LightType {
    LightDirectional = 0,
    LightPoint = 1,
}

impl Default for LightType {
    fn default() -> Self {
        Self::LightDirectional
    }
}

#[derive(Debug, Default, Clone)]
pub struct Light {
    pub enabled: bool,
    pub light_type: LightType,
    pub position: Vector3,
    pub target: Vector3,
    pub color: Color,
    pub enabled_loc: i32,
    pub type_loc: i32,
    pub pos_loc: i32,
    pub target_loc: i32,
    pub color_loc: i32,
}

static mut LIGHTS_COUNT: i32 = 0;

// Defines a light and get locations from PBR shader
pub fn create_light(
    light_type: LightType,
    pos: Vector3,
    targ: Vector3,
    color: Color,
    shader: &mut Shader,
) -> Light {
    let mut light = Light::default();

    if (unsafe { LIGHTS_COUNT as u32 } < MAX_LIGHTS) {
        light.enabled = true;
        light.light_type = light_type;
        light.position = pos.clone();
        light.target = targ.clone();
        light.color = color.clone();

        // TODO: Below code doesn't look good to me,
        // it assumes a specific shader naming and structure
        // Probably this implementation could be improved

        let lights_count = unsafe { LIGHTS_COUNT };
        let enabledName = format!("lights[{}].enabled", lights_count);
        let typeName = format!("lights[{}].type", lights_count);
        let posName = format!("lights[{}].position", lights_count);
        let targetName = format!("lights[{}].target", lights_count);
        let colorName = format!("lights[{}].color", lights_count);

        // Set location name [x] depending on lights count

        light.enabled_loc = shader.get_shader_location(&enabledName);
        light.type_loc = shader.get_shader_location(&typeName);
        light.pos_loc = shader.get_shader_location(&posName);
        light.target_loc = shader.get_shader_location(&targetName);
        light.color_loc = shader.get_shader_location(&colorName);

        update_light_values(shader, light.clone());
        unsafe {
            LIGHTS_COUNT += 1;
        }
    }

    return light;
}

pub fn update_light_values(shader: &mut Shader, light: Light) {
    // Send to shader light enabled state and type
    shader.set_shader_value(light.enabled_loc, light.enabled as i32);
    shader.set_shader_value(light.type_loc, light.light_type as i32);

    // Send to shader light position values
    shader.set_shader_value(light.pos_loc, light.position);

    // Send to shader light target position values
    shader.set_shader_value(light.target_loc, light.target);

    // Send to shader light color values

    let color: Vector4 = light.color.into();
    shader.set_shader_value(light.color_loc, color);
}
