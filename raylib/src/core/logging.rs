//! Functions to change the behavior of raylib logging.
use crate::consts::TraceLogType;
use crate::ffi;
use lazy_static::lazy_static;
use std::ffi::{ CStr, CString };
use std::os::raw::{ c_char, c_int };
use std::sync::Mutex;

lazy_static! {
    static ref TRACE_LOG_CALLBACK: Mutex<Option<Box<dyn FnMut(i32, &str) + Send>>> = Default::default();
}

/// Set the current threshold (minimum) log level
#[inline]
pub fn set_trace_log(types: TraceLogType) {
    unsafe {
        ffi::SetTraceLogLevel((types as u32) as i32);
    }
}

/// Set the exit threshold (minimum) log level
#[inline]
pub fn set_trace_log_exit(types: TraceLogType) {
    unsafe {
        ffi::SetTraceLogExit((types as u32) as i32);
    }
}

#[inline]
pub fn set_trace_log_callback(c: impl FnMut(i32, &str) + Send + 'static) {
    unsafe {
        *TRACE_LOG_CALLBACK.lock().unwrap() = Some(Box::new(c));
        ffi::set_trace_log_callback(callback_wrapper);
    }
}

/// Writes a trace log message (`Log::INFO`, `Log::WARNING`, `Log::ERROR`, `Log::DEBUG`).
#[inline]
pub fn trace_log(msg_type: TraceLogType, text: &str) {
    unsafe {
        let text = CString::new(text).unwrap();
        ffi::TraceLog((msg_type as u32) as i32, text.as_ptr());
    }
}

extern "C" fn callback_wrapper(i: c_int, s: *mut c_char) {
    if let Some(ref mut c) = *TRACE_LOG_CALLBACK.lock().unwrap() {
        let str = unsafe { CStr::from_ptr(s) };
        c(i as i32, str.to_str().unwrap());
    } else {
        panic!("no trace log callback set")
    }
}

#[cfg(test)]
mod test_logging {
    use super::*;
    #[test]
    fn test_logs() {
        use crate::consts::TraceLogType::*;
        set_trace_log(LOG_ALL);
        trace_log(LOG_DEBUG, "This Is From `test_logs`");
        set_trace_log(LOG_INFO);
    }
}
