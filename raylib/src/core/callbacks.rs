#![allow(non_camel_case_types)]

use crate::{audio::AudioStream, ffi, RaylibHandle};
use libc::c_void;
use parking_lot::Mutex;
use raylib_sys::TraceLogLevel;
use std::{
    ffi::{CStr, CString},
    ptr,
};

type TraceLogCallback = unsafe extern "C" fn(*mut i8, *const i8, ...);
extern "C" {
    fn SetTraceLogCallback(cb: Option<TraceLogCallback>);
}

type RustTraceLogCallback = Option<fn(TraceLogLevel, &str)>;
type RustSaveFileDataCallback = Option<fn(&str, &[u8]) -> bool>;
type RustLoadFileDataCallback = Option<fn(&str) -> Vec<u8>>;
type RustSaveFileTextCallback = Option<fn(&str, &str) -> bool>;
type RustLoadFileTextCallback = Option<fn(&str) -> String>;
type RustAudioStreamCallback = Option<fn(&[u8])>;

static __TRACE_LOG_CALLBACK: Mutex<RustTraceLogCallback> = Mutex::new(None);
static __SAVE_FILE_DATA_CALLBACK: Mutex<RustSaveFileDataCallback> = Mutex::new(None);
static __LOAD_FILE_DATA_CALLBACK: Mutex<RustLoadFileDataCallback> = Mutex::new(None);
static __SAVE_FILE_TEXT_CALLBACK: Mutex<RustSaveFileTextCallback> = Mutex::new(None);
static __LOAD_FILE_TEXT_CALLBACK: Mutex<RustLoadFileTextCallback> = Mutex::new(None);
static __AUDIO_STREAM_CALLBACK: Mutex<RustAudioStreamCallback> = Mutex::new(None);

fn trace_log_callback() -> RustTraceLogCallback {
    *__TRACE_LOG_CALLBACK.lock()
}
fn set_trace_log_callback(f: RustTraceLogCallback) {
    *__TRACE_LOG_CALLBACK.lock() = f
}
fn save_file_data_callback() -> RustSaveFileDataCallback {
    *__SAVE_FILE_DATA_CALLBACK.lock()
}
fn set_save_file_data_callback(f: RustSaveFileDataCallback) {
    *__SAVE_FILE_DATA_CALLBACK.lock() = f
}
fn load_file_data_callback() -> RustLoadFileDataCallback {
    *__LOAD_FILE_DATA_CALLBACK.lock()
}
fn set_load_file_data_callback(f: RustLoadFileDataCallback) {
    *__LOAD_FILE_DATA_CALLBACK.lock() = f
}

fn save_file_text_callback() -> RustSaveFileTextCallback {
    *__SAVE_FILE_TEXT_CALLBACK.lock()
}
fn set_save_file_text_callback(f: RustSaveFileTextCallback) {
    *__SAVE_FILE_TEXT_CALLBACK.lock() = f
}
fn load_file_text_callback() -> RustLoadFileTextCallback {
    *__LOAD_FILE_TEXT_CALLBACK.lock()
}
fn set_load_file_text_callback(f: RustLoadFileTextCallback) {
    *__LOAD_FILE_TEXT_CALLBACK.lock() = f
}

fn audio_stream_callback() -> RustAudioStreamCallback {
    *__AUDIO_STREAM_CALLBACK.lock()
}
fn set_audio_stream_callback(f: RustAudioStreamCallback) {
    *__AUDIO_STREAM_CALLBACK.lock() = f
}

#[no_mangle]
#[link_name = "custom_trace_log_callback"]
pub extern "C" fn custom_trace_log_callback(
    log_level: ::std::os::raw::c_int,
    text: *const ::std::os::raw::c_char,
) {
    if let Some(trace_log) = trace_log_callback() {
        let a = match log_level {
            0 => TraceLogLevel::LOG_ALL,
            1 => TraceLogLevel::LOG_TRACE,
            2 => TraceLogLevel::LOG_DEBUG,
            3 => TraceLogLevel::LOG_INFO,
            4 => TraceLogLevel::LOG_WARNING,
            5 => TraceLogLevel::LOG_ERROR,
            6 => TraceLogLevel::LOG_FATAL,
            7 => TraceLogLevel::LOG_NONE,
            _ => unreachable!("raylib gave invalid log level {}", log_level),
        };
        let b = if text.is_null() {
            CStr::from_bytes_until_nul("(MESSAGE WAS NULL)\0".as_bytes()).unwrap()
        } else {
            unsafe { CStr::from_ptr(text) }
        };

        trace_log(a, b.to_string_lossy().as_ref())
    }
}

extern "C" fn custom_save_file_data_callback(a: *const i8, b: *mut c_void, c: i32) -> bool {
    let save_file_data = save_file_data_callback().unwrap();
    let a = unsafe { CStr::from_ptr(a) };
    let b = unsafe { std::slice::from_raw_parts_mut(b as *mut u8, c as usize) };
    return save_file_data(a.to_str().unwrap(), b);
}

extern "C" fn custom_load_file_data_callback(a: *const i8, b: *mut i32) -> *mut u8 {
    let load_file_data = load_file_data_callback().unwrap();
    let a = unsafe { CStr::from_ptr(a) };
    let b = unsafe { b.as_mut().unwrap() };
    let d = load_file_data(a.to_str().unwrap());
    *b = d.len() as i32;
    if *b == 0 {
        return ptr::null_mut();
    } else {
        // Leak the data that we just created. It's in Raylib's hands now.
        let uh = Box::leak(Box::new(d)).as_mut_ptr();
        return uh;
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
    ($cb:expr, $getter:expr, $setter:expr, $rawsetter:expr, $ogfunc:expr, $ty:literal) => {
        if let None = $getter() {
            $setter(Some($cb));
            unsafe {
                $rawsetter(Some($ogfunc));
            }
            return Ok(());
        } else {
            return Err(SetLogError($ty));
        }
    };
}

impl RaylibHandle {
    /// Set custom trace log
    pub fn set_trace_log_callback(
        &mut self,
        cb: fn(TraceLogLevel, &str),
    ) -> Result<(), SetLogError> {
        set_trace_log_callback(Some(cb));
        unsafe { ffi::setLogCallbackWrapper() };
        return Ok(());
    }
    /// Set custom file binary data saver
    pub fn set_save_file_data_callback(
        &mut self,
        cb: fn(&str, &[u8]) -> bool,
    ) -> Result<(), SetLogError> {
        safe_callback_set_func!(
            cb,
            save_file_data_callback,
            set_save_file_data_callback,
            ffi::SetSaveFileDataCallback,
            custom_save_file_data_callback,
            "save file data"
        );
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
            load_file_data_callback,
            set_load_file_data_callback,
            ffi::SetLoadFileDataCallback,
            custom_load_file_data_callback,
            "load file data"
        );
    }
    /// Set custom file text data saver
    pub fn set_save_file_text_callback(
        &mut self,
        cb: fn(&str, &str) -> bool,
    ) -> Result<(), SetLogError> {
        safe_callback_set_func!(
            cb,
            save_file_text_callback,
            set_save_file_text_callback,
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
            load_file_text_callback,
            set_load_file_text_callback,
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
        if let None = audio_stream_callback() {
            set_audio_stream_callback(Some(cb));
            unsafe {
                ffi::SetAudioStreamCallback(stream.0, Some(custom_audio_stream_callback));
            }
            return Ok(());
        } else {
            return Err(SetLogError("audio stream"));
        }
    }
}
