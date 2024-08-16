extern crate raylib;
use raylib::prelude::*;
use std::{thread, time};
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    test_rslice(&opt);
    test_model_dropping(&opt);
    test_audio_dropping(&opt);
    test_font_dropping(&opt);
}

fn test_rslice(opt: &options::Opt) {
    let (_rl, _thread) = opt.open_window("Drop Allocs");
    let img = Image::gen_image_color(256, 256, Color::RED);
    let _pallet = img.extract_palette(16);
}

/// Checks that model files are droppable after window is closed
fn test_model_dropping(opt: &options::Opt) {
    let ten_millis = time::Duration::from_millis(10);
    {
        let (mut rl, thread) = opt.open_window("Drop Model");
        rl.load_model(&thread, "static/pbr/trooper.obj")
            .expect("couldn't load model");
    };
    thread::sleep(ten_millis);
    // Uncomment when we have actual meshes to unload
    // let mesh = {
    //     let (_rl, thread) = opt.open_window("Drop Mesh");
    //     Mesh::load_meshes(&thread, "static/pbr/trooper.obj").expect("couldn't load mesh");
    // };
    {
        let (mut rl, thread) = opt.open_window("Drop Anim");
        rl.load_model_animations(&thread, "static/guy/guy.iqm")
            .expect("couldn't load model");
    };

    thread::sleep(ten_millis);
}

/// Checks that audio files are droppable after window is closed
fn test_audio_dropping(opt: &options::Opt) {
    let ten_millis = time::Duration::from_millis(10);
    let ra = RaylibAudio::init_audio_device().expect("Failed to initialize audio");
    let w = {
        let (_, _thread) = raylib::init()
            .size(opt.width, opt.height)
            .title("Drop")
            .build();
        ra.new_wave("static/wave.ogg").expect("couldn't load wave")
    };
    thread::sleep(ten_millis);
    let _s = {
        let (_rl, _thread) = opt.open_window("Drop Sound");
        ra.new_sound("static/wave.ogg").expect("couldn't load wave")
    };
    thread::sleep(ten_millis);

    let _samples = w.load_samples();
    thread::sleep(ten_millis);

    // Broken on mac
    // let m = {
    //     let (_rl, thread) = opt.open_window("Drop Sound");
    //     // let m = Music::load_music_stream(&thread, "static/wave.ogg");
    //     let m = Music::load_music_stream(&thread, "static/wave.ogg").expect("couldn't load music");
    //     println!("music {:?}", m);
    //     drop(m);
    //     ()
    // };
    // thread::sleep(ten_millis);
}

/// checks that fonts can be dropped after window is closed
fn test_font_dropping(opt: &options::Opt) {
    {
        let (mut rl, thread) = raylib::init()
            .size(opt.width, opt.height)
            .title("Drop")
            .build();
        rl.load_font(&thread, "static/alagard.png")
            .expect("couldn't load font");
    };
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}
