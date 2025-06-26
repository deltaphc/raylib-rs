use raylib::prelude::*;
use ringbuf::{
    traits::{Consumer, Observer, RingBuffer},
    HeapRb,
};
use std::{
    cell::RefCell,
    env, f32,
    sync::{Arc, Mutex},
};

fn main() {
    // get file name
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage {} <music file>", args[0]);
    }
    let filename = args[1].as_str();

    // open window
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Music Player")
        .resizable()
        .vsync()
        .build();

    // open audio file
    let audio = raylib::core::audio::RaylibAudio::init_audio_device().unwrap();
    let music = audio.new_music(filename).unwrap();
    music.play_stream();

    let mut effects = vec![];
    let mut effects_control = vec![];

    // create effects
    for (k, key) in [
        (1, KeyboardKey::KEY_ONE),
        (2, KeyboardKey::KEY_TWO),
        (3, KeyboardKey::KEY_THREE),
    ] {
        let hall_switch = Arc::new(Mutex::new(RefCell::new(false)));
        let hall_switch_for_callback = Arc::clone(&hall_switch);
        let dist = k * 10000;
        let mut hall_buffer = HeapRb::<f32>::new(dist);
        let hall_callback = move |data: &mut [f32], _nb_channels: u32| {
            let hall_buffer = &mut hall_buffer;
            let hall_switch = hall_switch_for_callback.lock().unwrap();
            let delta = dist;
            let delta_alpha: f32 = 0.5;
            if *hall_switch.borrow_mut() {
                (0..data.len()).for_each(|idx| {
                    hall_buffer.push_overwrite(data[idx]);
                    while hall_buffer.occupied_len() > delta {
                        hall_buffer.try_pop().unwrap();
                    }
                    if hall_buffer.occupied_len() == delta {
                        data[idx] = data[idx] + delta_alpha * hall_buffer.try_pop().unwrap();
                    }
                });
            }
        };
        effects.push(hall_callback);
        effects_control.push((hall_switch, key));
    }

    let mut _keep_alive = vec![];
    for callback in effects.iter_mut() {
        _keep_alive.push(attach_audio_stream_processor_to_music(&music, callback));
    }

    // run main loop (use key to control music and effects)
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            let new_pos = f32::min(music.get_time_played() + 10.0, music.get_time_length());
            music.seek_stream(new_pos);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            let new_pos = f32::max(music.get_time_played() - 10.0, 0.0);
            music.seek_stream(new_pos);
        }
        for (switch, key) in effects_control.iter_mut() {
            if rl.is_key_pressed(*key) {
                let switch = switch.lock().unwrap();
                let mut b = switch.borrow_mut();
                *b = !(*b);
            }
        }

        music.update_stream();

        let mut d = rl.begin_drawing(&thread);
        let info_color = Color::get_color(0xffff00ff);
        let light_info_color = Color::get_color(0xa0a000ff);
        d.clear_background(Color::get_color(0x006040ff));
        d.draw_text(filename, 12, 42, 20, info_color);
        d.draw_fps(d.get_render_width() - 90, 12);
        let time_y = 80;
        let time_x = ((d.get_render_width() - 20) as f32
            * (music.get_time_played() / music.get_time_length())) as i32;
        d.draw_line(
            10,
            time_y,
            d.get_render_width() - 10,
            time_y,
            light_info_color,
        );
        d.draw_circle(time_x, time_y, 5.0, light_info_color);
        {
            let switch_states = effects_control
                .iter()
                .map(|(switch, _)| *RefCell::borrow(&switch.lock().unwrap()))
                .collect::<Vec<_>>();
            d.draw_text(
                &format!("Press <-, ->, 1,2,3...: {:?}", switch_states),
                12,
                90,
                20,
                light_info_color,
            );
        }
    }
}
