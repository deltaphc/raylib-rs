#![allow(non_camel_case_types)]

use crate::{audio::AudioStream, ffi, RaylibHandle};
pub use raylib_sys::TraceLogLevel;
use std::{
    borrow::Cow,
    convert::TryInto,
    ffi::{c_char, c_int, c_void, CStr, CString},
    mem::{size_of, transmute},
    pin::Pin,
    ptr::null_mut,
    slice::from_raw_parts_mut,
    sync::atomic::{AtomicUsize, Ordering},
};
mod stream_processor_with_user_data_wrapper;
use super::audio::Music;
use stream_processor_with_user_data_wrapper::*;

type TraceLogCallback = unsafe extern "C" fn(*mut i8, *const i8, ...);
extern "C" {
    fn SetTraceLogCallback(cb: Option<TraceLogCallback>);
}

type RustTraceLogCallback = fn(TraceLogLevel, &str);
type RustSaveFileDataCallback = fn(&str, &[u8]) -> bool;
type RustLoadFileDataCallback = fn(&str) -> Vec<u8>;
type RustSaveFileTextCallback = fn(&str, &str) -> bool;
type RustLoadFileTextCallback = fn(&str) -> String;
type RustAudioStreamCallback = fn(&[u8]);

static TRACE_LOG_CALLBACK: AtomicUsize = AtomicUsize::new(0);
static SAVE_FILE_DATA_CALLBACK: AtomicUsize = AtomicUsize::new(0);
static LOAD_FILE_DATA_CALLBACK: AtomicUsize = AtomicUsize::new(0);
static SAVE_FILE_TEXT_CALLBACK: AtomicUsize = AtomicUsize::new(0);
static LOAD_FILE_TEXT_CALLBACK: AtomicUsize = AtomicUsize::new(0);
static AUDIO_STREAM_CALLBACK: AtomicUsize = AtomicUsize::new(0);

fn trace_log_callback() -> Option<RustTraceLogCallback> {
    debug_assert!(size_of::<RustTraceLogCallback>() == size_of::<usize>());
    unsafe { transmute(TRACE_LOG_CALLBACK.load(Ordering::Relaxed)) }
}

fn save_file_data_callback() -> Option<RustSaveFileDataCallback> {
    debug_assert!(size_of::<RustSaveFileDataCallback>() == size_of::<usize>());
    unsafe { transmute(SAVE_FILE_DATA_CALLBACK.load(Ordering::Relaxed)) }
}

fn load_file_data_callback() -> Option<RustLoadFileDataCallback> {
    debug_assert!(size_of::<RustLoadFileDataCallback>() == size_of::<usize>());
    unsafe { transmute(LOAD_FILE_DATA_CALLBACK.load(Ordering::Relaxed)) }
}

fn save_file_text_callback() -> Option<RustSaveFileTextCallback> {
    debug_assert!(size_of::<RustSaveFileTextCallback>() == size_of::<usize>());
    unsafe { transmute(SAVE_FILE_TEXT_CALLBACK.load(Ordering::Relaxed)) }
}

fn load_file_text_callback() -> Option<RustLoadFileTextCallback> {
    debug_assert!(size_of::<RustLoadFileTextCallback>() == size_of::<usize>());
    unsafe { transmute(LOAD_FILE_TEXT_CALLBACK.load(Ordering::Relaxed)) }
}

fn audio_stream_callback() -> Option<RustAudioStreamCallback> {
    debug_assert!(size_of::<RustAudioStreamCallback>() == size_of::<usize>());
    unsafe { transmute(AUDIO_STREAM_CALLBACK.load(Ordering::Relaxed)) }
}

#[no_mangle]
pub unsafe extern "C" fn custom_trace_log_callback(level: TraceLogLevel, text: *const c_char) {
    if let Some(trace_log) = trace_log_callback() {
        let text = if text.is_null() {
            Cow::Borrowed("(MESSAGE WAS NULL)")
        } else {
            unsafe { CStr::from_ptr(text).to_string_lossy() }
        };

        trace_log(level, &text)
    }
}

extern "C" fn custom_save_file_data_callback(
    path: *const c_char,
    buffer: *mut c_void,
    size: c_int,
) -> bool {
    let save_file_data = save_file_data_callback().expect("no callback");
    let path = unsafe { CStr::from_ptr(path) };
    let buffer = unsafe { from_raw_parts_mut(buffer as *mut u8, size as usize) };

    save_file_data(path.to_str().expect("path is non utf-8"), buffer)
}

extern "C" fn custom_load_file_data_callback(path: *const c_char, size: *mut c_int) -> *mut u8 {
    let load_file_data = load_file_data_callback().expect("no callback");

    if let Some(size) = unsafe { size.as_mut() } {
        let path = unsafe { CStr::from_ptr(path) };
        let buffer = load_file_data(path.to_str().expect("path is non utf-8"));
        *size = buffer.len().try_into().expect("out of range buffer size");

        // Copy everything to the raylib world
        unsafe {
            let buffer_ffi =
                ffi::MemAlloc((*size).try_into().expect("non representable buffer size"))
                    as *mut u8;
            buffer_ffi.copy_from_nonoverlapping(buffer.as_ptr(), buffer.len());

            buffer_ffi
        }
    } else {
        null_mut()
    }
}

extern "C" fn custom_save_file_text_callback(a: *const c_char, b: *mut c_char) -> bool {
    let save_file_text = save_file_text_callback().unwrap();
    let a = unsafe { CStr::from_ptr(a) };
    let b = unsafe { CStr::from_ptr(b) };
    return save_file_text(a.to_str().unwrap(), b.to_str().unwrap());
}
extern "C" fn custom_load_file_text_callback(a: *const c_char) -> *mut c_char {
    let load_file_text = load_file_text_callback().unwrap();
    let a = unsafe { CStr::from_ptr(a) };
    let st = load_file_text(a.to_str().unwrap());
    let oh = Box::leak(Box::new(CString::new(st).unwrap()));
    oh.as_ptr() as *mut c_char
}

extern "C" fn custom_audio_stream_callback(a: *mut c_void, b: u32) {
    let audio_stream = audio_stream_callback().unwrap();
    let a = unsafe { std::slice::from_raw_parts(a as *mut u8, b as usize) };
    audio_stream(a);
}
#[derive(Debug)]
pub struct SetLogError<'a>(&'a str);

impl<'a> std::fmt::Display for SetLogError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("There is a {} callback already set.", self.0))
    }
}

impl<'a> std::error::Error for SetLogError<'a> {}

macro_rules! safe_callback_set_func {
    ($cb:expr, $target_cb:expr, $rawsetter:expr, $ogfunc:expr, $ty:literal) => {
        if $target_cb.load(Ordering::Acquire) == 0 {
            $target_cb.store($cb as usize, Ordering::Release);
            unsafe { $rawsetter(Some($ogfunc)) };
            Ok(())
        } else {
            Err(SetLogError($ty))
        }
    };
}

/// Set custom trace log
pub fn set_trace_log_callback<'a>(cb: fn(TraceLogLevel, &str)) -> Result<(), SetLogError<'a>> {
    TRACE_LOG_CALLBACK.store(cb as usize, Ordering::Relaxed);
    unsafe { ffi::setLogCallbackWrapper() };
    Ok(())
}
/// Set custom file binary data saver
pub fn set_save_file_data_callback<'a>(cb: fn(&str, &[u8]) -> bool) -> Result<(), SetLogError<'a>> {
    safe_callback_set_func!(
        cb,
        SAVE_FILE_DATA_CALLBACK,
        ffi::SetSaveFileDataCallback,
        custom_save_file_data_callback,
        "save file data"
    )
}
/// Set custom file binary data loader
///
/// Whatever you return from your callback will be intentionally leaked as Raylib is relied on to free it.
pub fn set_load_file_data_callback<'b>(cb: fn(&str) -> Vec<u8>) -> Result<(), SetLogError<'b>> {
    safe_callback_set_func!(
        cb,
        LOAD_FILE_DATA_CALLBACK,
        ffi::SetLoadFileDataCallback,
        custom_load_file_data_callback,
        "load file data"
    )
}
/// Set custom file text data saver
pub fn set_save_file_text_callback<'a>(cb: fn(&str, &str) -> bool) -> Result<(), SetLogError<'a>> {
    safe_callback_set_func!(
        cb,
        SAVE_FILE_TEXT_CALLBACK,
        ffi::SetSaveFileTextCallback,
        custom_save_file_text_callback,
        "load file data"
    )
}
/// Set custom file text data loader
///
/// Whatever you return from your callback will be intentionally leaked as Raylib is relied on to free it.
pub fn set_load_file_text_callback<'a>(cb: fn(&str) -> String) -> Result<(), SetLogError<'a>> {
    safe_callback_set_func!(
        cb,
        LOAD_FILE_TEXT_CALLBACK,
        ffi::SetLoadFileTextCallback,
        custom_load_file_text_callback,
        "load file text"
    )
}

// region: -- AudioStreamProcessorCallback --

/// This struct encapsulates a rust callback
/// and guarantees the lifetime to be long enough ('a)
/// (once `get_as_user_data` is called, it the struct
/// should not be moved again! -> use Pin<..>)
pub struct AudioStreamProcessorCallback<'a, F>
where
    F: FnMut(&mut [f32], u32) -> (),
{
    rust_callback: &'a mut F,
    nb_channels: u32,
    callback_index: Option<usize>,
}

impl<'a, F> AudioStreamProcessorCallback<'a, F>
where
    F: FnMut(&mut [f32], u32) -> (),
{
    fn new(closure: &'a mut F, nb_channels_from_music: u32) -> Self {
        Self {
            rust_callback: closure,
            nb_channels: nb_channels_from_music,
            callback_index: None,
        }
    }

    fn get_as_user_data(&mut self) -> *mut ::std::os::raw::c_void {
        return self as *mut Self as *mut ::std::os::raw::c_void;
    }

    fn get_c_callback(
        &mut self,
    ) -> extern "C" fn(
        *mut ::std::os::raw::c_void,
        *mut ::std::os::raw::c_void,
        ::std::os::raw::c_uint,
    ) -> () {
        Self::c_callback
    }

    extern "C" fn c_callback(
        user_data: *mut ::std::os::raw::c_void,
        data_ptr: *mut ::std::os::raw::c_void,
        frame_count: ::std::os::raw::c_uint,
    ) -> () {
        unsafe {
            let stream_processor_callback: &mut Self = user_data.cast::<Self>().as_mut().unwrap();
            let f32_ptr = data_ptr as *mut f32;
            let data = unsafe {
                std::slice::from_raw_parts_mut(
                    f32_ptr,
                    frame_count as usize * stream_processor_callback.nb_channels as usize,
                )
            };
            (stream_processor_callback.rust_callback)(data, stream_processor_callback.nb_channels);
        }
    }
}

impl<'a, F> Drop for AudioStreamProcessorCallback<'a, F>
where
    F: FnMut(&mut [f32], u32) -> (),
{
    fn drop(&mut self) {
        if let Some(index) = self.callback_index {
            detach_audio_stream_processor_with_user_data(index);
        }
    }
}

// endregion: -- AudioStreamProcessorCallback --

pub fn attach_audio_stream_processor_to_music<'a, F>(
    music: &'a Music<'a>,
    processor: &'a mut F,
) -> Pin<Box<AudioStreamProcessorCallback<'a, F>>>
where
    F: FnMut(&mut [f32], u32) -> () + Send + 'static, // static because the function is executed in another thread
{
    let mut stream_processor_callback =
        Box::new(AudioStreamProcessorCallback::<'a, F>::new(processor, 2));
    stream_processor_callback.callback_index = Some(attach_audio_stream_processor_with_user_data(
        music.stream,
        AudioCallbackWithUserData::new(
            stream_processor_callback.get_as_user_data(), // pass the address of the stream_processor_callback as void*
            stream_processor_callback.get_c_callback(),
        ),
    ));
    assert!(stream_processor_callback.callback_index.is_some());
    Box::into_pin(stream_processor_callback)
}

/// Audio thread callback to request new data
pub fn set_audio_stream_callback(stream: AudioStream, cb: fn(&[u8])) -> Result<(), SetLogError> {
    if AUDIO_STREAM_CALLBACK.load(Ordering::Acquire) == 0 {
        AUDIO_STREAM_CALLBACK.store(cb as _, Ordering::Release);
        unsafe { ffi::SetAudioStreamCallback(stream.0, Some(custom_audio_stream_callback)) }
        Ok(())
    } else {
        Err(SetLogError("audio stream"))
    }
}

impl RaylibHandle {
    /// Set custom trace log
    #[deprecated = "Decoupled from RaylibHandle. Use [set_trace_log_callback](core::callbacks::set_trace_log_callback) instead."]
    pub fn set_trace_log_callback(
        &mut self,
        cb: fn(TraceLogLevel, &str),
    ) -> Result<(), SetLogError> {
        set_trace_log_callback(cb)
    }
    /// Set custom file binary data saver
    #[deprecated = "Decoupled from RaylibHandle. Use [set_save_file_data_callback](core::callbacks::set_save_file_data_callback) instead."]
    pub fn set_save_file_data_callback(
        &mut self,
        cb: fn(&str, &[u8]) -> bool,
    ) -> Result<(), SetLogError> {
        set_save_file_data_callback(cb)
    }
    /// Set custom file binary data loader
    ///
    /// Whatever you return from your callback will be intentionally leaked as Raylib is relied on to free it.
    #[deprecated = "Decoupled from RaylibHandle. Use [set_load_file_data_callback](core::callbacks::set_load_file_data_callback) instead."]
    pub fn set_load_file_data_callback<'b>(
        &mut self,
        cb: fn(&str) -> Vec<u8>,
    ) -> Result<(), SetLogError> {
        set_load_file_data_callback(cb)
    }
    /// Set custom file text data saver
    #[deprecated = "Decoupled from RaylibHandle. Use [set_save_file_text_callback](core::callbacks::set_save_file_text_callback) instead."]
    pub fn set_save_file_text_callback(
        &mut self,
        cb: fn(&str, &str) -> bool,
    ) -> Result<(), SetLogError> {
        set_save_file_text_callback(cb)
    }
    /// Set custom file text data loader
    ///
    /// Whatever you return from your callback will be intentionally leaked as Raylib is relied on to free it.
    #[deprecated = "Decoupled from RaylibHandle. Use [set_load_file_text_callback](core::callbacks::set_load_file_text_callback) instead."]
    pub fn set_load_file_text_callback(
        &mut self,
        cb: fn(&str) -> String,
    ) -> Result<(), SetLogError> {
        set_load_file_text_callback(cb)
    }

    /// Audio thread callback to request new data
    #[deprecated = "Decoupled from RaylibHandle. Use [set_audio_stream_callback](core::callbacks::set_audio_stream_callback) instead."]
    pub fn set_audio_stream_callback(
        &mut self,
        stream: AudioStream,
        cb: fn(&[u8]),
    ) -> Result<(), SetLogError> {
        if AUDIO_STREAM_CALLBACK.load(Ordering::Acquire) == 0 {
            AUDIO_STREAM_CALLBACK.store(cb as _, Ordering::Release);
            unsafe { ffi::SetAudioStreamCallback(stream.0, Some(custom_audio_stream_callback)) }
            Ok(())
        } else {
            Err(SetLogError("audio stream"))
        }
    }
}
