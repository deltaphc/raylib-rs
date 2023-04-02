/*******************************************************************************************
*
*   raygui - Controls test
*
*   TEST CONTROLS:
*       - GuiScrollPanel()
*
*   DEPENDENCIES:
*       raylib 2.4  - Windowing/input management and drawing.
*       raygui 2.0  - Immediate-mode GUI controls.
*
*   COMPILATION (Windows - MinGW):
*       gcc -o $(NAME_PART).exe $(FILE_NAME) -I../../src -lraylib -lopengl32 -lgdi32 -std=c99
*
*   COMPILATION (Linux - gcc):
*	gcc -o $(NAME_PART) $(FILE_NAME) -I../../src -lraylib -std=c99
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2019 Vlad Adrian (@Demizdor) and Ramon Santamaria (@raysan5)
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
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - GuiScrollPanel()");

    let panelRec = rrect(20, 40, 200, 150);
    let mut panelContentRec = rrect(0, 0, 340, 340);
    let panelScroll = rvec2(99, -20);

    let mut showContentArea = true;

    rl.set_target_fps(60);
    //---------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // TODO: Implement required update logic
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        d.draw_text(&format!("[{}, {}]", panelScroll.x, panelScroll.y), 4, 4, 20,Color::RED);

        let (view, _) = d.gui_scroll_panel(panelRec, panelContentRec, &panelScroll);

        {

            let mut d = d.begin_scissor_mode(view.x as i32, view.y as i32, view.width as i32, view.height as i32);
            d.gui_grid(rrect(panelRec.x + panelScroll.x, panelRec.y + panelScroll.y, panelContentRec.width, panelContentRec.height), 16.0, 3);
        }

        if showContentArea
            {d.draw_rectangle((panelRec.x + panelScroll.x) as i32, (panelRec.y + panelScroll.y) as i32, panelContentRec.width as i32, panelContentRec.height as i32,Color::RED.fade(0.1));}

        DrawStyleEditControls(&mut d);

        showContentArea = d.gui_check_box(rrect(565, 80, 20, 20), Some(rstr!("SHOW CONTENT AREA")), showContentArea);

        panelContentRec.width = d.gui_slider_bar(rrect(590, 385, 145, 15), Some(rstr!("WIDTH")), Some(&rstr!("{}", panelContentRec.width)), 1f32, 0f32, 600f32);
        panelContentRec.height = d.gui_slider_bar(rrect(590, 410, 145, 15), Some(rstr!("HEIGHT")), Some(&rstr!("{}", panelContentRec.height)), 1f32, 0f32, 400f32);

        //----------------------------------------------------------------------------------
    },
    );
}

// Draw and process scroll bar style edition controls
fn DrawStyleEditControls(d: &mut RaylibDrawHandle) {
    use raylib::consts::GuiControl::*;
    use raylib::consts::GuiControlProperty::*;
    use raylib::consts::GuiListViewProperty::*;
    use raylib::consts::GuiScrollBarProperty::*;
    use raylib::consts::GuiScrollBarSide::*;
    use raylib::consts::GuiSliderProperty::*;
    // ScrollPanel style controls
    //----------------------------------------------------------
    d.gui_group_box(rrect(550, 170, 220, 205), Some(rstr!("SCROLLBAR STYLE")));

    let mut style = d.gui_get_style(SCROLLBAR, BORDER_WIDTH as i32);
    d.gui_label(rrect(555, 195, 110, 10), Some(rstr!("BORDER_WIDTH")));
    d.gui_spinner(rrect(670, 190, 90, 20), None, &mut style, 0, 6, false);
    d.gui_set_style(SCROLLBAR, BORDER_WIDTH as i32, style);

    style = d.gui_get_style(SCROLLBAR, ARROWS_SIZE as i32);
    d.gui_label(rrect(555, 220, 110, 10), Some(rstr!("ARROWS_SIZE")));
    d.gui_spinner(rrect(670, 215, 90, 20), None, &mut style, 4, 14, false);
    d.gui_set_style(SCROLLBAR, ARROWS_SIZE as i32, style);

    style = d.gui_get_style(SCROLLBAR, SLIDER_PADDING as i32);
    d.gui_label(rrect(555, 245, 110, 10), Some(rstr!("SLIDER_PADDING")));
    d.gui_spinner(rrect(670, 240, 90, 20), None, &mut style, 0, 14, false);
    d.gui_set_style(SCROLLBAR, SLIDER_PADDING as i32, style);

    let checked = if d.gui_get_style(SCROLLBAR, ARROWS_VISIBLE as i32) > 0 {
        true
    } else {
        false
    };
    style = if d.gui_check_box(
        rrect(565, 280, 20, 20),
        Some(rstr!("ARROWS_VISIBLE")),
        checked,
    ) {
        1
    } else {
        0
    };
    d.gui_set_style(SCROLLBAR, ARROWS_VISIBLE as i32, style);

    style = d.gui_get_style(SCROLLBAR, SLIDER_PADDING as i32);
    d.gui_label(rrect(555, 325, 110, 10), Some(rstr!("SLIDER_PADDING")));
    d.gui_spinner(rrect(670, 320, 90, 20), None, &mut style, 0, 14, false);
    d.gui_set_style(SCROLLBAR, SLIDER_PADDING as i32, style);

    style = d.gui_get_style(SCROLLBAR, SLIDER_WIDTH as i32);
    d.gui_label(rrect(555, 350, 110, 10), Some(rstr!("SLIDER_WIDTH")));
    d.gui_spinner(rrect(670, 345, 90, 20), None, &mut style, 2, 100, false);
    d.gui_set_style(SCROLLBAR, SLIDER_WIDTH as i32, style);

    let text = if d.gui_get_style(LISTVIEW, SCROLLBAR_SIDE as i32) == SCROLLBAR_LEFT_SIDE as i32 {
        Some(rstr!("SCROLLBAR: LEFT"))
    } else {
        Some(rstr!("SCROLLBAR: RIGHT"))
    };
    style = if d.gui_toggle(
        rrect(560, 110, 200, 35),
        text,
        if d.gui_get_style(LISTVIEW, SCROLLBAR_SIDE as i32) > 0 {
            true
        } else {
            false
        },
    ) {
        1
    } else {
        0
    };
    d.gui_set_style(LISTVIEW, SCROLLBAR_SIDE as i32, style);
    //----------------------------------------------------------

    // ScrollBar style controls
    //----------------------------------------------------------
    d.gui_group_box(rrect(550, 20, 220, 135), Some(rstr!("SCROLLPANEL STYLE")));

    style = d.gui_get_style(LISTVIEW, SCROLLBAR_WIDTH as i32);
    d.gui_label(rrect(555, 35, 110, 10), Some(rstr!("SCROLLBAR_WIDTH")));
    d.gui_spinner(rrect(670, 30, 90, 20), None, &mut style, 6, 30, false);
    d.gui_set_style(LISTVIEW, SCROLLBAR_WIDTH as i32, style);

    style = d.gui_get_style(DEFAULT, BORDER_WIDTH as i32);
    d.gui_label(rrect(555, 60, 110, 10), Some(rstr!("BORDER_WIDTH")));
    d.gui_spinner(rrect(670, 55, 90, 20), None, &mut style, 0, 20, false);
    d.gui_set_style(DEFAULT, BORDER_WIDTH as i32, style);
    //----------------------------------------------------------
}
