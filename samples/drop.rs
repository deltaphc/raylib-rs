extern crate raylib;
use raylib::core::*;
use raylib::ffi;
use std::ffi::CString;
use std::{thread, time};
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    test_shader_dropping(&opt);
    // test_model_dropping(&opt);
    // test_audio_dropping(&opt);
    // test_font_dropping(&opt);
}

/// Checks that shader files are droppable after window is closed
fn test_shader_dropping(opt: &options::Opt) {
    let ten_millis = time::Duration::from_millis(10);
    let v = {
        let (mut rl, thread) = opt.open_window("Drop Shader");
    rl.load_shader(&thread, None, Some("drop-static/shader/pbr.fs")).expect("shader didn't load")
    };
}

/// Checks that model files are droppable after window is closed
fn test_model_dropping(opt: &options::Opt) {
    let ten_millis = time::Duration::from_millis(10);
    let m = {
        let (mut rl, thread) = opt.open_window("Drop Model");
        rl.load_model(&thread, "drop-static/pbr/trooper.obj").expect("couldn't load model");
    };
    thread::sleep(ten_millis);
    // Uncomment when we have actual meshes to unload
    // let mesh = {
    //     let (_rl, thread) = opt.open_window("Drop Mesh");
    //     Mesh::load_meshes(&thread, "drop-static/pbr/trooper.obj").expect("couldn't load mesh");
    // };
    let anim = {
        let (_rl, _thread) = opt.open_window("Drop Anim");
        ModelAnimation::load_model_animations("drop-static/guy/guy.iqm").expect("couldn't load model");
    };
    
    thread::sleep(ten_millis);
}

/// Checks that audio files are droppable after window is closed
fn test_audio_dropping(opt: &options::Opt) {
    let ten_millis = time::Duration::from_millis(10);
    let w = {
        let (_, thread) = raylib::init()
            .size(opt.width, opt.height)
            .title("Drop")
            .build();
        Wave::load_wave("drop-static/wave.ogg").expect("couldn't load wave");
    };
    thread::sleep(ten_millis);
    let s = {
        let (_rl, _thread) = opt.open_window("Drop Sound");
        Sound::load_sound("drop-static/wave.ogg").expect("couldn't load wave");
    };
    thread::sleep(ten_millis);
    // Broken on mac
    // let m = {
    //     let (_rl, thread) = opt.open_window("Drop Sound");
    //     // let m = Music::load_music_stream(&thread, "drop-static/wave.ogg");
    //     let m = Music::load_music_stream(&thread, "drop-static/wave.ogg").expect("couldn't load music");
    //     println!("music {:?}", m);
    //     drop(m);
    //     ()
    // };
    // thread::sleep(ten_millis);
}

/// checks that fonts can be dropped after window is closed
fn test_font_dropping(opt: &options::Opt) {
    let _f = {
        let (mut rl, thread) = raylib::init()
            .size(opt.width, opt.height)
            .title("Drop")
            .build();
        rl.load_font(&thread, "drop-static/alagard.png")
            .expect("couldn't load font");
    };
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}
