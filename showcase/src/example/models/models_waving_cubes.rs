/*******************************************************************************************
*
*   raylib [models] example - Waving cubes
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Codecat (@codecat) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Codecat (@codecat) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

pub use raylib::prelude::*;


pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut
{
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_title(thread, "raylib [models] example - waving cubes");
    rl.set_window_size(screen_width, screen_height);

    // Initialize the camera
    let mut  camera = Camera3D::perspective(
     rvec3( 30.0, 20.0,30.0 ),
     rvec3( 0.0, 0.0,0.0 ),
    rvec3( 0.0, 1.0,0.0 ),
    70.0,
    );

    // Specify the amount of blocks in each direction
    let numBlocks = 15;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        let time = rl.get_time() as f32;

        // Calculate time scale for cube position and size
        let scale = 2.0 + time.sin()*0.7;

        // Move camera around the scene
        let cameraTime = time*0.3;
        camera.position.x = (cameraTime).cos()*40.0;
        camera.position.z = (cameraTime).sin()*40.0;
        //----------------------------------------------------------------------------------
        
        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

            d.clear_background(Color::RAYWHITE);

            {

                let mut d = d.begin_mode3D(&camera);
    
                    d.draw_grid(10, 5.0);
    
                    for x in 0..numBlocks
                    {
                        for y in 0..numBlocks 
                        {
                            for z in 0..numBlocks 
                            {
                                
                                // Scale of the blocks depends on x/y/z positions
                                let blockScale = (x as f32 + y as f32 + z as f32)/30.0;
    
                                // Scatter makes the waving effect by adding blockScale over time
                                let scatter = (blockScale*20.0 + time*4.0).sin();
    
                                // Calculate the cube position
                                let cubePos = rvec3(
                                    (x - numBlocks/2) as f32 *(scale*3.0) + scatter,
                                    (y - numBlocks/2) as f32 *(scale*2.0) + scatter,
                                    (z - numBlocks/2) as f32 *(scale*3.0) + scatter
                                );
    
                                // Pick a color with a hue depending on cube position for the rainbow color effect
                                let cubeColor = Color::color_from_hsv( (((x + y + z)*18)%360) as f32, 0.75,0.9 );
    
                                // Calculate cube size
                                let cubeSize = (2.4 - scale)*blockScale;
    
                                // And finally, draw the cube!
                                d.draw_cube(cubePos, cubeSize, cubeSize, cubeSize, cubeColor);
                            }
                        }
                    }
                    
            }
            
            d.draw_fps(10, 10);

        //----------------------------------------------------------------------------------
    });

}
