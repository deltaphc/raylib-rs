#![allow(non_snake_case)]
use raylib::ffi;
use raylib::prelude::*;

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
pub fn main() {
    // Initialization
    //---------------------------------------------------------------------------------------
    let screenWidth = 690;
    let screenHeight = 560;

    // let (mut rl, thread) = raylib::init()
    //     .size(screenWidth, screenHeight)
    //     .title("rgui")
    //     .build();
    // let logo = raylib::prelude::Image::load_image("static/logo.png").unwrap();
    // rl.set_window_icon(&logo);
    // rl.set_target_fps(60);

    unsafe {
        ffi::InitWindow(
            screenWidth,
            screenHeight,
            b"raygui - controls test suite\0".as_ptr() as *const _,
        );
        ffi::SetTargetFPS(60);
    }

    // GUI controls initialization
    //----------------------------------------------------------------------------------
    // let mut dropdownBox000Active = 0;
    // let mut dropDown000EditMode = false;

    // let mut dropdownBox001Active = 0;
    // let mut dropDown001EditMode = false;

    let mut spinner001Value = 0;
    let mut spinnerEditMode = false;

    // let mut valueBox002Value = 0;
    // let mut valueBoxEditMode = false;

    // let mut textBoxText = [0u8; 64];
    // textBoxText[..8].clone_from_slice(b"Text box");
    // let mut textBoxEditMode = false;

    // let mut listViewScrollIndex = 0;
    // let mut listViewActive = -1;

    // let mut listViewExScrollIndex = 0;
    // let mut listViewExActive = 2;
    // let mut listViewExFocus = -1;
    // let listViewExList = vec![
    //     rstr!("This"),
    //     rstr!("is"),
    //     rstr!("a"),
    //     rstr!("list view"),
    //     rstr!("with"),
    //     rstr!("disable"),
    //     rstr!("elements"),
    //     rstr!("amazing!"),
    // ];

    // let mut multiTextBoxText = [0u8; 256];
    // multiTextBoxText[..14].clone_from_slice(b"Multi text box");
    // let mut multiTextBoxEditMode = false;
    // let mut colorPickerValue = Color::RED;

    // let mut sliderValue = 50f32;
    // let mut sliderBarValue = 60f32;
    // let mut progressValue = 0.4;

    // let mut forceSquaredChecked = false;

    // let mut alphaValue = 0.5;

    // let mut comboBoxActive = 1;

    // let mut toggleGroupActive = 0;

    // let mut viewScroll = rvec2(0, 0);
    // //----------------------------------------------------------------------------------

    // // Custom GUI font loading
    // //Font font = LoadFontEx("fonts/rainyhearts16.ttf", 12, 0, 0);
    // //GuiSetFont(font);

    let mut exitWindow = false;
    // let mut showMessageBox = false;

    // let mut textInput = vec![0u8; 256];
    // let mut showTextInputBox = false;

    // let mut textInputFileName = [0u8; 256];

    //--------------------------------------------------------------------------------------

    // Main game loop
    while !exitWindow
    // Detect window close button or ESC key
    {
        use raylib::consts::GuiControl::*;
        use raylib::consts::GuiControlProperty::*;

        use raylib::consts::GuiTextAlignment::*;

        // Update
        //----------------------------------------------------------------------------------

        // exitWindow = rl.window_should_close();
        exitWindow = unsafe { ffi::WindowShouldClose() };

        // if rl.is_key_pressed(KEY_ESCAPE) {
        //     showMessageBox = !showMessageBox;
        // }

        // if rl.is_key_down(KEY_LEFT_CONTROL) && rl.is_key_pressed(KEY_S) {
        //     showTextInputBox = true;
        // }

        // if rl.is_file_dropped() {
        //     let droppedFiles = rl.load_dropped_files();

        //     if (droppedFiles.len() > 0) && droppedFiles[0].ends_with(".rgs") {
        //         rl.gui_load_style(Some(&CString::new(droppedFiles[0].clone()).unwrap()));
        //     }

        //     rl.unload_dropped_files();
        // }

        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        // let mut d = rl.begin_drawing(&thread);
        // let hex = d.gui_get_style(DEFAULT, BACKGROUND_COLOR as i32);
        // d.clear_background(Color::get_color(hex));

        unsafe {
            ffi::BeginDrawing();
            ffi::ClearBackground(Color::WHITE.into());
        }

        // // raygui: controls drawing
        // //----------------------------------------------------------------------------------
        // if dropDown000EditMode || dropDown001EditMode {
        //     d.gui_lock();
        // }
        // //GuiDisable();

        // // First GUI column
        // //GuiSetStyle(CHECKBOX, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT);
        // forceSquaredChecked = d.gui_check_box(
        //     rrect(25, 108, 15, 15),
        //     Some(rstr!("FORCE CHECK!")),
        //     forceSquaredChecked,
        // );

        unsafe {
            ffi::GuiSetStyle(
                TEXTBOX as i32,
                TEXT_ALIGNMENT as i32,
                TEXT_ALIGN_CENTER as i32,
            );
        }
        // dbg!(spinnerEditMode);
        // let pos = d.get_mouse_position();
        // let coll = rrect(25, 135, 125, 30).check_collision_point_rec(pos);
        // let down = d.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);

        // if d.gui_spinner(
        //     rrect(25, 135, 125, 30),
        //     None,
        //     &mut spinner001Value,
        //     0,
        //     100,
        //     spinnerEditMode,
        // ) {
        //     spinnerEditMode = dbg!(!spinnerEditMode);
        // }
        unsafe {
            ffi::DrawCircle(
                50,
                50,
                5.0,
                if spinnerEditMode {
                    Color::BLUE.into()
                } else {
                    Color::RED.into()
                },
            );
            if ffi::GuiSpinner(
                rrect(25, 135, 125, 30).into(),
                std::ptr::null(),
                &mut spinner001Value,
                0,
                100,
                spinnerEditMode,
            ) > 0
            {
                spinnerEditMode = dbg!(!spinnerEditMode);
            }
            // if ffi::GuiSpinner(
            //     rrect(25, 185, 125, 30).into(),
            //     std::ptr::null(),
            //     &mut spinner001Value,
            //     0,
            //     100,
            //     false,
            // ) {
            //     // spinnerEditMode = dbg!(!spinnerEditMode);
            // }
            // if ffi::GuiSpinner(
            //     rrect(25, 225, 125, 30).into(),
            //     std::ptr::null(),
            //     &mut spinner001Value,
            //     0,
            //     100,
            //     false,
            // ) {
            //     // spinnerEditMode = dbg!(!spinnerEditMode);
            // }
        }
        // if d.gui_value_box(
        //     rrect(25, 175, 125, 30),
        //     None,
        //     &mut valueBox002Value,
        //     0,
        //     100,
        //     valueBoxEditMode,
        // ) {
        //     valueBoxEditMode = !valueBoxEditMode;
        // }
        // d.gui_set_style(TEXTBOX, TEXT_ALIGNMENT as i32, TEXT_ALIGN_LEFT as i32);
        // if d.gui_text_box(rrect(25, 215, 125, 30), &mut textBoxText, textBoxEditMode) {
        //     textBoxEditMode = !textBoxEditMode;
        // }

        // d.gui_set_style(BUTTON, TEXT_ALIGNMENT as i32, TEXT_ALIGN_CENTER as i32);

        // let itext = d.gui_icon_text(RICON_FILE_SAVE, Some(rstr!("Save File")));
        // let itext = CString::new(itext).unwrap();
        // if d.gui_button(rrect(25, 255, 125, 30), Some(&itext)) {
        //     showTextInputBox = true;
        // }

        // d.gui_group_box(rrect(25, 310, 125, 150), Some(rstr!("STATES")));

        // d.gui_lock();
        // d.gui_set_state(_NORMAL);
        // if d.gui_button(rrect(30, 320, 115, 30), Some(rstr!("NORMAL"))) {}
        // d.gui_set_state(_FOCUSED);
        // if d.gui_button(rrect(30, 355, 115, 30), Some(rstr!("FOCUSED"))) {}
        // d.gui_set_state(_PRESSED);
        // if d.gui_button(rrect(30, 390, 115, 30), Some(rstr!("#15#PRESSED"))) {}
        // d.gui_set_state(_DISABLED);
        // if d.gui_button(rrect(30, 425, 115, 30), Some(rstr!("DISABLED"))) {}
        // d.gui_set_state(_NORMAL);
        // d.gui_unlock();

        // comboBoxActive = d.gui_combo_box(
        //     rrect(25, 470, 125, 30),
        //     Some(rstr!("ONE;TWO;THREE;FOUR")),
        //     comboBoxActive,
        // );

        // // NOTE: GuiDropdownBox must draw after any other control that can be covered on unfolding
        // d.gui_set_style(
        //     DROPDOWNBOX,
        //     TEXT_ALIGNMENT as i32,
        //     TEXT_ALIGN_LEFT as i32,
        // );
        // if d.gui_dropdown_box(
        //     rrect(25, 65, 125, 30),
        //     Some(rstr!("#01#ONE;#02#TWO;#03#THREE;#04#FOUR")),
        //     &mut dropdownBox001Active,
        //     dropDown001EditMode,
        // ) {
        //     dropDown001EditMode = !dropDown001EditMode;
        // }

        // d.gui_set_style(
        //     DROPDOWNBOX,
        //     TEXT_ALIGNMENT as i32,
        //     TEXT_ALIGN_CENTER as i32,
        // );
        // if d.gui_dropdown_box(
        //     rrect(25, 25, 125, 30),
        //     Some(rstr!("ONE;TWO;THREE")),
        //     &mut dropdownBox000Active,
        //     dropDown000EditMode,
        // ) {
        //     dropDown000EditMode = !dropDown000EditMode;
        // }

        // // Second GUI column
        // listViewActive = d.gui_list_view(
        //     rrect(165, 25, 140, 140),
        //     Some(rstr!(
        //         "Charmander;Bulbasaur;#18#Squirtel;Pikachu;Eevee;Pidgey"
        //     )),
        //     &mut listViewScrollIndex,
        //     listViewActive,
        // );
        // listViewExActive = d.gui_list_view_ex(
        //     rrect(165, 180, 140, 200),
        //     &listViewExList,
        //     8,
        //     &mut listViewExFocus,
        //     &mut listViewExScrollIndex,
        //     listViewExActive,
        // );

        // toggleGroupActive = d.gui_toggle_group(
        //     rrect(165, 400, 140, 25),
        //     Some(rstr!("#1#ONE\n#3#TWO\n#8#THREE\n#23#")),
        //     toggleGroupActive,
        // );

        // // Third GUI column
        // if d.gui_text_box_multi(
        //     rrect(320, 25, 225, 140),
        //     &mut multiTextBoxText,
        //     multiTextBoxEditMode,
        // ) {
        //     multiTextBoxEditMode = !multiTextBoxEditMode;
        // }
        // colorPickerValue = d.gui_color_picker(rrect(320, 185, 196, 192), colorPickerValue);

        // sliderValue = d.gui_slider(
        //     rrect(355, 400, 165, 20),
        //     Some(rstr!("TEST")),
        //     Some(&rstr!("{:.2}", sliderValue as f32).unwrap()),
        //     sliderValue,
        //     -50.0,
        //     100.0,
        // );
        // sliderBarValue = d.gui_slider_bar(
        //     rrect(320, 430, 200, 20),
        //     None,
        //     Some(&rstr!("{}", sliderBarValue).unwrap()),
        //     sliderBarValue,
        //     0.0,
        //     100.0,
        // );
        // progressValue = d.gui_progress_bar(
        //     rrect(320, 460, 200, 20),
        //     None,
        //     None,
        //     progressValue,
        //     0.0,
        //     1.0,
        // );

        // // NOTE: View rectangle could be used to perform some scissor test
        // let (_view, nextScroll) = d.gui_scroll_panel(
        //     rrect(560, 25, 100, 160),
        //     rrect(560, 25, 200, 400),
        //     viewScroll,
        // );
        // viewScroll = nextScroll.into();

        // d.gui_status_bar(
        //     rrect(0, d.get_screen_height() - 20, d.get_screen_width(), 20),
        //     Some(rstr!("This is a status bar")),
        // );

        // alphaValue = d.gui_color_bar_alpha(rrect(320, 490, 200, 30), alphaValue);

        // if showMessageBox {
        //     d.draw_rectangle(
        //         0,
        //         0,
        //         d.get_screen_width(),
        //         d.get_screen_height(),
        //         Color::RAYWHITE.fade(0.8),
        //     );
        //     let itext = d.gui_icon_text(RICON_EXIT, Some(rstr!("Close Window")));
        //     let itext = CString::new(itext).unwrap();
        //     let result = d.gui_message_box(
        //         rrect(
        //             d.get_screen_width() / 2 - 125,
        //             d.get_screen_height() / 2 - 50,
        //             250,
        //             100,
        //         ),
        //         Some(&itext),
        //         Some(rstr!("Do you really want to exit?")),
        //         Some(rstr!("Yes;No")),
        //     );

        //     if ((result == 0) || (result == 2)) {
        //         showMessageBox = false;
        //     } else if (result == 1) {
        //         exitWindow = true;
        //     }
        // }

        // if showTextInputBox {
        //     d.draw_rectangle(
        //         0,
        //         0,
        //         d.get_screen_width(),
        //         d.get_screen_height(),
        //         Color::RAYWHITE.fade(0.8),
        //     );
        //     let itext = unsafe { d.gui_icon_text(RICON_FILE_SAVE, Some(rstr!("Save file as..."))) };
        //     let itext = CString::new(itext).unwrap();
        //     let result = d.gui_text_input_box(
        //         rrect(
        //             d.get_screen_width() / 2 - 120,
        //             d.get_screen_height() / 2 - 60,
        //             240,
        //             140,
        //         ),
        //         Some(&itext),
        //         Some(rstr!("Introduce a save file name")),
        //         Some(rstr!("Ok;Cancel")),
        //         &mut textInput,
        //     );

        //     if (result == 1) {
        //         // TODO: Validate textInput value and save
        //         textInputFileName[..textInput.len()].clone_from_slice(&textInput);
        //     }

        //     if ((result == 0) || (result == 1) || (result == 2)) {
        //         showTextInputBox = false;
        //         textInput[0] = b'\0';
        //     }
        // }

        // d.gui_unlock();
        unsafe {
            ffi::EndDrawing();
        }
    }
}
