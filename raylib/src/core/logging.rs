///! Functions to change the behavior of raylib logging.
// TODO: refactor this entire thing to use log
use crate::consts::TraceLogLevel;
use crate::{ffi, RaylibHandle};
use std::ffi::CString;

impl RaylibHandle {
    /// Set the current threshold (minimum) log level
    #[inline]
    pub fn set_trace_log(&self, types: TraceLogLevel) {
        unsafe {
            ffi::SetTraceLogLevel((types as u32) as i32);
        }
    }

    /// Writes a trace log message (`Log::INFO`, `Log::WARNING`, `Log::ERROR`, `Log::DEBUG`).
    #[inline]
    pub fn trace_log(&self, msg_type: TraceLogLevel, text: &str) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::TraceLog((msg_type as u32) as i32, text.as_ptr());
        }
    }
}
