#![allow(non_snake_case)]
use raylib::prelude::*;
use std::ffi::CString;
#[allow(non_camel_case_types)]
#[repr(u32)]pub enum guiIconName {
    RICON_NONE,
    RICON_FOLDER_FILE_OPEN,
    RICON_FILE_SAVE_CLASSIC,
    RICON_FOLDER_OPEN,
    RICON_FOLDER_SAVE,
    RICON_FILE_OPEN,
    RICON_FILE_SAVE,
    RICON_FILE_EXPORT,
    RICON_FILE_NEW,
    RICON_FILE_DELETE,
    RICON_FILETYPE_TEXT,
    RICON_FILETYPE_AUDIO,
    RICON_FILETYPE_IMAGE,
    RICON_FILETYPE_PLAY,
    RICON_FILETYPE_VIDEO,
    RICON_FILETYPE_INFO,
    RICON_FILE_COPY,
    RICON_FILE_CUT,
    RICON_FILE_PASTE,
    RICON_CURSOR_HAND,
    RICON_CURSOR_POINTER,
    RICON_CURSOR_CLASSIC,
    RICON_PENCIL,
    RICON_PENCIL_BIG,
    RICON_BRUSH_CLASSIC,
    RICON_BRUSH_PAINTER,
    RICON_WATER_DROP,
    RICON_COLOR_PICKER,
    RICON_RUBBER,
    RICON_COLOR_BUCKET,
    RICON_TEXT_T,
    RICON_TEXT_A,
    RICON_SCALE,
    RICON_RESIZE,
    RICON_FILTER_POINT,
    RICON_FILTER_BILINEAR,
    RICON_CROP,
    RICON_CROP_ALPHA,
    RICON_SQUARE_TOGGLE,
    RICON_SYMMETRY,
    RICON_SYMMETRY_HORIZONTAL,
    RICON_SYMMETRY_VERTICAL,
    RICON_LENS,
    RICON_LENS_BIG,
    RICON_EYE_ON,
    RICON_EYE_OFF,
    RICON_FILTER_TOP,
    RICON_FILTER,
    RICON_TARGET_POINT,
    RICON_TARGET_SMALL,
    RICON_TARGET_BIG,
    RICON_TARGET_MOVE,
    RICON_CURSOR_MOVE,
    RICON_CURSOR_SCALE,
    RICON_CURSOR_SCALE_RIGHT,
    RICON_CURSOR_SCALE_LEFT,
    RICON_UNDO,
    RICON_REDO,
    RICON_REREDO,
    RICON_MUTATE,
    RICON_ROTATE,
    RICON_REPEAT,
    RICON_SHUFFLE,
    RICON_EMPTYBOX,
    RICON_TARGET,
    RICON_TARGET_SMALL_FILL,
    RICON_TARGET_BIG_FILL,
    RICON_TARGET_MOVE_FILL,
    RICON_CURSOR_MOVE_FILL,
    RICON_CURSOR_SCALE_FILL,
    RICON_CURSOR_SCALE_RIGHT_FILL,
    RICON_CURSOR_SCALE_LEFT_FILL,
    RICON_UNDO_FILL,
    RICON_REDO_FILL,
    RICON_REREDO_FILL,
    RICON_MUTATE_FILL,
    RICON_ROTATE_FILL,
    RICON_REPEAT_FILL,
    RICON_SHUFFLE_FILL,
    RICON_EMPTYBOX_SMALL,
    RICON_BOX,
    RICON_BOX_TOP,
    RICON_BOX_TOP_RIGHT,
    RICON_BOX_RIGHT,
    RICON_BOX_BOTTOM_RIGHT,
    RICON_BOX_BOTTOM,
    RICON_BOX_BOTTOM_LEFT,
    RICON_BOX_LEFT,
    RICON_BOX_TOP_LEFT,
    RICON_BOX_CENTER,
    RICON_BOX_CIRCLE_MASK,
    RICON_POT,
    RICON_ALPHA_MULTIPLY,
    RICON_ALPHA_CLEAR,
    RICON_DITHERING,
    RICON_MIPMAPS,
    RICON_BOX_GRID,
    RICON_GRID,
    RICON_BOX_CORNERS_SMALL,
    RICON_BOX_CORNERS_BIG,
    RICON_FOUR_BOXES,
    RICON_GRID_FILL,
    RICON_BOX_MULTISIZE,
    RICON_ZOOM_SMALL,
    RICON_ZOOM_MEDIUM,
    RICON_ZOOM_BIG,
    RICON_ZOOM_ALL,
    RICON_ZOOM_CENTER,
    RICON_BOX_DOTS_SMALL,
    RICON_BOX_DOTS_BIG,
    RICON_BOX_CONCENTRIC,
    RICON_BOX_GRID_BIG,
    RICON_OK_TICK,
    RICON_CROSS,
    RICON_ARROW_LEFT,
    RICON_ARROW_RIGHT,
    RICON_ARROW_BOTTOM,
    RICON_ARROW_TOP,
    RICON_ARROW_LEFT_FILL,
    RICON_ARROW_RIGHT_FILL,
    RICON_ARROW_BOTTOM_FILL,
    RICON_ARROW_TOP_FILL,
    RICON_AUDIO,
    RICON_FX,
    RICON_WAVE,
    RICON_WAVE_SINUS,
    RICON_WAVE_SQUARE,
    RICON_WAVE_TRIANGULAR,
    RICON_CROSS_SMALL,
    RICON_PLAYER_PREVIOUS,
    RICON_PLAYER_PLAY_BACK,
    RICON_PLAYER_PLAY,
    RICON_PLAYER_PAUSE,
    RICON_PLAYER_STOP,
    RICON_PLAYER_NEXT,
    RICON_PLAYER_RECORD,
    RICON_MAGNET,
    RICON_LOCK_CLOSE,
    RICON_LOCK_OPEN,
    RICON_CLOCK,
    RICON_TOOLS,
    RICON_GEAR,
    RICON_GEAR_BIG,
    RICON_BIN,
    RICON_HAND_POINTER,
    RICON_LASER,
    RICON_COIN,
    RICON_EXPLOSION,
    RICON_1UP,
    RICON_PLAYER,
    RICON_PLAYER_JUMP,
    RICON_KEY,
    RICON_DEMON,
    RICON_TEXT_POPUP,
    RICON_GEAR_EX,
    RICON_CRACK,
    RICON_CRACK_POINTS,
    RICON_STAR,
    RICON_DOOR,
    RICON_EXIT,
    RICON_MODE_2D,
    RICON_MODE_3D,
    RICON_CUBE,
    RICON_CUBE_FACE_TOP,
    RICON_CUBE_FACE_LEFT,
    RICON_CUBE_FACE_FRONT,
    RICON_CUBE_FACE_BOTTOM,
    RICON_CUBE_FACE_RIGHT,
    RICON_CUBE_FACE_BACK,
    RICON_CAMERA,
    RICON_SPECIAL,
    RICON_LINK_NET,
    RICON_LINK_BOXES,
    RICON_LINK_MULTI,
    RICON_LINK,
    RICON_LINK_BROKE,
    RICON_TEXT_NOTES,
    RICON_NOTEBOOK,
    RICON_SUITCASE,
    RICON_SUITCASE_ZIP,
    RICON_MAILBOX,
    RICON_MONITOR,
    RICON_PRINTER,
    RICON_PHOTO_CAMERA,
    RICON_PHOTO_CAMERA_FLASH,
    RICON_HOUSE,
    RICON_HEART,
    RICON_CORNER,
    RICON_VERTICAL_BARS,
    RICON_VERTICAL_BARS_FILL,
    RICON_LIFE_BARS,
    RICON_INFO,
    RICON_CROSSLINE,
    RICON_HELP,
    RICON_FILETYPE_ALPHA,
    RICON_FILETYPE_HOME,
    RICON_LAYERS_VISIBLE,
    RICON_LAYERS,
    RICON_WINDOW,
    RICON_HIDPI,
    RICON_200,
    RICON_201,
    RICON_202,
    RICON_203,
    RICON_204,
    RICON_205,
    RICON_206,
    RICON_207,
    RICON_208,
    RICON_209,
    RICON_210,
    RICON_211,
    RICON_212,
    RICON_213,
    RICON_214,
    RICON_215,
    RICON_216,
    RICON_217,
    RICON_218,
    RICON_219,
    RICON_220,
    RICON_221,
    RICON_222,
    RICON_223,
    RICON_224,
    RICON_225,
    RICON_226,
    RICON_227,
    RICON_228,
    RICON_229,
    RICON_230,
    RICON_231,
    RICON_232,
    RICON_233,
    RICON_234,
    RICON_235,
    RICON_236,
    RICON_237,
    RICON_238,
    RICON_239,
    RICON_240,
    RICON_241,
    RICON_242,
    RICON_243,
    RICON_244,
    RICON_245,
    RICON_246,
    RICON_247,
    RICON_248,
    RICON_249,
    RICON_250,
    RICON_251,
    RICON_252,
    RICON_253,
    RICON_254,
    RICON_255,
}
//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //---------------------------------------------------------------------------------------
    // #[cfg(target_os = "windows")] { // Macos has issues with high DPI
        let screen_width = 690;
        let screen_height = 560;

        rl.set_window_size(screen_width, screen_height);
    // }
    rl.set_window_title(thread, "raygui - controls test suite");


    // GUI controls initialization
    //----------------------------------------------------------------------------------
    let mut dropdownBox000Active = 0;
    let mut dropDown000EditMode = false;

    let mut dropdownBox001Active = 0;
    let mut dropDown001EditMode = false;

    let mut spinner001Value = 0;
    let mut spinnerEditMode = false;

    let mut valueBox002Value = 0;
    let mut valueBoxEditMode = false;

    let mut textBoxText = [0u8; 64];
    textBoxText[..8].clone_from_slice(b"Text box");
    let mut textBoxEditMode = false;

    let mut listViewScrollIndex = 0;
    let mut listViewActive = -1;

    let mut listViewExScrollIndex = 0;
    let mut listViewExActive = 2;
    let mut listViewExFocus = -1;
    let listViewExList = vec![
        rstr!("This"),
        rstr!("is"),
        rstr!("a"),
        rstr!("list view"),
        rstr!("with"),
        rstr!("disable"),
        rstr!("elements"),
        rstr!("amazing!"),
    ];

    let mut multiTextBoxText = [0u8; 256];
    multiTextBoxText[..14].clone_from_slice(b"Multi text box");
    let mut multiTextBoxEditMode = false;
    let mut colorPickerValue = Color::RED;

    let mut sliderValue = 50f32;
    let mut sliderBarValue = 60f32;
    let mut progressValue = 0.4;

    let mut forceSquaredChecked = false;

    let mut alphaValue = 0.5;

    let mut comboBoxActive = 1;

    let mut toggleGroupActive = 0;

    let mut viewScroll = rvec2(0, 0);
    //----------------------------------------------------------------------------------

    // Custom GUI font loading
    //Font font = LoadFontEx("fonts/rainyhearts16.ttf", 12, 0, 0);
    //GuiSetFont(font);

    let mut _exit_window = false;
    let mut showMessageBox = false;

    let mut textInput = vec![0u8; 256];
    let mut showTextInputBox = false;

    let mut textInputFileName = [0u8; 256];

    //--------------------------------------------------------------------------------------

    rl.set_target_fps(30);
    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> ()
    // Detect window close button or ESC key
    {
        //use raylib::consts::guiIconName::*;
        use raylib::consts::GuiControl::*;
        use raylib::consts::GuiControlProperty::*;
        use raylib::consts::GuiState::*;
        use raylib::consts::GuiDefaultProperty::*;
        use raylib::consts::GuiTextAlignment::*;

        // Update
        //----------------------------------------------------------------------------------

        #[cfg(not(target_arch = "wasm32"))]
        {
            _exit_window = rl.window_should_close();
        }

        if rl.is_key_pressed(crate::EXIT_KEY) {
            showMessageBox = !showMessageBox;
        }

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT_CONTROL) && rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S) {
            showTextInputBox = true;
        }

        if rl.is_file_dropped() {
            let droppedFiles = rl.load_dropped_files();

            if (droppedFiles.len() > 0) && droppedFiles[0].ends_with(".rgs") {
                rl.gui_load_style(Some(&CString::new(droppedFiles[0].as_bytes()).unwrap()));
            }

            rl.unload_dropped_files();
        }

        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);
        let hex = d.gui_get_style(DEFAULT, BACKGROUND_COLOR as i32);
        d.clear_background(Color::get_color(hex as u32));

        // raygui: controls drawing
        //----------------------------------------------------------------------------------
        if dropDown000EditMode || dropDown001EditMode {
            d.gui_lock();
        }
        //GuiDisable();

        // First GUI column
        //GuiSetStyle(CHECKBOX, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT);
        forceSquaredChecked = d.gui_check_box(
            rrect(25, 108, 15, 15),
            Some(rstr!("FORCE CHECK!")),
            forceSquaredChecked,
        );

        d.gui_set_style(TEXTBOX, TEXT_ALIGNMENT as i32, TEXT_ALIGN_CENTER as i32);
        if d.gui_spinner(
            rrect(25, 135, 125, 30),
            None,
            &mut spinner001Value,
            0,
            100,
            spinnerEditMode,
        ) {
            spinnerEditMode = !spinnerEditMode;
        }
        if d.gui_value_box(
            rrect(25, 175, 125, 30),
            None,
            &mut valueBox002Value,
            0,
            100,
            valueBoxEditMode,
        ) {
            valueBoxEditMode = !valueBoxEditMode;
        }
        d.gui_set_style(TEXTBOX, TEXT_ALIGNMENT as i32, TEXT_ALIGN_LEFT as i32);
        if d.gui_text_box(rrect(25, 215, 125, 30), &mut textBoxText, textBoxEditMode) {
            textBoxEditMode = !textBoxEditMode;
        }

        d.gui_set_style(BUTTON, TEXT_ALIGNMENT as i32, TEXT_ALIGN_CENTER as i32);

        let itext = d.gui_icon_text(RAYGUI_ICON_FILE_SAVE, Some(rstr!("Save File")));
        let itext = CString::new(itext).unwrap();
        if d.gui_button(rrect(25, 255, 125, 30), Some(&itext)) {
            showTextInputBox = true;
        }

        d.gui_group_box(rrect(25, 310, 125, 150), Some(rstr!("STATES")));

        d.gui_lock();
        d.gui_set_state(STATE_NORMAL);
        if d.gui_button(rrect(30, 320, 115, 30), Some(rstr!("NORMAL"))) {}
        d.gui_set_state(STATE_FOCUSED);
        if d.gui_button(rrect(30, 355, 115, 30), Some(rstr!("FOCUSED"))) {}
        d.gui_set_state(STATE_PRESSED);
        if d.gui_button(rrect(30, 390, 115, 30), Some(rstr!("#15#PRESSED"))) {}
        d.gui_set_state(STATE_DISABLED);
        if d.gui_button(rrect(30, 425, 115, 30), Some(rstr!("DISABLED"))) {}
        d.gui_set_state(STATE_NORMAL);
        d.gui_unlock();

        comboBoxActive = d.gui_combo_box(
            rrect(25, 470, 125, 30),
            Some(rstr!("ONE;TWO;THREE;FOUR")),
            comboBoxActive,
        );

        // NOTE: GuiDropdownBox must draw after any other control that can be covered on unfolding
        d.gui_set_style(
            DROPDOWNBOX,
            TEXT_ALIGNMENT as i32,
            TEXT_ALIGN_LEFT as i32,
        );
        if d.gui_dropdown_box(
            rrect(25, 65, 125, 30),
            Some(rstr!("#01#ONE;#02#TWO;#03#THREE;#04#FOUR")),
            &mut dropdownBox001Active,
            dropDown001EditMode,
        ) {
            dropDown001EditMode = !dropDown001EditMode;
        }

        d.gui_set_style(
            DROPDOWNBOX,
            TEXT_ALIGNMENT as i32,
            TEXT_ALIGN_CENTER as i32,
        );
        if d.gui_dropdown_box(
            rrect(25, 25, 125, 30),
            Some(rstr!("ONE;TWO;THREE")),
            &mut dropdownBox000Active,
            dropDown000EditMode,
        ) {
            dropDown000EditMode = !dropDown000EditMode;
        }

        // Second GUI column
        listViewActive = d.gui_list_view(
            rrect(165, 25, 140, 140),
            Some(rstr!(
                "Charmander;Bulbasaur;#18#Squirtel;Pikachu;Eevee;Pidgey"
            )),
            &mut listViewScrollIndex,
            listViewActive,
        );
        listViewExActive = d.gui_list_view_ex(
            rrect(165, 180, 140, 200),
            &listViewExList,
            &mut listViewExFocus,
            &mut listViewExScrollIndex,
            listViewExActive,
        );

        toggleGroupActive = d.gui_toggle_group(
            rrect(165, 400, 140, 25),
            Some(rstr!("#1#ONE\n#3#TWO\n#8#THREE\n#23#")),
            toggleGroupActive,
        );

        // Third GUI column
        if d.gui_text_box_multi(
            rrect(320, 25, 225, 140),
            &mut multiTextBoxText,
            multiTextBoxEditMode,
        ) {
            multiTextBoxEditMode = !multiTextBoxEditMode;
        }
        colorPickerValue = d.gui_color_picker(rrect(320, 185, 196, 192), colorPickerValue);

        sliderValue = d.gui_slider(
            rrect(355, 400, 165, 20),
            Some(rstr!("TEST")),
            Some(&rstr!("{:.2}", sliderValue as f32)),
            sliderValue,
            -50.0,
            100.0,
        );
        sliderBarValue = d.gui_slider_bar(
            rrect(320, 430, 200, 20),
            None,
            Some(&rstr!("{}", sliderBarValue)),
            sliderBarValue,
            0.0,
            100.0,
        );
        progressValue = d.gui_progress_bar(
            rrect(320, 460, 200, 20),
            None,
            None,
            progressValue,
            0.0,
            1.0,
        );

        // NOTE: View rectangle could be used to perform some scissor test
        let (_view, nextScroll) = d.gui_scroll_panel(
            rrect(560, 25, 100, 160),
            rrect(560, 25, 200, 400),
            viewScroll,
        );
        viewScroll = nextScroll.into();

        d.gui_status_bar(
            rrect(0, d.get_screen_height() - 20, d.get_screen_width(), 20),
            Some(rstr!("This is a status bar")),
        );

        alphaValue = d.gui_color_bar_alpha(rrect(320, 490, 200, 30), alphaValue);

        if showMessageBox {
            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::RAYWHITE.fade(0.8),
            );
            let itext = d.gui_icon_text(RAYGUI_ICON_EXIT, Some(rstr!("Close Window")));
            let itext = CString::new(itext).unwrap();
            let result = d.gui_message_box(
                rrect(
                    d.get_screen_width() / 2 - 125,
                    d.get_screen_height() / 2 - 50,
                    250,
                    100,
                ),
                Some(&itext),
                Some(rstr!("Do you really want to exit?")),
                Some(rstr!("Yes;No")),
            );

            if (result == 0) || (result == 2) {
                showMessageBox = false;
            } else if result == 1 {
                _exit_window = true;
            }
        }

        if showTextInputBox {
            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::RAYWHITE.fade(0.8),
            );
            let itext = unsafe { d.gui_icon_text(RAYGUI_ICON_FILE_SAVE, Some(rstr!("Save file as..."))) };
            let itext = CString::new(itext).unwrap();
            let result = d.gui_text_input_box(
                rrect(
                    d.get_screen_width() / 2 - 120,
                    d.get_screen_height() / 2 - 60,
                    240,
                    140,
                ),
                Some(&itext),
                Some(rstr!("Introduce a save file name")),
                Some(rstr!("Ok;Cancel")),
                &mut textInput,
            );

            if result == 1 {
                // TODO: Validate textInput value and save
                textInputFileName[..textInput.len()].clone_from_slice(&textInput);
            }

            if( result == 0) || (result == 1) || (result == 2) {
                showTextInputBox = false;
                textInput[0] = b'\0';
            }
        }

        d.gui_unlock();
    });
}
