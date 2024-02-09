/*******************************************************************************************
*
*   raylib [core] example - window scale letterbox (and virtual mouse)
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Anata (@anatagawa) and reviewed by Ramon Santamaria (@raysan5)
*
*   Copyright (c) 2019 Anata (@anatagawa) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;



    // Clamp Vector2 value with min and max and return a new vector2
    // NOTE: Required for virtual mouse, to clamp inside virtual game size
    fn clamp_value( value: Vector2,  min: Vector2,  max: Vector2) -> Vector2
{
    let mut result = value;
    result.x = result.x.max(min.x).min(max.x);
    result.y = result.y.max(min.y).min(max.y);
    return result;
}




pub fn run(rl
           : &mut RaylibHandle, thread
           : &RaylibThread)
    ->crate::SampleOut
{

    // Enable config flags for resizable window and vertical synchro
    rl.set_window_title(thread, "raylib [core] example - window scale letterbox");
    rl.set_window_min_size(320, 240);

    let game_screen_width: i32 = 640;
    let game_screen_height: i32  = 480;

    // Render texture initialization, used to hold the rendering result so we can easily resize it
    let mut  target = rl.load_render_texture(&thread, game_screen_width as u32, game_screen_height as u32).unwrap();
    target.texture().set_texture_filter(thread, raylib::consts::TextureFilter::TEXTURE_FILTER_BILINEAR);

    let mut  colors = [Color::default(); 10];
    for i in 0..10 
    {
        colors[i] = Color::new(rl.get_random_value::<i32>(100, 250) as u8, rl.get_random_value::<i32>(50, 150) as u8, rl.get_random_value::<i32>(10, 100) as u8, 255);

    }

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // Compute required framebuffer scaling
        let scale = (rl.get_screen_width() / game_screen_width).min(rl.get_screen_height() / game_screen_height) as f32;

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE)
        {
            // Recalculate random colors for the bars
            for i in 0..10
                {

                    colors[i] = Color::new(rl.get_random_value::<i32>(100, 250) as u8, rl.get_random_value::<i32>(50, 150) as u8, rl.get_random_value::<i32>(10, 100) as u8, 255);
                }
        }

        // Update virtual mouse (clamped mouse value behind game screen)
        let mouse = rl.get_mouse_position();
        let mut virtualMouse = Vector2::default();
        virtualMouse.x = (mouse.x - (rl.get_screen_width() as f32 - (game_screen_width as f32 * scale)) as f32 * 0.5) / scale;
        virtualMouse.y = (mouse.y - (rl.get_screen_height() as f32 - (game_screen_height as f32 * scale)) as f32  * 0.5) / scale;
        virtualMouse = clamp_value(virtualMouse, rvec2(0,  0), rvec2(game_screen_width,  game_screen_height));
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        {
            // Draw everything in the render texture, note this will not be rendered on screen, yet
        let mut d = d.begin_texture_mode(thread, &mut target);

        d.clear_background(Color::RAYWHITE); // Clear render texture background color

        for i in 0..10 
            {
                d.draw_rectangle(0, (game_screen_height / 10) * i, game_screen_width, game_screen_height / 10, colors[i as usize]);

            }

        d.draw_text("If executed inside a window,\nyou can resize the window,\nand see the screen scaling!", 10, 25, 20, Color::WHITE);

        d.draw_text(&format!("Default Mouse: [{} , {}]", mouse.x, mouse.y), 350, 25, 20, Color::GREEN);
        d.draw_text(&format!("Virtual Mouse: [{} , {}]", virtualMouse.x, virtualMouse.y), 350, 55, 20, Color::YELLOW);
}

        // Draw RenderTexture2D to window, properly scaled
        d.draw_texture_pro(target.texture(), rrect(0.0, 0.0, target.texture.width, -target.texture.height),
                       rrect((d.get_screen_width() as f32 - (game_screen_width as f32 * scale)) * 0.5, (d.get_screen_height() as f32 - (game_screen_height as f32 * scale)) * 0.5,
                                   game_screen_width as f32 * scale, game_screen_height as f32 * scale),
                       rvec2(0,  0), 0.0, Color::WHITE);

    });

}
