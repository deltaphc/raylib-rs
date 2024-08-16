#![allow(non_camel_case_types)]

use crate::{audio::AudioStream, ffi, RaylibHandle};
pub use raylib_sys::TraceLogLevel;
use std::{
    borrow::Cow,
    convert::TryInto,
    ffi::{c_char, c_int, c_void, CStr, CString},
    mem::{size_of, transmute},
    ptr::null_mut,
    slice::from_raw_parts_mut,
    sync::atomic::{AtomicUsize, Ordering},
};

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
    const { assert!(size_of::<RustTraceLogCallback>() == size_of::<usize>()) };
    unsafe { transmute(TRACE_LOG_CALLBACK.load(Ordering::Relaxed)) }
}

fn save_file_data_callback() -> Option<RustSaveFileDataCallback> {
    const { assert!(size_of::<RustSaveFileDataCallback>() == size_of::<usize>()) };
    unsafe { transmute(SAVE_FILE_DATA_CALLBACK.load(Ordering::Relaxed)) }
}

fn load_file_data_callback() -> Option<RustLoadFileDataCallback> {
    const { assert!(size_of::<RustLoadFileDataCallback>() == size_of::<usize>()) };
    unsafe { transmute(LOAD_FILE_DATA_CALLBACK.load(Ordering::Relaxed)) }
}

fn save_file_text_callback() -> Option<RustSaveFileTextCallback> {
    const { assert!(size_of::<RustSaveFileTextCallback>() == size_of::<usize>()) };
    unsafe { transmute(SAVE_FILE_TEXT_CALLBACK.load(Ordering::Relaxed)) }
}

fn load_file_text_callback() -> Option<RustLoadFileTextCallback> {
    const { assert!(size_of::<RustLoadFileTextCallback>() == size_of::<usize>()) };
    unsafe { transmute(LOAD_FILE_TEXT_CALLBACK.load(Ordering::Relaxed)) }
}

fn audio_stream_callback() -> Option<RustAudioStreamCallback> {
    const { assert!(size_of::<RustAudioStreamCallback>() == size_of::<usize>()) };
    unsafe { transmute(AUDIO_STREAM_CALLBACK.load(Ordering::Relaxed)) }
}

#[no_mangle]
pub extern "C" fn custom_trace_log_callback(level: TraceLogLevel, text: *const c_char) {
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

extern "C" fn custom_save_file_text_callback(a: *const i8, b: *mut i8) -> bool {
    let save_file_text = save_file_text_callback().unwrap();
    let a = unsafe { CStr::from_ptr(a) };
    let b = unsafe { CStr::from_ptr(b) };
    return save_file_text(a.to_str().unwrap(), b.to_str().unwrap());
}
extern "C" fn custom_load_file_text_callback(a: *const i8) -> *mut i8 {
    let load_file_text = load_file_text_callback().unwrap();
    let a = unsafe { CStr::from_ptr(a) };
    let st = load_file_text(a.to_str().unwrap());
    let oh = Box::leak(Box::new(CString::new(st).unwrap()));
    oh.as_ptr() as *mut i8
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

impl RaylibHandle {
    /// Set custom trace log
    pub fn set_trace_log_callback(
        &mut self,
        cb: fn(TraceLogLevel, &str),
    ) -> Result<(), SetLogError> {
        TRACE_LOG_CALLBACK.store(cb as usize, Ordering::Relaxed);
        unsafe { ffi::setLogCallbackWrapper() };
        Ok(())
    }
    /// Set custom file binary data saver
    pub fn set_save_file_data_callback(
        &mut self,
        cb: fn(&str, &[u8]) -> bool,
    ) -> Result<(), SetLogError> {
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
    pub fn set_load_file_data_callback<'b>(
        &mut self,
        cb: fn(&str) -> Vec<u8>,
    ) -> Result<(), SetLogError> {
        safe_callback_set_func!(
            cb,
            LOAD_FILE_DATA_CALLBACK,
            ffi::SetLoadFileDataCallback,
            custom_load_file_data_callback,
            "load file data"
        )
    }
    /// Set custom file text data saver
    pub fn set_save_file_text_callback(
        &mut self,
        cb: fn(&str, &str) -> bool,
    ) -> Result<(), SetLogError> {
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
    pub fn set_load_file_text_callback(
        &mut self,
        cb: fn(&str) -> String,
    ) -> Result<(), SetLogError> {
        safe_callback_set_func!(
            cb,
            LOAD_FILE_TEXT_CALLBACK,
            ffi::SetLoadFileTextCallback,
            custom_load_file_text_callback,
            "load file text"
        )
    }

    /// Audio thread callback to request new data
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
