/*******************************************************************************************
*
*   raygui - image exporter
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
********************************************************************************************/

use raylib::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;


pub fn bytes_to_str(raw: &[u8]) -> &str {
    std::str::from_utf8(raw.split(|b| *b == b'\0').next().unwrap()).expect("not utf-8")
}

    //------------------------------------------------------------------------------------
    // Program main entry point
    //------------------------------------------------------------------------------------
pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raygui - image exporter");


    // GUI controls initialization
    //----------------------------------------------------------------------------------
    let  windowBoxRec  = rrect(screen_width / 2 - 110,  screen_height / 2 - 100,  220,  190);
    let mut windowBoxActive = false;

    let mut fileFormatActive = 0;
    let  fileFormatTextList = ["IMAGE (.png)", "DATA (.raw)", "CODE (.h)"];
    let displayFileFormatTextList = fileFormatTextList.join(";");

    let mut pixelFormatActive = 0;
    let  pixelFormatTextList = ["GRAYSCALE", "GRAY ALPHA", "R5G6B5", "R8G8B8", "R5G5B5A1", "R4G4B4A4", "R8G8B8A8"];
    let displayPixelFormatTextList = pixelFormatTextList.join(";");

    let mut textBoxEditMode = false;
    let  mut fileName = [0u8; 32];
    fileName[..8].clone_from_slice(b"untitled");
    //--------------------------------------------------------------------------------------

    let mut image = Image::gen_image_color(256, 256, Color::BLACK);
    image.draw_text( "drop image into window", 0, 0, 16, Color::WHITE);
    let mut  texture = rl.load_texture_from_image(thread, &image).unwrap();

    let mut imageLoaded = true;
    let mut imageScale = 1.0;
    let mut imageRec = Rectangle::default();

    let mut btnExport = false;

    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        let fileNameStr = bytes_to_str(&fileName);
        // Update
        //----------------------------------------------------------------------------------
        if rl.is_file_dropped()
        {
            let droppedFiles = rl.get_dropped_files();

            if droppedFiles.len() == 1
            {

                if let Ok(imTemp) = Image::load_image(&droppedFiles[0]) 

                {
                    image = imTemp;

                    texture = rl.load_texture_from_image(thread, &image).unwrap();

                    imageLoaded = true;
                    pixelFormatActive = image.format - 1;

                    if texture.height() > texture.width()
                        {

                            imageScale = (screen_height - 100) as f32 / texture.height() as f32;
                        }
                    else
                        {
                            imageScale = (screen_width - 100) as f32 / texture.width() as f32;
                        }
                }
            }

            rl.clear_dropped_files();
        }

        if btnExport
        {
            if imageLoaded
            {
                image.set_format(unsafe { std::mem::transmute(pixelFormatActive + 1)});

                if fileFormatActive == 0 // PNG
                {
                    if Path::new(&fileNameStr).extension().map_or(true, |e| e != "png")
                        {
                            let next = format!("{}.png", &fileNameStr);
                            fileName[..next.len()].copy_from_slice(next.as_bytes());
                        }
                    let exportName = bytes_to_str(&fileName);
                    image.export_image(exportName);
                }
                else if fileFormatActive == 1 // RAW
                {
                    if Path::new(&fileNameStr).extension().map_or(true, |e| e != "raw")
                        {
                            let next = format!("{}.raw", &fileNameStr);
                            fileName[..next.len()].copy_from_slice(next.as_bytes()); // No extension providedsdw
                        }
                    
                    let fileNameTemp = bytes_to_str(&fileName);
                    let dataSize = image.get_pixel_data_size();
                    let mut file = File::create(fileNameTemp).expect("failed to open raw file");
                    let data = image.get_image_data();
                    let contents = unsafe {
                        std::slice::from_raw_parts(data.as_ptr() as *const u8, dataSize)
                    };
                    file.write_all(contents).expect("failed to write file");
                }
                else if fileFormatActive == 2 // CODE
                {
                    image.export_image_as_code(&fileNameStr);
                }
            }

            windowBoxActive = false;
        }

        if imageLoaded
        {
            imageScale += rl.get_mouse_wheel_move() as f32 * 0.05; // Image scale control
            if imageScale <= 0.1
            {
                imageScale = 0.1;

            }
            else if imageScale >= 5.0
            {

                imageScale = 5.0;
            }

            imageRec = rrect( (screen_width / 2) as f32 - image.width() as f32 * imageScale / 2.0,
                                   (screen_height / 2) as f32 - image.height() as f32 * imageScale / 2.0,
                                   image.width() as f32 * imageScale, image.height() as f32 * imageScale);
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::RAYWHITE);

        if texture.id > 0
        {
            d.draw_texture_ex(&texture, rvec2(screen_width / 2 - (texture.width() as f32 * imageScale / 2.0) as i32,  screen_height / 2 - (texture.height() as f32 * imageScale / 2.0) as i32), 0.0, imageScale, Color::WHITE);

            d.draw_rectangle_lines_ex(imageRec, 1, if imageRec.check_collision_point_rec(d.get_mouse_position()) {Color::RED } else { Color::DARKGRAY});
            d.draw_text(&format!("SCALE: {:.2}", imageScale * 100.0), 20, screen_height - 40, 20, Color::get_color(d.gui_get_style(raylib::consts::GuiControl::DEFAULT, raylib::consts::GuiDefaultProperty::LINE_COLOR as u32)));
        }
        else
        {
            d.draw_text("DRAG & DROP YOUR IMAGE!", 350, 200, 10, Color::DARKGRAY);
            d.gui_disable();
        }

        if d.gui_button(rrect(screen_width - 170, screen_height - 50, 150, 30), Some(rstr!("Image Export")))
            {

                windowBoxActive = true;
            }
        d.gui_enable();

        // Draw window box: windowBoxName
        //-----------------------------------------------------------------------------
        if windowBoxActive
        {
            d.draw_rectangle(0, 0, screen_width, screen_height, Color::get_color(d.gui_get_style(raylib::consts::GuiControl::DEFAULT, raylib::consts::GuiDefaultProperty::BACKGROUND_COLOR as u32)).fade( 0.7));
            windowBoxActive = !d.gui_window_box(rrect(windowBoxRec.x, windowBoxRec.y, 220, 190), Some(rstr!("Image Export Options")));

            d.gui_label(rrect(windowBoxRec.x + 10.0, windowBoxRec.y + 35.0, 60, 25), Some(rstr!("File format:")));
            fileFormatActive = d.gui_combo_box(rrect(windowBoxRec.x + 80.0, windowBoxRec.y + 35.0, 130, 25), Some(&rstr!("{}", displayFileFormatTextList)), fileFormatActive);
            d.gui_label(rrect(windowBoxRec.x + 10.0, windowBoxRec.y + 70.0, 63, 25), Some(rstr!("Pixel format:")));
            pixelFormatActive = d.gui_combo_box(rrect(windowBoxRec.x + 80.0, windowBoxRec.y + 70.0, 130, 25), Some(&rstr!("{}", displayPixelFormatTextList)), pixelFormatActive);
            d.gui_label(rrect(windowBoxRec.x + 10.0, windowBoxRec.y + 105.0, 50, 25), Some(rstr!("File name:")));
            if d.gui_text_box(rrect(windowBoxRec.x + 80.0, windowBoxRec.y + 105.0, 130, 25), &mut fileName, textBoxEditMode)
                {
                    textBoxEditMode = !textBoxEditMode;

                }

            btnExport = d.gui_button(rrect(windowBoxRec.x + 10.0, windowBoxRec.y + 145.0, 200, 30), Some(rstr!("Export Image")));
        }
        else
        {

            btnExport = false;
        }

        if btnExport
        {

            d.draw_text("Image exported!", 20, screen_height - 20, 20,Color::RED);
        }

        //-----------------------------------------------------------------------------

    });
}
