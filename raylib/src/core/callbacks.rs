use crate::{ffi, RaylibHandle};
use libc::{c_char, c_int};
use parking_lot::Mutex;
use raylib_sys::{TraceLogLevel, __va_list_tag};
use std::ffi::CStr;

extern "C" {
    fn sprintf(fmt: *const c_char, ...) -> c_int;
}

type RustTraceLogCallback = Option<fn(TraceLogLevel, &str)>;
static mut __TRACE_LOG_CALLBACK: Mutex<RustTraceLogCallback> = Mutex::new(None);
fn trace_log_callback() -> RustTraceLogCallback {
    unsafe { *__TRACE_LOG_CALLBACK.lock() }
}

fn set_trace_log_callback(f: RustTraceLogCallback) {
    unsafe { *__TRACE_LOG_CALLBACK.lock() = f }
}

extern "C" fn custom_trace_log_callback(
    log_level: ::std::os::raw::c_int,
    text: *const ::std::os::raw::c_char,
    args: *mut __va_list_tag,
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
            _ => panic!("raylib gave invalid log level {}", log_level),
        };
        let b = if text.is_null() {
            CStr::from_bytes_until_nul("(MESSAGE WAS NULL)\0".as_bytes()).unwrap()
        } else {
            const MAX_TRACELOG_MSG_LENGTH: usize = 386; // chosen because 256 is the max length in raylib's own code and 386 is a bit higher then that.
            let mut buf: [i8; MAX_TRACELOG_MSG_LENGTH] = [0; MAX_TRACELOG_MSG_LENGTH];

            unsafe { sprintf(buf.as_mut_ptr(), text, args) };

            unsafe { CStr::from_ptr(buf.as_ptr()) }
        };

        trace_log(a, b.to_str().unwrap());
    }
}

#[derive(Debug)]
pub struct TraceLogError;

impl std::fmt::Display for TraceLogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("There is a trace log callback already set.")
    }
}

impl std::error::Error for TraceLogError {}

impl RaylibHandle {
    pub fn set_trace_log_callback(
        &mut self,
        cb: fn(TraceLogLevel, &str),
    ) -> Result<(), TraceLogError> {
        if let None = trace_log_callback() {
            set_trace_log_callback(Some(cb));
            unsafe {
                ffi::SetTraceLogCallback(Some(custom_trace_log_callback));
            }
            Ok(())
        } else {
            return Err(TraceLogError);
        }
    }
}
