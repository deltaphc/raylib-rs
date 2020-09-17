/*******************************************************************************************
*
*   raygui - portable window
*
*   DEPENDENCIES:
*       raylib 2.1  - Windowing/input management and drawing.
*       raygui 2.0  - Immediate-mode GUI controls.
*
*   COMPILATION (Windows - MinGW):
*       gcc -o $(NAME_PART).exe $(FILE_NAME) -I../../src -lraylib -lopengl32 -lgdi32 -std=c99
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2020 Ramon Santamaria (@raysan5)
*
**********************************************************************************************/

use raylib::prelude::*;


    //------------------------------------------------------------------------------------
    // Program main entry point
    //------------------------------------------------------------------------------------
pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //---------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 600;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - portable window");


    // General variables
    let mut mousePosition = Vector2::default();
    let  mut windowPosition = rvec2(500, 200);
    let  mut panOffset = mousePosition;
    let mut  dragWindow = false;

    rl.set_window_position(windowPosition.x as i32, windowPosition.y as i32);

    let mut  exitWindow = false;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()  // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        mousePosition = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
        {
            if rrect(0, 0, screen_width, 20).check_collision_point_rec(mousePosition )
            {
                dragWindow = true;
                panOffset = mousePosition;
            }
        }

        if dragWindow
        {
            windowPosition.x += (mousePosition.x - panOffset.x);
            windowPosition.y += (mousePosition.y - panOffset.y);

            if rl.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON)
            {
                dragWindow = false;
            }

            rl.set_window_position(windowPosition.x as i32, windowPosition.y as i32);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        exitWindow = d.gui_window_box(rrect(0, 0, screen_width, screen_height), Some(rstr!("PORTABLE WINDOW")));

        d.draw_text(&format!("Mouse Position: [ {:.0}, {:.0} ]", mousePosition.x, mousePosition.y), 10, 40, 10, Color::DARKGRAY);

        //----------------------------------------------------------------------------------
 
    });

}
