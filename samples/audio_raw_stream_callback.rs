use raylib::callbacks::audio_stream_callback::{
    set_audio_stream_callback, unset_audio_stream_callback,
};
use raylib::error::UpdateAudioStreamError;
use raylib::prelude::glam::vec2;
use raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT;
use raylib::prelude::*;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

const MAX_SAMPLES: usize = 512;
const MAX_SAMPLES_PER_UPDATE: i32 = 4096;

const SAMPLE_RATE: u32 = 44100;
const SAMPLE_RATE_HALVED: f32 = 22050.0;

struct PCMData {
    frequency: f32,
    audio_frequency: f32,
    old_frequency: f32,
    sine_idx: f32,
}

fn audio_callback_16_mono(samples: &mut [i16], cb_data: &Arc<Mutex<PCMData>>) {
    let mut data = cb_data.lock().unwrap();
    data.audio_frequency = data.frequency + (data.audio_frequency - data.frequency) * 0.95;
    let incr = data.audio_frequency / SAMPLE_RATE as f32;
    for sample in samples.iter_mut() {
        let v = 32000.0 * (2.0 * PI * data.sine_idx).sin();
        *sample = v as i16;
        data.sine_idx += incr;
        if data.sine_idx > 1.0 {
            data.sine_idx -= 1.0;
        }
    }
}

fn audio_callback_16_stereo(samples: &mut [i16], cb_data: &Arc<Mutex<PCMData>>) {
    let mut data = cb_data.lock().unwrap();
    data.audio_frequency = data.frequency + (data.audio_frequency - data.frequency) * 0.95;
    let incr = data.audio_frequency / SAMPLE_RATE as f32;
    for frame in samples.chunks_exact_mut(2) {
        let sample = ((2.0 * PI * data.sine_idx).sin() * 32000.0) as i16;
        frame[0] = sample;
        frame[1] = sample;
        data.sine_idx += incr;
        if data.sine_idx > 1.0 {
            data.sine_idx -= 1.0;
        }
    }
}

fn audio_callback_8_mono(samples: &mut [u8], cb_data: &Arc<Mutex<PCMData>>) {
    let mut data = cb_data.lock().unwrap();
    data.audio_frequency = data.frequency + (data.audio_frequency - data.frequency) * 0.95;
    let incr = data.audio_frequency / SAMPLE_RATE as f32;
    for sample in samples.iter_mut() {
        let x = (2.0 * PI * data.sine_idx).sin();
        let v = (x * 127.0 + 128.0).clamp(0.0, 255.0);
        *sample = v as u8;
        data.sine_idx += incr;
        if data.sine_idx > 1.0 {
            data.sine_idx -= 1.0;
        }
    }
}

fn audio_callback_32_mono(samples: &mut [f32], cb_data: &Arc<Mutex<PCMData>>) {
    let mut data = cb_data.lock().unwrap();
    data.audio_frequency = data.frequency + (data.audio_frequency - data.frequency) * 0.95;
    let incr = data.audio_frequency / SAMPLE_RATE as f32;
    for sample in samples.iter_mut() {
        *sample = (2.0 * PI * data.sine_idx).sin();
        data.sine_idx += incr;
        if data.sine_idx > 1.0 {
            data.sine_idx -= 1.0;
        }
    }
}

fn main() {

    let screen_width = 800;
    let screen_height = 450;

    let (mut raylib_handle, raylib_thread) = init()
        .size(screen_width, screen_height)
        .title("raylib [audio] example - raw audio streaming")
        .build();

    raylib_handle.set_target_fps(30);

    let raylib_audio = RaylibAudio::init_audio_device().unwrap();

    error_cases_detour_test(&raylib_audio);  // run the Error handling cases

    raylib_audio.set_audio_stream_buffer_size_default(MAX_SAMPLES_PER_UPDATE);

    let pcm_data_state = Arc::new(Mutex::new(PCMData {
        frequency: 440.0,
        audio_frequency: 440.0,
        old_frequency: 1.0,
        sine_idx: 0.0,
    }));
    let cb_data = Arc::clone(&pcm_data_state);
    // TODO: may be more appropriate to move copy pasted test config patterns to the raylib-test location
    // let stream = raylib_audio.new_audio_stream(SAMPLE_RATE, 16, 1);
    let stream = raylib_audio.new_audio_stream(SAMPLE_RATE, 16, 2);
    // let stream = raylib_audio.new_audio_stream(SAMPLE_RATE, 8, 1);
    // let stream = raylib_audio.new_audio_stream(SAMPLE_RATE, 32, 1);
    set_audio_stream_callback(&stream, move |buffer| {
        // audio_callback_16_mono(buffer, &cb_data);
        audio_callback_16_stereo(buffer, &cb_data);
        // audio_callback_8_mono(buffer, &cb_data);
        // audio_callback_32_mono(buffer, &cb_data);
    })
    .unwrap();
    let mut data: [i16; MAX_SAMPLES] = [0; MAX_SAMPLES];
    stream.play();
    let mut position = vec2(0.0, 0.0);
    while !raylib_handle.window_should_close() {
        let mouse_position = raylib_handle.get_mouse_position();
        if raylib_handle.is_mouse_button_down(MOUSE_BUTTON_LEFT) {
            pcm_data_state.lock().unwrap().frequency = 40.0 + mouse_position.y;
            let invert_mouse_x_position = -1.0;
            let pan = invert_mouse_x_position * mouse_position.x / screen_width as f32;
            stream.set_pan(pan);
        }
        {
            let mut pcm_data = pcm_data_state.lock().unwrap();
            if pcm_data.frequency != pcm_data.old_frequency {
                let mut wave_length = (SAMPLE_RATE_HALVED / pcm_data.frequency) as usize;
                if wave_length > MAX_SAMPLES / 2 {
                    wave_length = MAX_SAMPLES / 2;
                }
                if wave_length < 1 {
                    wave_length = 1;
                }
                for i in 0..wave_length * 2 {
                    data[i] = ((2.0 * PI * i as f32 / wave_length as f32).sin()
                        * 32000f32) as i16;
                }
                for j in wave_length * 2..MAX_SAMPLES {
                    data[j] = 0;
                }
                pcm_data.old_frequency = pcm_data.frequency;
            }
        }
        let mut draw_handle = raylib_handle.begin_drawing(&raylib_thread);
        draw_handle.clear_background(Color::RAYWHITE);

        {
            let st = pcm_data_state.lock().unwrap();
            draw_handle.draw_text(
                &format!("sine frequency: {}", st.frequency as i32),
                screen_width - 220,
                10,
                20,
                Color::RED,
            );
        }

        draw_handle.draw_text(
            "click mouse button to change frequency or pan",
            10,
            10,
            20,
            Color::DARKGRAY,
        );

        for i in 0..screen_width {
            position.x = i as f32;
            position.y = 250.0
                + 50.0 * data[i as usize * MAX_SAMPLES / screen_width as usize] as f32 / 32000f32;
            draw_handle.draw_pixel_v(position, Color::RED);
        }
    }
}

fn error_cases_detour_test(audio: &RaylibAudio) {
    // 1) mismatched format: 16-bit stream + f32 callback -> SampleSizeMismatch
    let stream_mismatch = audio.new_audio_stream(SAMPLE_RATE, 16, 1);
    type Not16Bit = f32;
    let mismatch_result =
        set_audio_stream_callback::<Not16Bit, _>(&stream_mismatch, |_buf: &mut [Not16Bit]| {});
    println!("mismatch types (16-bit stream =/= Not16Bit callback): {mismatch_result:?}");
    assert!(matches!(
        mismatch_result,
        Err(UpdateAudioStreamError::SampleSizeMismatch { .. })
    ));

    // 2) successful registration: 16-bit stream + i16 callback → Ok and callback slot becomes busy
    let stream = audio.new_audio_stream(SAMPLE_RATE, 16, 1);
    let result = set_audio_stream_callback::<i16, _>(&stream, |_buf: &mut [i16]| {});
    println!("normal callback registration (16-bit + i16): {result:?}");
    assert!(result.is_ok());

    // 3) second registration while busy -> CallbackSlotBusy
    let stream_busy = audio.new_audio_stream(SAMPLE_RATE, 16, 1);
    let busy_result = set_audio_stream_callback::<i16, _>(&stream_busy, |_buf: &mut [i16]| {});
    println!("registration attempt while busy: {busy_result:?}");
    assert!(matches!(
        busy_result,
        Err(UpdateAudioStreamError::CallbackSlotBusy)
    ));

    // 4) cleanup
    unset_audio_stream_callback(&stream);
}
