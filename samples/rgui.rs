use raylib::prelude::GuiControl::*;
use raylib::prelude::GuiControlProperty::*;
use raylib::prelude::GuiDefaultProperty::*;
use raylib::prelude::GuiIconName::*;
use raylib::prelude::GuiSliderProperty::*;
use raylib::prelude::GuiState::*;
use raylib::prelude::GuiTextAlignment::*;
use raylib::prelude::GuiTextAlignmentVertical::*;
use raylib::prelude::GuiTextWrapMode::*;
use raylib::prelude::KeyboardKey::*;
use raylib::prelude::*;

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
pub fn main() {
    // Initialization
    //---------------------------------------------------------------------------------------
    let screenWidth = 960;
    let screenHeight = 560;

    let (mut rl, thread) = raylib::init()
        .width(screenWidth)
        .height(screenHeight)
        .title("raygui - controls test suite")
        .build();

    rl.set_exit_key(None);

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

    let mut textBoxText = String::from("Text box");
    let mut textBoxEditMode = false;

    let mut textBoxMultiText = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\nDuis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat Nonea pariatur.\n\nThisisastringlongerthanexpectedwithoutspacestotestcharbreaksforthosecases,checkingifworkingasexpected.\n\nExcepteur slet occaecatcupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
    let mut textBoxMultiEditMode = false;

    let mut listViewScrollIndex = 0;
    let mut listViewActive = -1;

    let mut listViewExScrollIndex = 0;
    let mut listViewExActive = 2;
    let mut listViewExFocus = -1;
    let mut listViewExList = [
        "This",
        "is",
        "a",
        "list view",
        "with",
        "disable",
        "elements",
        "amazing!",
    ];

    let colorPickerValue = Color::RED;

    let mut sliderValue = 50.0;
    let mut sliderBarValue = 60.0;
    let mut progressValue = 0.1;

    let mut forceSquaredChecked = false;

    let mut alphaValue = 0.5;

    //let comboBoxActive= 1;
    let mut visualStyleActive = 0;
    let mut prevVisualStyleActive = 0;

    let mut toggleGroupActive = 0;
    let mut toggleSliderActive = 0;

    let viewScroll = Vector2::new(0.0, 0.0);
    //----------------------------------------------------------------------------------

    // Custom GUI font loading
    //Font font = LoadFontEx("fonts/rainyhearts16.ttf", 12, 0, 0);
    //GuiSetFont(font);

    let mut exitWindow = false;
    let mut showMessageBox = false;

    let mut textInput = String::new();
    let mut textInputFileName = String::new();
    let mut showTextInputBox = false;

    let mut alpha = 1.0;

    // DEBUG: Testing how those two properties affect all controls!
    //rl.gui_set_style(DEFAULT, TEXT_PADDING, 0);
    //rl.gui_set_style(DEFAULT, TEXT_ALIGNMENT, TEXT_ALIGN_CENTER);

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    while !exitWindow
    // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        exitWindow = rl.window_should_close();

        if rl.is_key_pressed(KEY_ESCAPE) {
            showMessageBox = !showMessageBox
        };

        if rl.is_key_down(KEY_LEFT_CONTROL) && rl.is_key_pressed(KEY_S) {
            showTextInputBox = true
        };

        if rl.is_file_dropped() {
            let droppedFiles = rl.load_dropped_files();

            let paths = droppedFiles.paths();
            if droppedFiles.count > 0 && paths[0].ends_with(".rgs") {
                rl.gui_load_style(paths[0])
            };
        }

        //alpha -= 0.002;
        if alpha < 0.0 {
            alpha = 0.0;
        }
        if rl.is_key_pressed(KEY_SPACE) {
            {
                alpha = 1.0
            };
        }

        //progressValue += 0.002;
        if rl.is_key_pressed(KEY_LEFT) {
            {
                progressValue -= 0.1
            };
        } else if rl.is_key_pressed(KEY_RIGHT) {
            {
                progressValue += 0.1
            };
        }
        if progressValue > 1.0 {
            progressValue = 1.0;
        } else if progressValue < 0.0 {
            {
                progressValue = 0.0
            };
        }

        if visualStyleActive != prevVisualStyleActive {
            rl.gui_load_style_default();

            // switch (visualStyleActive)
            // {
            //     0 => break;      // Default style
            //     1 => rl.gui_load_styleJungle();
            //     2 => rl.gui_load_styleLavanda();
            //     3 => rl.gui_load_styleDark();
            //     4 => rl.gui_load_styleBluish();
            //     5 => rl.gui_load_styleCyber();
            //     6 => rl.gui_load_styleTerminal();
            //     _: break;
            // }

            rl.gui_set_style(LABEL, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT as i32);

            prevVisualStyleActive = visualStyleActive;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::get_color(
            d.gui_get_style(DEFAULT, BACKGROUND_COLOR) as u32
        ));
        d.gui_set_alpha(alpha);

        // raygui: controls drawing
        //----------------------------------------------------------------------------------
        // Check all possible events that require d.gui_lock
        if dropDown000EditMode || dropDown001EditMode {
            d.gui_lock()
        };

        // First GUI column
        //rl.gui_set_style(CHECKBOX, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT);
        d.gui_check_box(
            Rectangle::new(25.0, 108.0, 15.0, 15.0),
            "FORCE CHECK!",
            &mut forceSquaredChecked,
        );

        d.gui_set_style(TEXTBOX, TEXT_ALIGNMENT, TEXT_ALIGN_CENTER as i32);
        //d.gui_set_style(VALUEBOX, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT);
        if d.gui_spinner(
            Rectangle::new(25.0, 135.0, 125.0, 30.0),
            "",
            &mut spinner001Value,
            0,
            100,
            spinnerEditMode,
        ) {
            spinnerEditMode = !spinnerEditMode
        };
        if d.gui_value_box(
            Rectangle::new(25.0, 175.0, 125.0, 30.0),
            "",
            &mut valueBox002Value,
            0,
            100,
            valueBoxEditMode,
        ) {
            valueBoxEditMode = !valueBoxEditMode
        };
        d.gui_set_style(TEXTBOX, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT as i32);
        if d.gui_text_box(
            Rectangle::new(25.0, 215.0, 125.0, 30.0),
            &mut textBoxText,
            textBoxEditMode,
        ) {
            textBoxEditMode = !textBoxEditMode
        };

        d.gui_set_style(BUTTON, TEXT_ALIGNMENT, TEXT_ALIGN_CENTER as i32);

        let gui_icon_text = d.gui_icon_text(ICON_FILE_SAVE, "Save File");
        if d.gui_button(
            Rectangle::new(25.0, 255.0, 125.0, 30.0),
            gui_icon_text.as_str(),
        ) {
            showTextInputBox = true
        };

        d.gui_group_box(Rectangle::new(25.0, 310.0, 125.0, 150.0), "STATES");
        //d.gui_lock();
        d.gui_set_state(STATE_NORMAL);
        if d.gui_button(Rectangle::new(30.0, 320.0, 115.0, 30.0), "NORMAL") {}
        d.gui_set_state(STATE_FOCUSED);
        if d.gui_button(Rectangle::new(30.0, 355.0, 115.0, 30.0), "FOCUSED") {}
        d.gui_set_state(STATE_PRESSED);
        if d.gui_button(Rectangle::new(30.0, 390.0, 115.0, 30.0), "#15#PRESSED") {}
        d.gui_set_state(STATE_DISABLED);
        if d.gui_button(Rectangle::new(30.0, 425.0, 115.0, 30.0), "DISABLED") {}
        d.gui_set_state(STATE_NORMAL);
        //d.gui_unlock();

        d.gui_combo_box(
            Rectangle::new(25.0, 480.0, 125.0, 30.0),
            "default;Jungle;Lavanda;Dark;Bluish;Cyber;Terminal",
            &mut visualStyleActive,
        );

        // NOTE: d.gui_dropdown_box must draw after any other control that can be covered on unfolding
        d.gui_unlock();
        d.gui_set_style(DROPDOWNBOX, TEXT_PADDING, 4);
        d.gui_set_style(DROPDOWNBOX, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT as i32);
        if d.gui_dropdown_box(
            Rectangle::new(25.0, 65.0, 125.0, 30.0),
            "#01#ONE;#02#TWO;#03#THREE;#04#FOUR",
            &mut dropdownBox001Active,
            dropDown001EditMode,
        ) {
            dropDown001EditMode = !dropDown001EditMode
        };
        d.gui_set_style(DROPDOWNBOX, TEXT_ALIGNMENT, TEXT_ALIGN_CENTER as i32);
        d.gui_set_style(DROPDOWNBOX, TEXT_PADDING, 0);

        if d.gui_dropdown_box(
            Rectangle::new(25.0, 25.0, 125.0, 30.0),
            "ONE;TWO;THREE",
            &mut dropdownBox000Active,
            dropDown000EditMode,
        ) {
            dropDown000EditMode = !dropDown000EditMode
        };

        // Second GUI column
        d.gui_list_view(
            Rectangle::new(165.0, 25.0, 140.0, 124.0),
            "Charmander;Bulbasaur;#18#Squirtel;Pikachu;Eevee;Pidgey",
            &mut listViewScrollIndex,
            &mut listViewActive,
        );
        d.gui_list_view_ex(
            Rectangle::new(165.0, 162.0, 140.0, 184.0),
            listViewExList.iter(),
            &mut listViewExScrollIndex,
            &mut listViewExActive,
            &mut listViewExFocus,
        );

        //GuiToggle(Rectangle::new( 165, 400, 140, 25 ), "#1#ONE", &toggleGroupActive);
        d.gui_toggle_group(
            Rectangle::new(165.0, 360.0, 140.0, 24.0),
            "#1#ONE\n#3#TWO\n#8#THREE\n#23#",
            &mut toggleGroupActive,
        );
        //d.gui_disable();
        d.gui_set_style(SLIDER, SLIDER_PADDING, 2);
        d.gui_toggle_slider(
            Rectangle::new(165.0, 480.0, 140.0, 30.0),
            "ON;OFF",
            &mut toggleSliderActive,
        );
        d.gui_set_style(SLIDER, SLIDER_PADDING, 0);

        // Third GUI column
        d.gui_panel(Rectangle::new(320.0, 25.0, 225.0, 140.0), "Panel Info");
        d.gui_color_picker(
            Rectangle::new(320.0, 185.0, 196.0, 192.0),
            "",
            &colorPickerValue,
        );

        //d.gui_disable();
        d.gui_slider(
            Rectangle::new(355.0, 400.0, 165.0, 20.0),
            "TEST",
            format!("{}", sliderValue).as_str(),
            &mut sliderValue,
            -50.0,
            100.0,
        );
        d.gui_slider_bar(
            Rectangle::new(320.0, 430.0, 200.0, 20.0),
            "",
            format!("{}", sliderBarValue).as_str(),
            &mut sliderBarValue,
            0.0,
            100.0,
        );

        d.gui_progress_bar(
            Rectangle::new(320.0, 460.0, 200.0, 20.0),
            "",
            format!("{}", (progressValue * 100.0)).as_str(),
            &mut progressValue,
            0.0,
            1.0,
        );
        d.gui_enable();

        // NOTE: View rectangle could be used to perform some scissor test
        let view = Rectangle::new(0.0, 0.0, 0.0, 0.0);
        d.gui_scroll_panel(
            Rectangle::new(560.0, 25.0, 102.0, 354.0),
            "",
            Rectangle::new(560.0, 25.0, 300.0, 1200.0),
            &viewScroll,
            &view,
        );

        let mouseCell = Vector2::new(0.0, 0.0);
        d.gui_grid(
            Rectangle::new(560.0, 25.0 + 180.0 + 195.0, 100.0, 120.0),
            "",
            20.0,
            3,
        );

        d.gui_color_bar_alpha(
            Rectangle::new(320.0, 490.0, 200.0, 30.0),
            "",
            &mut alphaValue,
        );

        d.gui_set_style(DEFAULT, TEXT_ALIGNMENT_VERTICAL, TEXT_ALIGN_TOP as i32); // WARNING: Word-wrap does not work as expected in case of no-top alignment
        d.gui_set_style(DEFAULT, TEXT_WRAP_MODE, TEXT_WRAP_WORD as i32); // WARNING: If wrap mode enabled, text editing is not supported
        if d.gui_text_box(
            Rectangle::new(678.0, 25.0, 258.0, 492.0),
            &mut textBoxMultiText,
            textBoxMultiEditMode,
        ) {
            textBoxMultiEditMode = !textBoxMultiEditMode
        };
        d.gui_set_style(DEFAULT, TEXT_WRAP_MODE, TEXT_WRAP_NONE as i32);
        d.gui_set_style(DEFAULT, TEXT_ALIGNMENT_VERTICAL, TEXT_ALIGN_MIDDLE as i32);

        d.gui_set_style(DEFAULT, TEXT_ALIGNMENT, TEXT_ALIGN_LEFT as i32);
        d.gui_status_bar(
            Rectangle::new(
                0.0,
                d.get_screen_height() as f32 - 20.0,
                d.get_screen_width() as f32,
                20.0,
            ),
            "This is a status bar",
        );
        d.gui_set_style(DEFAULT, TEXT_ALIGNMENT, TEXT_ALIGN_CENTER as i32);
        //d.gui_set_style(STATUSBAR, TEXT_INDENTATION, 20);

        if showMessageBox {
            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::RAYWHITE.fade(0.8),
            );
            let gui_icon_text = d.gui_icon_text(ICON_EXIT, "Close Window");
            let result = d.gui_message_box(
                Rectangle::new(
                    (d.get_screen_width() / 2 - 125) as f32,
                    (d.get_screen_height() / 2 - 50) as f32,
                    250.0,
                    100.0,
                ),
                gui_icon_text.as_str(),
                "Do you really want to exit?",
                "Yes;No",
            );

            if result == 0 || (result == 2) {
                showMessageBox = false
            } else if result == 1 {
                exitWindow = true
            };
        }

        if showTextInputBox {
            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::RAYWHITE.fade(0.8),
            );
            let mut _act = true;
            let gui_icon_text = d.gui_icon_text(ICON_FILE_SAVE, "Save file as...");
            let result = d.gui_text_input_box(
                Rectangle::new(
                    (d.get_screen_width() / 2 - 120) as f32,
                    (d.get_screen_height() / 2 - 60) as f32,
                    240.0,
                    140.0,
                ),
                gui_icon_text.as_str(),
                "Introduce output file name:",
                "Ok;Cancel",
                &mut textInput,
                255,
                &mut _act,
            );

            if result == 1 {
                // TODO: Validate textInput value and save
                textInputFileName = textInput.clone();
            }

            if result == 0 || (result == 1 || (result == 2)) {
                showTextInputBox = false;
                textInput.truncate(0);
            }
        }
    }
}
