use raylib::prelude::*;

const CUBEMAP_SIZE: i32 = 512; // Cubemap texture size
const IRRADIANCE_SIZE: i32 = 32; // Irradiance texture size
const PREFILTERED_SIZE: i32 = 256; // Prefiltered HDR environment texture size
const BRDF_SIZE: i32 = 512; // BRDF LUT texture size

// PBR material loading
// static Material LoadMaterialPBR(Color albedo, float metalness, float roughness);

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread)-> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - pbr material");
    rl.set_window_size(screen_width, screen_height);

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(4.0, 4.0, 4.0),
        rvec3(0.0, 0.5, 0.0),
        Vector3::up(),
        45.0,
    );

    // Load model and PBR material
    let mut model = rl
        .load_model(thread, "original/models/resources/pbr/trooper.obj")
        .unwrap();

    // Mesh tangents are generated... and uploaded to GPU
    // NOTE: New VBO for tangents is generated at default location and also binded to mesh VAO
    model.meshes_mut()[0].mesh_tangents(thread);

    let pbrmat = load_material_pbr(rl, thread, rcolor(255, 255, 255, 255), 1.0, 1.0);
    model.materials_mut()[0] = pbrmat;
    
    // Create lights
    // NOTE: Lights are added to an internal lights pool automatically
    use LightType::*;
    create_light(
        LIGHT_POINT,
        rvec3(
            3.5,
            1.0,
            0.0,
        ),
        rvec3(0.0, 0.0, 0.0),
        rcolor(255, 0, 0, 255),
        model.materials()[0].shader_mut(),
    );
    create_light(
        LIGHT_POINT,
        rvec3(
            0.0,
            1.0,
            3.5,
        ),
        rvec3(0.0, 0.0, 0.0),
        rcolor(0, 255, 0, 255),
        model.materials()[0].shader_mut(),
    );
    create_light(
        LIGHT_POINT,
        rvec3(
            -3.5,
            1.0,
            0.0,
        ),
        rvec3(0.0, 0.0, 0.0),
        rcolor(0, 0, 255, 255),
        model.materials()[0].shader_mut(),
    );
    create_light(
        LIGHT_DIRECTIONAL,
        rvec3(
            0.0,
            1.0 * 2.0,
            -3.5,
        ),
        rvec3(0.0, 0.0, 0.0),
        rcolor(255, 0, 255, 255),
        model.materials()[0].shader_mut(),
    );

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () 
    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        // Send to material PBR shader camera view position
        let mut camera_pos: [f32; 3] = [camera.position.x, camera.position.y, camera.position.z];
        let loc = model.materials()[0].shader().locs()
            [raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize];
        model.materials_mut()[0]
            .shader_mut()
            .set_shader_value(loc, camera_pos);
        //----------------------------------------------------------------------------------

        {

            // Draw
            //----------------------------------------------------------------------------------
            let mut d = rl.begin_drawing(thread);
    
            d.clear_background(Color::RAYWHITE);
    
            {
                let mut d = d.begin_mode3D(&camera);
    
                d.draw_model(&mut model, Vector3::zero(), 1.0, Color::WHITE);
    
                d.draw_grid(10, 1.0);
            }
    
            d.draw_fps(10, 10);
        }

        //---------------------------------------------------------------------------------
        if rl.is_key_pressed(crate::EXIT_KEY) {
            // De-Initialization
            //--------------------------------------------------------------------------------------
            // Shaders and textures must be unloaded by user,
            // they could be in use by other models
            use raylib::consts::MaterialMapIndex::*;
            unsafe {
                rl.unload_texture(
                    thread,
                    model.materials()[0].maps()[MATERIAL_MAP_ALBEDO as usize]
                        .texture()
                        .clone(),
                );
                rl.unload_texture(
                    thread,
                    model.materials()[0].maps()[MATERIAL_MAP_NORMAL as usize]
                        .texture()
                        .clone(),
                );
                rl.unload_texture(
                    thread,
                    model.materials()[0].maps()[MATERIAL_MAP_METALNESS as usize]
                        .texture()
                        .clone(),
                );
                rl.unload_texture(
                    thread,
                    model.materials()[0].maps()[MATERIAL_MAP_ROUGHNESS as usize]
                        .texture()
                        .clone(),
                );
                rl.unload_texture(
                    thread,
                    model.materials()[0].maps()[MATERIAL_MAP_OCCLUSION as usize]
                        .texture()
                        .clone(),
                );
                rl.unload_texture(
                    thread,
                    model.materials()[0].maps()[MATERIAL_MAP_IRRADIANCE as usize]
                        .texture()
                        .clone(),
                );
                rl.unload_texture(
                    thread,
                    model.materials()[0].maps()[MATERIAL_MAP_PREFILTER as usize]
                        .texture()
                        .clone(),
                );
                
            }
        }
    });

    

    //--------------------------------------------------------------------------------------
}

/// Load PBR material (Supports: ALBEDO, NORMAL, METALNESS, ROUGHNESS, AO, EMMISIVE, HEIGHT maps)
/// NOTE: PBR shader is loaded inside this function
fn load_material_pbr(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    albedo: Color,
    metalness: f32,
    roughness: f32,
) -> WeakMaterial {
    use raylib::consts::MaterialMapIndex::*;
    use raylib::consts::ShaderLocationIndex::*;
    use raylib::consts::TextureFilter::*;

    let mut mat = rl.load_material_default(thread);

    #[cfg(target_arch = "wasm32")]
    unsafe {
        *mat.shader_mut() = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/pbr.vs"),
                Some("original/models/resources/shaders/glsl330/pbr.fs"),
            )
            .unwrap()
            .make_weak();
    }
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        *mat.shader_mut() = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/pbr.vs"),
                Some("original/models/resources/shaders/glsl330/pbr.fs"),
            )
            .unwrap()
            .make_weak();
    }

    // Get required locations points for PBR material
    // NOTE: Those location names must be available and used in the shader code
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_ALBEDO as usize] =
        mat.shader().get_shader_location("albedo.sampler");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_METALNESS as usize] =
        mat.shader().get_shader_location("metalness.sampler");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_NORMAL as usize] =
        mat.shader().get_shader_location("normals.sampler");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_ROUGHNESS as usize] =
        mat.shader().get_shader_location("roughness.sampler");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_OCCLUSION as usize] =
        mat.shader().get_shader_location("occlusion.sampler");
    //mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_EMISSION] = mat.shader().get_shader_location( "emission.sampler");
    //mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_HEIGHT] = mat.shader().get_shader_location( "height.sampler");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_IRRADIANCE as usize] =
        mat.shader().get_shader_location("irradianceMap");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_PREFILTER as usize] =
        mat.shader().get_shader_location("prefilterMap");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MAP_BRDF as usize] =
        mat.shader().get_shader_location("brdfLUT");

    // Set view matrix location
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
        mat.shader().get_shader_location("matModel");
    //mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MATRIX_VIEW as usize] = mat.shader().get_shader_location( "view");
    mat.shader_mut().locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
        mat.shader().get_shader_location("viewPos");

    // Set PBR standard maps
    unsafe {
        mat.maps_mut()[MATERIAL_MAP_ALBEDO as usize].texture = *rl
            .load_texture(thread, "original/models/resources/pbr/trooper_albedo.png")
            .unwrap()
            .make_weak()
            .as_ref();
        mat.maps_mut()[MATERIAL_MAP_NORMAL as usize].texture = *rl
            .load_texture(thread, "original/models/resources/pbr/trooper_normals.png")
            .unwrap()
            .make_weak()
            .as_ref();
        mat.maps_mut()[MATERIAL_MAP_METALNESS as usize].texture = *rl
            .load_texture(
                thread,
                "original/models/resources/pbr/trooper_metalness.png",
            )
            .unwrap()
            .make_weak()
            .as_ref();
        mat.maps_mut()[MATERIAL_MAP_ROUGHNESS as usize].texture = *rl
            .load_texture(
                thread,
                "original/models/resources/pbr/trooper_roughness.png",
            )
            .unwrap()
            .make_weak()
            .as_ref();
        mat.maps_mut()[MATERIAL_MAP_OCCLUSION as usize].texture = *rl
            .load_texture(thread, "original/models/resources/pbr/trooper_ao.png")
            .unwrap()
            .make_weak()
            .as_ref();
    }

    let mut shdr_cubemap;
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        shdr_cubemap = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/pbr.vs"),
                Some("original/models/resources/shaders/glsl330/pbr.fs"),
            )
            .unwrap()
            .make_weak();
    }
    #[cfg(target_arch = "wasm32")]
    unsafe {
        shdr_cubemap = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl100/cubemap.vs"),
                Some("original/models/resources/shaders/glsl100/cubemap.fs"),
            )
            .unwrap()
            .make_weak();
    }

    let mut shdr_irradiance;
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        shdr_irradiance = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/skybox.vss"),
                Some("original/models/resources/shaders/glsl330/irradiance.fs"),
            )
            .unwrap()
            .make_weak();
    }
    #[cfg(target_arch = "wasm32")]
    unsafe {
        shdr_irradiance = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl100/skybox.vs"),
                Some("original/models/resources/shaders/glsl100/irradiance.fs"),
            )
            .unwrap()
            .make_weak();
    }

    let mut shdr_pre_filter;
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        shdr_pre_filter = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/skybox.vs"),
                Some("original/models/resources/shaders/glsl330/irradiance.fs"),
            )
            .unwrap()
            .make_weak();
    }
    #[cfg(target_arch = "wasm32")]
    unsafe {
        shdr_pre_filter = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl100/skybox.vs"),
                Some("original/models/resources/shaders/glsl100/irradiance.fs"),
            )
            .unwrap()
            .make_weak();
    }

    #[allow(non_snake_case)]
    let shdrBRDF;
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        shdrBRDF = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl330/brdf.vs"),
                Some("original/models/resources/shaders/glsl330/brdf.fs"),
            )
            .unwrap()
            .make_weak();
    }
    #[cfg(target_arch = "wasm32")]
    unsafe {
        shdrBRDF = rl
            .load_shader(
                thread,
                Some("original/models/resources/shaders/glsl100/brdf.vs"),
                Some("original/models/resources/shaders/glsl100/brdf.fs"),
            )
            .unwrap()
            .make_weak();
    }

    // Setup required shader locations
    shdr_cubemap.set_shader_value(shdr_cubemap.get_shader_location("equirectangularMap"), 0i32);
    shdr_irradiance.set_shader_value(shdr_irradiance.get_shader_location("environmentMap"), 0i32);
    shdr_pre_filter.set_shader_value(shdr_pre_filter.get_shader_location("environmentMap"), 0i32);

    #[allow(non_snake_case)]
    let texHDR = rl
        .load_texture(thread, "original/models/resources/dresden_square.hdr")
        .unwrap();
    let cubemap = rl.gen_texture_cubemap(thread, &shdr_cubemap, &texHDR, CUBEMAP_SIZE, ffi::PixelFormat::UNCOMPRESSED_R32G32B32);
    unsafe {
        *mat.maps_mut()[MATERIAL_MAP_IRRADIANCE as usize].texture_mut() = rl
            .gen_texture_irradiance(thread, &shdr_irradiance, &cubemap, IRRADIANCE_SIZE)
            .make_weak();
        *mat.maps_mut()[MATERIAL_MAP_PREFILTER as usize].texture_mut() = rl
            .gen_texture_prefilter(thread, &shdr_pre_filter, &cubemap, PREFILTERED_SIZE)
            .make_weak();
        *mat.maps_mut()[MATERIAL_MAP_BRDF as usize].texture_mut() = rl
            .gen_texture_brdf(thread, &shdrBRDF, BRDF_SIZE)
            .make_weak();
    }

    // Set textures filtering for better quality
    mat.maps_mut()[MATERIAL_MAP_ALBEDO as usize]
        .texture_mut()
        .set_texture_filter(thread, FILTER_BILINEAR);
    mat.maps_mut()[MATERIAL_MAP_NORMAL as usize]
        .texture_mut()
        .set_texture_filter(thread, FILTER_BILINEAR);
    mat.maps_mut()[MATERIAL_MAP_METALNESS as usize]
        .texture_mut()
        .set_texture_filter(thread, FILTER_BILINEAR);
    mat.maps_mut()[MATERIAL_MAP_ROUGHNESS as usize]
        .texture_mut()
        .set_texture_filter(thread, FILTER_BILINEAR);
    mat.maps_mut()[MATERIAL_MAP_OCCLUSION as usize]
        .texture_mut()
        .set_texture_filter(thread, FILTER_BILINEAR);

    // Enable sample usage in shader for assigned textures
    let loc = mat.shader().get_shader_location("albedo.useSampler");
    mat.shader_mut().set_shader_value(loc, 1i32);
    let loc = mat.shader().get_shader_location("normals.useSampler");
    mat.shader_mut().set_shader_value(loc, 1i32);
    let loc = mat.shader().get_shader_location("metalness.useSampler");
    mat.shader_mut().set_shader_value(loc, 1i32);
    let loc = mat.shader().get_shader_location("roughness.useSampler");
    mat.shader_mut().set_shader_value(loc, 1i32);
    let loc = mat.shader().get_shader_location("occlusion.useSampler");
    mat.shader_mut().set_shader_value(loc, 1i32);

    let render_mode_loc = mat.shader().get_shader_location("renderMode");
    mat.shader_mut().set_shader_value(render_mode_loc, 0i32);

    // Set up material properties color
    *mat.maps_mut()[MATERIAL_MAP_ALBEDO as usize].color_mut() = albedo;
    *mat.maps_mut()[MATERIAL_MAP_NORMAL as usize].color_mut() = rcolor(128, 128, 255, 255);
    *mat.maps_mut()[MATERIAL_MAP_METALNESS as usize].value_mut() = metalness;
    *mat.maps_mut()[MATERIAL_MAP_ROUGHNESS as usize].value_mut() = roughness;
    *mat.maps_mut()[MATERIAL_MAP_OCCLUSION as usize].value_mut() = 1.0;
    *mat.maps_mut()[MATERIAL_MAP_EMISSION as usize].value_mut() = 0.5;
    *mat.maps_mut()[MATERIAL_MAP_HEIGHT as usize].value_mut() = 0.5;

    return mat;
}


const MAX_LIGHTS:u32 = 4;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LightType {
    LIGHT_DIRECTIONAL = 0,
    LIGHT_POINT = 1,
}

impl Default for LightType {
    fn default() -> Self {
        Self::LIGHT_DIRECTIONAL
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
    shader: &mut WeakShader,
) -> Light {
    let mut light = Light::default();

    if (unsafe { LIGHTS_COUNT as u32 } < MAX_LIGHTS)
    {
        light.enabled = true;
        light.light_type = light_type;
        light.position = pos.clone();
        light.target = targ.clone();
        light.color = color.clone();

        // TODO: Below code doesn't look good to me, 
        // it assumes a specific shader naming and structure
        // Probably this implementation could be improved
       
        let lights_count = unsafe { LIGHTS_COUNT};
            let enabledName = format!("lights[{}].enabled", lights_count);
            let typeName = format!("lights[{}].type", lights_count);
            let posName = format!("lights[{}].position", lights_count);
            let targetName = format!("lights[{}].target", lights_count);
            let colorName = format!("lights[{}].color", lights_count);
        
        // Set location name [x] depending on lights count
       

        light.enabled_loc = shader.get_shader_location( &enabledName);
        light.type_loc = shader.get_shader_location( &typeName);
        light.pos_loc = shader.get_shader_location( &posName);
        light.target_loc = shader.get_shader_location( &targetName);
        light.color_loc = shader.get_shader_location( &colorName);

        update_light_values(shader, light.clone());
        unsafe {
            LIGHTS_COUNT += 1;
        }
    }

    return light;
}

pub fn update_light_values(shader: &mut WeakShader, light: Light) {
         // Send to shader light enabled state and type
    shader.set_shader_value( light.enabled_loc, light.enabled as i32);
    shader.set_shader_value( light.type_loc, light.light_type as i32);

    // Send to shader light position values
    shader.set_shader_value( light.pos_loc, light.position);

    // Send to shader light target position values
    shader.set_shader_value( light.target_loc, light.target);

    // Send to shader light color values
    
    let color: Vector4 = light.color.into();
                       shader.set_shader_value( light.color_loc, color);
}